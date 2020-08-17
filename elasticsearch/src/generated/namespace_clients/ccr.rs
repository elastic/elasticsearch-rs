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
// cargo run -p api_generator
// -----------------------------------------------
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
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Delete Auto Follow Pattern API"]
pub enum CcrDeleteAutoFollowPatternParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> CcrDeleteAutoFollowPatternParts<'b> {
    #[doc = "Builds a relative URL path to the Ccr Delete Auto Follow Pattern API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrDeleteAutoFollowPatternParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CcrDeleteAutoFollowPatternParts<'b> {
    #[doc = "Builds a [CcrDeleteAutoFollowPatternParts::Name] for the Ccr Delete Auto Follow Pattern API"]
    fn from(t: &'b str) -> CcrDeleteAutoFollowPatternParts<'b> {
        CcrDeleteAutoFollowPatternParts::Name(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Delete Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-delete-auto-follow-pattern.html)\n\nDeletes auto-follow patterns."]
pub struct CcrDeleteAutoFollowPattern<'a, 'b> {
    transport: &'a Transport,
    parts: CcrDeleteAutoFollowPatternParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrDeleteAutoFollowPattern<'a, 'b> {
    #[doc = "Creates a new instance of [CcrDeleteAutoFollowPattern] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CcrDeleteAutoFollowPatternParts<'b>>,
    {
        let headers = HeaderMap::new();
        CcrDeleteAutoFollowPattern {
            transport,
            parts: parts.into(),
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Delete Auto Follow Pattern API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Follow API"]
pub enum CcrFollowParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> CcrFollowParts<'b> {
    #[doc = "Builds a relative URL path to the Ccr Follow API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrFollowParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_ccr/follow");
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CcrFollowParts<'b> {
    #[doc = "Builds a [CcrFollowParts::Index] for the Ccr Follow API"]
    fn from(t: &'b str) -> CcrFollowParts<'b> {
        CcrFollowParts::Index(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-put-follow.html)\n\nCreates a new follower index configured to follow the referenced leader index."]
pub struct CcrFollow<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrFollowParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> CcrFollow<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrFollow] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CcrFollowParts<'b>>,
    {
        let headers = HeaderMap::new();
        CcrFollow {
            transport,
            parts: parts.into(),
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrFollow<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrFollow {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            wait_for_active_shards: self.wait_for_active_shards,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before returning. Defaults to 0. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Follow API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Follow Info API"]
pub enum CcrFollowInfoParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CcrFollowInfoParts<'b> {
    #[doc = "Builds a relative URL path to the Ccr Follow Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrFollowInfoParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_ccr/info");
                p.into()
            }
        }
    }
}
impl<'b> From<&'b [&'b str]> for CcrFollowInfoParts<'b> {
    #[doc = "Builds a [CcrFollowInfoParts::Index] for the Ccr Follow Info API"]
    fn from(t: &'b [&'b str]) -> CcrFollowInfoParts<'b> {
        CcrFollowInfoParts::Index(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Follow Info API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-get-follow-info.html)\n\nRetrieves information about all follower indices, including parameters and status for each follower index"]
pub struct CcrFollowInfo<'a, 'b> {
    transport: &'a Transport,
    parts: CcrFollowInfoParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrFollowInfo<'a, 'b> {
    #[doc = "Creates a new instance of [CcrFollowInfo] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CcrFollowInfoParts<'b>>,
    {
        let headers = HeaderMap::new();
        CcrFollowInfo {
            transport,
            parts: parts.into(),
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Follow Info API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Follow Stats API"]
pub enum CcrFollowStatsParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CcrFollowStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Ccr Follow Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrFollowStatsParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_ccr/stats");
                p.into()
            }
        }
    }
}
impl<'b> From<&'b [&'b str]> for CcrFollowStatsParts<'b> {
    #[doc = "Builds a [CcrFollowStatsParts::Index] for the Ccr Follow Stats API"]
    fn from(t: &'b [&'b str]) -> CcrFollowStatsParts<'b> {
        CcrFollowStatsParts::Index(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Follow Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-get-follow-stats.html)\n\nRetrieves follower stats. return shard-level stats about the following tasks associated with each shard for the specified indices."]
pub struct CcrFollowStats<'a, 'b> {
    transport: &'a Transport,
    parts: CcrFollowStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrFollowStats<'a, 'b> {
    #[doc = "Creates a new instance of [CcrFollowStats] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CcrFollowStatsParts<'b>>,
    {
        let headers = HeaderMap::new();
        CcrFollowStats {
            transport,
            parts: parts.into(),
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Follow Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Forget Follower API"]
pub enum CcrForgetFollowerParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> CcrForgetFollowerParts<'b> {
    #[doc = "Builds a relative URL path to the Ccr Forget Follower API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrForgetFollowerParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_ccr/forget_follower");
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CcrForgetFollowerParts<'b> {
    #[doc = "Builds a [CcrForgetFollowerParts::Index] for the Ccr Forget Follower API"]
    fn from(t: &'b str) -> CcrForgetFollowerParts<'b> {
        CcrForgetFollowerParts::Index(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Forget Follower API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-post-forget-follower.html)\n\nRemoves the follower retention leases from the leader."]
pub struct CcrForgetFollower<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrForgetFollowerParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrForgetFollower<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrForgetFollower] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CcrForgetFollowerParts<'b>>,
    {
        let headers = HeaderMap::new();
        CcrForgetFollower {
            transport,
            parts: parts.into(),
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrForgetFollower<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrForgetFollower {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Forget Follower API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Get Auto Follow Pattern API"]
pub enum CcrGetAutoFollowPatternParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> CcrGetAutoFollowPatternParts<'b> {
    #[doc = "Builds a relative URL path to the Ccr Get Auto Follow Pattern API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrGetAutoFollowPatternParts::None => "/_ccr/auto_follow".into(),
            CcrGetAutoFollowPatternParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CcrGetAutoFollowPatternParts<'b> {
    #[doc = "Builds a [CcrGetAutoFollowPatternParts::Name] for the Ccr Get Auto Follow Pattern API"]
    fn from(t: &'b str) -> CcrGetAutoFollowPatternParts<'b> {
        CcrGetAutoFollowPatternParts::Name(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Get Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-get-auto-follow-pattern.html)\n\nGets configured auto-follow patterns. Returns the specified auto-follow pattern collection."]
pub struct CcrGetAutoFollowPattern<'a, 'b> {
    transport: &'a Transport,
    parts: CcrGetAutoFollowPatternParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrGetAutoFollowPattern<'a, 'b> {
    #[doc = "Creates a new instance of [CcrGetAutoFollowPattern] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CcrGetAutoFollowPatternParts<'b>>,
    {
        let headers = HeaderMap::new();
        CcrGetAutoFollowPattern {
            transport,
            parts: parts.into(),
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Get Auto Follow Pattern API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Pause Auto Follow Pattern API"]
pub enum CcrPauseAutoFollowPatternParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> CcrPauseAutoFollowPatternParts<'b> {
    #[doc = "Builds a relative URL path to the Ccr Pause Auto Follow Pattern API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrPauseAutoFollowPatternParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/pause");
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CcrPauseAutoFollowPatternParts<'b> {
    #[doc = "Builds a [CcrPauseAutoFollowPatternParts::Name] for the Ccr Pause Auto Follow Pattern API"]
    fn from(t: &'b str) -> CcrPauseAutoFollowPatternParts<'b> {
        CcrPauseAutoFollowPatternParts::Name(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Pause Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-pause-auto-follow-pattern.html)\n\nPauses an auto-follow pattern"]
pub struct CcrPauseAutoFollowPattern<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrPauseAutoFollowPatternParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrPauseAutoFollowPattern<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrPauseAutoFollowPattern] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CcrPauseAutoFollowPatternParts<'b>>,
    {
        let headers = HeaderMap::new();
        CcrPauseAutoFollowPattern {
            transport,
            parts: parts.into(),
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrPauseAutoFollowPattern<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrPauseAutoFollowPattern {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Pause Auto Follow Pattern API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Pause Follow API"]
pub enum CcrPauseFollowParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> CcrPauseFollowParts<'b> {
    #[doc = "Builds a relative URL path to the Ccr Pause Follow API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrPauseFollowParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_ccr/pause_follow");
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CcrPauseFollowParts<'b> {
    #[doc = "Builds a [CcrPauseFollowParts::Index] for the Ccr Pause Follow API"]
    fn from(t: &'b str) -> CcrPauseFollowParts<'b> {
        CcrPauseFollowParts::Index(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Pause Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-post-pause-follow.html)\n\nPauses a follower index. The follower index will not fetch any additional operations from the leader index."]
pub struct CcrPauseFollow<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrPauseFollowParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrPauseFollow<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrPauseFollow] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CcrPauseFollowParts<'b>>,
    {
        let headers = HeaderMap::new();
        CcrPauseFollow {
            transport,
            parts: parts.into(),
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrPauseFollow<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrPauseFollow {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Pause Follow API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Put Auto Follow Pattern API"]
pub enum CcrPutAutoFollowPatternParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> CcrPutAutoFollowPatternParts<'b> {
    #[doc = "Builds a relative URL path to the Ccr Put Auto Follow Pattern API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrPutAutoFollowPatternParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CcrPutAutoFollowPatternParts<'b> {
    #[doc = "Builds a [CcrPutAutoFollowPatternParts::Name] for the Ccr Put Auto Follow Pattern API"]
    fn from(t: &'b str) -> CcrPutAutoFollowPatternParts<'b> {
        CcrPutAutoFollowPatternParts::Name(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Put Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-put-auto-follow-pattern.html)\n\nCreates a new named collection of auto-follow patterns against a specified remote cluster. Newly created indices on the remote cluster matching any of the specified patterns will be automatically configured as follower indices."]
pub struct CcrPutAutoFollowPattern<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrPutAutoFollowPatternParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrPutAutoFollowPattern<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrPutAutoFollowPattern] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CcrPutAutoFollowPatternParts<'b>>,
    {
        let headers = HeaderMap::new();
        CcrPutAutoFollowPattern {
            transport,
            parts: parts.into(),
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrPutAutoFollowPattern<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrPutAutoFollowPattern {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Put Auto Follow Pattern API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Resume Auto Follow Pattern API"]
pub enum CcrResumeAutoFollowPatternParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> CcrResumeAutoFollowPatternParts<'b> {
    #[doc = "Builds a relative URL path to the Ccr Resume Auto Follow Pattern API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrResumeAutoFollowPatternParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(25usize + encoded_name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/resume");
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CcrResumeAutoFollowPatternParts<'b> {
    #[doc = "Builds a [CcrResumeAutoFollowPatternParts::Name] for the Ccr Resume Auto Follow Pattern API"]
    fn from(t: &'b str) -> CcrResumeAutoFollowPatternParts<'b> {
        CcrResumeAutoFollowPatternParts::Name(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Resume Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-resume-auto-follow-pattern.html)\n\nResumes an auto-follow pattern that has been paused"]
pub struct CcrResumeAutoFollowPattern<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrResumeAutoFollowPatternParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrResumeAutoFollowPattern<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrResumeAutoFollowPattern] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CcrResumeAutoFollowPatternParts<'b>>,
    {
        let headers = HeaderMap::new();
        CcrResumeAutoFollowPattern {
            transport,
            parts: parts.into(),
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrResumeAutoFollowPattern<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrResumeAutoFollowPattern {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Resume Auto Follow Pattern API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Resume Follow API"]
pub enum CcrResumeFollowParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> CcrResumeFollowParts<'b> {
    #[doc = "Builds a relative URL path to the Ccr Resume Follow API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrResumeFollowParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_ccr/resume_follow");
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CcrResumeFollowParts<'b> {
    #[doc = "Builds a [CcrResumeFollowParts::Index] for the Ccr Resume Follow API"]
    fn from(t: &'b str) -> CcrResumeFollowParts<'b> {
        CcrResumeFollowParts::Index(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Resume Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-post-resume-follow.html)\n\nResumes a follower index that has been paused"]
pub struct CcrResumeFollow<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrResumeFollowParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrResumeFollow<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrResumeFollow] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CcrResumeFollowParts<'b>>,
    {
        let headers = HeaderMap::new();
        CcrResumeFollow {
            transport,
            parts: parts.into(),
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrResumeFollow<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrResumeFollow {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Resume Follow API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Stats API"]
pub enum CcrStatsParts {
    #[doc = "No parts"]
    None,
}
impl CcrStatsParts {
    #[doc = "Builds a relative URL path to the Ccr Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrStatsParts::None => "/_ccr/stats".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-get-stats.html)\n\nGets all stats related to cross-cluster replication."]
pub struct CcrStats<'a, 'b> {
    transport: &'a Transport,
    parts: CcrStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrStats<'a, 'b> {
    #[doc = "Creates a new instance of [CcrStats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        CcrStats {
            transport,
            parts: CcrStatsParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Unfollow API"]
pub enum CcrUnfollowParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> CcrUnfollowParts<'b> {
    #[doc = "Builds a relative URL path to the Ccr Unfollow API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrUnfollowParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_ccr/unfollow");
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CcrUnfollowParts<'b> {
    #[doc = "Builds a [CcrUnfollowParts::Index] for the Ccr Unfollow API"]
    fn from(t: &'b str) -> CcrUnfollowParts<'b> {
        CcrUnfollowParts::Index(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Unfollow API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-post-unfollow.html)\n\nStops the following task associated with a follower index and removes index metadata and settings associated with cross-cluster replication."]
pub struct CcrUnfollow<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrUnfollowParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrUnfollow<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrUnfollow] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CcrUnfollowParts<'b>>,
    {
        let headers = HeaderMap::new();
        CcrUnfollow {
            transport,
            parts: parts.into(),
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrUnfollow<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrUnfollow {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Unfollow API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Cross Cluster Replication APIs"]
pub struct Ccr<'a> {
    transport: &'a Transport,
}
impl<'a> Ccr<'a> {
    #[doc = "Creates a new instance of [Ccr]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Ccr Delete Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-delete-auto-follow-pattern.html)\n\nDeletes auto-follow patterns."]
    pub fn delete_auto_follow_pattern<'b, P>(
        &'a self,
        parts: P,
    ) -> CcrDeleteAutoFollowPattern<'a, 'b>
    where
        P: Into<CcrDeleteAutoFollowPatternParts<'b>>,
    {
        CcrDeleteAutoFollowPattern::new(self.transport(), parts)
    }
    #[doc = "[Ccr Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-put-follow.html)\n\nCreates a new follower index configured to follow the referenced leader index."]
    pub fn follow<'b, P>(&'a self, parts: P) -> CcrFollow<'a, 'b, ()>
    where
        P: Into<CcrFollowParts<'b>>,
    {
        CcrFollow::new(self.transport(), parts)
    }
    #[doc = "[Ccr Follow Info API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-get-follow-info.html)\n\nRetrieves information about all follower indices, including parameters and status for each follower index"]
    pub fn follow_info<'b, P>(&'a self, parts: P) -> CcrFollowInfo<'a, 'b>
    where
        P: Into<CcrFollowInfoParts<'b>>,
    {
        CcrFollowInfo::new(self.transport(), parts)
    }
    #[doc = "[Ccr Follow Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-get-follow-stats.html)\n\nRetrieves follower stats. return shard-level stats about the following tasks associated with each shard for the specified indices."]
    pub fn follow_stats<'b, P>(&'a self, parts: P) -> CcrFollowStats<'a, 'b>
    where
        P: Into<CcrFollowStatsParts<'b>>,
    {
        CcrFollowStats::new(self.transport(), parts)
    }
    #[doc = "[Ccr Forget Follower API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-post-forget-follower.html)\n\nRemoves the follower retention leases from the leader."]
    pub fn forget_follower<'b, P>(&'a self, parts: P) -> CcrForgetFollower<'a, 'b, ()>
    where
        P: Into<CcrForgetFollowerParts<'b>>,
    {
        CcrForgetFollower::new(self.transport(), parts)
    }
    #[doc = "[Ccr Get Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-get-auto-follow-pattern.html)\n\nGets configured auto-follow patterns. Returns the specified auto-follow pattern collection."]
    pub fn get_auto_follow_pattern<'b, P>(&'a self, parts: P) -> CcrGetAutoFollowPattern<'a, 'b>
    where
        P: Into<CcrGetAutoFollowPatternParts<'b>>,
    {
        CcrGetAutoFollowPattern::new(self.transport(), parts)
    }
    #[doc = "[Ccr Pause Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-pause-auto-follow-pattern.html)\n\nPauses an auto-follow pattern"]
    pub fn pause_auto_follow_pattern<'b, P>(
        &'a self,
        parts: P,
    ) -> CcrPauseAutoFollowPattern<'a, 'b, ()>
    where
        P: Into<CcrPauseAutoFollowPatternParts<'b>>,
    {
        CcrPauseAutoFollowPattern::new(self.transport(), parts)
    }
    #[doc = "[Ccr Pause Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-post-pause-follow.html)\n\nPauses a follower index. The follower index will not fetch any additional operations from the leader index."]
    pub fn pause_follow<'b, P>(&'a self, parts: P) -> CcrPauseFollow<'a, 'b, ()>
    where
        P: Into<CcrPauseFollowParts<'b>>,
    {
        CcrPauseFollow::new(self.transport(), parts)
    }
    #[doc = "[Ccr Put Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-put-auto-follow-pattern.html)\n\nCreates a new named collection of auto-follow patterns against a specified remote cluster. Newly created indices on the remote cluster matching any of the specified patterns will be automatically configured as follower indices."]
    pub fn put_auto_follow_pattern<'b, P>(&'a self, parts: P) -> CcrPutAutoFollowPattern<'a, 'b, ()>
    where
        P: Into<CcrPutAutoFollowPatternParts<'b>>,
    {
        CcrPutAutoFollowPattern::new(self.transport(), parts)
    }
    #[doc = "[Ccr Resume Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-resume-auto-follow-pattern.html)\n\nResumes an auto-follow pattern that has been paused"]
    pub fn resume_auto_follow_pattern<'b, P>(
        &'a self,
        parts: P,
    ) -> CcrResumeAutoFollowPattern<'a, 'b, ()>
    where
        P: Into<CcrResumeAutoFollowPatternParts<'b>>,
    {
        CcrResumeAutoFollowPattern::new(self.transport(), parts)
    }
    #[doc = "[Ccr Resume Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-post-resume-follow.html)\n\nResumes a follower index that has been paused"]
    pub fn resume_follow<'b, P>(&'a self, parts: P) -> CcrResumeFollow<'a, 'b, ()>
    where
        P: Into<CcrResumeFollowParts<'b>>,
    {
        CcrResumeFollow::new(self.transport(), parts)
    }
    #[doc = "[Ccr Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-get-stats.html)\n\nGets all stats related to cross-cluster replication."]
    pub fn stats<'b>(&'a self) -> CcrStats<'a, 'b> {
        CcrStats::new(self.transport())
    }
    #[doc = "[Ccr Unfollow API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/ccr-post-unfollow.html)\n\nStops the following task associated with a follower index and removes index metadata and settings associated with cross-cluster replication."]
    pub fn unfollow<'b, P>(&'a self, parts: P) -> CcrUnfollow<'a, 'b, ()>
    where
        P: Into<CcrUnfollowParts<'b>>,
    {
        CcrUnfollow::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Cross Cluster Replication APIs"]
    pub fn ccr(&self) -> Ccr {
        Ccr::new(self.transport())
    }
}
