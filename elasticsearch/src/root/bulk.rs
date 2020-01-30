use crate::http::request::Body;
use crate::params::VersionType;
use crate::Error;
use bytes::buf::BufMutExt;
use bytes::{BufMut, BytesMut};
use serde;
use serde::{
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};
use serde_with;

/// Bulk operation action
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
enum BulkAction {
    /// Index a document
    #[serde(rename = "index")]
    Index,
    /// Create a new document
    #[serde(rename = "create")]
    Create,
    /// Update an existing document
    #[serde(rename = "update")]
    Update,
    /// Delete an existing document
    #[serde(rename = "delete")]
    Delete,
}

/// Bulk operation metadata
///
/// the specific bulk action metadata such as the id of the source document, index, etc.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Default)]
struct BulkMetadata<'a> {
    _index: Option<&'a str>,
    // TODO: intentionally omit type for now, as it's going away.
    //_type: Option<&'a str>,
    _id: &'a str,
    pipeline: Option<&'a str>,
    if_seq_no: Option<i64>,
    if_primary_term: Option<i64>,
    routing: Option<&'a str>,
    retry_on_conflict: Option<i32>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}

/// Bulk operation header
///
/// The header contains the bulk action and the specific action metadata
/// such as the id of the source document, index, etc.
struct BulkHeader<'a> {
    action: BulkAction,
    metadata: BulkMetadata<'a>,
}

impl<'a> Serialize for BulkHeader<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        let action = match self.action {
            BulkAction::Create => "create",
            BulkAction::Delete => "delete",
            BulkAction::Index => "index",
            BulkAction::Update => "update",
        };
        map.serialize_entry(action, &self.metadata)?;
        map.end()
    }
}

/// Bulk operation
///
/// A bulk operation consists of a header that indicates the bulk action and the related metadata
/// for the action, and an optional source document.
///
/// A collection of bulk operations can be sent to the [Bulk API](struct.Bulk.html) in the body of the API call
///
///
/// # Example
///
/// Using [serde_json]'s `json!` macro to constuct [serde_json::Value] from JSON literals, for
/// the source for each bulk operation
///
/// ```rust,no_run
/// # use elasticsearch::{
/// #     BulkOperation,
/// #     BulkParts,
/// #     auth::Credentials,
/// #     Error, Elasticsearch,
/// #     http::transport::{TransportBuilder,SingleNodeConnectionPool},
/// # };
/// # use url::Url;
/// # use serde_json::{json, Value};
/// # async fn run() -> Result<(), Error> {
/// # let url = Url::parse("https://example.com")?;
/// # let conn_pool = SingleNodeConnectionPool::new(url);
/// # let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
/// # let client = Elasticsearch::new(transport);
/// let mut ops: Vec<BulkOperation<Value>> = Vec::with_capacity(4);
/// ops.push(BulkOperation::index("1", json!({
///         "user": "kimchy",
///         "post_date": "2009-11-15T00:00:00Z",
///         "message": "Trying out Elasticsearch, so far so good?"
///     }))
///     .pipeline("process_tweet")
///     .into()
/// );
/// ops.push(BulkOperation::create("2", json!({
///         "user": "forloop",
///         "post_date": "2020-01-08T00:00:00Z",
///         "message": "Indexing with the rust client, yeah!"
///     }))
///     .pipeline("process_tweet")
///     .into()
/// );
/// ops.push(BulkOperation::update("3", json!({
///         "doc": {
///             "message": "Tweets are _meant_ to be immutable!"
///         },
///         "doc_as_upsert": true
///     }))
///     .into()
/// );
/// ops.push(BulkOperation::delete("4")
///     .index("old_tweets")
///     .into()
/// );
///
/// let bulk_response = client.bulk(BulkParts::Index("tweets"))
///     .body(ops)
///     .send()
///     .await?;
///
/// # Ok(())
/// # }
/// ```
pub struct BulkOperation<'a, B> {
    header: BulkHeader<'a>,
    source: Option<B>,
}

