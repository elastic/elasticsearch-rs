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

//! Watcher (Alerting) APIs
//!
//! Enable [watching for changes or anomalies in data and perform the necessary actions in response](https://www.elastic.co/guide/en/elasticsearch/reference/master/xpack-alerting.html),
//! by creating and managing watches that take action based on a met condition.

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
#[doc = "API parts for the Watcher Ack Watch API"]
pub enum WatcherAckWatchParts<'b> {
    #[doc = "WatchId"]
    WatchId(&'b str),
    #[doc = "WatchId and ActionId"]
    WatchIdActionId(&'b str, &'b [&'b str]),
}
impl<'b> WatcherAckWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Ack Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherAckWatchParts::WatchId(ref watch_id) => {
                let encoded_watch_id: Cow<str> =
                    percent_encode(watch_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_watch_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(encoded_watch_id.as_ref());
                p.push_str("/_ack");
                p.into()
            }
            WatcherAckWatchParts::WatchIdActionId(ref watch_id, ref action_id) => {
                let action_id_str = action_id.join(",");
                let encoded_watch_id: Cow<str> =
                    percent_encode(watch_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_action_id: Cow<str> =
                    percent_encode(action_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    22usize + encoded_watch_id.len() + encoded_action_id.len(),
                );
                p.push_str("/_watcher/watch/");
                p.push_str(encoded_watch_id.as_ref());
                p.push_str("/_ack/");
                p.push_str(encoded_action_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Watcher Ack Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-ack-watch.html)\n\nAcknowledges a watch, manually throttling the execution of the watch's actions."]
#[derive(Clone, Debug)]
pub struct WatcherAckWatch<'a, 'b, B> {
    transport: &'a Transport,
    parts: WatcherAckWatchParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WatcherAckWatch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherAckWatch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WatcherAckWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherAckWatch {
            transport,
            parts,
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
    pub fn body<T>(self, body: T) -> WatcherAckWatch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherAckWatch {
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
    #[doc = "Creates an asynchronous call to the Watcher Ack Watch API that can be awaited"]
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
#[doc = "API parts for the Watcher Activate Watch API"]
pub enum WatcherActivateWatchParts<'b> {
    #[doc = "WatchId"]
    WatchId(&'b str),
}
impl<'b> WatcherActivateWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Activate Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherActivateWatchParts::WatchId(ref watch_id) => {
                let encoded_watch_id: Cow<str> =
                    percent_encode(watch_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(26usize + encoded_watch_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(encoded_watch_id.as_ref());
                p.push_str("/_activate");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Watcher Activate Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-activate-watch.html)\n\nActivates a currently inactive watch."]
#[derive(Clone, Debug)]
pub struct WatcherActivateWatch<'a, 'b, B> {
    transport: &'a Transport,
    parts: WatcherActivateWatchParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WatcherActivateWatch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherActivateWatch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WatcherActivateWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherActivateWatch {
            transport,
            parts,
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
    pub fn body<T>(self, body: T) -> WatcherActivateWatch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherActivateWatch {
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
    #[doc = "Creates an asynchronous call to the Watcher Activate Watch API that can be awaited"]
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
#[doc = "API parts for the Watcher Deactivate Watch API"]
pub enum WatcherDeactivateWatchParts<'b> {
    #[doc = "WatchId"]
    WatchId(&'b str),
}
impl<'b> WatcherDeactivateWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Deactivate Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherDeactivateWatchParts::WatchId(ref watch_id) => {
                let encoded_watch_id: Cow<str> =
                    percent_encode(watch_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(28usize + encoded_watch_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(encoded_watch_id.as_ref());
                p.push_str("/_deactivate");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Watcher Deactivate Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-deactivate-watch.html)\n\nDeactivates a currently active watch."]
#[derive(Clone, Debug)]
pub struct WatcherDeactivateWatch<'a, 'b, B> {
    transport: &'a Transport,
    parts: WatcherDeactivateWatchParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WatcherDeactivateWatch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherDeactivateWatch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WatcherDeactivateWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherDeactivateWatch {
            transport,
            parts,
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
    pub fn body<T>(self, body: T) -> WatcherDeactivateWatch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherDeactivateWatch {
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
    #[doc = "Creates an asynchronous call to the Watcher Deactivate Watch API that can be awaited"]
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
#[doc = "API parts for the Watcher Delete Watch API"]
pub enum WatcherDeleteWatchParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> WatcherDeleteWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Delete Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherDeleteWatchParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Watcher Delete Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-delete-watch.html)\n\nRemoves a watch from Watcher."]
#[derive(Clone, Debug)]
pub struct WatcherDeleteWatch<'a, 'b> {
    transport: &'a Transport,
    parts: WatcherDeleteWatchParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> WatcherDeleteWatch<'a, 'b> {
    #[doc = "Creates a new instance of [WatcherDeleteWatch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WatcherDeleteWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherDeleteWatch {
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
    #[doc = "Creates an asynchronous call to the Watcher Delete Watch API that can be awaited"]
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
#[doc = "API parts for the Watcher Execute Watch API"]
pub enum WatcherExecuteWatchParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "No parts"]
    None,
}
impl<'b> WatcherExecuteWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Execute Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherExecuteWatchParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(25usize + encoded_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(encoded_id.as_ref());
                p.push_str("/_execute");
                p.into()
            }
            WatcherExecuteWatchParts::None => "/_watcher/watch/_execute".into(),
        }
    }
}
#[doc = "Builder for the [Watcher Execute Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-execute-watch.html)\n\nForces the execution of a stored watch."]
#[derive(Clone, Debug)]
pub struct WatcherExecuteWatch<'a, 'b, B> {
    transport: &'a Transport,
    parts: WatcherExecuteWatchParts<'b>,
    body: Option<B>,
    debug: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WatcherExecuteWatch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherExecuteWatch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WatcherExecuteWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherExecuteWatch {
            transport,
            parts,
            headers,
            body: None,
            debug: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherExecuteWatch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherExecuteWatch {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            debug: self.debug,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "indicates whether the watch should execute in debug mode"]
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = Some(debug);
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
    #[doc = "Creates an asynchronous call to the Watcher Execute Watch API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                debug: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                debug: self.debug,
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
#[doc = "API parts for the Watcher Get Watch API"]
pub enum WatcherGetWatchParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> WatcherGetWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Get Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherGetWatchParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Watcher Get Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-get-watch.html)\n\nRetrieves a watch by its ID."]
#[derive(Clone, Debug)]
pub struct WatcherGetWatch<'a, 'b> {
    transport: &'a Transport,
    parts: WatcherGetWatchParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> WatcherGetWatch<'a, 'b> {
    #[doc = "Creates a new instance of [WatcherGetWatch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WatcherGetWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherGetWatch {
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
    #[doc = "Creates an asynchronous call to the Watcher Get Watch API that can be awaited"]
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
#[doc = "API parts for the Watcher Put Watch API"]
pub enum WatcherPutWatchParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> WatcherPutWatchParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Put Watch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherPutWatchParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Watcher Put Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-put-watch.html)\n\nCreates a new watch, or updates an existing one."]
#[derive(Clone, Debug)]
pub struct WatcherPutWatch<'a, 'b, B> {
    transport: &'a Transport,
    parts: WatcherPutWatchParts<'b>,
    active: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    version: Option<i64>,
}
impl<'a, 'b, B> WatcherPutWatch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherPutWatch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WatcherPutWatchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherPutWatch {
            transport,
            parts,
            headers,
            active: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            pretty: None,
            request_timeout: None,
            source: None,
            version: None,
        }
    }
    #[doc = "Specify whether the watch is in/active by default"]
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherPutWatch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherPutWatch {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            active: self.active,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            if_primary_term: self.if_primary_term,
            if_seq_no: self.if_seq_no,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            version: self.version,
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
    #[doc = "only update the watch if the last operation that has changed the watch has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "only update the watch if the last operation that has changed the watch has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
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
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Creates an asynchronous call to the Watcher Put Watch API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                active: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                if_primary_term: Option<i64>,
                if_seq_no: Option<i64>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                version: Option<i64>,
            }
            let query_params = QueryParams {
                active: self.active,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                pretty: self.pretty,
                source: self.source,
                version: self.version,
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
#[doc = "API parts for the Watcher Query Watches API"]
pub enum WatcherQueryWatchesParts {
    #[doc = "No parts"]
    None,
}
impl WatcherQueryWatchesParts {
    #[doc = "Builds a relative URL path to the Watcher Query Watches API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherQueryWatchesParts::None => "/_watcher/_query/watches".into(),
        }
    }
}
#[doc = "Builder for the [Watcher Query Watches API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-query-watches.html)\n\nRetrieves stored watches."]
#[derive(Clone, Debug)]
pub struct WatcherQueryWatches<'a, 'b, B> {
    transport: &'a Transport,
    parts: WatcherQueryWatchesParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WatcherQueryWatches<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherQueryWatches]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        WatcherQueryWatches {
            transport,
            parts: WatcherQueryWatchesParts::None,
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
    pub fn body<T>(self, body: T) -> WatcherQueryWatches<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherQueryWatches {
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
    #[doc = "Creates an asynchronous call to the Watcher Query Watches API that can be awaited"]
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Watcher Start API"]
pub enum WatcherStartParts {
    #[doc = "No parts"]
    None,
}
impl WatcherStartParts {
    #[doc = "Builds a relative URL path to the Watcher Start API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherStartParts::None => "/_watcher/_start".into(),
        }
    }
}
#[doc = "Builder for the [Watcher Start API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-start.html)\n\nStarts Watcher if it is not already running."]
#[derive(Clone, Debug)]
pub struct WatcherStart<'a, 'b, B> {
    transport: &'a Transport,
    parts: WatcherStartParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WatcherStart<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherStart]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        WatcherStart {
            transport,
            parts: WatcherStartParts::None,
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
    pub fn body<T>(self, body: T) -> WatcherStart<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherStart {
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
    #[doc = "Creates an asynchronous call to the Watcher Start API that can be awaited"]
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
#[doc = "API parts for the Watcher Stats API"]
pub enum WatcherStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Metric"]
    Metric(&'b [&'b str]),
}
impl<'b> WatcherStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Watcher Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherStatsParts::None => "/_watcher/stats".into(),
            WatcherStatsParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_metric.len());
                p.push_str("/_watcher/stats/");
                p.push_str(encoded_metric.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Watcher Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-stats.html)\n\nRetrieves the current Watcher metrics."]
#[derive(Clone, Debug)]
pub struct WatcherStats<'a, 'b> {
    transport: &'a Transport,
    parts: WatcherStatsParts<'b>,
    emit_stacktraces: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    metric: Option<&'b [&'b str]>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> WatcherStats<'a, 'b> {
    #[doc = "Creates a new instance of [WatcherStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WatcherStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WatcherStats {
            transport,
            parts,
            headers,
            emit_stacktraces: None,
            error_trace: None,
            filter_path: None,
            human: None,
            metric: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Emits stack traces of currently running watches"]
    pub fn emit_stacktraces(mut self, emit_stacktraces: bool) -> Self {
        self.emit_stacktraces = Some(emit_stacktraces);
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
    #[doc = "Controls what additional stat metrics should be include in the response"]
    pub fn metric(mut self, metric: &'b [&'b str]) -> Self {
        self.metric = Some(metric);
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
    #[doc = "Creates an asynchronous call to the Watcher Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                emit_stacktraces: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                metric: Option<&'b [&'b str]>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                emit_stacktraces: self.emit_stacktraces,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                metric: self.metric,
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
#[doc = "API parts for the Watcher Stop API"]
pub enum WatcherStopParts {
    #[doc = "No parts"]
    None,
}
impl WatcherStopParts {
    #[doc = "Builds a relative URL path to the Watcher Stop API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WatcherStopParts::None => "/_watcher/_stop".into(),
        }
    }
}
#[doc = "Builder for the [Watcher Stop API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-stop.html)\n\nStops Watcher if it is running."]
#[derive(Clone, Debug)]
pub struct WatcherStop<'a, 'b, B> {
    transport: &'a Transport,
    parts: WatcherStopParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WatcherStop<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WatcherStop]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        WatcherStop {
            transport,
            parts: WatcherStopParts::None,
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
    pub fn body<T>(self, body: T) -> WatcherStop<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherStop {
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
    #[doc = "Creates an asynchronous call to the Watcher Stop API that can be awaited"]
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
#[doc = "Namespace client for Watcher APIs"]
pub struct Watcher<'a> {
    transport: &'a Transport,
}
impl<'a> Watcher<'a> {
    #[doc = "Creates a new instance of [Watcher]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Watcher Ack Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-ack-watch.html)\n\nAcknowledges a watch, manually throttling the execution of the watch's actions."]
    pub fn ack_watch<'b>(&'a self, parts: WatcherAckWatchParts<'b>) -> WatcherAckWatch<'a, 'b, ()> {
        WatcherAckWatch::new(self.transport(), parts)
    }
    #[doc = "[Watcher Activate Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-activate-watch.html)\n\nActivates a currently inactive watch."]
    pub fn activate_watch<'b>(
        &'a self,
        parts: WatcherActivateWatchParts<'b>,
    ) -> WatcherActivateWatch<'a, 'b, ()> {
        WatcherActivateWatch::new(self.transport(), parts)
    }
    #[doc = "[Watcher Deactivate Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-deactivate-watch.html)\n\nDeactivates a currently active watch."]
    pub fn deactivate_watch<'b>(
        &'a self,
        parts: WatcherDeactivateWatchParts<'b>,
    ) -> WatcherDeactivateWatch<'a, 'b, ()> {
        WatcherDeactivateWatch::new(self.transport(), parts)
    }
    #[doc = "[Watcher Delete Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-delete-watch.html)\n\nRemoves a watch from Watcher."]
    pub fn delete_watch<'b>(
        &'a self,
        parts: WatcherDeleteWatchParts<'b>,
    ) -> WatcherDeleteWatch<'a, 'b> {
        WatcherDeleteWatch::new(self.transport(), parts)
    }
    #[doc = "[Watcher Execute Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-execute-watch.html)\n\nForces the execution of a stored watch."]
    pub fn execute_watch<'b>(
        &'a self,
        parts: WatcherExecuteWatchParts<'b>,
    ) -> WatcherExecuteWatch<'a, 'b, ()> {
        WatcherExecuteWatch::new(self.transport(), parts)
    }
    #[doc = "[Watcher Get Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-get-watch.html)\n\nRetrieves a watch by its ID."]
    pub fn get_watch<'b>(&'a self, parts: WatcherGetWatchParts<'b>) -> WatcherGetWatch<'a, 'b> {
        WatcherGetWatch::new(self.transport(), parts)
    }
    #[doc = "[Watcher Put Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-put-watch.html)\n\nCreates a new watch, or updates an existing one."]
    pub fn put_watch<'b>(&'a self, parts: WatcherPutWatchParts<'b>) -> WatcherPutWatch<'a, 'b, ()> {
        WatcherPutWatch::new(self.transport(), parts)
    }
    #[doc = "[Watcher Query Watches API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-query-watches.html)\n\nRetrieves stored watches."]
    pub fn query_watches<'b>(&'a self) -> WatcherQueryWatches<'a, 'b, ()> {
        WatcherQueryWatches::new(self.transport())
    }
    #[doc = "[Watcher Start API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-start.html)\n\nStarts Watcher if it is not already running."]
    pub fn start<'b>(&'a self) -> WatcherStart<'a, 'b, ()> {
        WatcherStart::new(self.transport())
    }
    #[doc = "[Watcher Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-stats.html)\n\nRetrieves the current Watcher metrics."]
    pub fn stats<'b>(&'a self, parts: WatcherStatsParts<'b>) -> WatcherStats<'a, 'b> {
        WatcherStats::new(self.transport(), parts)
    }
    #[doc = "[Watcher Stop API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/watcher-api-stop.html)\n\nStops Watcher if it is running."]
    pub fn stop<'b>(&'a self) -> WatcherStop<'a, 'b, ()> {
        WatcherStop::new(self.transport())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Watcher APIs"]
    pub fn watcher(&self) -> Watcher {
        Watcher::new(self.transport())
    }
}
