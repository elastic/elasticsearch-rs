use crate::{
    http::{request::Body, response::Response, transport::Transport, Method, headers::HeaderMap},
    Error,
};

use serde::{Serialize, Serializer};

/// Serializes an Option<&[&str]> with some value to a comma separated string of values.
/// Used to serialize values within the query string
pub fn serialize_coll_qs<S>(
    value: &Option<&[&str]>,
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

/// Root client for top level APIs
#[derive(Clone, Debug, Default)]
pub struct Elasticsearch {
    transport: Transport,
}

impl Elasticsearch {
    /// Creates a new instance of the root client
    pub fn new(transport: Transport) -> Self {
        Elasticsearch { transport }
    }

    /// Creates an asynchronous request that can be awaited
    ///
    /// Accepts the HTTP method and relative path to an API,
    /// and optional query string and body.
    pub async fn send<B, Q>(
        &self,
        method: Method,
        path: &str,
        headers: HeaderMap,
        query_string: Option<&Q>,
        body: Option<B>,
    ) -> Result<Response, Error>
    where
        B: Body,
        Q: Serialize + ?Sized,
    {
        self.transport.send(method, path, headers, query_string, body).await
    }
}
