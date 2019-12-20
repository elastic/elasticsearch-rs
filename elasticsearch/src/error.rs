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
use crate::http::transport::BuildError;
use serde_json;
use std::error;
use std::fmt;
use std::io;

/// An error within the client.
///
/// Errors that can occur include IO and parsing errors, as well as specific
/// errors from Elasticsearch and internal errors from this library
#[derive(Debug)]
pub enum Error {
    /// An error building the client
    Build(BuildError),

    /// A general error from this library
    Lib(String),

    /// An error reported in a JSON response from Elasticsearch
    Server(String),

    /// HTTP library error
    Http(reqwest::Error),

    /// IO error
    Io(io::Error),

    /// JSON error
    Json(serde_json::error::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Http(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::Json(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::Lib(err.to_string())
    }
}

impl From<BuildError> for Error {
    fn from(err: BuildError) -> Error {
        Error::Build(err)
    }
}

impl<'a> From<&'a mut reqwest::Response> for Error {
    fn from(err: &'a mut reqwest::Response) -> Error {
        // TODO: figure out how to read the response body synchronously
        //let body = err.text().await?;
        Error::Server(format!("{} status code received", err.status()))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Build(ref err) => err.description(),
            Error::Lib(ref err) => err,
            Error::Server(ref err) => err,
            Error::Http(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Build(ref err) => Some(err as &dyn error::Error),
            Error::Lib(_) => None,
            Error::Server(_) => None,
            Error::Http(ref err) => Some(err as &dyn error::Error),
            Error::Io(ref err) => Some(err as &dyn error::Error),
            Error::Json(ref err) => Some(err as &dyn error::Error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Build(ref err) => fmt::Display::fmt(err, f),
            Error::Lib(ref s) => fmt::Display::fmt(s, f),
            Error::Server(ref s) => fmt::Display::fmt(s, f),
            Error::Http(ref err) => fmt::Display::fmt(err, f),
            Error::Io(ref err) => fmt::Display::fmt(err, f),
            Error::Json(ref err) => fmt::Display::fmt(err, f),
        }
    }
}
