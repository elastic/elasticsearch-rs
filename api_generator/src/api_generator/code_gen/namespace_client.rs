use crate::api_generator::*;

use inflector::Inflector;
use quote::Tokens;

/// A builder to generate a namespaced client
pub struct Builder {
    ident: syn::Ident,
}

impl Builder {
    pub fn new(name: &str) -> Self {
        Builder {
            ident: syn::Ident::from(name),
        }
    }
}

pub fn generate_namespace_clients(api: &Api) -> Result<Vec<(String, String)>, failure::Error> {
    let mut output = Vec::new();

    for namespace in &api.namespaces {
        let mut tokens = quote::Tokens::new();

        let name = namespace.0;
        let namespace_client_name = syn::Ident::from(format!("{}NamespaceClient", name.to_pascal_case()));

        let methods = namespace.1;

        let header = quote!(
            use serde::{Deserialize};
        );

        tokens.append(header);

        let s = quote!(
            pub struct #namespace_client_name {
                client: &ElasticsearchClient
            }

            impl #namespace_client_name {
                pub fn new(client: &ElasticsearchClient) -> Self {
                    #namespace_client_name {
                        client
                    }
                }
            }
        );

        tokens.append(s);

        let generated = rust_fmt(tokens.to_string())?;
        output.push((namespace.0.to_string(), generated));
    }

    Ok(output)
}
