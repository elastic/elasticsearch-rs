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

//! ES|QL APIs
//!
//! The Elasticsearch Query Language (ES|QL) provides a powerful way to filter, transform, and analyze data stored in Elasticsearch, and in the future in other runtimes. For an overview of ES|QL and related tutorials, see [ES|QL](https://www.elastic.co/guide/en/elasticsearch/reference/current/esql.html).

#![allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        self,
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        transport::Transport,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::{borrow::Cow, time::Duration};
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Esql Async Query API"]
pub enum EsqlAsyncQueryParts {
    #[doc = "No parts"]
    None,
}
impl EsqlAsyncQueryParts {
    #[doc = "Builds a relative URL path to the Esql Async Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            EsqlAsyncQueryParts::None => "/_query/async".into(),
        }
    }
}
#[doc = "Builder for the [Esql Async Query API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/esql-async-query-api.html)\n\nExecutes an ESQL request asynchronously"]
#[derive(Clone, Debug)]
pub struct EsqlAsyncQuery<'a, 'b, B> {
    transport: &'a Transport,
    parts: EsqlAsyncQueryParts,
    body: Option<B>,
    delimiter: Option<&'b str>,
    drop_null_columns: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> EsqlAsyncQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [EsqlAsyncQuery]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        EsqlAsyncQuery {
            transport,
            parts: EsqlAsyncQueryParts::None,
            headers,
            body: None,
            delimiter: None,
            drop_null_columns: None,
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
    pub fn body<T>(self, body: T) -> EsqlAsyncQuery<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        EsqlAsyncQuery {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            delimiter: self.delimiter,
            drop_null_columns: self.drop_null_columns,
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
    #[doc = "The character to use between values within a CSV row. Only valid for the csv format."]
    pub fn delimiter(mut self, delimiter: &'b str) -> Self {
        self.delimiter = Some(delimiter);
        self
    }
    #[doc = "Should entirely null columns be removed from the results? Their name and type will be returning in a new `all_columns` section."]
    pub fn drop_null_columns(mut self, drop_null_columns: bool) -> Self {
        self.drop_null_columns = Some(drop_null_columns);
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
    #[doc = "Creates an asynchronous call to the Esql Async Query API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                delimiter: Option<&'b str>,
                drop_null_columns: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                delimiter: self.delimiter,
                drop_null_columns: self.drop_null_columns,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Esql Async Query Get API"]
pub enum EsqlAsyncQueryGetParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> EsqlAsyncQueryGetParts<'b> {
    #[doc = "Builds a relative URL path to the Esql Async Query Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            EsqlAsyncQueryGetParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_id.len());
                p.push_str("/_query/async/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Esql Async Query Get API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/esql-async-query-get-api.html)\n\nRetrieves the results of a previously submitted async query request given its ID."]
#[derive(Clone, Debug)]
pub struct EsqlAsyncQueryGet<'a, 'b> {
    transport: &'a Transport,
    parts: EsqlAsyncQueryGetParts<'b>,
    drop_null_columns: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    keep_alive: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_completion_timeout: Option<&'b str>,
}
impl<'a, 'b> EsqlAsyncQueryGet<'a, 'b> {
    #[doc = "Creates a new instance of [EsqlAsyncQueryGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: EsqlAsyncQueryGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        EsqlAsyncQueryGet {
            transport,
            parts,
            headers,
            drop_null_columns: None,
            error_trace: None,
            filter_path: None,
            human: None,
            keep_alive: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_for_completion_timeout: None,
        }
    }
    #[doc = "Should entirely null columns be removed from the results? Their name and type will be returning in a new `all_columns` section."]
    pub fn drop_null_columns(mut self, drop_null_columns: bool) -> Self {
        self.drop_null_columns = Some(drop_null_columns);
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
    #[doc = "Specify the time interval in which the results (partial or final) for this search will be available"]
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
    #[doc = "Specify the time that the request should block waiting for the final response"]
    pub fn wait_for_completion_timeout(mut self, wait_for_completion_timeout: &'b str) -> Self {
        self.wait_for_completion_timeout = Some(wait_for_completion_timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Esql Async Query Get API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                drop_null_columns: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                keep_alive: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                wait_for_completion_timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                drop_null_columns: self.drop_null_columns,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Esql Query API"]
pub enum EsqlQueryParts {
    #[doc = "No parts"]
    None,
}
impl EsqlQueryParts {
    #[doc = "Builds a relative URL path to the Esql Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            EsqlQueryParts::None => "/_query".into(),
        }
    }
}
#[doc = "Builder for the [Esql Query API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/esql-query-api.html)\n\nExecutes an ESQL request"]
#[derive(Clone, Debug)]
pub struct EsqlQuery<'a, 'b, B> {
    transport: &'a Transport,
    parts: EsqlQueryParts,
    body: Option<B>,
    delimiter: Option<&'b str>,
    drop_null_columns: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> EsqlQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [EsqlQuery]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        EsqlQuery {
            transport,
            parts: EsqlQueryParts::None,
            headers,
            body: None,
            delimiter: None,
            drop_null_columns: None,
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
    pub fn body<T>(self, body: T) -> EsqlQuery<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        EsqlQuery {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            delimiter: self.delimiter,
            drop_null_columns: self.drop_null_columns,
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
    #[doc = "The character to use between values within a CSV row. Only valid for the csv format."]
    pub fn delimiter(mut self, delimiter: &'b str) -> Self {
        self.delimiter = Some(delimiter);
        self
    }
    #[doc = "Should entirely null columns be removed from the results? Their name and type will be returning in a new `all_columns` section."]
    pub fn drop_null_columns(mut self, drop_null_columns: bool) -> Self {
        self.drop_null_columns = Some(drop_null_columns);
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
    #[doc = "Creates an asynchronous call to the Esql Query API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                delimiter: Option<&'b str>,
                drop_null_columns: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                delimiter: self.delimiter,
                drop_null_columns: self.drop_null_columns,
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
#[doc = "Namespace client for Esql APIs"]
pub struct Esql<'a> {
    transport: &'a Transport,
}
impl<'a> Esql<'a> {
    #[doc = "Creates a new instance of [Esql]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Esql Async Query API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/esql-async-query-api.html)\n\nExecutes an ESQL request asynchronously"]
    pub fn async_query<'b>(&'a self) -> EsqlAsyncQuery<'a, 'b, ()> {
        EsqlAsyncQuery::new(self.transport())
    }
    #[doc = "[Esql Async Query Get API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/esql-async-query-get-api.html)\n\nRetrieves the results of a previously submitted async query request given its ID."]
    pub fn async_query_get<'b>(
        &'a self,
        parts: EsqlAsyncQueryGetParts<'b>,
    ) -> EsqlAsyncQueryGet<'a, 'b> {
        EsqlAsyncQueryGet::new(self.transport(), parts)
    }
    #[doc = "[Esql Query API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/esql-query-api.html)\n\nExecutes an ESQL request"]
    pub fn query<'b>(&'a self) -> EsqlQuery<'a, 'b, ()> {
        EsqlQuery::new(self.transport())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Esql APIs"]
    pub fn esql(&self) -> Esql {
        Esql::new(self.transport())
    }
}
