use crate::api_generator::*;

use array_tool::vec::Intersect;
use inflector::Inflector;
use quote::Tokens;
use reduce::Reduce;
use syn::{Field, ImplItem};

/// Generates the source code for a namespaced client
pub fn generate(api: &Api) -> Result<Vec<(String, String)>, failure::Error> {
    let mut output = Vec::new();

    let common_fields: Vec<Field> = api
        .common_params
        .iter()
        .map(code_gen::create_field)
        .collect();

    let common_builder_fns : Vec<ImplItem> = api
        .common_params
        .iter()
        .map(code_gen::create_fn)
        .collect();

    for (namespace, namespace_methods) in &api.namespaces {
        let mut tokens = quote::Tokens::new();

        let namespace_client_name = code_gen::ident(namespace.to_pascal_case());
        let namespace_doc = code_gen::doc(format!(
            "{} APIs",
            namespace.replace("_", " ").to_pascal_case()
        ));

        let namespace_name = code_gen::ident(namespace.to_string());

        let header = quote!(
            use super::super::client::Elasticsearch;
            use super::super::http_method::HttpMethod;
            use super::super::enums::*;
            use reqwest::{Result, Response, Request, Error, StatusCode};
            use crate::client::Sender;
            use crate::response::ElasticsearchResponse;
            use serde::de::DeserializeOwned;
            use reqwest::header::HeaderMap;
        );

        tokens.append(header);

        // AST for builder types
        let builders: Vec<Tokens> = namespace_methods
            .iter()
            .map(|(name, endpoint)| {
                let builder_name = format!(
                    "{}{}",
                    namespace.to_pascal_case(),
                    name.to_pascal_case()
                );

                let builder_ident = code_gen::ident(builder_name);

                // url parts that are common across all urls.
                // These are required parameters for the builder ctor new() fn
                let required_parts: Option<Vec<&str>> = endpoint
                    .url
                    .paths
                    .iter()
                    .map(|p| {
                        p.params()
                    })
                    .reduce(|a, b| a.intersect(b));

                // collect all the fields for the builder struct. Start with url parts
                let mut fields: Vec<syn::Field> = endpoint
                    .url
                    .parts
                    .iter()
                    .map(code_gen::create_field)
                    .collect();

                // url parameters
                fields.append(&mut endpoint
                    .url
                    .params
                    .iter()
                    .map(code_gen::create_field)
                    .collect());

                // Combine common fields with struct fields, sort and deduplicate
                // clone common_fields, since quote!() consumes the Vec<Field>
                fields.append(&mut common_fields.clone());
                fields.sort_by(|a, b| a.ident.cmp(&b.ident));
                fields.dedup_by(|a, b| a.ident.eq(&b.ident));

                // collect all the functions for the builder struct
                let mut builder_fns: Vec<ImplItem> = endpoint
                    .url
                    .params
                    .iter()
                    .map(code_gen::create_fn)
                    .collect();

                // Combine common fns with builder fns, sort and deduplicate
                // clone is required, since quote!() consumes the Vec<Item>
                builder_fns.append(&mut common_builder_fns.clone());
                builder_fns.sort_by(|a, b| a.ident.cmp(&b.ident));
                builder_fns.dedup_by(|a, b| a.ident.eq(&b.ident));

                quote!(
                    #[derive(Default)]
                    pub struct #builder_ident {
                        client: Elasticsearch,
                        #(#fields),*,
                    }

                    impl #builder_ident {
                        // TODO: add required_parts to new fn
                        pub fn new(client: Elasticsearch) -> Self {
                            #builder_ident {
                                client,
                                ..Default::default()
                            }
                        }
                        #(#builder_fns)*
                    }

                    impl Sender for #builder_ident {
                        fn send<T>(self) -> Result<ElasticsearchResponse<T>> where T:DeserializeOwned {
                              // TODO: build up the url based on parameters passed, and execute request
                              Ok(ElasticsearchResponse {
                                   headers: HeaderMap::new(),
                                   status_code: StatusCode::OK,
                                   body: None
                              })

                        }
                    }
                )
            })
            .collect();

        // AST for methods on namespace client that return builder types
        let methods: Vec<Tokens> = namespace_methods
            .iter()
            .map(|(name, endpoint)| {
                let struct_name =
                    format!("{}{}", namespace.to_pascal_case(), name.to_pascal_case());
                let builder_ident = code_gen::ident(struct_name.to_string());
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
                    pub fn #method_name(&self) -> #builder_ident {
                        #builder_ident::new(self.client.clone())
                    }
                )
            })
            .collect();

        // namespace client method on Elasticsearch
        let implementation = quote!(
            #(#builders)*

            #namespace_doc
            pub struct #namespace_client_name {
                client: Elasticsearch
            }

            impl #namespace_client_name {
                pub fn new(client: Elasticsearch) -> Self {
                    #namespace_client_name {
                        client
                    }
                }
                #(#methods)*
            }

            impl Elasticsearch {
                #namespace_doc
                pub fn #namespace_name(&self) -> #namespace_client_name {
                    #namespace_client_name::new(self.clone())
                }
            }
        );

        tokens.append(implementation);
        let generated = rust_fmt(tokens.to_string())?;
        output.push((namespace.to_string(), generated));
    }

    Ok(output)
}
