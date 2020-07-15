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
//! HTTP response components
use crate::{
    error::Error as ClientError,
    http::{headers::HeaderMap, Method, StatusCode, Url},
};
use serde::{de, de::{MapAccess, Visitor, DeserializeOwned}, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, fmt, str::FromStr};
use void::Void;

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
    pub fn error_for_status_code(self) -> Result<Self, ClientError> {
        match self.0.error_for_status_ref() {
            Ok(_) => Ok(self),
            Err(err) => Err(err.into()),
        }
    }

    /// Turn the response into an [Error] if Elasticsearch returned an error.
    pub fn error_for_status_code_ref(&self) -> Result<&Self, ClientError> {
        match self.0.error_for_status_ref() {
            Ok(_) => Ok(self),
            Err(err) => Err(err.into()),
        }
    }

    /// Asynchronously reads the response body into an [ElasticsearchException] if
    /// Elasticsearch returned an error.
    ///
    /// Reading the response body consumes `self`
    pub async fn exception(self) -> Result<Option<ElasticsearchException>, ClientError> {
        if self.status_code().is_client_error() || self.status_code().is_server_error() {
            let ex = self.json().await?;
            Ok(Some(ex))
        } else {
            Ok(None)
        }
    }

    /// Asynchronously reads the response body as JSON
    ///
    /// Reading the response body consumes `self`
    pub async fn json<B>(self) -> Result<B, ClientError>
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
    pub async fn text(self) -> Result<String, ClientError> {
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
        self.0.headers().get_all("Warning").iter().map(|w| {
            let s = w.to_str().unwrap();
            let first_quote = s.find(r#"""#).unwrap();
            let last_quote = s.len() - 1;
            &s[first_quote + 1..last_quote]
        })
    }
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Response")
            .field("method", &self.method())
            .field("url", self.url())
            .field("status_code", &self.status_code())
            .field("headers", self.headers())
            .finish()
    }
}

/// An exception raised by Elasticsearch.
///
/// Contains details that indicate why the exception was raised which can help
#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct ElasticsearchException {
    status: Option<u16>,
    #[serde(deserialize_with = "crate::string_or_struct")]
    error: Error,
}

impl ElasticsearchException {
    /// The status code of the exception, if available.
    pub fn status(&self) -> Option<u16> {
        self.status
    }

    /// The error details for the exception
    pub fn error(&self) -> &Error {
        &self.error
    }
}

/// Details about the exception raised by Elasticsearch
#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Error {
    #[serde(default = "BTreeMap::new", deserialize_with = "header_map")]
    header: BTreeMap<String, Vec<String>>,
    #[serde(default = "Vec::new")]
    root_cause: Vec<Cause>,
    reason: Option<String>,
    stack_trace: Option<String>,
    #[serde(rename = "type")]
    ty: String,
    #[serde(default = "BTreeMap::new", flatten)]
    additional_details: BTreeMap<String, Value>,
}

/// Deserializes the headers map where the map values may be a string or a sequence of strings
fn header_map<'de, D>(deserializer: D) -> Result<BTreeMap<String, Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "crate::string_or_seq_string")] Vec<String>);

    let v: BTreeMap<String, Wrapper> = BTreeMap::deserialize(deserializer)?;
    Ok(v.into_iter().map(|(k, Wrapper(v))| (k, v)).collect())
}

impl Error {
    /// The root causes for the exception
    pub fn root_cause(&self) -> &Vec<Cause> {
        &self.root_cause
    }

    /// The headers
    pub fn header(&self) -> &BTreeMap<String, Vec<String>> {
        &self.header
    }

    /// The reason for the exception
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_ref().map(|r| r.as_str())
    }

    /// The exception stack trace, if available.
    ///
    /// Available if `error_trace` is specified on the request
    pub fn stack_trace(&self) -> Option<&str> {
        self.stack_trace.as_ref().map(|r| r.as_str())
    }

    /// The type of exception
    pub fn ty(&self) -> &str {
        self.ty.as_str()
    }

    /// Additional details about the cause.
    ///
    /// Elasticsearch can return additional details about an exception, depending
    /// on context, which do not map to fields on [Error]. These are collected here
    pub fn additional_details(&self) -> &BTreeMap<String, Value> {
        &self.additional_details
    }
}

// An error in an Elasticsearch exception can be returned as a simple message string only, or
// as a JSON object. Handle both cases by corralling the simple message into the reason field
impl FromStr for Error {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Error {
            header: Default::default(),
            root_cause: Vec::new(),
            reason: Some(s.to_string()),
            stack_trace: None,
            ty: String::new(),
            additional_details: Default::default(),
        })
    }
}

/// The cause of an exception
#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Cause {
    #[serde(deserialize_with = "option_box_cause", default)]
    caused_by: Option<Box<Cause>>,
    reason: Option<String>,
    stack_trace: Option<String>,
    #[serde(rename = "type")]
    ty: String,
    #[serde(default = "BTreeMap::new", flatten)]
    additional_details: BTreeMap<String, Value>,
}

