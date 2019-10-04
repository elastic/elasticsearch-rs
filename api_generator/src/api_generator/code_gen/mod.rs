pub mod enums;
pub mod namespace_clients;
pub mod root;
pub mod url;

use crate::api_generator::{Type, TypeKind};
use inflector::Inflector;
use quote::Tokens;
use std::str;

/// AST for a literal
fn lit<I: Into<String>>(lit: I) -> syn::Lit {
    syn::Lit::Str(lit.into(), syn::StrStyle::Cooked)
}

/// AST for an identifier
fn ident<I: AsRef<str>>(name: I) -> syn::Ident {
    syn::Ident::from(name.as_ref())
}

/// AST for doc attribute
fn doc(comment: String) -> syn::Attribute {
    syn::Attribute {
        style: syn::AttrStyle::Outer,
        value: syn::MetaItem::NameValue(ident("doc".to_string()), lit(comment)),
        is_sugared_doc: true,
    }
}

/// AST for an expression parsed from quoted tokens
pub fn parse_expr(input: quote::Tokens) -> syn::Expr {
    syn::parse_expr(input.to_string().as_ref()).unwrap()
}

/// Ensures that the name generated is one that is valid for Rust
fn valid_name(s: &str) -> &str {
    match s {
        "type" => "ty",
        s => s,
    }
}

/// AST for a simple path variable.
fn path_none(path_ident: &str) -> syn::Path {
    path(path_ident, vec![], vec![])
}

/// AST for a path variable.
fn path(path: &str, lifetimes: Vec<syn::Lifetime>, types: Vec<syn::Ty>) -> syn::Path {
    path_segments(vec![(path, lifetimes, types)])
}

/// AST for a path variable.
fn path_segments(paths: Vec<(&str, Vec<syn::Lifetime>, Vec<syn::Ty>)>) -> syn::Path {
    syn::Path {
        global: false,
        segments: paths
            .into_iter()
            .map(|(path, lifetimes, types)| syn::PathSegment {
                ident: syn::Ident::new(valid_name(path)),
                parameters: syn::PathParameters::AngleBracketed(syn::AngleBracketedParameterData {
                    lifetimes,
                    types,
                    bindings: vec![],
                }),
            })
            .collect(),
    }
}

fn ty(name: &str, kind: &TypeKind) -> syn::Ty {
    match kind {
        TypeKind::None => syn::parse_type("Option<String>").unwrap(),
        TypeKind::List => syn::parse_type("Option<Vec<String>>").unwrap(),
        TypeKind::Enum => {
            let mut v = String::from("Option<");
            v.push_str(name.to_pascal_case().as_str());
            v.push_str(">");
            syn::parse_type(v.as_str()).unwrap()
        }
        TypeKind::String => syn::parse_type("Option<String>").unwrap(),
        TypeKind::Text => syn::parse_type("Option<String>").unwrap(),
        TypeKind::Boolean => syn::parse_type("Option<bool>").unwrap(),
        TypeKind::Number => syn::parse_type("Option<i64>").unwrap(),
        TypeKind::Float => syn::parse_type("Option<f32>").unwrap(),
        TypeKind::Double => syn::parse_type("Option<f64>").unwrap(),
        TypeKind::Integer => syn::parse_type("Option<i32>").unwrap(),
        TypeKind::Long => syn::parse_type("Option<i64>").unwrap(),
        TypeKind::Date => syn::parse_type("Option<String>").unwrap(),
        TypeKind::Time => syn::parse_type("Option<String>").unwrap(),
    }
}

fn create_field(f: (&String, &Type)) -> syn::Field {
    syn::Field {
        ident: Some(ident(valid_name(f.0).to_lowercase())),
        vis: syn::Visibility::Inherited,
        attrs: vec![],
        ty: ty(&f.0, &f.1.ty),
    }
}

fn create_builder_method(f: (&String, &Type)) -> Tokens {
    let name = ident(valid_name(f.0).to_lowercase());
    let value = ty(&f.0, &f.1.ty);
    let doc = match &f.1.description {
        Some(docs) => Some(doc(docs.into())),
        _ => None,
    };

    quote!(
        #doc
        pub fn #name(mut self, #name: #value) -> Self {
            self.#name = #name;
            self
        }
    )
}

pub fn shift_while<F>(i: &[u8], f: F) -> &[u8]
where
    F: Fn(u8) -> bool,
{
    let mut ctr = 0;
    for c in i {
        if f(*c) {
            ctr += 1;
        } else {
            break;
        }
    }

    &i[ctr..]
}

pub fn take_while<F>(i: &[u8], f: F) -> (&[u8], &str)
where
    F: Fn(u8) -> bool,
{
    let mut ctr = 0;

    for c in i {
        if f(*c) {
            ctr += 1;
        } else {
            break;
        }
    }

    (&i[ctr..], str::from_utf8(&i[0..ctr]).unwrap())
}

pub fn take_while1<F>(i: &[u8], f: F) -> (&[u8], &str)
where
    F: Fn(u8) -> bool,
{
    let mut ctr = 0;

    for c in i {
        if f(*c) || ctr == 0 {
            ctr += 1;
        } else {
            break;
        }
    }

    (&i[ctr..], str::from_utf8(&i[0..ctr]).unwrap())
}

pub fn shift(i: &[u8], c: usize) -> &[u8] {
    match c {
        c if c >= i.len() => &[],
        _ => &i[c..],
    }
}
