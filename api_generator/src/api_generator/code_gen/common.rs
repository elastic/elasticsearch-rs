use crate::api_generator::*;

use inflector::Inflector;
use quote::Tokens;

pub fn generate(api: &Api) -> Result<Tokens, failure::Error> {
    let mut tokens = quote::Tokens::new();
    let header = quote!(
        use serde::{Deserialize};
    );
    tokens.append(header);

    // TODO: generate common token params

    tokens
}
