use crate::api_generator::*;

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

        let generated = rust_fmt(tokens.to_string())?;
        output.push((namespace.0.to_string(), generated));
    }

    Ok(output)
}
