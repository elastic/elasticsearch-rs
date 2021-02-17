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

//! EQL APIs
//!
//! [Event Query Language (EQL)](https://www.elastic.co/guide/en/elasticsearch/reference/master/eql.html) is a query
//! language for event-based time series data, such as logs, metrics, and traces.
//!
//!

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
#[doc = "API parts for the Eql Delete API"]
pub enum EqlDeleteParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> EqlDeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Eql Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            EqlDeleteParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_id.len());
                p.push_str("/_eql/search/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Eql Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/eql-search-api.html)\n\nDeletes an async EQL search by ID. If the search is still running, the search request will be cancelled. Otherwise, the saved search results are deleted."]
#[derive(Clone, Debug)]
pub struct EqlDelete<'a, 'b> {
    transport: &'a Transport,
    parts: EqlDeleteParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> EqlDelete<'a, 'b> {
    #[doc = "Creates a new instance of [EqlDelete] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: EqlDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        EqlDelete {
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
    #[doc = "Creates an asynchronous call to the Eql Delete API that can be awaited"]
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
#[doc = "API parts for the Eql Get API"]
pub enum EqlGetParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> EqlGetParts<'b> {
    #[doc = "Builds a relative URL path to the Eql Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            EqlGetParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_id.len());
                p.push_str("/_eql/search/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Eql Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/eql-search-api.html)\n\nReturns async results from previously executed Event Query Language (EQL) search"]
#[derive(Clone, Debug)]
pub struct EqlGet<'a, 'b> {
    transport: &'a Transport,
    parts: EqlGetParts<'b>,
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
impl<'a, 'b> EqlGet<'a, 'b> {
    #[doc = "Creates a new instance of [EqlGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: EqlGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        EqlGet {
            transport,
            parts,
            headers,
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
    #[doc = "Update the time interval in which the results (partial or final) for this search will be available"]
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
    #[doc = "Creates an asynchronous call to the Eql Get API that can be awaited"]
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
                keep_alive: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                wait_for_completion_timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Eql Get Status API"]
pub enum EqlGetStatusParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> EqlGetStatusParts<'b> {
    #[doc = "Builds a relative URL path to the Eql Get Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            EqlGetStatusParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_id.len());
                p.push_str("/_eql/search/status/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Eql Get Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/eql-search-api.html)\n\nReturns the status of a previously submitted async or stored Event Query Language (EQL) search"]
#[derive(Clone, Debug)]
pub struct EqlGetStatus<'a, 'b> {
    transport: &'a Transport,
    parts: EqlGetStatusParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> EqlGetStatus<'a, 'b> {
    #[doc = "Creates a new instance of [EqlGetStatus] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: EqlGetStatusParts<'b>) -> Self {
        let headers = HeaderMap::new();
        EqlGetStatus {
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
    #[doc = "Creates an asynchronous call to the Eql Get Status API that can be awaited"]
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
#[doc = "API parts for the Eql Search API"]
pub enum EqlSearchParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> EqlSearchParts<'b> {
    #[doc = "Builds a relative URL path to the Eql Search API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            EqlSearchParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_eql/search");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Eql Search API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/eql-search-api.html)\n\nReturns results matching a query expressed in Event Query Language (EQL)"]
#[derive(Clone, Debug)]
pub struct EqlSearch<'a, 'b, B> {
    transport: &'a Transport,
    parts: EqlSearchParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    keep_alive: Option<&'b str>,
    keep_on_completion: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_completion_timeout: Option<&'b str>,
}
impl<'a, 'b, B> EqlSearch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [EqlSearch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: EqlSearchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        EqlSearch {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            keep_alive: None,
            keep_on_completion: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_for_completion_timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> EqlSearch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        EqlSearch {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            keep_alive: self.keep_alive,
            keep_on_completion: self.keep_on_completion,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            wait_for_completion_timeout: self.wait_for_completion_timeout,
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
    #[doc = "Update the time interval in which the results (partial or final) for this search will be available"]
    pub fn keep_alive(mut self, keep_alive: &'b str) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }
    #[doc = "Control whether the response should be stored in the cluster if it completed within the provided [wait_for_completion] time (default: false)"]
    pub fn keep_on_completion(mut self, keep_on_completion: bool) -> Self {
        self.keep_on_completion = Some(keep_on_completion);
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
    #[doc = "Creates an asynchronous call to the Eql Search API that can be awaited"]
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
                keep_alive: Option<&'b str>,
                keep_on_completion: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                wait_for_completion_timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                keep_alive: self.keep_alive,
                keep_on_completion: self.keep_on_completion,
                pretty: self.pretty,
                source: self.source,
                wait_for_completion_timeout: self.wait_for_completion_timeout,
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
#[doc = "Namespace client for Eql APIs"]
pub struct Eql<'a> {
    transport: &'a Transport,
}
impl<'a> Eql<'a> {
    #[doc = "Creates a new instance of [Eql]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Eql Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/eql-search-api.html)\n\nDeletes an async EQL search by ID. If the search is still running, the search request will be cancelled. Otherwise, the saved search results are deleted."]
    pub fn delete<'b>(&'a self, parts: EqlDeleteParts<'b>) -> EqlDelete<'a, 'b> {
        EqlDelete::new(self.transport(), parts)
    }
    #[doc = "[Eql Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/eql-search-api.html)\n\nReturns async results from previously executed Event Query Language (EQL) search"]
    pub fn get<'b>(&'a self, parts: EqlGetParts<'b>) -> EqlGet<'a, 'b> {
        EqlGet::new(self.transport(), parts)
    }
    #[doc = "[Eql Get Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/eql-search-api.html)\n\nReturns the status of a previously submitted async or stored Event Query Language (EQL) search"]
    pub fn get_status<'b>(&'a self, parts: EqlGetStatusParts<'b>) -> EqlGetStatus<'a, 'b> {
        EqlGetStatus::new(self.transport(), parts)
    }
    #[doc = "[Eql Search API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/eql-search-api.html)\n\nReturns results matching a query expressed in Event Query Language (EQL)"]
    pub fn search<'b>(&'a self, parts: EqlSearchParts<'b>) -> EqlSearch<'a, 'b, ()> {
        EqlSearch::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Eql APIs"]
    pub fn eql(&self) -> Eql {
        Eql::new(self.transport())
    }
}
