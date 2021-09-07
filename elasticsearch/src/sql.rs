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

// -----------------------------------------------
// This file is generated, Please do not edit it manually.
// Run the following in the root of the repo to regenerate:
//
// cargo make generate-api
// -----------------------------------------------

//! SQL APIs
//!
//! [Execute SQL queries against Elasticsearch indices and return results in tabular format](https://www.elastic.co/guide/en/elasticsearch/reference/master/xpack-sql.html).

#![allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        transport::Transport,
        Method,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::{borrow::Cow, time::Duration};
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Sql Clear Cursor API"]
pub enum SqlClearCursorParts {
    #[doc = "No parts"]
    None,
}
impl SqlClearCursorParts {
    #[doc = "Builds a relative URL path to the Sql Clear Cursor API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SqlClearCursorParts::None => "/_sql/close".into(),
        }
    }
}
#[doc = "Builder for the [Sql Clear Cursor API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/clear-sql-cursor-api.html)\n\nClears the SQL cursor"]
#[derive(Clone, Debug)]
pub struct SqlClearCursor<'a, 'b, B> {
    transport: &'a Transport,
    parts: SqlClearCursorParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SqlClearCursor<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SqlClearCursor]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SqlClearCursor {
            transport,
            parts: SqlClearCursorParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SqlClearCursor<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SqlClearCursor {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Sql Clear Cursor API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Sql Delete Async API"]
pub enum SqlDeleteAsyncParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> SqlDeleteAsyncParts<'b> {
    #[doc = "Builds a relative URL path to the Sql Delete Async API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SqlDeleteAsyncParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_id.len());
                p.push_str("/_sql/async/delete/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Sql Delete Async API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/delete-async-sql-search-api.html)\n\nDeletes an async SQL search or a stored synchronous SQL search. If the search is still running, the API cancels it."]
#[derive(Clone, Debug)]
pub struct SqlDeleteAsync<'a, 'b> {
    transport: &'a Transport,
    parts: SqlDeleteAsyncParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SqlDeleteAsync<'a, 'b> {
    #[doc = "Creates a new instance of [SqlDeleteAsync] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SqlDeleteAsyncParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SqlDeleteAsync {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Sql Delete Async API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Sql Get Async API"]
pub enum SqlGetAsyncParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> SqlGetAsyncParts<'b> {
    #[doc = "Builds a relative URL path to the Sql Get Async API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SqlGetAsyncParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_id.len());
                p.push_str("/_sql/async/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Sql Get Async API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/get-async-sql-search-api.html)\n\nReturns the current status and available results for an async SQL search or stored synchronous SQL search"]
#[derive(Clone, Debug)]
pub struct SqlGetAsync<'a, 'b> {
    transport: &'a Transport,
    parts: SqlGetAsyncParts<'b>,
    delimiter: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    keep_alive: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_completion_timeout: Option<&'b str>,
}
impl<'a, 'b> SqlGetAsync<'a, 'b> {
    #[doc = "Creates a new instance of [SqlGetAsync] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SqlGetAsyncParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SqlGetAsync {
            transport,
            parts,
            headers,
            delimiter: None,
            error_trace: None,
            filter_path: None,
            format: None,
            human: None,
            keep_alive: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_for_completion_timeout: None,
        }
    }
    #[doc = "Separator for CSV results"]
    pub fn delimiter(mut self, delimiter: &'b str) -> Self {
        self.delimiter = Some(delimiter);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Retention period for the search and its results"]
    pub fn keep_alive(mut self, keep_alive: &'b str) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Duration to wait for complete results"]
    pub fn wait_for_completion_timeout(mut self, wait_for_completion_timeout: &'b str) -> Self {
        self.wait_for_completion_timeout = Some(wait_for_completion_timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Sql Get Async API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                delimiter: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                human: Option<bool>,
                keep_alive: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                wait_for_completion_timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                delimiter: self.delimiter,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                human: self.human,
                keep_alive: self.keep_alive,
                pretty: self.pretty,
                source: self.source,
                wait_for_completion_timeout: self.wait_for_completion_timeout,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Sql Get Async Status API"]
pub enum SqlGetAsyncStatusParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> SqlGetAsyncStatusParts<'b> {
    #[doc = "Builds a relative URL path to the Sql Get Async Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SqlGetAsyncStatusParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_id.len());
                p.push_str("/_sql/async/status/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Sql Get Async Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/get-async-sql-search-status-api.html)\n\nReturns the current status of an async SQL search or a stored synchronous SQL search"]
#[derive(Clone, Debug)]
pub struct SqlGetAsyncStatus<'a, 'b> {
    transport: &'a Transport,
    parts: SqlGetAsyncStatusParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SqlGetAsyncStatus<'a, 'b> {
    #[doc = "Creates a new instance of [SqlGetAsyncStatus] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SqlGetAsyncStatusParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SqlGetAsyncStatus {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Sql Get Async Status API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Sql Query API"]
pub enum SqlQueryParts {
    #[doc = "No parts"]
    None,
}
impl SqlQueryParts {
    #[doc = "Builds a relative URL path to the Sql Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SqlQueryParts::None => "/_sql".into(),
        }
    }
}
#[doc = "Builder for the [Sql Query API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/sql-search-api.html)\n\nExecutes a SQL request"]
#[derive(Clone, Debug)]
pub struct SqlQuery<'a, 'b, B> {
    transport: &'a Transport,
    parts: SqlQueryParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SqlQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SqlQuery]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SqlQuery {
            transport,
            parts: SqlQueryParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            format: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SqlQuery<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SqlQuery {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            format: self.format,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Sql Query API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Sql Translate API"]
pub enum SqlTranslateParts {
    #[doc = "No parts"]
    None,
}
impl SqlTranslateParts {
    #[doc = "Builds a relative URL path to the Sql Translate API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SqlTranslateParts::None => "/_sql/translate".into(),
        }
    }
}
#[doc = "Builder for the [Sql Translate API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/sql-translate-api.html)\n\nTranslates SQL into Elasticsearch queries"]
#[derive(Clone, Debug)]
pub struct SqlTranslate<'a, 'b, B> {
    transport: &'a Transport,
    parts: SqlTranslateParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SqlTranslate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SqlTranslate]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SqlTranslate {
            transport,
            parts: SqlTranslateParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SqlTranslate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SqlTranslate {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Sql Translate API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Sql APIs"]
pub struct Sql<'a> {
    transport: &'a Transport,
}
impl<'a> Sql<'a> {
    #[doc = "Creates a new instance of [Sql]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Sql Clear Cursor API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/clear-sql-cursor-api.html)\n\nClears the SQL cursor"]
    pub fn clear_cursor<'b>(&'a self) -> SqlClearCursor<'a, 'b, ()> {
        SqlClearCursor::new(self.transport())
    }
    #[doc = "[Sql Delete Async API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/delete-async-sql-search-api.html)\n\nDeletes an async SQL search or a stored synchronous SQL search. If the search is still running, the API cancels it."]
    pub fn delete_async<'b>(&'a self, parts: SqlDeleteAsyncParts<'b>) -> SqlDeleteAsync<'a, 'b> {
        SqlDeleteAsync::new(self.transport(), parts)
    }
    #[doc = "[Sql Get Async API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/get-async-sql-search-api.html)\n\nReturns the current status and available results for an async SQL search or stored synchronous SQL search"]
    pub fn get_async<'b>(&'a self, parts: SqlGetAsyncParts<'b>) -> SqlGetAsync<'a, 'b> {
        SqlGetAsync::new(self.transport(), parts)
    }
    #[doc = "[Sql Get Async Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/get-async-sql-search-status-api.html)\n\nReturns the current status of an async SQL search or a stored synchronous SQL search"]
    pub fn get_async_status<'b>(
        &'a self,
        parts: SqlGetAsyncStatusParts<'b>,
    ) -> SqlGetAsyncStatus<'a, 'b> {
        SqlGetAsyncStatus::new(self.transport(), parts)
    }
    #[doc = "[Sql Query API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/sql-search-api.html)\n\nExecutes a SQL request"]
    pub fn query<'b>(&'a self) -> SqlQuery<'a, 'b, ()> {
        SqlQuery::new(self.transport())
    }
    #[doc = "[Sql Translate API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/sql-translate-api.html)\n\nTranslates SQL into Elasticsearch queries"]
    pub fn translate<'b>(&'a self) -> SqlTranslate<'a, 'b, ()> {
        SqlTranslate::new(self.transport())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Sql APIs"]
    pub fn sql(&self) -> Sql {
        Sql::new(self.transport())
    }
}
