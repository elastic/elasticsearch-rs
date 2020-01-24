//! HTTP header names and values, including those specific to Elasticsearch

pub use reqwest::header::*;

/// The default user-agent header value sent by the client
pub static DEFAULT_USER_AGENT: &str = concat!("elasticsearch-rs/", env!("CARGO_PKG_VERSION"));

/// The default content-type header value of `application/json`
pub static DEFAULT_CONTENT_TYPE: &str = "application/json";

/// The default accept header value of `application/json`
pub static DEFAULT_ACCEPT: &str = "application/json";

/// The X-Opaque-Id header name, used to track certain calls, or associate
/// certain tasks with a client that started them.
pub static X_OPAQUE_ID: &str = "x-opaque-id";
