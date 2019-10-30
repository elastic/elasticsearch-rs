use crate::api_generator::*;

use inflector::Inflector;
use quote::Tokens;
use syn::{Field, ImplItem};

/// Generates the source code for the methods on the root of Elasticsearch
pub fn generate(api: &Api) -> Result<String, failure::Error> {
    let mut tokens = quote::Tokens::new();
    tokens.append(code_gen::use_declarations());

    let common_fields: Vec<Field> = api
        .common_params
        .iter()
        .map(code_gen::create_optional_field)
        .collect();

    let common_builder_fns: Vec<ImplItem> = api
        .common_params
        .iter()
        .map(code_gen::create_optional_fn)
        .collect();

    // AST for builder structs
    let builders: Vec<Tokens> = api
        .root
        .iter()
        .map(|(name, endpoint)| {
            code_gen::create_builder_struct(
                name.to_pascal_case(),
                endpoint,
                &common_fields,
                &common_builder_fns,
            )
        })
        .collect();

    // AST for methods on Elasticsearch that return builder types
    let methods: Vec<Tokens> = api
        .root
        .iter()
        .map(|(name, endpoint)| {
            code_gen::create_builder_struct_ctor_fns(name.to_pascal_case(), name, endpoint, true)
        })
        .collect();

    tokens.append(quote!(
        #(#builders)*

        impl Elasticsearch {
            #(#methods)*
        }
    ));

    let generated = rust_fmt(tokens.to_string())?;
    Ok(generated)
}