impl<'a, B> BulkOperation<'a, B>
    where
        B: Serialize,
{
    /// Creates a new instance of a bulk create operation
    pub fn create(id: &'a str, source: B) -> BulkCreateOperation<'a, B> {
        BulkCreateOperation::new(id, source)
    }

    /// Creates a new instance of a bulk index operation
    pub fn index(id: &'a str, source: B) -> BulkIndexOperation<'a, B> {
        BulkIndexOperation::new(id, source)
    }

    /// Creates a new instance of a bulk delete operation
    pub fn delete(id: &'a str) -> BulkDeleteOperation<'a, B> {
        BulkDeleteOperation::new(id)
    }

    /// Creates a new instance of a bulk update operation
    pub fn update(id: &'a str, source: B) -> BulkUpdateOperation<'a, B> {
        BulkUpdateOperation::new(id, source)
    }
}

impl<'a, B> Body for BulkOperation<'a, B>
    where
        B: Serialize,
{
    fn write(&self, bytes: &mut BytesMut) -> Result<(), Error> {
        let writer = bytes.writer();
        serde_json::to_writer(writer, &self.header)?;
        bytes.put_u8(b'\n');

        if let Some(source) = &self.source {
            let writer = bytes.writer();
            serde_json::to_writer(writer, source)?;
            bytes.put_u8(b'\n');
        }

        Ok(())
    }
}

/// Bulk create operation
pub struct BulkCreateOperation<'a, B> {
    operation: BulkOperation<'a, B>,
}

impl<'a, B> BulkCreateOperation<'a, B> {
    /// Creates a new instance of [BulkCreateOperation]
    pub fn new(id: &'a str, source: B) -> Self {
        Self {
            operation: BulkOperation {
                header: BulkHeader {
                    action: BulkAction::Create,
                    metadata: BulkMetadata {
                        _id: id,
                        ..Default::default()
                    },
                },
                source: Some(source),
            },
        }
    }

    /// specify the name of the index to perform the bulk update operation against.
    ///
    /// Each bulk operation can specify an index to operate against. If all bulk operations
    /// in one Bulk API call will operate against the same index however, it is better to specify
    /// the index on [Bulk](struct.Bulk.html) using [BulkParts::Index](enum.BulkParts.html),
    /// and omit specifying the same index for each bulk operation.
    pub fn index(mut self, index: &'a str) -> Self {
        self.operation.header.metadata._index = Some(index);
        self
    }

    /// The ID of the pipeline to use to preprocess incoming documents
    pub fn pipeline(mut self, pipeline: &'a str) -> Self {
        self.operation.header.metadata.pipeline = Some(pipeline);
        self
    }

    /// Target the specified primary shard
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.operation.header.metadata.routing = Some(routing);
        self
    }
}

impl<'a, B> From<BulkCreateOperation<'a, B>> for BulkOperation<'a, B> {
    fn from(b: BulkCreateOperation<'a, B>) -> Self {
        b.operation
    }
}

/// Bulk index operation
pub struct BulkIndexOperation<'a, B> {
    operation: BulkOperation<'a, B>,
}

impl<'a, B> BulkIndexOperation<'a, B> {
    /// Creates a new instance of [BulkIndexOperation]
    pub fn new(id: &'a str, source: B) -> Self {
        Self {
            operation: BulkOperation {
                header: BulkHeader {
                    action: BulkAction::Index,
                    metadata: BulkMetadata {
                        _id: id,
                        ..Default::default()
                    },
                },
                source: Some(source),
            },
        }
    }

    /// specify the name of the index to perform the bulk update operation against.
    ///
    /// Each bulk operation can specify an index to operate against. If all bulk operations
    /// in one Bulk API call will operate against the same index however, it is better to specify
    /// the index on [Bulk](struct.Bulk.html) using [BulkParts::Index](enum.BulkParts.html),
    /// and omit specifying the same index for each bulk operation.
    pub fn index(mut self, index: &'a str) -> Self {
        self.operation.header.metadata._index = Some(index);
        self
    }

    /// The ID of the pipeline to use to preprocess incoming documents
    pub fn pipeline(mut self, pipeline: &'a str) -> Self {
        self.operation.header.metadata.pipeline = Some(pipeline);
        self
    }

    /// Target the specified primary shard
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.operation.header.metadata.routing = Some(routing);
        self
    }

    /// specify a sequence number to use for optimistic concurrency control
    pub fn if_seq_no(mut self, seq_no: i64) -> Self {
        self.operation.header.metadata.if_seq_no = Some(seq_no);
        self
    }

    // TODO? Should seq_no and primary_term be set together with one function call?
    /// specify a primary term to use for optimistic concurrency control
    pub fn if_primary_term(mut self, primary_term: i64) -> Self {
        self.operation.header.metadata.if_primary_term = Some(primary_term);
        self
    }

    /// specify a version number to use for optimistic concurrency control
    pub fn version(mut self, version: i64) -> Self {
        self.operation.header.metadata.version = Some(version);
        self
    }

    /// The type of versioning used when a version is specified
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.operation.header.metadata.version_type = Some(version_type);
        self
    }
}

impl<'a, B> From<BulkIndexOperation<'a, B>> for BulkOperation<'a, B> {
    fn from(b: BulkIndexOperation<'a, B>) -> Self {
        b.operation
    }
}

/// Bulk delete operation
pub struct BulkDeleteOperation<'a, B> {
    operation: BulkOperation<'a, B>,
}

