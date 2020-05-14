//! HTTP response components

use crate::error::Error;
use crate::http::{headers::HeaderMap, Method, StatusCode, Url};
use serde::de::DeserializeOwned;

/// A response from Elasticsearch
pub struct Response(reqwest::Response, Method);

impl Response {
    /// Creates a new instance of an Elasticsearch response
    pub fn new(response: reqwest::Response, method: Method) -> Self {
        Self(response, method)
    }

    /// Get the response content-length, if known.
    ///
    /// Reasons it may not be known:
    ///
    /// - The server didn't send a `content-length` header.
    /// - The response is compressed and automatically decoded (thus changing
    ///   the actual decoded length).
    pub fn content_length(&self) -> Option<u64> {
        self.0.content_length()
    }

    /// Gets the response content-type.
    pub fn content_type(&self) -> &str {
        self.0
            .headers()
            .get(crate::http::headers::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap()
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

    /// Asynchronously reads the response body as JSON
    ///
    /// Reading the response body consumes `self`
    pub async fn json<B>(self) -> Result<B, Error>
    where
        B: DeserializeOwned,
    {
        let body = self.0.json::<B>().await?;
        Ok(body)
    }

    /// Gets the response headers.
    pub fn headers(&self) -> &HeaderMap {
        self.0.headers()
    }

    /// Gets the request method.
    pub fn method(&self) -> Method {
        self.1
    }

    /// Get the HTTP status code of the response
    pub fn status_code(&self) -> StatusCode {
        self.0.status()
    }

    /// Asynchronously reads the response body as plain text
    ///
    /// Reading the response body consumes `self`
    pub async fn text(self) -> Result<String, Error> {
        let body = self.0.text().await?;
        Ok(body)
    }

    /// Gets the request URL
    pub fn url(&self) -> &Url {
        self.0.url()
    }

    /// Gets the Deprecation warning response headers
    ///
    /// Deprecation headers signal the use of Elasticsearch functionality
    /// or features that are deprecated and will be removed in a future release.
    pub fn warning_headers(&self) -> impl Iterator<Item = &str> {
        self.0
            .headers()
            .get_all("Warning")
            .iter()
            .map(|w| w.to_str().unwrap())
    }
}
