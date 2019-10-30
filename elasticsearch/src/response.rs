extern crate reqwest;

use self::reqwest::header::HeaderMap;
use self::reqwest::{Error, Response, StatusCode};
use crate::error::ElasticsearchError;
use serde::de::DeserializeOwned;

/// An Elasticsearch response
pub struct ElasticsearchResponse(reqwest::Response);

impl ElasticsearchResponse {
    /// Creates a new instance of an Elasticsearch response
    pub fn new(response: Response) -> Self {
        ElasticsearchResponse(response)
    }

    /// The HTTP status code of the response
    pub fn status_code(&self) -> StatusCode {
        self.0.status()
    }

    /// The response headers
    pub fn headers(&self) -> &HeaderMap {
        self.0.headers()
    }

    /// Reads the response body
    pub fn read_body<R>(&mut self) -> Result<R, ElasticsearchError>
    where
        R: DeserializeOwned,
    {
        Ok(self.0.json::<R>()?)
    }
}
