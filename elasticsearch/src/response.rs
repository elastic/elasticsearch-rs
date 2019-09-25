extern crate reqwest;

use self::reqwest::header::HeaderMap;
use self::reqwest::{Error, Response, StatusCode};
use serde::de::DeserializeOwned;

/// A response from Elasticsearch
pub struct ElasticsearchResponse<T: DeserializeOwned> {
    pub headers: HeaderMap,
    pub status_code: StatusCode,
    pub body: Option<T>,
}
