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
    use crate::{
        client::{Elasticsearch, Sender},
        Connection, ConnectionSettings,
    };
    use reqwest::StatusCode;
    use serde_json::Value;
    use url::Url;
    use failure;

    #[test]
    fn test_search_with_body() -> Result<(), failure::Error> {

        let client = Elasticsearch::default();
        let mut response = client
            .search::<Value>()
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
            .search::<()>()
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
}
