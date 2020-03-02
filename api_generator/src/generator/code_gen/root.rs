use crate::generator::*;

use crate::generator::code_gen::request::request_builder::RequestBuilder;
use crate::generator::code_gen::*;
use inflector::Inflector;
use quote::Tokens;

/// Generates the source code for the methods on the root of Elasticsearch
pub fn generate(api: &Api) -> Result<String, failure::Error> {
    let mut tokens = Tokens::new();
    tokens.append(use_declarations());

    // AST for builder structs and methods
    let (builders, methods): (Vec<Tokens>, Vec<Tokens>) = api
        .root
        .iter()
        .map(|(name, endpoint)| {
            let builder_name = name.to_pascal_case();
            RequestBuilder::new(name, &builder_name, &api.common_params, endpoint, true).build()
        })
        .unzip();

    tokens.append(quote!(
        #(#builders)*

        impl Elasticsearch {
            #(#methods)*
        }
    ));

    let generated = rust_fmt(tokens.to_string())?;
    Ok(generated)
}
