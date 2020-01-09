//! Official Rust client for [Elasticsearch](https://www.elastic.co/products/elasticsearch)
//!
//! `Elasticsearch` is an official Rust client for Elasticsearch, providing an efficient asynchronous
//! client for all stable Elasticsearch APIs that's easy to use.
//!
//! # Versions and Compatibility
//!
//! | Rust client | Elasticsearch |
//! |-------------|---------------|
//! | 7.x         | 7.x           |
//!
//! The major version of the client is compatible with the same major version of Elasticsearch.
//! Since Elasticsearch is developed following [Semantic Versioning](https://semver.org/) principles,
//! Any minor/patch version of the client can be used against any minor/patch version of Elasticsearch
//! **within the same major version lineage**. For example,
//!
//! - A `7.5.0` client can be used against `7.0.0` Elasticsearch
//! - A `7.4.0` client can be used against `7.5.1` Elasticsearch
//!
//! In the former case, a 7.5.0 client may contain additional API functions that are not available
//! in 7.0.0 Elasticsearch. In this case, these APIs cannot be used, but for any APIs available in
//! Elasticsearch, the respective API functions on the client will be compatible.
//!
//! In the latter case, a 7.4.0 client won't contain API functions for APIs that are introduced in
//! Elasticsearch 7.5.0+, but for all other APIs available in Elasticsearch, the respective API
//! functions on the client will be compatible.
//!
//! **No compatibility assurances are given between different major versions of the client and
//! Elasticsearch**. Major differences likely exist between major versions of Elasticsearch, particularly
//! around request and response object formats, but also around API urls and behaviour.
//!
//! # Getting started
//!
//! Add the `elasticsearch` crate and version to Cargo.toml. Choose the version that is compatible with
//! the version of Elasticsearch you're using
//!
//! ```toml,no_run
//! [dependencies]
//! elasticsearch = "7.5.1-alpha.1"
//! ```
//! The following _optional_ dependencies may also be useful to create requests and read responses
//!
//! ```toml,no_run
//! serde = "~1"
//! serde_json = "~1"
//! ```
//! ## Create a client
//!
//! To create a client to make API calls to Elasticsearch running on `http://localhost:9200`
//!
//! ```rust,no_run
//! # use elasticsearch;
//! # use elasticsearch::Error;
//! # use elasticsearch::Elasticsearch;
//! # async fn run() -> Result<(), Error> {
//! let client = Elasticsearch::default();
//! # Ok(())
//! # }
//! ```
//! Alternatively, you can create a client to make API calls against Elasticsearch running on a
//! specific [url::Url]
//!
//! ```rust,no_run
//! # use elasticsearch;
//! # use elasticsearch::Error;
//! # use elasticsearch::Elasticsearch;
//! # use elasticsearch::http::transport::{Transport, SingleNodeConnectionPool};
//! # async fn run() -> Result<(), Error> {
//! let transport = Transport::single_node("https://example.com")?;
//! let client = Elasticsearch::new(transport);
//! # Ok(())
//! # }
//! ```
//!
//! If you're running against an Elasticsearch deployment in [Elastic Cloud](https://www.elastic.co/cloud/),
//! a client can be created using a [Cloud ID](https://www.elastic.co/guide/en/cloud/current/ec-cloud-id.html)
//! and credentials retrieved from the Cloud web console
//!
//! ```rust,no_run
//! # use elasticsearch;
//! # use elasticsearch::Error;
//! # use elasticsearch::Elasticsearch;
//! # use url::Url;
//! # use elasticsearch::http::transport::Transport;
//! # use elasticsearch::auth::Credentials;
//! # async fn run() -> Result<(), Error> {
//! let cloud_id = "cluster_name:Y2xvdWQtZW5kcG9pbnQuZXhhbXBsZSQzZGFkZjgyM2YwNTM4ODQ5N2VhNjg0MjM2ZDkxOGExYQ==";
//! let credentials = Credentials::Basic("<username>".into(), "<password>".into());
//! let transport = Transport::cloud(cloud_id, credentials)?;
//! let client = Elasticsearch::new(transport);
//! # Ok(())
//! # }
//! ```
//!
//! More control over how a [Transport](http::transport::Transport) is built can be
//! achieved using [TransportBuilder](http::transport::TransportBuilder) to build a transport, and
//! passing it to [Elasticsearch::new] create a new instance of [Elasticsearch]
//!
//! ```rust,no_run
//! # use elasticsearch;
//! # use elasticsearch::Error;
//! # use elasticsearch::Elasticsearch;
//! # use url::Url;
//! # use elasticsearch::http::transport::{TransportBuilder, SingleNodeConnectionPool};
//! # use elasticsearch::auth::Credentials;
//! # async fn run() -> Result<(), Error> {
//! let url = Url::parse("https://example.com")?;
//! let conn_pool = SingleNodeConnectionPool::new(url);
//! let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
//! let client = Elasticsearch::new(transport);
//! # Ok(())
//! # }
//! ```
//!
//! ## Making API calls
//!
//! The client exposes all stable Elasticsearch APIs, either on the root [Elasticsearch] client,
//! or on a _namespace_ client that groups related APIs, such as [Cat](cat::Cat), which groups the
//! Cat related APIs. All API functions are `async` and can be `await`ed.
//!
//! The following makes an API call to the cat indices API
//!
//! ```rust,no_run
//! # use elasticsearch;
//! # use elasticsearch::{Elasticsearch, Error, cat::CatIndicesParts};
//! # use url::Url;
//! # use elasticsearch::auth::Credentials;
//! # use serde_json::{json, Value};
//! # async fn run() -> Result<(), Error> {
//! # let client = Elasticsearch::default();
//! let response = client
//!     .cat()
//!     .indices(CatIndicesParts::Index(&["*"]))
//!     .send()
//!     .await?;
//!
//! let response_body = response.read_body::<Value>().await?;
//! for record in response_body.as_array().unwrap() {
//!     // print the name of each index
//!     println!("{}", record["index"].as_str().unwrap());
//! }
//! # Ok(())
//! # }
//! ```
//! For APIs that contain parts of the Url path to be provided by the consumer, the Url path
//! variants are modelled as an `enum`, such as [CatIndicesParts](cat::CatIndicesParts) in the above example, which models
//! the variants of the [CatIndices](cat::CatIndices) API.
//!
//! ### Indexing
//!
//! Indexing a single document can be achieved with the index API
//!
//! ```rust,no_run
//! # use elasticsearch;
//! # use elasticsearch::{Elasticsearch, Error, SearchParts, IndexParts};
//! # use url::Url;
//! # use elasticsearch::auth::Credentials;
//! # use serde_json::{json, Value};
//! # async fn run() -> Result<(), Error> {
//! # let client = Elasticsearch::default();
//! let response = client
//!     .index(IndexParts::IndexId("tweets", "1"))
//!     .body(json!({
//!         "id": 1,
//!         "user": "kimchy",
//!         "post_date": "2009-11-15T00:00:00Z",
//!         "message": "Trying out Elasticsearch, so far so good?"
//!     }))
//!     .send()
//!     .await?;
//!
//! let successful = response.status_code().is_success();
//! # Ok(())
//! # }
//! ```
//!
//! For indexing multiple documents, the bulk API is a better option, allowing multiple operations
//! to be sent in one API call
//!
//! ```rust,no_run
//! # use elasticsearch;
//! # use elasticsearch::{Elasticsearch, Error, IndexParts, BulkParts, http::request::JsonBody};
//! # use url::Url;
//! # use elasticsearch::auth::Credentials;
//! # use serde_json::{json, Value};
//! # async fn run() -> Result<(), Error> {
//! # let client = Elasticsearch::default();
//! let mut body: Vec<JsonBody<_>> = Vec::with_capacity(4);
//!
//! // add the first operation and document
//! body.push(json!({"index": {"_id": "1"}}).into());
//! body.push(json!({
//!     "id": 1,
//!     "user": "kimchy",
//!     "post_date": "2009-11-15T00:00:00Z",
//!     "message": "Trying out Elasticsearch, so far so good?"
//! }).into());
//!
//! // add the second operation and document
//! body.push(json!({"index": {"_id": "2"}}).into());
//! body.push(json!({
//!     "id": 2,
//!     "user": "forloop",
//!     "post_date": "2020-01-08T00:00:00Z",
//!     "message": "Bulk indexing with the rust client, yeah!"
//! }).into());
//!
//! let response = client
//!     .bulk(BulkParts::Index("tweets"))
//!     .body(body)
//!     .send()
//!     .await?;
//!
//! let response_body = response.read_body::<Value>().await?;
//! let successful = response_body["errors"].as_bool().unwrap() == false;
//! # Ok(())
//! # }
//! ```
//! ### Searching
//!
//! The following makes an API call to `tweets/_search` with the json body
//! `{"query":{"match":{"message":"Elasticsearch"}}}`
//!
//! ```rust,no_run
//! # use elasticsearch;
//! # use elasticsearch::{Elasticsearch, Error, SearchParts};
//! # use url::Url;
//! # use elasticsearch::auth::Credentials;
//! # use serde_json::{json, Value};
//! # async fn run() -> Result<(), Error> {
//! # let client = Elasticsearch::default();
//! let response = client
//!     .search(SearchParts::Index(&["tweets"]))
//!     .from(0)
//!     .size(10)
//!     .body(json!({
//!         "query": {
//!             "match": {
//!                 "message": "Elasticsearch rust"
//!             }
//!         }
//!     }))
//!     .send()
//!     .await?;
//!
//! let response_body = response.read_body::<Value>().await?;
//! let took = response_body["took"].as_i64().unwrap();
//! for hit in response_body["hits"]["hits"].as_array().unwrap() {
//!     // print the source document
//!     println!("{:?}", hit["_source"]);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Request bodies
//!
//! For APIs that expect JSON, the `body` associated function of the API constrains the input
//! to a type that implements [serde::Serialize] trait. An example of this was the indexing a single
//! document example above.
//!
//! Some APIs expect newline delimited JSON
//! (NDJSON) however, so the `body` associated for these APIs constrain the input to a vector of
//! types that implement [Body](http::request::Body) trait.  An example of this was the bulk indexing multiple documents
//! above.
//!
//! The [Body](http::request::Body) trait represents the body of an API call, allowing for different body implementations.
//! As well as those to represent JSON and NDJSON, a few other types also have implementations for
//! [Body](http::request::Body), such as byte slice. Whilst these can't be passed to the API functions directly,
//! [Elasticsearch::send] can be used
//!
//! ```rust,no_run
//! # use elasticsearch;
//! # use elasticsearch::{Elasticsearch, Error, SearchParts};
//! # use url::Url;
//! # use elasticsearch::auth::Credentials;
//! # use serde_json::{json, Value};
//! # use http::HeaderMap;
//! # use elasticsearch::http::Method;
//! # async fn run() -> Result<(), Error> {
//! # let client = Elasticsearch::default();
//! let body = b"{\"query\":{\"match_all\":{}}}";
//!
//! let response = client
//!     .send(Method::Post,
//!         SearchParts::Index(&["tweets"]).url().as_ref(),
//!         HeaderMap::new(),
//!         Option::<&Value>::None,
//!         Some(body.as_ref())
//!     )
//!     .await?;
//!
//! # Ok(())
//! # }
//! ```
//!
//!
//!
//!

