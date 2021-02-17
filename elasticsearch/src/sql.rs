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
#[doc = "Builder for the [Sql Clear Cursor API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/sql-pagination.html)\n\nClears the SQL cursor"]
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
#[doc = "Builder for the [Sql Query API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/sql-rest-overview.html)\n\nExecutes a SQL request"]
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
#[doc = "Builder for the [Sql Translate API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/sql-translate.html)\n\nTranslates SQL into Elasticsearch queries"]
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
    #[doc = "[Sql Clear Cursor API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/sql-pagination.html)\n\nClears the SQL cursor"]
    pub fn clear_cursor<'b>(&'a self) -> SqlClearCursor<'a, 'b, ()> {
        SqlClearCursor::new(self.transport())
    }
    #[doc = "[Sql Query API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/sql-rest-overview.html)\n\nExecutes a SQL request"]
    pub fn query<'b>(&'a self) -> SqlQuery<'a, 'b, ()> {
        SqlQuery::new(self.transport())
    }
    #[doc = "[Sql Translate API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/sql-translate.html)\n\nTranslates SQL into Elasticsearch queries"]
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
