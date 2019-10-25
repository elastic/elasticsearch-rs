use crate::{
    connection::Connection, http_method::HttpMethod, response::ElasticsearchResponse,
    settings::ConnectionSettings,
};

use reqwest::{header::HeaderMap, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
use url::Url;

/// Sender trait for terminal method to send the request to Elasticsearch
pub trait Sender {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned;
}

/// Client used to make API calls to Elasticsearch
#[derive(Clone, Debug, Default)]
pub struct Elasticsearch {
    settings: ConnectionSettings,
    connection: Connection,
}

impl Elasticsearch {
    pub fn new<T>(settings: ConnectionSettings, connection: Connection) -> Self {
        Elasticsearch {
            settings,
            connection,
        }
    }

    pub fn send<T>(
        &self,
        method: HttpMethod,
        path: &str,
        query_string: Option<&[(String, String)]>,
        body: Option<Vec<u8>>,
    ) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        self.connection.send(method, path, query_string, body)
    }
}
