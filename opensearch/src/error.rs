/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
/* Error type based on the error type from es-rs:
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
 */
use crate::http::{transport::BuildError, StatusCode};
use std::{error, fmt, io};

/// An error with the client.
///
/// Errors that can occur include IO and parsing errors, as well as specific
/// errors from Elasticsearch and internal errors from the client.
#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
    /// An error building the client
    Build(BuildError),

    /// A general error from this library
    Lib(String),

    /// HTTP library error
    Http(reqwest::Error),

    /// IO error
    Io(io::Error),

    /// JSON error
    Json(serde_json::error::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error {
            kind: Kind::Io(err),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error {
            kind: Kind::Http(err),
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error {
            kind: Kind::Json(err),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error {
            kind: Kind::Lib(err.to_string()),
        }
    }
}

impl From<BuildError> for Error {
    fn from(err: BuildError) -> Error {
        Error {
            kind: Kind::Build(err),
        }
    }
}

pub(crate) fn lib(err: impl Into<String>) -> Error {
    Error {
        kind: Kind::Lib(err.into()),
    }
}

impl Error {
    /// The status code, if the error was generated from a response
    pub fn status_code(&self) -> Option<StatusCode> {
        match &self.kind {
            Kind::Http(err) => err.status(),
            _ => None,
        }
    }

    /// Returns true if the error is related to a timeout
    pub fn is_timeout(&self) -> bool {
        match &self.kind {
            Kind::Http(err) => err.is_timeout(),
            _ => false,
        }
    }

    /// Returns true if the error is related to serialization or deserialization
    pub fn is_json(&self) -> bool {
        match &self.kind {
            Kind::Json(_) => true,
            _ => false,
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            Kind::Build(err) => Some(err),
            Kind::Lib(_) => None,
            Kind::Http(err) => Some(err),
            Kind::Io(err) => Some(err),
            Kind::Json(err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Kind::Build(err) => err.fmt(f),
            Kind::Lib(err) => err.fmt(f),
            Kind::Http(err) => err.fmt(f),
            Kind::Io(err) => err.fmt(f),
            Kind::Json(err) => err.fmt(f),
        }
    }
}
