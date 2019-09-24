use crate::api_generator::*;

use inflector::Inflector;
use quote::Tokens;
use syn::{Field, FieldValue};

fn create_field(f: (&String, &Type)) -> syn::Field {
    syn::Field {
        ident: Some(code_gen::ident(code_gen::valid_name(f.0).to_lowercase())),
        vis: syn::Visibility::Inherited,
        attrs: vec![],
        ty: code_gen::ty(&f.1.ty),
    }
}

fn create_builder_method(f: (&String, &Type)) -> Tokens {
    let name = code_gen::ident(code_gen::valid_name(f.0).to_lowercase());
    let value = code_gen::ty(&f.1.ty);
    let doc = match &f.1.description {
        Some(docs) => Some(code_gen::doc(docs.into())),
        _ => None,
    };

    quote!(
        #doc
        pub fn #name(mut self, #name: #value) -> Self {
            self.#name = #name;
            self
        }
    )
}

/// Generates the source code for a namespace
pub fn generate(api: &Api) -> Result<Vec<(String, String)>, failure::Error> {
    let mut output = Vec::new();

    let common_fields: Vec<Field> = api.common_params.iter().map(create_field).collect();

    let common_builder_methods: Vec<Tokens> = api
        .common_params
        .iter()
        .map(create_builder_method)
        .collect();

    for (namespace, namespace_methods) in &api.namespaces {
        let mut tokens = quote::Tokens::new();

        let namespace_client_name =
            code_gen::ident(format!("{}Client", namespace.to_pascal_case()));

        let namespace_doc = code_gen::doc(format!(
            "{} APIs",
            namespace.replace("_", " ").to_pascal_case()
        ));

        let namespace_name = code_gen::ident(namespace.to_string());

        let header = quote!(
            use super::super::client::ElasticsearchClient;
            use super::super::http_method::HttpMethod;
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
                    "{}{}Builder",
                    namespace.to_pascal_case(),
                    name.to_pascal_case()
                );

                let builder_ident = code_gen::ident(builder_name);

                let fields: Vec<syn::Field> = endpoint
                    .url
                    .params
                    .iter()
                    .map(create_field)
                    .collect();

                let builder_methods: Vec<Tokens> = endpoint
                    .url
                    .params
                    .iter()
                    .map(create_builder_method)
                    .collect();

                // clone is required as quote! consumes the Vec<Field>
                let common_fields_clone = common_fields.clone();
                let common_builder_methods_clone = common_builder_methods.clone();

                quote!(
                    #[derive(Default)]
                    pub struct #builder_ident {
                        client: ElasticsearchClient,
                        #(#common_fields_clone),*,
                        #(#fields),*
                    }

                    impl #builder_ident {
                        pub fn new(client: ElasticsearchClient) -> Self {
                            #builder_ident {
                                client,
                                ..Default::default()
                            }
                        }
                        #(#common_builder_methods_clone)*
                        #(#builder_methods)*
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
                let struct_ident = code_gen::ident(struct_name.to_string());
                let builder_ident = code_gen::ident(format!("{}Builder", struct_name.to_string()));
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

        // namespace client method on ElasticsearchClient
        let implementation = quote!(
            #(#builders)*

            #namespace_doc
            pub struct #namespace_client_name {
                client: ElasticsearchClient
            }

            impl #namespace_client_name {
                pub fn new(client: ElasticsearchClient) -> Self {
                    #namespace_client_name {
                        client
                    }
                }
                #(#methods)*
            }

            impl ElasticsearchClient {
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
