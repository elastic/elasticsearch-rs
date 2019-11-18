use crate::{
    connection::Connection, http_method::HttpMethod, response::ElasticsearchResponse,
    settings::ConnectionSettings, ElasticsearchError,
};

use reqwest::{header::HeaderMap, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer};
use url::Url;

/// Sends a synchronous API request to Elasticsearch
pub trait Sender {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError>;
}

/// Serializes an Option<Vec<String>> with some value to a comma separated string of values.
/// Used to serialize values within the query string
pub fn serialize_vec_qs<S>(value: &Option<Vec<String>>, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
    S: Serializer {
    match value {
        Some(v) => {
            let joined = v.join(",");
            serializer.serialize_str(joined.as_ref())
        }
        None => serializer.serialize_none()
    }
}

/// Client used to make API requests to Elasticsearch
#[derive(Clone, Debug, Default)]
pub struct Elasticsearch {
    settings: ConnectionSettings,
    connection: Connection,
}

impl Elasticsearch {
    /// Creates a new instance of Elasticsearch
    pub fn new(settings: ConnectionSettings, connection: Connection) -> Self {
        Elasticsearch {
            settings,
            connection,
        }
    }

    /// Sends a synchronous API request to Elasticsearch
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
