use crate::api_generator::*;

use inflector::Inflector;
use quote::Tokens;

/// A builder to generate a namespaced client
pub struct Builder {
    ident: syn::Ident,
}

impl Builder {
    fn new(name: &str) -> Self {
        Builder {
            ident: syn::Ident::from(name),
        }
    }
}

pub fn generate_namespace_clients(api: &Api) -> Result<Vec<(String, String)>, failure::Error> {
    let mut output = Vec::new();

    for namespace in &api.namespaces {
        let mut tokens = quote::Tokens::new();

        let namespace_client_name =
            syn::Ident::from(format!("{}NamespaceClient", namespace.0.to_pascal_case()));

        let header = quote!(
            use super::client::ElasticsearchClient;
            use reqwest;
            use serde::{Deserialize};
        );

        tokens.append(header);

        let methods: Vec<Tokens> = namespace
            .1
            .iter()
            .map(|(name, endpoint)| {
                let method_name = syn::Ident::from(name.to_string());
                let path = endpoint.url.paths.first().unwrap();
                let method = endpoint.methods.first().unwrap();
                quote!(
                    pub fn #method_name(&self) -> reqwest::Result<reqwest::Response, Error> {
                        self.client.send(#method, #path)
                    }
                )
            })
            .collect();

        let s = quote!(
            pub struct #namespace_client_name<'a> {
                client: &'a ElasticsearchClient
            }

            impl #namespace_client_name {
                pub fn new(client: &ElasticsearchClient) -> Self {
                    #namespace_client_name {
                        client
                    }
                }

                #(#methods)*
            }
        );

        tokens.append(s);
        let generated = rust_fmt(tokens.to_string())?;
        output.push((namespace.0.to_string(), generated));
    }

    Ok(output)
}
