/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
use crate::{
    http::request::Body,
    params::{SourceFilter, VersionType},
    Error,
};
use bytes::{BufMut, Bytes, BytesMut};
use serde::{
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};

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
struct BulkMetadata {
    _index: Option<String>,
    // TODO: intentionally omit type for now, as it's going away.
    //_type: Option<String>,
    _id: Option<String>,
    pipeline: Option<String>,
    if_seq_no: Option<i64>,
    if_primary_term: Option<i64>,
    routing: Option<String>,
    retry_on_conflict: Option<i32>,
    _source: Option<SourceFilter>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}

/// Bulk operation header
///
/// The header contains the bulk action and the specific action metadata
/// such as the id of the source document, index, etc.
struct BulkHeader {
    action: BulkAction,
    metadata: BulkMetadata,
}

impl Serialize for BulkHeader {
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

/// A bulk operation consists of a header that indicates the bulk action and the related metadata
/// for the action, and an optional source document.
///
/// A collection of bulk operations can be sent to the [Bulk API](struct.Bulk.html) in the body of the API call.
///
/// For serializing a collection of bulk operations that model the source document of each bulk operation
/// using different structs, take a look at [BulkOperations].
///
/// # Example
///
/// Using [serde_json]'s `json!` macro to constuct [serde_json::Value] from JSON literals, for
/// the source document of each bulk operation
///
/// ```rust,no_run
/// # use elasticsearch::{
/// #     BulkOperation,
/// #     BulkParts,
/// #     Error, Elasticsearch,
/// # };
/// # use url::Url;
/// # use serde_json::{json, Value};
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = Elasticsearch::default();
/// let mut ops: Vec<BulkOperation<Value>> = Vec::with_capacity(4);
/// ops.push(BulkOperation::index(json!({
///         "user": "kimchy",
///         "post_date": "2009-11-15T00:00:00Z",
///         "message": "Trying out Elasticsearch, so far so good?"
///     }))
///     .id("1")
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
pub struct BulkOperation<B> {
    header: BulkHeader,
    source: Option<B>,
}

impl<B> BulkOperation<B>
where
    B: Serialize,
{
    /// Creates a new instance of a [bulk create operation](BulkCreateOperation)
    pub fn create<S>(id: S, source: B) -> BulkCreateOperation<B>
    where
        S: Into<String>,
    {
        BulkCreateOperation::new(id, source)
    }

    /// Creates a new instance of a [bulk index operation](BulkIndexOperation)
    pub fn index(source: B) -> BulkIndexOperation<B> {
        BulkIndexOperation::new(source)
    }

    /// Creates a new instance of a [bulk delete operation](BulkDeleteOperation)
    pub fn delete<S>(id: S) -> BulkDeleteOperation<B>
    where
        S: Into<String>,
    {
        BulkDeleteOperation::new(id)
    }

    /// Creates a new instance of a [bulk update operation](BulkUpdateOperation)
    pub fn update<S>(id: S, source: B) -> BulkUpdateOperation<B>
    where
        S: Into<String>,
    {
        BulkUpdateOperation::new(id, source)
    }
}

impl<B> Body for BulkOperation<B>
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
pub struct BulkCreateOperation<B> {
    operation: BulkOperation<B>,
}

impl<B> BulkCreateOperation<B> {
    /// Creates a new instance of [BulkCreateOperation]
    pub fn new<S>(id: S, source: B) -> Self
    where
        S: Into<String>,
    {
        Self {
            operation: BulkOperation {
                header: BulkHeader {
                    action: BulkAction::Create,
                    metadata: BulkMetadata {
                        _id: Some(id.into()),
                        ..Default::default()
                    },
                },
                source: Some(source),
            },
        }
    }

    /// Specify the name of the index to perform the bulk update operation against.
    ///
    /// Each bulk operation can specify an index to operate against. If all bulk operations
    /// in one Bulk API call will operate against the same index, specify
    /// the index on [Bulk](struct.Bulk.html) using [BulkParts::Index](enum.BulkParts.html),
    /// and omit specifying the index on each bulk operation.
    pub fn index<S>(mut self, index: S) -> Self
    where
        S: Into<String>,
    {
        self.operation.header.metadata._index = Some(index.into());
        self
    }

