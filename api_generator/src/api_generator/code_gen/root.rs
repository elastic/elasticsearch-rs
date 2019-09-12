use crate::api_generator::*;

use inflector::Inflector;
use quote::Tokens;

pub fn generate(api: &Api) -> Result<String, failure::Error> {
    let mut tokens = quote::Tokens::new();

    let methods: Vec<Tokens> = api
        .global
        .iter()
        .map(|(name, endpoint)| {
            let method_name = code_gen::ident(name.to_string());
            let path = endpoint.url.paths.first().unwrap();
            let method = endpoint.methods.first().unwrap();

            let supports_body = endpoint
                .methods
                .iter()
                .any(|m| m == &HttpMethod::Post || m == &HttpMethod::Put)
                || endpoint.body.is_some();

            let method_doc = match &endpoint.documentation {
                Some(docs) => Some(code_gen::doc(docs.into())),
                _ => None,
            };

            quote!(
                #method_doc
                pub fn #method_name(&self) -> Result<Response> {
                    self.client.send(#method, #path)
                }
            )
        })
        .collect();

    let header = quote!(
        use super::super::client::ElasticsearchClient;
        use super::super::http_method::HttpMethod;
        use reqwest::{Result, Response, Request, Error};
    );

    tokens.append(header);
    let implementation = quote!(
        impl ElasticsearchClient {
            #(#methods)*
        }
    );

    tokens.append(implementation);
    let generated = rust_fmt(tokens.to_string())?;
    Ok(generated)
}