impl<'a, B> BulkDeleteOperation<'a, B> {
    /// Creates a new instance of [BulkDeleteOperation]
    pub fn new(id: &'a str) -> Self {
        Self {
            operation: BulkOperation {
                header: BulkHeader {
                    action: BulkAction::Delete,
                    metadata: BulkMetadata {
                        _id: id,
                        ..Default::default()
                    },
                },
                source: Option::<B>::None,
            },
        }
    }

    /// specify the name of the index to perform the bulk update operation against.
    ///
    /// Each bulk operation can specify an index to operate against. If all bulk operations
    /// in one Bulk API call will operate against the same index however, it is better to specify
    /// the index on [Bulk](struct.Bulk.html) using [BulkParts::Index](enum.BulkParts.html),
    /// and omit specifying the same index for each bulk operation.
    pub fn index(mut self, index: &'a str) -> Self {
        self.operation.header.metadata._index = Some(index);
        self
    }

    /// Target the specified primary shard
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.operation.header.metadata.routing = Some(routing);
        self
    }

    /// specify a sequence number to use for optimistic concurrency control
    pub fn if_seq_no(mut self, seq_no: i64) -> Self {
        self.operation.header.metadata.if_seq_no = Some(seq_no);
        self
    }

    // TODO? Should seq_no and primary_term be set together with one function call?
    /// specify a primary term to use for optimistic concurrency control
    pub fn if_primary_term(mut self, primary_term: i64) -> Self {
        self.operation.header.metadata.if_primary_term = Some(primary_term);
        self
    }

    /// specify a version number to use for optimistic concurrency control
    pub fn version(mut self, version: i64) -> Self {
        self.operation.header.metadata.version = Some(version);
        self
    }

    /// The type of versioning used when a version is specified
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.operation.header.metadata.version_type = Some(version_type);
        self
    }
}

impl<'a, B> From<BulkDeleteOperation<'a, B>> for BulkOperation<'a, B> {
    fn from(b: BulkDeleteOperation<'a, B>) -> Self {
        b.operation
    }
}

/// Bulk update operation
pub struct BulkUpdateOperation<'a, B> {
    operation: BulkOperation<'a, B>,
}

impl<'a, B> BulkUpdateOperation<'a, B>
    where
        B: serde::Serialize,
{
    /// Creates a new instance of [BulkUpdateOperation]
    pub fn new(id: &'a str, source: B) -> Self {
        Self {
            operation: BulkOperation {
                header: BulkHeader {
                    action: BulkAction::Update,
                    metadata: BulkMetadata {
                        _id: id,
                        ..Default::default()
                    },
                },
                source: Some(source),
            },
        }
    }

    /// specify the name of the index to perform the bulk update operation against.
    ///
    /// Each bulk operation can specify an index to operate against. If all bulk operations
    /// in one Bulk API call will operate against the same index however, it is better to specify
    /// the index on [Bulk](struct.Bulk.html) using [BulkParts::Index](enum.BulkParts.html),
    /// and omit specifying the same index for each bulk operation.
    pub fn index(mut self, index: &'a str) -> Self {
        self.operation.header.metadata._index = Some(index);
        self
    }

    /// Target the specified primary shard
    pub fn routing(mut self, routing: &'a str) -> Self {
        self.operation.header.metadata.routing = Some(routing);
        self
    }

    /// specify a sequence number to use for optimistic concurrency control
    pub fn if_seq_no(mut self, seq_no: i64) -> Self {
        self.operation.header.metadata.if_seq_no = Some(seq_no);
        self
    }

    // TODO? Should seq_no and primary_term be set together with one function call?
    /// specify a primary term to use for optimistic concurrency control
    pub fn if_primary_term(mut self, primary_term: i64) -> Self {
        self.operation.header.metadata.if_primary_term = Some(primary_term);
        self
    }

    /// specify a version number to use for optimistic concurrency control
    pub fn version(mut self, version: i64) -> Self {
        self.operation.header.metadata.version = Some(version);
        self
    }

    /// The type of versioning used when a version is specified
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.operation.header.metadata.version_type = Some(version_type);
        self
    }

    /// specify how many times an update should be retried in the case of a version conflict
    pub fn retry_on_conflict(mut self, retry_on_conflict: i32) -> Self {
        self.operation.header.metadata.retry_on_conflict = Some(retry_on_conflict);
        self
    }
}

