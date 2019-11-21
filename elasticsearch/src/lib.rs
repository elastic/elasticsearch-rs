// TODO: whilst developing
#![allow(unused_imports)]
#![allow(dead_code)]

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
mod settings;

pub use crate::{
    client::Elasticsearch, connection::Connection, enums::*, error::ElasticsearchError,
    http_method::HttpMethod, response::ElasticsearchResponse, settings::ConnectionSettings,
};

#[cfg(test)]
pub mod tests {
    // TODO: Need to have a running Elasticsearch instance with some data
    use crate::root::SearchUrlParts;
    use crate::{
        client::{Elasticsearch, Sender},
        Connection, ConnectionSettings,
    };
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
        let parts = SearchUrlParts::Index(vec!["index-1".into(), "index-2".into()]);
        let url = parts.build();
        assert_eq!(url, "/index-1,index-2/_search");
    }

    #[test]
    fn build_search_on_selected_indices_and_types() {
        let parts = SearchUrlParts::IndexType(
            vec!["index-1".into(), "index-2".into()],
            vec!["type-1".into(), "type-2".into()],
        );
        let url = parts.build();
        assert_eq!(url, "/index-1,index-2/type-1,type-2/_search");
    }

    #[test]
    fn test_search_with_body() -> Result<(), failure::Error> {
        let client = Elasticsearch::default();
        let mut response = client
            .search::<Value>(SearchUrlParts::None)
            .body(Some(json!({
                "query": {
                    "match_all": {}
                }
            })))
            .allow_no_indices(Some(true))
            .send()?;

        assert_eq!(response.status_code(), StatusCode::OK);
        let response_body = response.read_body::<Value>()?;
        assert!(response_body["took"].as_i64().unwrap() > 0);

        Ok(())
    }

    #[test]
    fn test_search_no_body() -> Result<(), failure::Error> {
        let client = Elasticsearch::default();
        let mut response = client
            .search::<()>(SearchUrlParts::None)
            .pretty(Some(true))
            .q(Some("title:Elasticsearch".into()))
            .send()?;

        assert_eq!(response.status_code(), StatusCode::OK);
        let response_body = response.read_body::<Value>()?;
        assert!(response_body["took"].as_i64().unwrap() > 0);

        for hit in response_body["hits"]["hits"].as_array().unwrap() {
            assert!(hit["_source"]["title"].as_str().is_some());
        }

        Ok(())
    }

    #[test]
    fn test_serialize_vec_string_on_querystring() -> Result<(), failure::Error> {
        let client = Elasticsearch::default();
        let mut response = client
            .search::<()>(SearchUrlParts::None)
            .pretty(Some(true))
            .filter_path(Some(vec!["took".into()]))
            .q(Some("title:Elasticsearch".into()))
            .send()?;

        assert_eq!(response.status_code(), StatusCode::OK);
        let response_body = response.read_body::<Value>()?;

        assert!(response_body["took"].as_i64().unwrap() > 0);
        assert!(response_body.get("hits").is_none());

        Ok(())
    }

    #[test]
    fn test_clone_search_with_body() -> Result<(), failure::Error> {
        let client = Elasticsearch::default();

        let request = client
            .search::<Value>(SearchUrlParts::None)
            .body(Some(json!({
                "query": {
                    "match_all": {}
                }
            })))
            .allow_no_indices(Some(true));

        let request_clone = request
            .clone()
            .q(Some("title:Elasticsearch".into()))
            .size(Some(1))
            .body(None)
            .allow_no_indices(None);

        let mut response = request_clone.send()?;

        assert_eq!(response.status_code(), StatusCode::OK);
        let response_body = response.read_body::<Value>()?;

        assert_eq!(response_body["hits"]["hits"].as_array().unwrap().len(), 1);

        Ok(())
    }
}