    /// The ID of the pipeline to use to preprocess incoming documents
    pub fn pipeline<S>(mut self, pipeline: S) -> Self
    where
        S: Into<String>,
    {
        self.operation.header.metadata.pipeline = Some(pipeline.into());
        self
    }

    /// Target the specified primary shard
    pub fn routing<S>(mut self, routing: S) -> Self
    where
        S: Into<String>,
    {
        self.operation.header.metadata.routing = Some(routing.into());
        self
    }
}

impl<B> From<BulkCreateOperation<B>> for BulkOperation<B> {
    fn from(b: BulkCreateOperation<B>) -> Self {
        b.operation
    }
}

/// Bulk index operation
pub struct BulkIndexOperation<B> {
    operation: BulkOperation<B>,
}

impl<B> BulkIndexOperation<B> {
    /// Creates a new instance of [BulkIndexOperation]
    pub fn new(source: B) -> Self {
        Self {
            operation: BulkOperation {
                header: BulkHeader {
                    action: BulkAction::Index,
                    metadata: BulkMetadata {
                        ..Default::default()
                    },
                },
                source: Some(source),
            },
        }
    }

    /// Specify the id for the document
    ///
    /// If an id is not specified, Elasticsearch will generate an id for the document
    /// which will be returned in the response.
    pub fn id<S>(mut self, id: S) -> Self
    where
        S: Into<String>,
    {
        self.operation.header.metadata._id = Some(id.into());
        self
    }

    /// Specify the name of the index to perform the bulk update operation against.
    ///
    /// Each bulk operation can specify an index to operate against. If all bulk operations
    /// in one Bulk API call will operate against the same index, specify
    /// the index on [Bulk](struct.Bulk.html) using [BulkParts::Index](enum.BulkParts.html),
    /// and omit specifying the index on each bulk operation.
    pub fn index<S>(mut self, index: S) -> Self
    where
        S: Into<String>,
    {
        self.operation.header.metadata._index = Some(index.into());
        self
    }

    /// The ID of the pipeline to use to preprocess incoming documents
    pub fn pipeline<S>(mut self, pipeline: S) -> Self
    where
        S: Into<String>,
    {
        self.operation.header.metadata.pipeline = Some(pipeline.into());
        self
    }

