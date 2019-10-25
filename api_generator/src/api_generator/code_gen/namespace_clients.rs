use crate::api_generator::*;

use inflector::Inflector;
use quote::Tokens;
use syn::{Field, ImplItem};

/// Generates the source code for a namespaced client
pub fn generate(api: &Api) -> Result<Vec<(String, String)>, failure::Error> {
    let mut output = Vec::new();

    let common_fields: Vec<Field> = api
        .common_params
        .iter()
        .map(code_gen::create_field)
        .collect();

    let common_builder_fns: Vec<ImplItem> =
        api.common_params.iter().map(code_gen::create_fn).collect();

    for (namespace, namespace_methods) in &api.namespaces {
        let mut tokens = quote::Tokens::new();
        tokens.append(code_gen::use_declarations());

        let namespace_client_name = code_gen::ident(namespace.to_pascal_case());
        let namespace_doc = code_gen::doc(format!(
            "{} APIs",
            namespace.replace("_", " ").to_pascal_case()
        ));
        let namespace_name = code_gen::ident(namespace.to_string());

        // AST for builder structs
        let builders: Vec<Tokens> = namespace_methods
            .iter()
            .map(|(name, endpoint)| {
                let builder_name = format!(
                    "{}{}",
                    namespace.to_pascal_case(),
                    name.to_pascal_case()
                );

                code_gen::create_builder_struct(builder_name, endpoint, &common_fields, &common_builder_fns)
            })
            .collect();

        // AST for methods on namespace client that return builder types
        let methods: Vec<Tokens> = namespace_methods
            .iter()
            .map(|(name, endpoint)| {
                let builder_name =
                    format!("{}{}", namespace.to_pascal_case(), name.to_pascal_case());

                code_gen::create_builder_struct_ctor_fns(builder_name, name, endpoint)
            })
            .collect();

        // namespace client method on Elasticsearch
        tokens.append(quote!(
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
        ));

        let generated = rust_fmt(tokens.to_string())?;
        output.push((namespace.to_string(), generated));
    }

    Ok(output)
}
