pub mod headers;
pub mod request;
pub mod response;
pub mod transport;

/// Http methods supported by Elasticsearch
pub enum Method {
    Get,
    Put,
    Post,
    Delete,
    Head,
}
