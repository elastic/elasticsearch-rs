//! HTTP response components

extern crate reqwest;

use crate::error::Error;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

/// A response from Elasticsearch
pub struct Response(reqwest::Response);

impl Response {
    /// Creates a new instance of an Elasticsearch response
    pub fn new(response: reqwest::Response) -> Self {
        Self(response)
    }

    /// The HTTP status code of the response
    pub fn status_code(&self) -> StatusCode {
        self.0.status()
    }

    /// The response headers
    pub fn headers(&self) -> &HeaderMap {
        self.0.headers()
    }

    /// Asynchronously read the response body
    pub async fn read_body<B>(self) -> Result<B, Error>
    where
        B: DeserializeOwned,
    {
        let body = self.0.json::<B>().await?;
        Ok(body)
    }
}
