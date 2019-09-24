use crate::api_generator::*;

use inflector::Inflector;
use quote::Tokens;
use syn::Field;

/// Generates the source code for the methods on Elasticsearch root
pub fn generate(api: &Api) -> Result<String, failure::Error> {
    let mut tokens = quote::Tokens::new();

    let common_fields: Vec<Field> = api
        .common_params
        .iter()
        .map(code_gen::create_field)
        .collect();

    let common_builder_methods: Vec<Tokens> = api
        .common_params
        .iter()
        .map(code_gen::create_builder_method)
        .collect();

    let builders: Vec<Tokens> = api
        .root
        .iter()
        .map(|(name, endpoint)| {
            let builder_name = format!("{}Builder", name.to_pascal_case());

            let builder_ident = code_gen::ident(builder_name);

            let fields: Vec<syn::Field> = endpoint
                .url
                .params
                .iter()
                .map(code_gen::create_field)
                .collect();

            let builder_methods: Vec<Tokens> = endpoint
                .url
                .params
                .iter()
                .map(code_gen::create_builder_method)
                .collect();

            // clone is required as quote! consumes the Vec<Field>
            let common_fields_clone = common_fields.clone();
            let common_builder_methods_clone = common_builder_methods.clone();

            quote!(
                #[derive(Default)]
                pub struct #builder_ident {
                    client: Elasticsearch,
                    #(#common_fields_clone),*,
                    #(#fields),*
                }

                impl #builder_ident {
                    pub fn new(client: Elasticsearch) -> Self {
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

    let methods: Vec<Tokens> = api
        .root
        .iter()
        .map(|(name, endpoint)| {
            let builder_ident = code_gen::ident(format!("{}Builder", name.to_pascal_case()));
            let method_name = code_gen::ident(name.to_string());
            let path = endpoint.url.paths.first().unwrap();
            let method = endpoint.methods.first().unwrap();

            let supports_body = endpoint.supports_body();

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

    let header = quote!(
        use super::super::client::Elasticsearch;
        use super::super::http_method::HttpMethod;
        use super::super::enums::*;
        use reqwest::{Result, Response, Request, Error};
        use crate::client::Sender;
        use crate::response::ElasticsearchResponse;
        use serde::de::DeserializeOwned;
        use reqwest::header::HeaderMap;
    );

    tokens.append(header);
    let implementation = quote!(
        #(#builders)*

        impl Elasticsearch {
            #(#methods)*
        }
    );

    tokens.append(implementation);
    let generated = rust_fmt(tokens.to_string())?;
    Ok(generated)
}
