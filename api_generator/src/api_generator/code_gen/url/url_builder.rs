// From https://github.com/elastic-rs/elastic
// Licensed under Apache 2.0: https://github.com/elastic-rs/elastic/blob/51298dd64278f34d2db911bd1a35eb757c336198/LICENSE-APACHE

use std::fmt;
use std::iter::Iterator;
use std::str;
use syn;

use serde::{Deserialize, Deserializer};

use crate::api_generator::code_gen::*;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Path(#[serde(deserialize_with = "rooted_path_string")] pub String);

// Ensure all deserialized paths have a leading `/`
fn rooted_path_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if !s.starts_with('/') {
        Ok(format!("/{}", s))
    } else {
        Ok(s)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Path {
    pub fn split(&self) -> Vec<PathPart> {
        Path::parse(self.0.as_bytes(), PathParseState::Literal, Vec::new())
    }

    pub fn params(&self) -> Vec<&str> {
        self.split()
            .iter()
            .filter_map(|p| match *p {
                PathPart::Param(p) => Some(p),
                _ => None,
            })
            .collect()
    }

    fn parse<'a>(i: &'a [u8], state: PathParseState, r: Vec<PathPart<'a>>) -> Vec<PathPart<'a>> {
        if i.len() == 0 {
            return r;
        }

        let mut r = r;
        match state {
            PathParseState::Literal => {
                let (rest, part) = Path::parse_literal(i);
                if part.len() > 0 {
                    r.push(PathPart::Literal(part));
                }

                Path::parse(rest, PathParseState::Param, r)
            }
            PathParseState::Param => {
                let (rest, part) = Path::parse_param(i);
                if part.len() > 0 {
                    r.push(PathPart::Param(part));
                }

                Path::parse(rest, PathParseState::Literal, r)
            }
        }
    }

    fn parse_literal(i: &[u8]) -> (&[u8], &str) {
        if i[0] == b'}' {
            let i = shift(i, 1);
            take_while(i, |c| c != b'{')
        } else {
            take_while(i, |c| c != b'{')
        }
    }

    fn parse_param(i: &[u8]) -> (&[u8], &str) {
        if i[0] == b'{' {
            let i = shift(i, 1);
            take_while(i, |c| c != b'}')
        } else {
            take_while(i, |c| c != b'}')
        }
    }
}

enum PathParseState {
    Literal,
    Param,
}

#[derive(Debug, PartialEq)]
pub enum PathPart<'a> {
    Literal(&'a str),
    Param(&'a str),
}

pub trait PathParams<'a> {
    fn params(&'a self) -> Vec<&'a str>;
}

impl<'a> PathParams<'a> for Vec<PathPart<'a>> {
    fn params(&'a self) -> Vec<&'a str> {
        self.iter()
            .filter_map(|p| match *p {
                PathPart::Param(p) => Some(p),
                _ => None,
            })
            .collect()
    }
}

/// Builder for an efficient url value replacer.
///
/// The inputs are expected to be `AsRef<str>` and the output is a `UrlPath<'a>`.
struct ReplaceBuilder<'a> {
    url: Vec<PathPart<'a>>,
}

impl<'a> ReplaceBuilder<'a> {
    pub fn new(url: Vec<PathPart<'a>>) -> Self {
        ReplaceBuilder { url: url }
    }

    /// Build an allocated url from the path literals and params.
    fn build_owned(self) -> syn::Block {
        let lit_len_expr = Self::literal_length_expr(&self.url);

        let mut params_len_exprs = Self::parameter_length_exprs(&self.url);

        let mut len_exprs = vec![lit_len_expr];
        len_exprs.append(&mut params_len_exprs);

        let len_expr = Self::summed_length_expr(len_exprs);

        let url_ident = ident("url");
        let url_ty = ident("UrlPath");

        let let_stmt = Self::let_url_stmt(url_ident.clone(), len_expr);

        let mut push_stmts = Self::push_part_stmts(url_ident.clone(), &self.url);

        let return_expr = syn::Stmt::Expr(Box::new(parse_expr(quote!(#url_ty ::from(#url_ident)))));

        let mut stmts = vec![let_stmt];

        stmts.append(&mut push_stmts);

        stmts.push(return_expr);

        syn::Block { stmts: stmts }
    }

    /// Build a non-allocated url from the path literals.
    fn build_borrowed(self) -> syn::Block {
        let path: Vec<&'a str> = self
            .url
            .iter()
            .map(|p| match *p {
                PathPart::Literal(p) => p,
                _ => panic!("Only PathPart::Literal is supported by a borrowed url."),
            })
            .collect();

        let path = path.join("");

        let lit = syn::Lit::Str(path, syn::StrStyle::Cooked);
        let url_ty = ident("UrlPath");

        let expr = parse_expr(quote!(#url_ty ::from(#lit)));

        syn::Block {
            stmts: vec![syn::Stmt::Expr(Box::new(expr))],
        }
    }

    /// Get the number of chars in all literal parts for the url.
    fn literal_length_expr(url: &[PathPart<'a>]) -> syn::Expr {
        let len = url
            .iter()
            .filter_map(|p| match *p {
                PathPart::Literal(p) => Some(p),
                _ => None,
            })
            .fold(0, |acc, p| acc + p.len());

        syn::ExprKind::Lit(syn::Lit::Int(len as u64, syn::IntTy::Usize)).into()
    }

    /// Get an expression to find the number of chars in each parameter part for the url.
    fn parameter_length_exprs(url: &[PathPart<'a>]) -> Vec<syn::Expr> {
        url.iter()
            .filter_map(|p| match *p {
                PathPart::Param(p) => Some(
                    syn::ExprKind::MethodCall(
                        ident("len"),
                        vec![],
                        vec![syn::ExprKind::Path(None, path_none(p)).into()],
                    )
                    .into(),
                ),
                _ => None,
            })
            .collect()
    }

    /// Get an expression that is the binary addition of each of the given expressions.
    fn summed_length_expr(len_exprs: Vec<syn::Expr>) -> syn::Expr {
        match len_exprs.len() {
            1 => len_exprs.into_iter().next().unwrap(),
            _ => {
                let mut len_iter = len_exprs.into_iter();

                let first_expr = Box::new(len_iter.next().unwrap());

                *(len_iter.map(|p| Box::new(p)).fold(first_expr, |acc, p| {
                    Box::new(syn::ExprKind::Binary(syn::BinOp::Add, acc, p).into())
                }))
            }
        }
    }

    /// Get a statement to build a `String` with a capacity of the given expression.
    fn let_url_stmt(url_ident: syn::Ident, len_expr: syn::Expr) -> syn::Stmt {
        let string_with_capacity = syn::ExprKind::Call(
            Box::new(
                syn::ExprKind::Path(None, {
                    let mut method = path_none("String");
                    method
                        .segments
                        .push(syn::PathSegment::from("with_capacity"));
                    method
                })
                .into(),
            ),
            vec![len_expr],
        )
        .into();

        syn::Stmt::Local(Box::new(syn::Local {
            pat: Box::new(syn::Pat::Ident(
                syn::BindingMode::ByValue(syn::Mutability::Mutable),
                url_ident.to_owned(),
                None,
            )),
            ty: None,
            init: Some(Box::new(string_with_capacity)),
            attrs: vec![],
        }))
    }

    /// Get a list of statements that append each part to a `String` in order.
    fn push_part_stmts(url_ident: syn::Ident, url: &[PathPart<'a>]) -> Vec<syn::Stmt> {
        url.iter()
            .map(|p| match *p {
                PathPart::Literal(p) => {
                    let lit = syn::Lit::Str(p.to_string(), syn::StrStyle::Cooked);

                    syn::Stmt::Semi(Box::new(parse_expr(quote!(#url_ident.push_str(#lit)))))
                }
                PathPart::Param(p) => {
                    let ident = ident(p);

                    syn::Stmt::Semi(Box::new(parse_expr(
                        quote!(#url_ident.push_str(#ident.as_ref())),
                    )))
                }
            })
            .collect()
    }

    pub fn build(self) -> syn::Block {
        let has_params = self.url.iter().any(|p| match *p {
            PathPart::Param(_) => true,
            _ => false,
        });

        if has_params {
            self.build_owned()
        } else {
            self.build_borrowed()
        }
    }
}

impl<'a, I: IntoIterator<Item = PathPart<'a>>> From<I> for ReplaceBuilder<'a> {
    fn from(value: I) -> Self {
        ReplaceBuilder::new(value.into_iter().collect())
    }
}