impl<'a, B> From<BulkUpdateOperation<'a, B>> for BulkOperation<'a, B> {
    fn from(b: BulkUpdateOperation<'a, B>) -> Self {
        b.operation
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        http::request::{Body, NdBody},
        BulkOperation, Error,
    };
    use bytes::{BufMut, Bytes, BytesMut};
    use serde::Serialize;
    use serde_json::{json, Value};
    use std::{cmp::Ordering, str};

    pub fn compare(a: &[u8], b: &[u8]) -> Ordering {
        a.iter()
            .zip(b)
            .map(|(x, y)| x.cmp(y))
            .find(|&ord| ord != Ordering::Equal)
            .unwrap_or(a.len().cmp(&b.len()))
    }

    #[test]
    fn serialize_bulk_operations_with_same_type_writes_to_bytes() -> Result<(), failure::Error> {
        let mut bytes = BytesMut::new();
        let mut ops: Vec<BulkOperation<Value>> = Vec::with_capacity(4);

        ops.push(BulkOperation::index("1", json!({ "foo": "index" })).into());
        ops.push(BulkOperation::create("2", json!({ "bar": "create" })).into());
        ops.push(BulkOperation::update("3", json!({ "baz": "update" })).into());
        ops.push(BulkOperation::delete("4").into());

        let body = NdBody(ops);
        let _ = body.write(&mut bytes)?;

        let mut expected = BytesMut::new();
        expected.put_slice(b"{\"index\":{\"_id\":\"1\"}}\n");
        expected.put_slice(b"{\"foo\":\"index\"}\n");
        expected.put_slice(b"{\"create\":{\"_id\":\"2\"}}\n");
        expected.put_slice(b"{\"bar\":\"create\"}\n");
        expected.put_slice(b"{\"update\":{\"_id\":\"3\"}}\n");
        expected.put_slice(b"{\"baz\":\"update\"}\n");
        expected.put_slice(b"{\"delete\":{\"_id\":\"4\"}}\n");

        assert_eq!(
            compare(&expected[..], &bytes[..]),
            Ordering::Equal,
            "expected {} but found {}",
            str::from_utf8(&expected[..]).unwrap(),
            str::from_utf8(&bytes[..]).unwrap()
        );
        Ok(())
    }

    #[test]
    fn serialize_bulk_operations_with_different_types_writes_to_bytes() -> Result<(), failure::Error>
    {
        #[derive(Serialize)]
        struct IndexDoc<'a> {
            foo: &'a str,
        }
        #[derive(Serialize)]
        struct CreateDoc<'a> {
            bar: &'a str,
        }
        #[derive(Serialize)]
        struct UpdateDoc<'a> {
            baz: &'a str,
        }

        struct BulkOperations {
            buf: BytesMut,
        }

        impl BulkOperations {
            pub fn new() -> Self {
                Self {
                    buf: BytesMut::new(),
                }
            }

            pub fn push<B>(&mut self, op: BulkOperation<B>) -> Result<(), Error>
                where
                    B: Serialize,
            {
                op.write(&mut self.buf)
            }
        }

        impl Body for BulkOperations {
            fn bytes(&self) -> Option<Bytes> {
                Some(self.buf.clone().freeze())
            }

            fn write(&self, bytes: &mut BytesMut) -> Result<(), Error> {
                self.buf.write(bytes)
            }
        }

        let mut bytes = BytesMut::new();
        let mut ops = BulkOperations::new();

        ops.push(
            BulkOperation::index("1", IndexDoc { foo: "index" })
                .pipeline("pipeline")
                .index("index_doc")
                .routing("routing")
                .into(),
        )?;
        ops.push(BulkOperation::create("2", CreateDoc { bar: "create" }).into())?;
        ops.push(BulkOperation::update("3", UpdateDoc { baz: "update" }).into())?;
        ops.push(BulkOperation::<()>::delete("4").into())?;

        let body = NdBody(vec![ops]);
        let _ = body.write(&mut bytes)?;

        let mut expected = BytesMut::new();
        expected.put_slice(b"{\"index\":{\"_index\":\"index_doc\",\"_id\":\"1\",\"pipeline\":\"pipeline\",\"routing\":\"routing\"}}\n");
        expected.put_slice(b"{\"foo\":\"index\"}\n");
        expected.put_slice(b"{\"create\":{\"_id\":\"2\"}}\n");
        expected.put_slice(b"{\"bar\":\"create\"}\n");
        expected.put_slice(b"{\"update\":{\"_id\":\"3\"}}\n");
        expected.put_slice(b"{\"baz\":\"update\"}\n");
        expected.put_slice(b"{\"delete\":{\"_id\":\"4\"}}\n");

        assert_eq!(
            compare(&expected[..], &bytes[..]),
            Ordering::Equal,
            "expected {} but found {}",
            str::from_utf8(&expected[..]).unwrap(),
            str::from_utf8(&bytes[..]).unwrap()
        );
        Ok(())
    }
}