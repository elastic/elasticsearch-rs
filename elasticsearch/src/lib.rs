// TODO: whilst developing
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

extern crate reqwest;

extern crate serde;

#[macro_use]
extern crate serde_json;

pub mod auth;
pub mod enums;
pub mod http;

mod client;
mod error;
mod namespace_clients;
mod root;

// exposed at the library root level
pub use crate::{
    client::*, error::*, http::transport::DEFAULT_ADDRESS, namespace_clients::*, root::*,
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
