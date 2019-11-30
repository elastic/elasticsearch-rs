// TODO: whilst developing
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

extern crate reqwest;

extern crate serde;

#[macro_use]
extern crate serde_json;

mod client;
mod connection;
mod enums;
mod error;
mod http_method;
mod namespace_clients;
mod response;
mod root;

pub use crate::{
    client::Elasticsearch, connection::*, enums::*, error::ElasticsearchError,
    http_method::HttpMethod, response::ElasticsearchResponse, root::*,
};
pub use reqwest::Client;

#[cfg(test)]
pub mod tests {
    use crate::root::SearchUrlParts;
    use crate::{client::Elasticsearch, Connection};
    use failure;
    use reqwest::StatusCode;
    use serde_json::Value;
    use url::Url;

    #[test]
    fn build_search_on_all_indices_and_types() {
        let parts = SearchUrlParts::None;
        let url = parts.build();
        assert_eq!(url, "/_search");
    }

    #[test]
    fn build_search_on_selected_indices() {
        let parts = SearchUrlParts::Index(&["index-1", "index-2"]);
        let url = parts.build();
        assert_eq!(url, "/index-1,index-2/_search");
    }

    #[test]
    fn build_search_on_selected_indices_and_types() {
        let parts = SearchUrlParts::IndexType(
            &["index-1", "index-2"],
            &["type-1", "type-2"],
        );
        let url = parts.build();
        assert_eq!(url, "/index-1,index-2/type-1,type-2/_search");
    }
}
