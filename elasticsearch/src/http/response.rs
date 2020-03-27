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

    /// Get the content-length of this response, if known.
    ///
    /// Reasons it may not be known:
    ///
    /// - The server didn't send a `content-length` header.
    /// - The response is compressed and automatically decoded (thus changing
    ///   the actual decoded length).
    pub fn content_length(&self) -> Option<u64> {
        self.0.content_length()
    }

    /// Get the HTTP status code of the response
    pub fn status_code(&self) -> StatusCode {
        self.0.status()
    }

    /// Turn the response into an [Error] if Elasticsearch returned an error.
    pub fn error_for_status_code(self) -> Result<Self, Error> {
        match self.0.error_for_status_ref() {
            Ok(_) => Ok(self),
            Err(err) => Err(err.into()),
        }
    }

    /// Turn the response into an [Error] if Elasticsearch returned an error.
    pub fn error_for_status_code_ref(&self) -> Result<&Self, Error> {
        match self.0.error_for_status_ref() {
            Ok(_) => Ok(self),
            Err(err) => Err(err.into()),
        }
    }

    /// Get the response headers
    pub fn headers(&self) -> &HeaderMap {
        self.0.headers()
    }

    /// Get the deprecation warning response headers
    pub fn warning_headers(&self) -> impl Iterator<Item = &str> {
        self.headers()
            .get_all("Warning")
            .iter()
            .map(|w| w.to_str().unwrap())
    }

    /// Asynchronously read the response body
    ///
    /// Reading the response body consumes `self`
    pub async fn read_body<B>(self) -> Result<B, Error>
    where
        B: DeserializeOwned,
    {
        let body = self.0.json::<B>().await?;
        Ok(body)
    }
}
