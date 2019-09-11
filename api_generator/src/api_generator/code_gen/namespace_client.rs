use crate::api_generator::*;
use std::path::PathBuf;

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

pub fn generate_namespace_clients(api: &Api, generated_dir: &PathBuf) {
    for namespace in &api.namespaces {
        let mut tokens = quote::Tokens::new();
    }
}
