pub mod enums;
pub mod namespace_clients;
pub mod root;
pub mod url;

use crate::api_generator::{Type, TypeKind, ApiEndpoint};
use array_tool::vec::Intersect;
use inflector::Inflector;
use quote::{Tokens, ToTokens};
use reduce::Reduce;
use std::str;
use syn::{Field, ImplItem, FieldValue, FnArg};
use std::collections::BTreeMap;

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

pub trait GetPath {
    fn get_path(&self) -> &syn::Path;
}

impl GetPath for syn::Ty {
    fn get_path(&self) -> &syn::Path {
        match self {
            &syn::Ty::Path(_, ref p) => &p,
            _ => panic!("Only path types are supported."),
        }
    }
}

impl GetPath for syn::Path {
    fn get_path(&self) -> &syn::Path {
        &self
    }
}

/// Gets the Ty syntax token for a TypeKind
fn ty(name: &str, kind: &TypeKind, required: bool) -> syn::Ty {
    let mut v = String::new();
    if !required {
        v.push_str("Option<");
    }

    match kind {
        TypeKind::None => v.push_str("String"),
        TypeKind::List => v.push_str("Vec<String>"),
        TypeKind::Enum => v.push_str(name.to_pascal_case().as_str()),
        TypeKind::String => v.push_str("String"),
        TypeKind::Text => v.push_str("String"),
        TypeKind::Boolean => v.push_str("bool"),
        TypeKind::Number => v.push_str("i64"),
        TypeKind::Float => v.push_str("f32"),
        TypeKind::Double => v.push_str("f64"),
        TypeKind::Integer => v.push_str("i32"),
        TypeKind::Long => v.push_str("i64"),
        TypeKind::Date => v.push_str("String"),
        TypeKind::Time => v.push_str("String"),
    };

    if !required {
        v.push_str(">");
    }

    syn::parse_type(v.as_str()).unwrap()
}

/// A standard `'a` lifetime
pub fn lifetime_a() -> syn::Lifetime {
    syn::Lifetime {
        ident: syn::Ident::new("'a"),
    }
}

pub trait HasLifetime {
    fn has_lifetime(&self) -> bool;
}

impl<T: GetPath> HasLifetime for T {
    fn has_lifetime(&self) -> bool {
        match &self.get_path().segments[0].parameters {
            &syn::PathParameters::AngleBracketed(ref params) => params.lifetimes.len() > 0,
            _ => false,
        }
    }
}

/// Generics with a standard `'a` lifetime
pub fn generics_a() -> syn::Generics {
    generics(vec![lifetime_a()], vec![])
}

/// Generics with no parameters.
pub fn generics_none() -> syn::Generics {
    generics(vec![], vec![])
}

/// Generics with the given lifetimes and type bounds.
pub fn generics(lifetimes: Vec<syn::Lifetime>, types: Vec<syn::TyParam>) -> syn::Generics {
    syn::Generics {
        lifetimes: lifetimes
            .into_iter()
            .map(|l| syn::LifetimeDef {
                attrs: vec![],
                lifetime: l,
                bounds: vec![],
            })
            .collect(),
        ty_params: types,
        where_clause: syn::WhereClause::none(),
    }
}

fn create_required_field(f: (&String, &Type)) -> syn::Field {
    create_field(f, true)
}

fn create_optional_field(f: (&String, &Type)) -> syn::Field {
    create_field(f, false)
}

/// Creates a field for a struct
fn create_field(f: (&String, &Type), required: bool) -> syn::Field {
    syn::Field {
        ident: Some(ident(valid_name(&f.0).to_lowercase())),
        vis: syn::Visibility::Inherited,
        attrs: vec![],
        ty: ty(&f.0, &f.1.ty, required),
    }
}

fn create_required_fn(f: (&String, &Type)) -> syn::ImplItem {
    create_fn(f, true)
}

fn create_optional_fn(f: (&String, &Type)) -> syn::ImplItem {
    create_fn(f, false)
}