// also test examples in README
#![feature(external_doc)]
#[doc(include="../../README.md")]
#[cfg(doctest)]
type _DoctestReadme = ();

// TODO: turn on before releasing :) Will require adding documentation within all REST API specs
//#![deny(missing_docs)]

#[macro_use]
extern crate dyn_clone;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub mod auth;
pub mod http;
pub mod cat;
pub mod ccr;
pub mod cluster;
pub mod enrich;
pub mod graph;
pub mod ilm;
pub mod indices;
pub mod ingest;
pub mod license;
pub mod migration;
pub mod ml;
pub mod nodes;
pub mod params;
pub mod security;
pub mod slm;
pub mod snapshot;
pub mod sql;
pub mod ssl;
pub mod tasks;
pub mod watcher;
pub mod xpack;

mod client;
mod error;
mod generated;

// exposes types within modules at the library root level
pub use crate::{
    client::*, error::*, generated::root::*,
    http::transport::DEFAULT_ADDRESS,
};

#[cfg(test)]
pub mod tests {
    use crate::SearchParts;

    #[test]
    fn build_search_on_all_indices_and_types() {
        let parts = SearchParts::None;
        let url = parts.url();
        assert_eq!(url, "/_search");
    }

    #[test]
    fn build_search_on_selected_indices() {
        let parts = SearchParts::Index(&["index-1", "index-2"]);
        let url = parts.url();
        assert_eq!(url, "/index-1,index-2/_search");
    }

    #[test]
    fn build_search_on_selected_indices_and_types() {
        let parts = SearchParts::IndexType(&["index-1", "index-2"], &["type-1", "type-2"]);
        let url = parts.url();
        assert_eq!(url, "/index-1,index-2/type-1,type-2/_search");
    }
}
