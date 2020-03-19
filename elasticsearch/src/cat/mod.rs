//! Cat APIs
//!
//! The [Cat APIs](https://www.elastic.co/guide/en/elasticsearch/reference/master/cat.html) aim to
//! meet the needs of humans when looking at data returned from Elasticsearch,
//! formatting it as compact, column aligned text, making it easier on human eyes.
//!
//! # Plain text responses
//!
//! By default, all Cat APIs are configured to send requests with `text/plain` content-type
//! and accept headers, returning plain text responses
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
//!     .cat()
//!     .nodes()
//!     .send()
//!     .await?;
//!
//! let response_body = response.read_body_as_text().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # JSON responses
//!
//! JSON responses can be returned from Cat APIs either by using `.format("json")`
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
//!     .cat()
//!     .nodes()
//!     .format("json")
//!     .send()
//!     .await?;
//!
//! let response_body = response.read_body::<Value>().await?;
//! # Ok(())
//! # }
//! ```
//!
//! Or by setting an accept header using `.headers()`
//!
//! ```rust,no_run
//! # use elasticsearch;
//! # use elasticsearch::{Elasticsearch, Error, SearchParts, http::headers::{HeaderValue, DEFAULT_ACCEPT, ACCEPT}};
//! # use url::Url;
//! # use elasticsearch::auth::Credentials;
//! # use serde_json::{json, Value};
//! # async fn run() -> Result<(), Error> {
//! # let client = Elasticsearch::default();
//! let response = client
//!     .cat()
//!     .nodes()
//!     .header(ACCEPT, HeaderValue::from_static(DEFAULT_ACCEPT))
//!     .send()
//!     .await?;
//!
//! let response_body = response.read_body::<Value>().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Column Headers
//!
//! The column headers to return can be controlled with `.h()`
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
//!     .cat()
//!     .nodes()
//!     .h(&["ip", "port", "heapPercent", "name"])
//!     .send()
//!     .await?;
//!
//! let response_body = response.read_body::<String>().await?;
//! # Ok(())
//! # }
//! ```
//!
//!

pub use super::generated::namespace_clients::cat::*;
