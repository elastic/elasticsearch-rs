use crate::api_generator::*;

use inflector::Inflector;
use quote::Tokens;

pub fn generate(api: &Api) -> Result<String, failure::Error> {
    let mut tokens = quote::Tokens::new();
    let header = quote!(
        use serde::{Serialize, Deserialize};
    );
    tokens.append(header);
    for e in &api.enums {
        generate_enum(&mut tokens, &e);
    }

    rust_fmt(tokens.to_string())
}

fn generate_enum(tokens: &mut Tokens, e: &ApiEnum) {
    let name = syn::Ident::from(e.name.to_pascal_case());
    let renames = e
        .values
        .iter()
        .filter(|v| !v.is_empty())
        .collect::<Vec<_>>();
    let values: Vec<syn::Ident> = renames
        .iter()
        .map(|v| syn::Ident::from(v.to_pascal_case()))
        .collect();

    let generated_enum_tokens = quote!(
        #[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
        pub enum #name {
            #(#[serde(rename = #renames)] #values),*
        }
    );

    tokens.append(generated_enum_tokens);
}
