use crate::api_generator::*;

use crate::api_generator::code_gen::request::request_builder::RequestBuilder;
use crate::api_generator::code_gen::*;
use inflector::Inflector;
use quote::Tokens;

/// Generates the source code for a namespaced client
pub fn generate(api: &Api) -> Result<Vec<(String, String)>, failure::Error> {
    let mut output = Vec::new();

    for (namespace, namespace_methods) in &api.namespaces {
        let mut tokens = Tokens::new();
        tokens.append(use_declarations());

        let namespace_pascal_case = namespace.to_pascal_case();
        let namespace_client_name = ident(&namespace_pascal_case);
        let namespace_doc = doc(format!(
            "{} APIs",
            namespace.replace("_", " ").to_pascal_case()
        ));
        let namespace_name = ident(namespace.to_string());

        let (builders, methods): (Vec<Tokens>, Vec<Tokens>) = namespace_methods
            .iter()
            .map(|(name, endpoint)| {
                let builder_name = format!("{}{}", &namespace_pascal_case, name.to_pascal_case());
                RequestBuilder::new(name, &builder_name, &api.common_params, &endpoint, false)
                    .build()
            })
            .unzip();

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
