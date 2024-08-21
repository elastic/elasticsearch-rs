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
#[doc = "API parts for the Profiling Flamegraph API"]
pub enum ProfilingFlamegraphParts {
    #[doc = "No parts"]
    None,
}
impl ProfilingFlamegraphParts {
    #[doc = "Builds a relative URL path to the Profiling Flamegraph API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ProfilingFlamegraphParts::None => "/_profiling/flamegraph".into(),
        }
    }
}
#[doc = "Builder for the [Profiling Flamegraph API](https://www.elastic.co/guide/en/observability/8.15/universal-profiling.html)\n\nExtracts a UI-optimized structure to render flamegraphs from Universal Profiling."]
#[derive(Clone, Debug)]
pub struct ProfilingFlamegraph<'a, 'b, B> {
    transport: &'a Transport,
    parts: ProfilingFlamegraphParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ProfilingFlamegraph<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ProfilingFlamegraph]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ProfilingFlamegraph {
            transport,
            parts: ProfilingFlamegraphParts::None,
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
    pub fn body<T>(self, body: T) -> ProfilingFlamegraph<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ProfilingFlamegraph {
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
    #[doc = "Creates an asynchronous call to the Profiling Flamegraph API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Profiling Stacktraces API"]
pub enum ProfilingStacktracesParts {
    #[doc = "No parts"]
    None,
}
impl ProfilingStacktracesParts {
    #[doc = "Builds a relative URL path to the Profiling Stacktraces API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ProfilingStacktracesParts::None => "/_profiling/stacktraces".into(),
        }
    }
}
#[doc = "Builder for the [Profiling Stacktraces API](https://www.elastic.co/guide/en/observability/8.15/universal-profiling.html)\n\nExtracts raw stacktrace information from Universal Profiling."]
#[derive(Clone, Debug)]
pub struct ProfilingStacktraces<'a, 'b, B> {
    transport: &'a Transport,
    parts: ProfilingStacktracesParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ProfilingStacktraces<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ProfilingStacktraces]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ProfilingStacktraces {
            transport,
            parts: ProfilingStacktracesParts::None,
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
    pub fn body<T>(self, body: T) -> ProfilingStacktraces<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ProfilingStacktraces {
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
    #[doc = "Creates an asynchronous call to the Profiling Stacktraces API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Profiling Status API"]
pub enum ProfilingStatusParts {
    #[doc = "No parts"]
    None,
}
impl ProfilingStatusParts {
    #[doc = "Builds a relative URL path to the Profiling Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ProfilingStatusParts::None => "/_profiling/status".into(),
        }
    }
}
#[doc = "Builder for the [Profiling Status API](https://www.elastic.co/guide/en/observability/8.15/universal-profiling.html)\n\nReturns basic information about the status of Universal Profiling."]
#[derive(Clone, Debug)]
pub struct ProfilingStatus<'a, 'b> {
    transport: &'a Transport,
    parts: ProfilingStatusParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_resources_created: Option<bool>,
}
impl<'a, 'b> ProfilingStatus<'a, 'b> {
    #[doc = "Creates a new instance of [ProfilingStatus]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ProfilingStatus {
            transport,
            parts: ProfilingStatusParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_resources_created: None,
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
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Whether to return immediately or wait until resources have been created"]
    pub fn wait_for_resources_created(mut self, wait_for_resources_created: bool) -> Self {
        self.wait_for_resources_created = Some(wait_for_resources_created);
        self
    }
    #[doc = "Creates an asynchronous call to the Profiling Status API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
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
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_resources_created: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                wait_for_resources_created: self.wait_for_resources_created,
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
#[doc = "API parts for the Profiling Topn Functions API"]
pub enum ProfilingTopnFunctionsParts {
    #[doc = "No parts"]
    None,
}
impl ProfilingTopnFunctionsParts {
    #[doc = "Builds a relative URL path to the Profiling Topn Functions API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ProfilingTopnFunctionsParts::None => "/_profiling/topn/functions".into(),
        }
    }
}
#[doc = "Builder for the [Profiling Topn Functions API](https://www.elastic.co/guide/en/observability/8.15/universal-profiling.html)\n\nExtracts a list of topN functions from Universal Profiling."]
#[derive(Clone, Debug)]
pub struct ProfilingTopnFunctions<'a, 'b, B> {
    transport: &'a Transport,
    parts: ProfilingTopnFunctionsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ProfilingTopnFunctions<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ProfilingTopnFunctions]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ProfilingTopnFunctions {
            transport,
            parts: ProfilingTopnFunctionsParts::None,
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
    pub fn body<T>(self, body: T) -> ProfilingTopnFunctions<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ProfilingTopnFunctions {
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
    #[doc = "Creates an asynchronous call to the Profiling Topn Functions API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[doc = "Namespace client for Profiling APIs"]
pub struct Profiling<'a> {
    transport: &'a Transport,
}
impl<'a> Profiling<'a> {
    #[doc = "Creates a new instance of [Profiling]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Profiling Flamegraph API](https://www.elastic.co/guide/en/observability/8.15/universal-profiling.html)\n\nExtracts a UI-optimized structure to render flamegraphs from Universal Profiling."]
    pub fn flamegraph<'b>(&'a self) -> ProfilingFlamegraph<'a, 'b, ()> {
        ProfilingFlamegraph::new(self.transport())
    }
    #[doc = "[Profiling Stacktraces API](https://www.elastic.co/guide/en/observability/8.15/universal-profiling.html)\n\nExtracts raw stacktrace information from Universal Profiling."]
    pub fn stacktraces<'b>(&'a self) -> ProfilingStacktraces<'a, 'b, ()> {
        ProfilingStacktraces::new(self.transport())
    }
    #[doc = "[Profiling Status API](https://www.elastic.co/guide/en/observability/8.15/universal-profiling.html)\n\nReturns basic information about the status of Universal Profiling."]
    pub fn status<'b>(&'a self) -> ProfilingStatus<'a, 'b> {
        ProfilingStatus::new(self.transport())
    }
    #[doc = "[Profiling Topn Functions API](https://www.elastic.co/guide/en/observability/8.15/universal-profiling.html)\n\nExtracts a list of topN functions from Universal Profiling."]
    pub fn topn_functions<'b>(&'a self) -> ProfilingTopnFunctions<'a, 'b, ()> {
        ProfilingTopnFunctions::new(self.transport())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Profiling APIs"]
    pub fn profiling(&self) -> Profiling {
        Profiling::new(self.transport())
    }
}