    /// Target the specified primary shard
    pub fn routing<S>(mut self, routing: S) -> Self
    where
        S: Into<String>,
    {
        self.operation.header.metadata.routing = Some(routing.into());
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

impl<B> From<BulkIndexOperation<B>> for BulkOperation<B> {
    fn from(b: BulkIndexOperation<B>) -> Self {
        b.operation
    }
}

/// Bulk delete operation
///
/// The bulk delete operation is generic over `B` to allow delete operations to be specified
/// in a collection of operations over `B`, even though the source of any delete operation will
/// always be `None`
pub struct BulkDeleteOperation<B> {
    operation: BulkOperation<B>,
}

impl<B> BulkDeleteOperation<B> {
    /// Creates a new instance of [BulkDeleteOperation]
    pub fn new<S>(id: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            operation: BulkOperation {
                header: BulkHeader {
                    action: BulkAction::Delete,
                    metadata: BulkMetadata {
                        _id: Some(id.into()),
                        ..Default::default()
                    },
                },
                source: Option::<B>::None,
            },
        }
    }

    /// Specify the name of the index to perform the bulk update operation against.
    ///
    /// Each bulk operation can specify an index to operate against. If all bulk operations
    /// in one Bulk API call will operate against the same index, specify
    /// the index on [Bulk](struct.Bulk.html) using [BulkParts::Index](enum.BulkParts.html),
    /// and omit specifying the index on each bulk operation.
    pub fn index<S>(mut self, index: S) -> Self
    where
        S: Into<String>,
    {
        self.operation.header.metadata._index = Some(index.into());
        self
    }

    /// Target the specified primary shard
    pub fn routing<S>(mut self, routing: S) -> Self
    where
        S: Into<String>,
    {
        self.operation.header.metadata.routing = Some(routing.into());
        self
    }

    /// Specify a sequence number to use for optimistic concurrency control
    pub fn if_seq_no(mut self, seq_no: i64) -> Self {
        self.operation.header.metadata.if_seq_no = Some(seq_no);
        self
    }

    // TODO? Should seq_no and primary_term be set together with one function call?
    /// Specify a primary term to use for optimistic concurrency control
    pub fn if_primary_term(mut self, primary_term: i64) -> Self {
        self.operation.header.metadata.if_primary_term = Some(primary_term);
        self
    }

    /// Specify a version number to use for optimistic concurrency control
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

impl<B> From<BulkDeleteOperation<B>> for BulkOperation<B> {
    fn from(b: BulkDeleteOperation<B>) -> Self {
        b.operation
    }
}

/// Bulk update operation
pub struct BulkUpdateOperation<B> {
    operation: BulkOperation<B>,
}

impl<B> BulkUpdateOperation<B>
where
    B: serde::Serialize,
{
    /// Creates a new instance of [BulkUpdateOperation]
    pub fn new<S>(id: S, source: B) -> Self
    where
        S: Into<String>,
    {
        Self {
            operation: BulkOperation {
                header: BulkHeader {
                    action: BulkAction::Update,
                    metadata: BulkMetadata {
                        _id: Some(id.into()),
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
    /// in one Bulk API call will operate against the same index, specify
    /// the index on [Bulk](struct.Bulk.html) using [BulkParts::Index](enum.BulkParts.html),
    /// and omit specifying the index on each bulk operation.
    pub fn index<S>(mut self, index: S) -> Self
    where
        S: Into<String>,
    {
        self.operation.header.metadata._index = Some(index.into());
        self
    }

    /// Target the specified primary shard
    pub fn routing<S>(mut self, routing: S) -> Self
    where
        S: Into<String>,
    {
        self.operation.header.metadata.routing = Some(routing.into());
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

    /// specify how the `_source` field is returned for the update operation.
    ///
    /// This can also be specified as part of the update action source payload instead.
    pub fn source<S>(mut self, source: S) -> Self
    where
        S: Into<SourceFilter>,
    {
        self.operation.header.metadata._source = Some(source.into());
        self
    }
}

impl<B> From<BulkUpdateOperation<B>> for BulkOperation<B> {
    fn from(b: BulkUpdateOperation<B>) -> Self {
        b.operation
    }
}

/// A collection of bulk operations.
///
/// A collection of bulk operations can perform operations against multiple different indices,
/// specifying a different source document for each. When modelling source documents with
/// different structs, it becomes difficult to construct a collection of bulk operations with such
/// a setup. [BulkOperations] alleviates this difficulty by serializing bulk operations ahead of
/// time of the bulk API call, into an internal byte buffer, using the buffered bytes as the body of
/// the bulk API call.
///
/// # Example
///
/// Using [BulkOperations] to construct a collection of bulk operations that use different
/// structs to model source documents
///
/// ```rust,no_run
/// # use elasticsearch::{
/// #     BulkOperation,
/// #     BulkOperations,
/// #     BulkParts,
/// #     Error, Elasticsearch,
/// # };
/// # use serde::Serialize;
/// # use serde_json::{json, Value};
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = Elasticsearch::default();
/// #[derive(Serialize)]
/// struct IndexDoc<'a> {
///     foo: &'a str,
/// }
///
/// #[derive(Serialize)]
/// struct CreateDoc<'a> {
///     bar: &'a str,
/// }
///
/// #[derive(Serialize)]
/// struct UpdateDoc<'a> {
///     baz: &'a str,
/// }
///
/// let mut ops = BulkOperations::new();
/// ops.push(BulkOperation::index(IndexDoc { foo: "index" })
///     .id("1")
///     .pipeline("pipeline")
///     .index("index_doc")
///     .routing("routing")
/// )?;
/// ops.push(BulkOperation::create("2", CreateDoc { bar: "create" }))?;
/// ops.push(BulkOperation::update("3", UpdateDoc { baz: "update" }))?;
/// ops.push(BulkOperation::<()>::delete("4"))?;
///
/// let bulk_response = client.bulk(BulkParts::Index("tweets"))
///     .body(vec![ops])
///     .send()
///     .await?;
///
/// # Ok(())
/// # }
/// ```
pub struct BulkOperations {
    buf: BytesMut,
}

impl BulkOperations {
    /// Initializes a new instance of [BulkOperations]
    pub fn new() -> Self {
        Self {
            buf: BytesMut::new(),
        }
    }

    /// Initializes a new instance of [BulkOperations], using the passed
    /// [bytes::BytesMut] as the buffer to write operations to
    pub fn with_bytes(buf: BytesMut) -> Self {
        Self { buf }
    }

    /// Pushes a bulk operation into the collection of bulk operations.
    ///
    /// The operation is serialized and written to the underlying byte buffer.
    pub fn push<O, B>(&mut self, op: O) -> Result<(), Error>
    where
        O: Into<BulkOperation<B>>,
        B: Serialize,
    {
        op.into().write(&mut self.buf)
    }
}

impl Default for BulkOperations {
    fn default() -> Self {
        Self::new()
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

#[cfg(test)]
mod tests {
    use crate::{
        http::request::{Body, NdBody},
        params::VersionType,
        BulkOperation, BulkOperations,
    };
    use bytes::{BufMut, BytesMut};
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

        ops.push(
            BulkOperation::index(json!({ "foo": "index" }))
                .id("1")
                .pipeline("pipeline")
                .routing("routing")
                .if_seq_no(1)
                .if_primary_term(2)
                .version(3)
                .version_type(VersionType::Internal)
                .into(),
        );
        ops.push(
            BulkOperation::create("2", json!({ "bar": "create" }))
                .pipeline("pipeline")
                .routing("routing")
                .index("create_index")
                .into(),
        );
        ops.push(
            BulkOperation::update("3", json!({ "baz": "update_1" }))
                .source(false)
                .into(),
        );
        ops.push(
            BulkOperation::update("4", json!({ "baz": "update_2" }))
                .source("baz")
                .into(),
        );
        ops.push(
            BulkOperation::update("5", json!({ "baz": "update_3" }))
                .source(vec!["baz"])
                .into(),
        );
        ops.push(
            BulkOperation::update("6", json!({ "baz": "update_4" }))
                .source((vec!["baz"], vec!["bar"]))
                .into(),
        );
        ops.push(BulkOperation::delete("7").into());

        let body = NdBody(ops);
        let _ = body.write(&mut bytes)?;

        let mut expected = BytesMut::new();
        expected.put_slice(b"{\"index\":{\"_id\":\"1\",\"pipeline\":\"pipeline\",\"if_seq_no\":1,\"if_primary_term\":2,\"routing\":\"routing\",\"version\":3,\"version_type\":\"internal\"}}\n");
        expected.put_slice(b"{\"foo\":\"index\"}\n");
        expected.put_slice(b"{\"create\":{\"_index\":\"create_index\",\"_id\":\"2\",\"pipeline\":\"pipeline\",\"routing\":\"routing\"}}\n");
        expected.put_slice(b"{\"bar\":\"create\"}\n");
        expected.put_slice(b"{\"update\":{\"_id\":\"3\",\"_source\":false}}\n");
        expected.put_slice(b"{\"baz\":\"update_1\"}\n");
        expected.put_slice(b"{\"update\":{\"_id\":\"4\",\"_source\":\"baz\"}}\n");
        expected.put_slice(b"{\"baz\":\"update_2\"}\n");
        expected.put_slice(b"{\"update\":{\"_id\":\"5\",\"_source\":[\"baz\"]}}\n");
        expected.put_slice(b"{\"baz\":\"update_3\"}\n");
        expected.put_slice(b"{\"update\":{\"_id\":\"6\",\"_source\":{\"includes\":[\"baz\"],\"excludes\":[\"bar\"]}}}\n");
        expected.put_slice(b"{\"baz\":\"update_4\"}\n");
        expected.put_slice(b"{\"delete\":{\"_id\":\"7\"}}\n");

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

        let mut bytes = BytesMut::new();
        let mut ops = BulkOperations::new();

        ops.push(
            BulkOperation::index(IndexDoc { foo: "index" })
                .id("1")
                .pipeline("pipeline")
                .index("index_doc")
                .routing("routing"),
        )?;
        ops.push(BulkOperation::create("2", CreateDoc { bar: "create" }))?;
        ops.push(BulkOperation::update("3", UpdateDoc { baz: "update" }))?;
        ops.push(BulkOperation::<()>::delete("4"))?;

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
