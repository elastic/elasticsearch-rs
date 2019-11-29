use crate::{
    connection::Connection, http_method::HttpMethod, response::ElasticsearchResponse,
    ElasticsearchError,
};

use reqwest::{header::HeaderMap, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer};
use url::Url;

/// Serializes an Option<Vec<String>> with some value to a comma separated string of values.
/// Used to serialize values within the query string
pub fn serialize_vec_qs<S>(
    value: &Option<Vec<String>>,
    serializer: S,
) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
where
    S: Serializer,
{
    let vec = value
        .as_ref()
        .expect("attempt to serialize Option::None value");
    let joined = vec.join(",");
    serializer.serialize_str(joined.as_ref())
}

/// Client used to make API requests to Elasticsearch
#[derive(Clone, Debug, Default)]
pub struct Elasticsearch {
    connection: Connection,
}

impl Elasticsearch {
    /// Creates a new instance of Elasticsearch
    pub fn new(connection: Connection) -> Self {
        Elasticsearch {
            connection,
        }
    }

    /// Creates an asynchronous request that can be awaited
    pub async fn send<B, Q>(
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
        self.connection.send(method, path, query_string, body).await
    }
}
