use crate::api_generator::*;

use inflector::Inflector;
use quote::Tokens;
use regex::Regex;

pub fn generate(api: &Api) -> Result<String, failure::Error> {
    let mut tokens = quote::Tokens::new();
    let header = quote!(
        use serde::{Serialize, Deserialize};
    );
    tokens.append(header);
    for e in &api.enums {
        generate_param(&mut tokens, &e);
    }

    rust_fmt(tokens.to_string())
}

fn generate_param(tokens: &mut Tokens, e: &ApiEnum) {
    let name = syn::Ident::from(e.name.to_pascal_case());
    let (renames, variants): (Vec<String>, Vec<syn::Ident>) = e
        .values
        .iter()
        .filter(|v| !v.is_empty())
        .map(|v| {
            if !v.contains("(") {
                (v.to_owned(), syn::Ident::from(v.to_pascal_case()))
            } else {
                lazy_static! {
                    static ref PARENS_REGEX: Regex = Regex::new(r"^(.*?)\s*\(.*?\)\s*$").unwrap();
                }
                if let Some(c) = PARENS_REGEX.captures(v) {
                    (c.get(1).unwrap().as_str().to_owned(), syn::Ident::from(c.get(1).unwrap().as_str().to_pascal_case()))
                } else {
                    (v.to_owned(), syn::Ident::from(v.to_pascal_case()))
                }
            }
        })
        .unzip();

    let generated_enum_tokens = quote!(
        #[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
        pub enum #name {
            #(#[serde(rename = #renames)] #variants),*
        }
    );

    tokens.append(generated_enum_tokens);
}