/// Creates a builder fn for a builder impl
fn create_fn(f: (&String, &Type), required: bool) -> syn::ImplItem {
    let name = valid_name(&f.0).to_lowercase();
    let impl_ident = ident(&name);
    let field_ident = ident(&name);
    let value_ident = ident(&name);
    let ty = ty(&f.0, &f.1.ty, required);
    let doc_attr = match &f.1.description {
        Some(docs) => vec![doc(docs.into())],
        _ => vec![],
    };

    syn::ImplItem {
        ident: impl_ident,
        vis: syn::Visibility::Public,
        defaultness: syn::Defaultness::Final,
        attrs: doc_attr,
        node: syn::ImplItemKind::Method(
            syn::MethodSig {
                unsafety: syn::Unsafety::Normal,
                constness: syn::Constness::NotConst,
                abi: None,
                decl: syn::FnDecl {
                    inputs: vec![
                        syn::FnArg::SelfValue(syn::Mutability::Mutable),
                        syn::FnArg::Captured(syn::Pat::Path(None, path_none(name.as_str())), ty),
                    ],
                    // TODO: is there a Self syn type?
                    output: syn::FunctionRetTy::Ty(syn::parse_type("Self").unwrap()),
                    variadic: false,
                },
                generics: generics_none(),
            },
            // generates a fn body of the form
            // --------
            // self.<field> = <field>;
            // self
            // ---------
            syn::Block {
                stmts: vec![
                    syn::Stmt::Semi(Box::new(parse_expr(
                        quote!(self.#field_ident = #value_ident),
                    ))),
                    syn::Stmt::Expr(Box::new(parse_expr(quote!(self)))),
                ],
            },
        ),
    }
}

/// use declarations common across builders
pub fn use_declarations() -> Tokens {
    quote!(
        use super::super::client::Elasticsearch;
        use super::super::http_method::HttpMethod;
        use super::super::enums::*;
        use reqwest::{Result, Response, Request, Error, StatusCode};
        use crate::client::Sender;
        use crate::response::ElasticsearchResponse;
        use serde::de::DeserializeOwned;
        use reqwest::header::HeaderMap;
    )
}

/// Creates the function arguments for a builder struct new fn
fn create_new_fnargs(builder_ident: &syn::Ident, required_parts: &Vec<(&String,&Type)>) -> Vec<FnArg> {
    match required_parts.len() {
        0 => vec![],
        _ => {
            required_parts.iter()
                .map(|part| (valid_name(part.0), part.1))
                .map(|part| {
                    syn::FnArg::Captured(syn::Pat::Path(None, path_none(part.0)), ty(part.0, &part.1.ty, true))
                })
                .collect::<Vec<syn::FnArg>>()
        }
    }
}

/// Creates the field values for a builder struct new fn call
fn create_new_fields(builder_ident: &syn::Ident, required_parts: &Vec<(&String,&Type)>) -> Vec<FieldValue> {
    match required_parts.len() {
        0 => vec![],
        _ => {
            required_parts.iter()
                .map(|part| valid_name(part.0))
                .map(|part| {
                    syn::FieldValue {
                        attrs: vec![],
                        ident: ident(part),
                        expr: syn::ExprKind::Path(None, path_none(ident(part).as_ref())).into(),
                        is_shorthand: false,
                    }
                })
                .collect()
        }
    }
}

fn create_new_fn(builder_ident: &syn::Ident, required_parts: &Vec<(&String,&Type)>) -> Tokens {
    let fnargs = create_new_fnargs(builder_ident, &required_parts);
    let fields = create_new_fields(builder_ident, &required_parts);
    match required_parts.len() {
        0 => quote!(
                pub fn new(client: Elasticsearch) -> Self {
                    #builder_ident {
                        client,
                        ..Default::default()
                    }
                }
            ),
        _ => {
            quote!(
                 pub fn new(client: Elasticsearch, #(#fnargs),*) -> Self {
                    #builder_ident {
                        client,
                        #(#fields),*,
                        ..Default::default()
                    }
                }
            )
        }
    }

}

/// creates the AST for a builder struct
pub fn create_builder_struct(builder_name: String, endpoint: &ApiEndpoint, common_fields: &Vec<Field>, common_builder_fns: &Vec<ImplItem>) -> Tokens {
    let builder_ident = ident(builder_name);

    // url parts that are common across all urls.
    // These are required parameters for the builder ctor new() fn
    let required_parts: Vec<&str> = endpoint.url.required_part_names();

    // collect all the fields for the builder struct. Start with url parts
    let mut fields: Vec<syn::Field> = endpoint
        .url
        .parts
        .iter()
        .map(|f| create_field(f, required_parts.contains(&&**f.0)))
        .collect();

    // url parameters
    fields.append(&mut endpoint
        .url
        .params
        .iter()
        .map(create_optional_field)
        .collect());

    // Combine common fields with struct fields, sort and deduplicate
    // clone common_fields, since quote!() consumes the Vec<Field>
    fields.append(&mut common_fields.clone());
    fields.sort_by(|a, b| a.ident.cmp(&b.ident));
    fields.dedup_by(|a, b| a.ident.eq(&b.ident));

    // collect all the functions for the builder struct
    let mut builder_fns: Vec<ImplItem> = endpoint
        .url
        .params
        .iter()
        .map(|f| create_fn(f, required_parts.contains(&&**f.0)))
        .collect();

    // Combine common fns with builder fns, sort and deduplicate
    // clone is required, since quote!() consumes the Vec<Item>
    builder_fns.append(&mut common_builder_fns.clone());
    builder_fns.sort_by(|a, b| a.ident.cmp(&b.ident));
    builder_fns.dedup_by(|a, b| a.ident.eq(&b.ident));

    let new_fn = create_new_fn(&builder_ident, &endpoint.url.required_parts());

    quote!(
        #[derive(Default)]
        pub struct #builder_ident {
            client: Elasticsearch,
            #(#fields),*,
        }

        impl #builder_ident {
            #new_fn
            #(#builder_fns)*
        }

        impl Sender for #builder_ident {
            fn send<T>(self) -> Result<ElasticsearchResponse<T>> where T:DeserializeOwned {
                  // TODO: build up the url based on parameters passed, and execute request
                  Ok(ElasticsearchResponse {
                       headers: HeaderMap::new(),
                       status_code: StatusCode::OK,
                       body: None
                  })

            }
        }
    )
}

/// Creates the AST for a fn that returns a new instance of a builder struct
pub fn create_builder_struct_ctor_fns(builder_name: String, name: &String, endpoint: &ApiEndpoint) -> Tokens {
    let builder_ident = ident(builder_name.to_pascal_case());
    let fn_name = ident(name.to_string());
    let path = endpoint.url.paths.first().unwrap();
    let method = endpoint.methods.first().unwrap();
    let supports_body = endpoint.supports_body();
    let method_doc = match &endpoint.documentation {
        Some(docs) => Some(doc(docs.into())),
        _ => None,
    };

    let fnargs = create_new_fnargs(&builder_ident, &endpoint.url.required_parts());
    let builder_args: Vec<syn::Pat> = fnargs
        .clone()
        .into_iter()
        .filter_map(|f| {
            match f {
                FnArg::Captured(p, ty) => Some(p),
                _ => None
            }
        })
        .collect();

    quote!(
        #method_doc
        pub fn #fn_name(&self, #(#fnargs),*) -> #builder_ident {
            // TODO: Add fn a
            #builder_ident::new(self.client.clone(),#(#builder_args),*)
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
