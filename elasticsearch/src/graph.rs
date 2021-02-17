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

//! Graph APIs
//!
//! Enable extraction and summarization of information about documents and terms in Elasticsearch
//! indices, [inferring relationships across documents](https://www.elastic.co/what-is/elasticsearch-graph),
//! and allowing the [exploration of such relationships](https://www.elastic.co/guide/en/elasticsearch/reference/master/graph-explore-api.html).

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
#[doc = "API parts for the Graph Explore API"]
pub enum GraphExploreParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> GraphExploreParts<'b> {
    #[doc = "Builds a relative URL path to the Graph Explore API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GraphExploreParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_graph/explore");
                p.into()
            }
            GraphExploreParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_ty: Cow<str> = percent_encode(ty_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_index.len() + encoded_ty.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/");
                p.push_str(encoded_ty.as_ref());
                p.push_str("/_graph/explore");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Graph Explore API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/graph-explore-api.html)\n\nExplore extracted and summarized information about the documents and terms in an index."]
#[derive(Clone, Debug)]
pub struct GraphExplore<'a, 'b, B> {
    transport: &'a Transport,
    parts: GraphExploreParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> GraphExplore<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [GraphExplore] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: GraphExploreParts<'b>) -> Self {
        let headers = HeaderMap::new();
        GraphExplore {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            routing: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> GraphExplore<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        GraphExplore {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            routing: self.routing,
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
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
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
    #[doc = "Creates an asynchronous call to the Graph Explore API that can be awaited"]
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
                routing: Option<&'b str>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                routing: self.routing,
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
#[doc = "Namespace client for Graph APIs"]
pub struct Graph<'a> {
    transport: &'a Transport,
}
impl<'a> Graph<'a> {
    #[doc = "Creates a new instance of [Graph]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Graph Explore API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/graph-explore-api.html)\n\nExplore extracted and summarized information about the documents and terms in an index."]
    pub fn explore<'b>(&'a self, parts: GraphExploreParts<'b>) -> GraphExplore<'a, 'b, ()> {
        GraphExplore::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Graph APIs"]
    pub fn graph(&self) -> Graph {
        Graph::new(self.transport())
    }
}
