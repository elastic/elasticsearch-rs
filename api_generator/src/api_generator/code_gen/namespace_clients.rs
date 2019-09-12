use crate::api_generator::*;

use inflector::Inflector;
use quote::Tokens;

fn lit<I: Into<String>>(lit: I) -> syn::Lit {
    syn::Lit::Str(lit.into(), syn::StrStyle::Cooked)
}

fn ident(name: String) -> syn::Ident {
    syn::Ident::from(name)
}

fn doc(comment: String) -> syn::Attribute {
    syn::Attribute {
        style: syn::AttrStyle::Outer,
        value: syn::MetaItem::NameValue(ident("doc".to_string()), lit(comment)),
        is_sugared_doc: true,
    }
}

fn sanitize_param_name(s: &str) -> &str {
    match s {
        "type" => "ty",
        s=> s
    }
}

pub fn generate(api: &Api) -> Result<Vec<(String, String)>, failure::Error> {
    let mut output = Vec::new();

    for namespace in &api.namespaces {
        let mut tokens = quote::Tokens::new();

        let namespace_client_name = ident(format!("{}NamespaceClient", namespace.0.to_pascal_case()));

        let namespace_doc = doc(format!(
            "{} APIs",
            namespace.0.replace("_", " ").to_pascal_case()
        ));

        let namespace_name = ident(namespace.0.to_string());
        
        let header = quote!(
            use super::super::client::ElasticsearchClient;
            use super::super::http_method::HttpMethod;
            use reqwest::{Result, Response, Request, Error};
        );

        tokens.append(header);

        let builders: Vec<Tokens> = namespace
            .1
            .iter()
            .map(|(name, endpoint)| {
                let struct_name = format!("{}{}Request", namespace.0.to_pascal_case(), name.to_pascal_case());
                let struct_ident = ident(struct_name.to_string());
                let builder_ident = ident(format!("{}Builder", struct_name.to_string()));

                let params : Vec<Tokens> = endpoint.url.params.iter().map(|(n, t)| {
                    let param_name = ident(sanitize_param_name(n).to_lowercase());

                    let param_ty = match t.ty {
                        TypeKind::None => syn::parse_type("&'a String").unwrap(),
                        TypeKind::List => syn::parse_type("&'a Vec<String>").unwrap(),
                        TypeKind::Enum => syn::parse_type("Option<&'a i32>").unwrap(),
                        TypeKind::String => syn::parse_type("&'a String").unwrap(),
                        TypeKind::Text => syn::parse_type("&'a String").unwrap(),
                        TypeKind::Boolean => syn::parse_type("Option<&'a bool>").unwrap(),
                        TypeKind::Number => syn::parse_type("Option<&'a i64>").unwrap(),
                        TypeKind::Float => syn::parse_type("Option<&'a f32>").unwrap(),
                        TypeKind::Double => syn::parse_type("Option<&'a f64>").unwrap(),
                        TypeKind::Integer => syn::parse_type("Option<&'a i32>").unwrap(),
                        TypeKind::Long => syn::parse_type("Option<&'a i64>").unwrap(),
                        TypeKind::Date => syn::parse_type("&'a String").unwrap(),
                        TypeKind::Time => syn::parse_type("&'a String").unwrap(),
                        TypeKind::Duration => syn::parse_type("&'a String").unwrap(),
                    };

                    quote!(#param_name: #param_ty)
                }).collect();

                let builder_params = params.clone();

                let assignments : Vec<Tokens> = endpoint.url.params.iter().map(|(n, t)| {
                    let param_name = ident(sanitize_param_name(n).to_lowercase());
                    quote!(#param_name: self.#param_name)
                }).collect();

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

        let methods: Vec<Tokens> = namespace
            .1
            .iter()
            .map(|(name, endpoint)| {
                let struct_name = format!("{}{}Request", namespace.0.to_pascal_case(), name.to_pascal_case());
                let struct_ident = ident(struct_name.to_string());

                let method_name = ident(name.to_string());
                let path = endpoint.url.paths.first().unwrap();
                let method = endpoint.methods.first().unwrap();

                let supports_body = endpoint
                    .methods
                    .iter()
                    .any(|m| m == &HttpMethod::Post || m == &HttpMethod::Put)
                    || endpoint.body.is_some();

                let method_doc = match &endpoint.documentation {
                    Some(docs) => Some(doc(docs.into())),
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
        output.push((namespace.0.to_string(), generated));
    }

    Ok(output)
}
