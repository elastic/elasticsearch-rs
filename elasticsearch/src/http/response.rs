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

    /// Turn the response into an `Error` if Elasticsearch returned an error.
    pub fn error_for_status_code(self) -> Result<Self, Error> {
        match self.0.error_for_status_ref() {
            Ok(_) => Ok(self),
            Err(err) => Err(err.into()),
        }
    }

    /// Turn the response into an `Error` if Elasticsearch returned an error.
    pub fn error_for_status_code_ref(&self) -> Result<&Self, Error> {
        match self.0.error_for_status_ref() {
            Ok(_) => Ok(self),
            Err(err) => Err(err.into()),
        }
    }

    /// The response headers
    pub fn headers(&self) -> &HeaderMap {
        self.0.headers()
    }

    /// Asynchronously reads the response body as JSON
    ///
    /// Reading the response body consumes `self`
    pub async fn read_body<B>(self) -> Result<B, Error>
    where
        B: DeserializeOwned,
    {
        let body = self.0.json::<B>().await?;
        Ok(body)
    }

    /// Asynchronously reads the response body as plain text
    ///
    /// Reading the response body consumes `self`
    pub async fn read_body_as_text(self) -> Result<String, Error> {
        let body = self.0.text().await?;
        Ok(body)
    }
}
