//! Cat APIs
//!
//! The [Cat APIs](https://www.elastic.co/guide/en/elasticsearch/reference/master/cat.html) aim to
//! meet the needs of humans when looking at data returned from Elasticsearch,
//! formatting it as compact, column aligned text, making it easier on human eyes.
//!
//! # Headers
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