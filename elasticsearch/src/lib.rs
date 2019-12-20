//! Official Rust client fo [Elasticsearch](https://www.elastic.co/products/elasticsearch)
//!
//! # Versions and Compatibility
//!
//! | Rust client | Elasticsearch |
//! |-------------|---------------|
//! | 7.x         | 7.x           |
//!
//! # Getting started
//!
//! Add the crate name and version to Cargo.toml. Choose the version that is compatible with
//! the version of Elasticsearch you're using
//!
//! ```toml,no_run
//! [dependencies]
//! elasticsearch = "7.5.0-alpha.1"
//! ```
//! The following dependencies may also be useful
//!
//! ```toml,no_run
//! serde = "~1"
//! serde_json = "~1"
//! ```
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
//! You can create a client to make API calls against Elasticsearch running on a
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
//! and credentials
//!
//! ```rust,no_run
//! # use elasticsearch;
//! # use elasticsearch::Error;
//! # use elasticsearch::Elasticsearch;
//! # use url::Url;
//! # use elasticsearch::http::transport::{Transport};
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
//! More control over how the [Transport](http::transport::Transport) is configured can be
//! achieved using [TransportBuilder](http::transport::TransportBuilder)
//!
//! ```rust,no_run
//! # use elasticsearch;
//! # use elasticsearch::Error;
//! # use elasticsearch::Elasticsearch;
//! # use url::Url;
//! # use elasticsearch::http::transport::{TransportBuilder, SingleNodeConnectionPool};
//! # use elasticsearch::auth::Credentials;
//! # async fn run() -> Result<(), Error> {
//! let conn_pool = SingleNodeConnectionPool::new(Url::parse("https://example.com")?);
//! let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
//! let client = Elasticsearch::new(transport);
//! # Ok(())
//! # }
//! ```
//!
//! # Making API calls
//!
//! The client exposes all stable Elasticsearch APIs, either on the root [Elasticsearch] client,
//! or on a _namespace_ client such as [Cat](cat::Cat), that groups related APIs. All API functions
//! are `async` and can be `await`ed
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
//!                 "message": "Elasticsearch"
//!             }
//!         }
//!     }))
//!     .send()
//!     .await?;
//!
//! let response_body = response.read_body::<Value>().await?;
//! let took = response_body["took"].as_i64().unwrap();
//! # Ok(())
//! # }
//! ```
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
extern crate objekt;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub mod auth;
pub mod http;

mod client;
mod error;
mod generated;

// exposes types within modules at the library root level
pub use crate::{
    client::*, error::*, generated::namespace_clients::*, generated::params, generated::root::*,
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
