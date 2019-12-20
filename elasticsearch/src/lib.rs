// TODO: turn on before releasing :) Will require adding documentation within all REST API specs
//#![deny(missing_docs)]

#[macro_use]
extern crate objekt;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub mod auth;
pub mod http;

mod client;
mod error;
mod generated;

// exposes types within modules at the library root level
pub use crate::{
    client::*, error::*, generated::namespace_clients::*, generated::params, generated::root::*,
    http::transport::DEFAULT_ADDRESS,
};

#[cfg(test)]
pub mod tests {
    use crate::SearchParts;

    #[test]
    fn build_search_on_all_indices_and_types() {
        let parts = SearchParts::None;
        let url = parts.url();
        assert_eq!(url, "/_search");
    }

    #[test]
    fn build_search_on_selected_indices() {
        let parts = SearchParts::Index(&["index-1", "index-2"]);
        let url = parts.url();
        assert_eq!(url, "/index-1,index-2/_search");
    }

    #[test]
    fn build_search_on_selected_indices_and_types() {
        let parts = SearchParts::IndexType(&["index-1", "index-2"], &["type-1", "type-2"]);
        let url = parts.url();
        assert_eq!(url, "/index-1,index-2/type-1,type-2/_search");
    }
}
