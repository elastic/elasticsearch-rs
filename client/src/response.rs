extern crate reqwest;

use self::reqwest::{Error, Response, StatusCode};
use self::reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

/// A response from Elasticsearch
pub struct ElasticsearchResponse<T: DeserializeOwned> {
    headers: HeaderMap,
    status_code: StatusCode,
    body: Option<T>
}
