pub mod enums;
pub mod namespace_clients;
pub mod root;

use crate::api_generator::{Type, TypeKind};
use quote::Tokens;

/// AST for a literal
fn lit<I: Into<String>>(lit: I) -> syn::Lit {
    syn::Lit::Str(lit.into(), syn::StrStyle::Cooked)
}

/// AST for an identifier
fn ident(name: String) -> syn::Ident {
    syn::Ident::from(name)
}

/// AST for doc attribute
fn doc(comment: String) -> syn::Attribute {
    syn::Attribute {
        style: syn::AttrStyle::Outer,
        value: syn::MetaItem::NameValue(ident("doc".to_string()), lit(comment)),
        is_sugared_doc: true,
    }
}

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

fn ty(type_kind: &TypeKind) -> syn::Ty {
    match type_kind {
        TypeKind::None => syn::parse_type("Option<String>").unwrap(),
        TypeKind::List => syn::parse_type("Option<Vec<String>>").unwrap(),
        TypeKind::Enum => syn::parse_type("Option<i32>").unwrap(),
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
        TypeKind::Duration => syn::parse_type("Option<String>").unwrap(),
    }
}

//pub struct Field {
//    pub description: Option<syn::Attribute>,
//    pub name: syn::Ident,
//    pub ty: syn::Ty
//}
//
//impl Field {
//    pub fn new(name: &str, ty: &Type) -> Self {
//        Self {
//            description: match &ty.description {
//                Some(desc) => Some(doc(desc.to_string())),
//                _ => None,
//            },
//            name: ident(valid_name(name).to_lowercase()),
//            ty: self::ty(&ty.ty),
//        }
//    }
//
//    pub fn declaration(&self) -> Tokens {
//        let mut tokens = quote::Tokens::new();
//        let name = &self.name;
//        let ty = &self.ty;
//        let description = &self.description;
//
//        let t = quote!(
//            #description
//            #name: #ty
//        );
//
//        tokens.append(t);
//        tokens
//    }
//
//    pub fn method(&self, builder_ident: syn::Ident) -> Tokens {
//        let mut tokens = quote::Tokens::new();
//        let name = &self.name;
//        let ty = &self.ty;
//        let description = &self.description;
//
//        let  t = quote!(
//            #description
//            pub fn #name(&self, value: #ty) -> #builder_ident {
//                self.#name = value;
//                self
//            }
//        );
//
//        tokens.append(t);
//        tokens
//    }
//}
