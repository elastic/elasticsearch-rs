//! HTTP components

pub mod headers;
pub mod request;
pub mod response;
pub mod transport;

pub use reqwest::StatusCode;
pub use url::Url;

/// Http methods supported by Elasticsearch
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Method {
    /// get
    Get,
    /// put
    Put,
    /// post
    Post,
    /// delete
    Delete,
    /// head
    Head,
}
