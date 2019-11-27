/* Error type based on error type from es-rs:
 *
 * Copyright 2015-2018 Ben Ashford
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use serde_json;
use std::error::Error;
use std::fmt;
use std::io::{self, Read};

/// Error that can occur include IO and parsing errors, as well as specific
/// errors from the Elasticsearch server and logic errors from this library
#[derive(Debug)]
pub enum ElasticsearchError {
    /// An internal error from this library
    LibError(String),

    /// An error reported in a JSON response from Elasticsearch
    ServerError(String),

    /// HTTP library error
    HttpError(reqwest::Error),

    /// IO error
    IoError(io::Error),

    /// JSON error
    JsonError(serde_json::error::Error),
}

impl From<io::Error> for ElasticsearchError {
    fn from(err: io::Error) -> ElasticsearchError {
        ElasticsearchError::IoError(err)
    }
}

impl From<reqwest::Error> for ElasticsearchError {
    fn from(err: reqwest::Error) -> ElasticsearchError {
        ElasticsearchError::HttpError(err)
    }
}

impl From<serde_json::error::Error> for ElasticsearchError {
    fn from(err: serde_json::error::Error) -> ElasticsearchError {
        ElasticsearchError::JsonError(err)
    }
}

impl From<url::ParseError> for ElasticsearchError {
    fn from(err: url::ParseError) -> ElasticsearchError {
        ElasticsearchError::LibError(err.to_string())
    }
}

impl<'a> From<&'a mut reqwest::Response> for ElasticsearchError {
    fn from(err: &'a mut reqwest::Response) -> ElasticsearchError {
        // TODO: figure out how to read the response body synchronously
        //let body = err.text().await?;
        ElasticsearchError::ServerError(format!("{} status code received", err.status()))
    }
}

impl Error for ElasticsearchError {
    fn description(&self) -> &str {
        match *self {
            ElasticsearchError::LibError(ref err) => err,
            ElasticsearchError::ServerError(ref err) => err,
            ElasticsearchError::HttpError(ref err) => err.description(),
            ElasticsearchError::IoError(ref err) => err.description(),
            ElasticsearchError::JsonError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ElasticsearchError::LibError(_) => None,
            ElasticsearchError::ServerError(_) => None,
            ElasticsearchError::HttpError(ref err) => Some(err as &dyn Error),
            ElasticsearchError::IoError(ref err) => Some(err as &dyn Error),
            ElasticsearchError::JsonError(ref err) => Some(err as &dyn Error),
        }
    }
}

impl fmt::Display for ElasticsearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ElasticsearchError::LibError(ref s) => fmt::Display::fmt(s, f),
            ElasticsearchError::ServerError(ref s) => fmt::Display::fmt(s, f),
            ElasticsearchError::HttpError(ref err) => fmt::Display::fmt(err, f),
            ElasticsearchError::IoError(ref err) => fmt::Display::fmt(err, f),
            ElasticsearchError::JsonError(ref err) => fmt::Display::fmt(err, f),
        }
    }
}
