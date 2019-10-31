use crate::{
    connection::Connection, http_method::HttpMethod, response::ElasticsearchResponse,
    settings::ConnectionSettings, ElasticsearchError,
};

use reqwest::{header::HeaderMap, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

/// Sender trait for terminal method to send a synchronous request to Elasticsearch
pub trait Sender {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError>;
}

/// Client used to make API calls to Elasticsearch
#[derive(Clone, Debug, Default)]
pub struct Elasticsearch {
    settings: ConnectionSettings,
    connection: Connection,
}

impl Elasticsearch {
    pub fn new(settings: ConnectionSettings, connection: Connection) -> Self {
        Elasticsearch {
            settings,
            connection,
        }
    }

    /// Sends an API request to Elasticsearch
    pub fn send<B, Q>(
        &self,
        method: HttpMethod,
        path: &str,
        query_string: Option<&Q>,
        body: Option<B>,
    ) -> Result<ElasticsearchResponse, ElasticsearchError>
    where
        B: Serialize,
        Q: Serialize + ?Sized,
    {
        self.connection.send(method, path, query_string, body)
    }
}
