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
#[doc = "API parts for the Streams Logs Disable API"]
pub enum StreamsLogsDisableParts {
    #[doc = "No parts"]
    None,
}
impl StreamsLogsDisableParts {
    #[doc = "Builds a relative URL path to the Streams Logs Disable API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            StreamsLogsDisableParts::None => "/_streams/logs/_disable".into(),
        }
    }
}
#[doc = "Builder for the [Streams Logs Disable API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/streams-logs-disable.html)\n\nDisable the Logs Streams feature for this cluster"]
#[derive(Clone, Debug)]
pub struct StreamsLogsDisable<'a, 'b, B> {
    transport: &'a Transport,
    parts: StreamsLogsDisableParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> StreamsLogsDisable<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [StreamsLogsDisable]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        StreamsLogsDisable {
            transport,
            parts: StreamsLogsDisableParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> StreamsLogsDisable<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        StreamsLogsDisable {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
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
    #[doc = "Period to wait for a connection to the master node. If no response is received before the timeout expires, the request fails and returns an error."]
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
    #[doc = "Period to wait for a response. If no response is received before the timeout expires, the request fails and returns an error."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Streams Logs Disable API that can be awaited"]
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
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Streams Logs Enable API"]
pub enum StreamsLogsEnableParts {
    #[doc = "No parts"]
    None,
}
impl StreamsLogsEnableParts {
    #[doc = "Builds a relative URL path to the Streams Logs Enable API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            StreamsLogsEnableParts::None => "/_streams/logs/_enable".into(),
        }
    }
}
#[doc = "Builder for the [Streams Logs Enable API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/streams-logs-enable.html)\n\nEnable the Logs Streams feature for this cluster"]
#[derive(Clone, Debug)]
pub struct StreamsLogsEnable<'a, 'b, B> {
    transport: &'a Transport,
    parts: StreamsLogsEnableParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> StreamsLogsEnable<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [StreamsLogsEnable]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        StreamsLogsEnable {
            transport,
            parts: StreamsLogsEnableParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> StreamsLogsEnable<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        StreamsLogsEnable {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
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
    #[doc = "Period to wait for a connection to the master node. If no response is received before the timeout expires, the request fails and returns an error."]
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
    #[doc = "Period to wait for a response. If no response is received before the timeout expires, the request fails and returns an error."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Streams Logs Enable API that can be awaited"]
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
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Streams Status API"]
pub enum StreamsStatusParts {
    #[doc = "No parts"]
    None,
}
impl StreamsStatusParts {
    #[doc = "Builds a relative URL path to the Streams Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            StreamsStatusParts::None => "/_streams/status".into(),
        }
    }
}
#[doc = "Builder for the [Streams Status API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/streams-status.html)\n\nReturn the current status of the streams feature for each streams type"]
#[derive(Clone, Debug)]
pub struct StreamsStatus<'a, 'b> {
    transport: &'a Transport,
    parts: StreamsStatusParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    mater_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> StreamsStatus<'a, 'b> {
    #[doc = "Creates a new instance of [StreamsStatus]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        StreamsStatus {
            transport,
            parts: StreamsStatusParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            mater_timeout: None,
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
    #[doc = "Period to wait for a response. If no response is received before the timeout expires, the request fails and returns an error."]
    pub fn mater_timeout(mut self, mater_timeout: &'b str) -> Self {
        self.mater_timeout = Some(mater_timeout);
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
    #[doc = "Creates an asynchronous call to the Streams Status API that can be awaited"]
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
                mater_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                mater_timeout: self.mater_timeout,
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
#[doc = "Namespace client for Streams APIs"]
pub struct Streams<'a> {
    transport: &'a Transport,
}
impl<'a> Streams<'a> {
    #[doc = "Creates a new instance of [Streams]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Streams Logs Disable API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/streams-logs-disable.html)\n\nDisable the Logs Streams feature for this cluster"]
    pub fn logs_disable<'b>(&'a self) -> StreamsLogsDisable<'a, 'b, ()> {
        StreamsLogsDisable::new(self.transport())
    }
    #[doc = "[Streams Logs Enable API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/streams-logs-enable.html)\n\nEnable the Logs Streams feature for this cluster"]
    pub fn logs_enable<'b>(&'a self) -> StreamsLogsEnable<'a, 'b, ()> {
        StreamsLogsEnable::new(self.transport())
    }
    #[doc = "[Streams Status API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/streams-status.html)\n\nReturn the current status of the streams feature for each streams type"]
    pub fn status<'b>(&'a self) -> StreamsStatus<'a, 'b> {
        StreamsStatus::new(self.transport())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Streams APIs"]
    pub fn streams(&self) -> Streams<'_> {
        Streams::new(self.transport())
    }
}
