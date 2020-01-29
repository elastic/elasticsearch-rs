//! HTTP components

pub mod headers;
pub mod request;
pub mod response;
pub mod transport;

/// Http methods supported by Elasticsearch
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
