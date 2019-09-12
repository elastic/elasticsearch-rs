use crate::api_generator::*;

use inflector::Inflector;
use quote::Tokens;

pub fn generate(api: &Api) -> Result<Vec<(String, String)>, failure::Error> {
    let mut output = Vec::new();

    for (namespace, namespace_methods) in &api.namespaces {
        let mut tokens = quote::Tokens::new();

        let namespace_client_name =
            code_gen::ident(format!("{}NamespaceClient", namespace.to_pascal_case()));

        let namespace_doc = code_gen::doc(format!(
            "{} APIs",
            namespace.replace("_", " ").to_pascal_case()
        ));

        let namespace_name = code_gen::ident(namespace.to_string());

        let header = quote!(
            use super::super::client::ElasticsearchClient;
            use super::super::http_method::HttpMethod;
            use reqwest::{Result, Response, Request, Error};
        );

        tokens.append(header);

        let builders: Vec<Tokens> = namespace_methods
            .iter()
            .map(|(name, endpoint)| {
                let struct_name = format!(
                    "{}{}Request",
                    namespace.to_pascal_case(),
                    name.to_pascal_case()
                );
                let struct_ident = code_gen::ident(struct_name.to_string());
                let builder_ident = code_gen::ident(format!("{}Builder", struct_name.to_string()));

                let params: Vec<Tokens> = endpoint
                    .url
                    .params
                    .iter()
                    .map(|(n, t)| {
                        let param_name = code_gen::ident(code_gen::valid_name(n).to_lowercase());
                        let param_ty = code_gen::ty(&t.ty);
                        quote!(#param_name: #param_ty)
                    })
                    .collect();

                let builder_params = params.clone();

                let assignments: Vec<Tokens> = endpoint
                    .url
                    .params
                    .iter()
                    .map(|(n, t)| {
                        let param_name = code_gen::ident(code_gen::valid_name(n).to_lowercase());
                        quote!(#param_name: self.#param_name)
                    })
                    .collect();

                quote!(
                    pub struct #struct_ident<'a> {
                        #(#params),*
                    }

                    pub struct #builder_ident<'a> {
                        #(#builder_params),*
                    }

                    impl<'a> #builder_ident<'a> {
                        // TODO: add build methods

                        pub fn build(&self) -> #struct_ident<'a> {
                            #struct_ident {
                                #(#assignments),*
                            }
                        }
                    }
                )
            })
            .collect();

        let methods: Vec<Tokens> = namespace_methods
            .iter()
            .map(|(name, endpoint)| {
                let struct_name = format!(
                    "{}{}Request",
                    namespace.to_pascal_case(),
                    name.to_pascal_case()
                );
                let struct_ident = code_gen::ident(struct_name.to_string());

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
                    pub fn #method_name(&self, request: &#struct_ident) -> Result<Response> {
                        self.client.send(#method, #path)
                    }
                )
            })
            .collect();

        let implementation = quote!(
            #(#builders)*

            #namespace_doc
            pub struct #namespace_client_name<'a> {
                client: &'a ElasticsearchClient
            }

            impl<'a> #namespace_client_name<'a> {
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

        tokens.append(implementation);
        let generated = rust_fmt(tokens.to_string())?;
        output.push((namespace.to_string(), generated));
    }

    Ok(output)
}
