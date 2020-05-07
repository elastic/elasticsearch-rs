use crate::{
    http::{headers::HeaderMap, request::Body, response::Response, transport::Transport, Method},
    Error,
};

use serde::{Serialize, Serializer};

/// Serializes an `Option<&[Serialize]>` with
/// `Some(value)` to a comma separated string of values.
/// Used to serialize values within the query string
pub(crate) fn serialize_coll_qs<S, T>(
    value: &Option<&[T]>,
    serializer: S,
) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
where
    S: Serializer,
    T: Serialize
{
    let vec = value
        .expect("attempt to serialize Option::None value");

    // TODO: There must be a better way of serializing a Vec<Serialize> to a comma-separated url encoded string...
    // (mis)use serde_json to_string and trim the surrounding quotes...
    let serialized = vec
        .iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect::<Vec<_>>();

    let target = serialized
        .iter()
        .map(|s| s.trim_matches('"'))
        .collect::<Vec<_>>()
        .join(",");

    serializer.serialize_str(&target)
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
        self.transport
            .send(method, path, headers, query_string, body)
            .await
    }
}
