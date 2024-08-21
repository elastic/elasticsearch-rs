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

//! The [shutdown APIs](https://www.elastic.co/guide/en/elasticsearch/reference/master/node-lifecycle-api.html) allows preparing nodes for temporary or permanent shutdown, monitor the shutdown status, and enable a previously shut-down node to resume normal operations.

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
#[doc = "API parts for the Shutdown Delete Node API"]
pub enum ShutdownDeleteNodeParts<'b> {
    #[doc = "NodeId"]
    NodeId(&'b str),
}
impl<'b> ShutdownDeleteNodeParts<'b> {
    #[doc = "Builds a relative URL path to the Shutdown Delete Node API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ShutdownDeleteNodeParts::NodeId(node_id) => {
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_node_id.len());
                p.push_str("/_nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/shutdown");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Shutdown Delete Node API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15)\n\nRemoves a node from the shutdown list. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
#[derive(Clone, Debug)]
pub struct ShutdownDeleteNode<'a, 'b> {
    transport: &'a Transport,
    parts: ShutdownDeleteNodeParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ShutdownDeleteNode<'a, 'b> {
    #[doc = "Creates a new instance of [ShutdownDeleteNode] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ShutdownDeleteNodeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ShutdownDeleteNode {
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
    #[doc = "Creates an asynchronous call to the Shutdown Delete Node API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Delete;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Shutdown Get Node API"]
pub enum ShutdownGetNodeParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "NodeId"]
    NodeId(&'b str),
}
impl<'b> ShutdownGetNodeParts<'b> {
    #[doc = "Builds a relative URL path to the Shutdown Get Node API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ShutdownGetNodeParts::None => "/_nodes/shutdown".into(),
            ShutdownGetNodeParts::NodeId(node_id) => {
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_node_id.len());
                p.push_str("/_nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/shutdown");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Shutdown Get Node API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15)\n\nRetrieve status of a node or nodes that are currently marked as shutting down. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
#[derive(Clone, Debug)]
pub struct ShutdownGetNode<'a, 'b> {
    transport: &'a Transport,
    parts: ShutdownGetNodeParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ShutdownGetNode<'a, 'b> {
    #[doc = "Creates a new instance of [ShutdownGetNode] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ShutdownGetNodeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ShutdownGetNode {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
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
    #[doc = "Timeout for processing on master node"]
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
    #[doc = "Creates an asynchronous call to the Shutdown Get Node API that can be awaited"]
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
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Shutdown Put Node API"]
pub enum ShutdownPutNodeParts<'b> {
    #[doc = "NodeId"]
    NodeId(&'b str),
}
impl<'b> ShutdownPutNodeParts<'b> {
    #[doc = "Builds a relative URL path to the Shutdown Put Node API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ShutdownPutNodeParts::NodeId(node_id) => {
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_node_id.len());
                p.push_str("/_nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/shutdown");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Shutdown Put Node API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15)\n\nAdds a node to be shut down. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
#[derive(Clone, Debug)]
pub struct ShutdownPutNode<'a, 'b, B> {
    transport: &'a Transport,
    parts: ShutdownPutNodeParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ShutdownPutNode<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ShutdownPutNode] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ShutdownPutNodeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ShutdownPutNode {
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
    pub fn body<T>(self, body: T) -> ShutdownPutNode<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ShutdownPutNode {
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
    #[doc = "Creates an asynchronous call to the Shutdown Put Node API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
#[doc = "Namespace client for Shutdown APIs"]
pub struct Shutdown<'a> {
    transport: &'a Transport,
}
impl<'a> Shutdown<'a> {
    #[doc = "Creates a new instance of [Shutdown]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Shutdown Delete Node API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15)\n\nRemoves a node from the shutdown list. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
    pub fn delete_node<'b>(
        &'a self,
        parts: ShutdownDeleteNodeParts<'b>,
    ) -> ShutdownDeleteNode<'a, 'b> {
        ShutdownDeleteNode::new(self.transport(), parts)
    }
    #[doc = "[Shutdown Get Node API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15)\n\nRetrieve status of a node or nodes that are currently marked as shutting down. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
    pub fn get_node<'b>(&'a self, parts: ShutdownGetNodeParts<'b>) -> ShutdownGetNode<'a, 'b> {
        ShutdownGetNode::new(self.transport(), parts)
    }
    #[doc = "[Shutdown Put Node API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15)\n\nAdds a node to be shut down. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
    pub fn put_node<'b>(&'a self, parts: ShutdownPutNodeParts<'b>) -> ShutdownPutNode<'a, 'b, ()> {
        ShutdownPutNode::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Shutdown APIs"]
    pub fn shutdown(&self) -> Shutdown {
        Shutdown::new(self.transport())
    }
}
