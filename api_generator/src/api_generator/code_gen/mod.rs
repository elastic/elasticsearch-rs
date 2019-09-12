pub mod enums;
pub mod root;
pub mod namespace_clients;

use crate::api_generator::TypeKind;
use quote::Tokens;

fn lit<I: Into<String>>(lit: I) -> syn::Lit {
    syn::Lit::Str(lit.into(), syn::StrStyle::Cooked)
}

fn ident(name: String) -> syn::Ident {
    syn::Ident::from(name)
}

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

fn ty(type_kind: &TypeKind) -> syn::Ty {
    match type_kind {
        TypeKind::None => syn::parse_type("&'a String").unwrap(),
        TypeKind::List => syn::parse_type("&'a Vec<String>").unwrap(),
        TypeKind::Enum => syn::parse_type("Option<&'a i32>").unwrap(),
        TypeKind::String => syn::parse_type("&'a String").unwrap(),
        TypeKind::Text => syn::parse_type("&'a String").unwrap(),
        TypeKind::Boolean => syn::parse_type("Option<&'a bool>").unwrap(),
        TypeKind::Number => syn::parse_type("Option<&'a i64>").unwrap(),
        TypeKind::Float => syn::parse_type("Option<&'a f32>").unwrap(),
        TypeKind::Double => syn::parse_type("Option<&'a f64>").unwrap(),
        TypeKind::Integer => syn::parse_type("Option<&'a i32>").unwrap(),
        TypeKind::Long => syn::parse_type("Option<&'a i64>").unwrap(),
        TypeKind::Date => syn::parse_type("&'a String").unwrap(),
        TypeKind::Time => syn::parse_type("&'a String").unwrap(),
        TypeKind::Duration => syn::parse_type("&'a String").unwrap(),
    }
}