/// Deserializes a string or a map into Some boxed [Cause]. A missing field
/// for `caused_by` is handled by serde's default attribute on the struct field,
/// which will assign None to the field.
fn option_box_cause<'de, D>(deserializer: D) -> Result<Option<Box<Cause>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct CauseVisitor;
    impl<'de> Visitor<'de> for CauseVisitor {
        type Value = Cause;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Cause {
                caused_by: None,
                reason: Some(value.to_string()),
                stack_trace: None,
                ty: String::new(),
                additional_details: Default::default(),
            })
        }

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer
        .deserialize_any(CauseVisitor)
        .map(|c| Some(Box::new(c)))
}

impl Cause {
    /// The cause of the exception
    pub fn caused_by(&self) -> Option<&Box<Cause>> {
        self.caused_by.as_ref()
    }

    /// The reason for the exception
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_ref().map(|r| r.as_str())
    }

    /// The exception stack trace, if available.
    ///
    /// Available if `error_trace` is specified on the request
    pub fn stack_trace(&self) -> Option<&str> {
        self.stack_trace.as_ref().map(|r| r.as_str())
    }

    /// The type of exception
    pub fn ty(&self) -> &str {
        self.ty.as_str()
    }

    /// Additional details about the cause.
    ///
    /// Elasticsearch can return additional details about an exception, depending
    /// on context, which do not map to fields on [Error]. These are collected here
    pub fn additional_details(&self) -> &BTreeMap<String, Value> {
        &self.additional_details
    }
}

#[cfg(test)]
pub mod tests {
    use crate::http::response::ElasticsearchException;
    use serde_json::json;

    #[test]
    fn deserialize_exception_with_additional_details() -> Result<(), failure::Error> {
        let json = r#"{
            "error":{
                "root_cause" : [
                    {
                        "type" : "parse_exception",
                        "reason" : "failed to parse date field [-1m] with format [strict_date_optional_time||epoch_millis]",
                        "caused_by" : {
                            "type" : "parse_exception",
                            "reason" : "failed to parse date field [-1m] with format [strict_date_optional_time||epoch_millis]",
                            "index" : null,
                            "resource.id" : ["alias1", "alias2"],
                            "script_stack" : ["alias1", "alias2"],
                            "unknown_prop" : ["alias1", "alias2"],
                            "caused_by" : {
                                "type" : "illegal_argument_exception",
                                "reason" : "Parse failure at index [2] of [-1m]",
                                "caused_by" : "x"
                            }
                        }
                    }
                ],
                "type" : "search_phase_execution_exception",
                "reason" : "all shards failed",
                "phase" : "query",
                "grouped" : true,
                "failed_shards" : [
                {
                    "shard" : 0,
                    "index" : "project",
                    "node" : "Uo6PBln_QrmD8Y9o1NKdQw",
                    "unknown_prop" : "x",
                    "reason" : {
                        "type" : "parse_exception",
                        "reason" : "failed to parse date field [-1m] with format [strict_date_optional_time||epoch_millis]",
                        "caused_by" : {
                            "type" : "illegal_argument_exception",
                            "reason" : "Parse failure at index [2] of [-1m]"
                        }
                    },
                    "status" : "x"
                }
                ],
                "header" : {
                    "WWW-Authenticate" : "Bearer ...",
                    "x" : ["y", "z"]
                },
                "license.expired.feature" : "ml",
                "index" : "index",
                "index_uuid" : "x9h1ks",
                "unknown_prop" : {},
                "unknown_prop2" : false,
                "resource.type" : "aliases",
                "resource.id" : "alias1",
                "shard" : "1",
                "line" : 12,
                "col" : 199,
                "bytes_wanted" : 1298312,
                "bytes_limit" : 8912031,
                "script_stack" : "x",
                "script" : "some script",
                "lang" : "painless"
            }
        }"#;

        let ex: ElasticsearchException = serde_json::from_str(json)?;
        let error = ex.error();

        assert!(ex.status().is_none());
        assert_eq!(error.reason(), Some("all shards failed"));
        assert!(!error.additional_details().is_empty());
        assert_eq!(
            error.additional_details().get("script_stack"),
            Some(&json!("x"))
        );
        assert_eq!(error.header().len(), 2);
        assert_eq!(
            error.header().get("x"),
            Some(&vec!["y".to_string(), "z".to_string()])
        );

        assert!(!error.root_cause().is_empty());
        let first_root_cause = &error.root_cause()[0];
        assert!(first_root_cause.caused_by().is_some());
        let caused_by = first_root_cause.caused_by().unwrap();
        assert_eq!(caused_by.reason(), Some("failed to parse date field [-1m] with format [strict_date_optional_time||epoch_millis]"));

        assert!(caused_by.caused_by().is_some());
        let inner_caused_by = caused_by.caused_by().unwrap();
        assert_eq!(
            inner_caused_by.reason(),
            Some("Parse failure at index [2] of [-1m]")
        );

        assert!(inner_caused_by.caused_by().is_some());
        let inner_inner_caused_by = inner_caused_by.caused_by().unwrap();
        assert_eq!(inner_inner_caused_by.reason(), Some("x"));

        assert!(inner_inner_caused_by.caused_by().is_none());

        Ok(())
    }
}
