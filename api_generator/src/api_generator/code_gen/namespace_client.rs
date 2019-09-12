use crate::api_generator::*;

use inflector::Inflector;
use quote::Tokens;

fn lit<I: Into<String>>(lit: I) -> syn::Lit {
    syn::Lit::Str(lit.into(), syn::StrStyle::Cooked)
}

fn doc(comment: String) -> syn::Attribute {
    syn::Attribute {
        style: syn::AttrStyle::Outer,
        value: syn::MetaItem::NameValue(syn::Ident::from("doc"), lit(comment)),
        is_sugared_doc: true,
    }
}

pub fn generate(api: &Api) -> Result<Vec<(String, String)>, failure::Error> {
    let mut output = Vec::new();

    for namespace in &api.namespaces {
        let mut tokens = quote::Tokens::new();

        let namespace_client_name =
            syn::Ident::from(format!("{}NamespaceClient", namespace.0.to_pascal_case()));

        let namespace_doc = doc(format!(
            "{} APIs",
            namespace.0.replace("_", " ").to_pascal_case()
        ));

        let namespace_name = syn::Ident::from(namespace.0.to_string());
        let commit = &api.commit;

        let header = quote!(
            use super::super::client::ElasticsearchClient;
            use super::super::http_method::HttpMethod;
            use reqwest::{Result, Response, Request, Error};
            use serde::Deserialize;
        );

        tokens.append(header);

        let methods: Vec<Tokens> = namespace
            .1
            .iter()
            .map(|(name, endpoint)| {
                let method_name = syn::Ident::from(name.to_string());
                let path = endpoint.url.paths.first().unwrap();
                let method = endpoint.methods.first().unwrap();
                let method_doc = match &endpoint.documentation {
                    Some(docs) => Some(doc(docs.into())),
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

        let s = quote!(
            #namespace_doc
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

            impl ElasticsearchClient {
                #namespace_doc
                pub fn #namespace_name(&self) -> #namespace_client_name {
                    #namespace_client_name::new(self)
                }
            }
        );

        tokens.append(s);
        let generated = rust_fmt(tokens.to_string())?;
        output.push((namespace.0.to_string(), generated));
    }

    Ok(output)
}
