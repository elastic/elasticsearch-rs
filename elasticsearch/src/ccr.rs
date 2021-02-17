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

//! Cross-cluster Replication APIs
//!
//! [Enable replication of indices in remote clusters to a local cluster](https://www.elastic.co/guide/en/elasticsearch/reference/master/xpack-ccr.html).
//! This functionality can be used in some common production use cases:
//!
//! - Disaster recovery in case a primary cluster fails. A secondary cluster can serve as a hot backup
//! - Geo-proximity so that reads can be served locally

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
#[doc = "Builder for the [Ccr Delete Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-delete-auto-follow-pattern.html)\n\nDeletes auto-follow patterns."]
#[derive(Clone, Debug)]
pub struct CcrDeleteAutoFollowPattern<'a, 'b> {
    transport: &'a Transport,
    parts: CcrDeleteAutoFollowPatternParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrDeleteAutoFollowPattern<'a, 'b> {
    #[doc = "Creates a new instance of [CcrDeleteAutoFollowPattern] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CcrDeleteAutoFollowPatternParts<'b>) -> Self {
        let headers = HeaderMap::new();
        CcrDeleteAutoFollowPattern {
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
    #[doc = "Creates an asynchronous call to the Ccr Delete Auto Follow Pattern API that can be awaited"]
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
#[doc = "Builder for the [Ccr Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-put-follow.html)\n\nCreates a new follower index configured to follow the referenced leader index."]
#[derive(Clone, Debug)]
pub struct CcrFollow<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrFollowParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> CcrFollow<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrFollow] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CcrFollowParts<'b>) -> Self {
        let headers = HeaderMap::new();
        CcrFollow {
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
            request_timeout: self.request_timeout,
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
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
#[doc = "Builder for the [Ccr Follow Info API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-get-follow-info.html)\n\nRetrieves information about all follower indices, including parameters and status for each follower index"]
#[derive(Clone, Debug)]
pub struct CcrFollowInfo<'a, 'b> {
    transport: &'a Transport,
    parts: CcrFollowInfoParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrFollowInfo<'a, 'b> {
    #[doc = "Creates a new instance of [CcrFollowInfo] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CcrFollowInfoParts<'b>) -> Self {
        let headers = HeaderMap::new();
        CcrFollowInfo {
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
    #[doc = "Creates an asynchronous call to the Ccr Follow Info API that can be awaited"]
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
#[doc = "Builder for the [Ccr Follow Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-get-follow-stats.html)\n\nRetrieves follower stats. return shard-level stats about the following tasks associated with each shard for the specified indices."]
#[derive(Clone, Debug)]
pub struct CcrFollowStats<'a, 'b> {
    transport: &'a Transport,
    parts: CcrFollowStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrFollowStats<'a, 'b> {
    #[doc = "Creates a new instance of [CcrFollowStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CcrFollowStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        CcrFollowStats {
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
    #[doc = "Creates an asynchronous call to the Ccr Follow Stats API that can be awaited"]
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
#[doc = "Builder for the [Ccr Forget Follower API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-post-forget-follower.html)\n\nRemoves the follower retention leases from the leader."]
#[derive(Clone, Debug)]
pub struct CcrForgetFollower<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrForgetFollowerParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrForgetFollower<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrForgetFollower] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CcrForgetFollowerParts<'b>) -> Self {
        let headers = HeaderMap::new();
        CcrForgetFollower {
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
    #[doc = "Creates an asynchronous call to the Ccr Forget Follower API that can be awaited"]
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
#[doc = "Builder for the [Ccr Get Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-get-auto-follow-pattern.html)\n\nGets configured auto-follow patterns. Returns the specified auto-follow pattern collection."]
#[derive(Clone, Debug)]
pub struct CcrGetAutoFollowPattern<'a, 'b> {
    transport: &'a Transport,
    parts: CcrGetAutoFollowPatternParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrGetAutoFollowPattern<'a, 'b> {
    #[doc = "Creates a new instance of [CcrGetAutoFollowPattern] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CcrGetAutoFollowPatternParts<'b>) -> Self {
        let headers = HeaderMap::new();
        CcrGetAutoFollowPattern {
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
    #[doc = "Creates an asynchronous call to the Ccr Get Auto Follow Pattern API that can be awaited"]
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
#[doc = "Builder for the [Ccr Pause Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-pause-auto-follow-pattern.html)\n\nPauses an auto-follow pattern"]
#[derive(Clone, Debug)]
pub struct CcrPauseAutoFollowPattern<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrPauseAutoFollowPatternParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrPauseAutoFollowPattern<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrPauseAutoFollowPattern] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CcrPauseAutoFollowPatternParts<'b>) -> Self {
        let headers = HeaderMap::new();
        CcrPauseAutoFollowPattern {
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
    #[doc = "Creates an asynchronous call to the Ccr Pause Auto Follow Pattern API that can be awaited"]
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
#[doc = "Builder for the [Ccr Pause Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-post-pause-follow.html)\n\nPauses a follower index. The follower index will not fetch any additional operations from the leader index."]
#[derive(Clone, Debug)]
pub struct CcrPauseFollow<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrPauseFollowParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrPauseFollow<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrPauseFollow] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CcrPauseFollowParts<'b>) -> Self {
        let headers = HeaderMap::new();
        CcrPauseFollow {
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
    #[doc = "Creates an asynchronous call to the Ccr Pause Follow API that can be awaited"]
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
#[doc = "Builder for the [Ccr Put Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-put-auto-follow-pattern.html)\n\nCreates a new named collection of auto-follow patterns against a specified remote cluster. Newly created indices on the remote cluster matching any of the specified patterns will be automatically configured as follower indices."]
#[derive(Clone, Debug)]
pub struct CcrPutAutoFollowPattern<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrPutAutoFollowPatternParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrPutAutoFollowPattern<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrPutAutoFollowPattern] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CcrPutAutoFollowPatternParts<'b>) -> Self {
        let headers = HeaderMap::new();
        CcrPutAutoFollowPattern {
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
    #[doc = "Creates an asynchronous call to the Ccr Put Auto Follow Pattern API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
#[doc = "Builder for the [Ccr Resume Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-resume-auto-follow-pattern.html)\n\nResumes an auto-follow pattern that has been paused"]
#[derive(Clone, Debug)]
pub struct CcrResumeAutoFollowPattern<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrResumeAutoFollowPatternParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrResumeAutoFollowPattern<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrResumeAutoFollowPattern] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CcrResumeAutoFollowPatternParts<'b>) -> Self {
        let headers = HeaderMap::new();
        CcrResumeAutoFollowPattern {
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
    #[doc = "Creates an asynchronous call to the Ccr Resume Auto Follow Pattern API that can be awaited"]
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
#[doc = "Builder for the [Ccr Resume Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-post-resume-follow.html)\n\nResumes a follower index that has been paused"]
#[derive(Clone, Debug)]
pub struct CcrResumeFollow<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrResumeFollowParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrResumeFollow<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrResumeFollow] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CcrResumeFollowParts<'b>) -> Self {
        let headers = HeaderMap::new();
        CcrResumeFollow {
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
    #[doc = "Creates an asynchronous call to the Ccr Resume Follow API that can be awaited"]
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
#[doc = "Builder for the [Ccr Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-get-stats.html)\n\nGets all stats related to cross-cluster replication."]
#[derive(Clone, Debug)]
pub struct CcrStats<'a, 'b> {
    transport: &'a Transport,
    parts: CcrStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
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
    #[doc = "Creates an asynchronous call to the Ccr Stats API that can be awaited"]
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
#[doc = "Builder for the [Ccr Unfollow API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-post-unfollow.html)\n\nStops the following task associated with a follower index and removes index metadata and settings associated with cross-cluster replication."]
#[derive(Clone, Debug)]
pub struct CcrUnfollow<'a, 'b, B> {
    transport: &'a Transport,
    parts: CcrUnfollowParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrUnfollow<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrUnfollow] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CcrUnfollowParts<'b>) -> Self {
        let headers = HeaderMap::new();
        CcrUnfollow {
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
    #[doc = "Creates an asynchronous call to the Ccr Unfollow API that can be awaited"]
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
    #[doc = "[Ccr Delete Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-delete-auto-follow-pattern.html)\n\nDeletes auto-follow patterns."]
    pub fn delete_auto_follow_pattern<'b>(
        &'a self,
        parts: CcrDeleteAutoFollowPatternParts<'b>,
    ) -> CcrDeleteAutoFollowPattern<'a, 'b> {
        CcrDeleteAutoFollowPattern::new(self.transport(), parts)
    }
    #[doc = "[Ccr Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-put-follow.html)\n\nCreates a new follower index configured to follow the referenced leader index."]
    pub fn follow<'b>(&'a self, parts: CcrFollowParts<'b>) -> CcrFollow<'a, 'b, ()> {
        CcrFollow::new(self.transport(), parts)
    }
    #[doc = "[Ccr Follow Info API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-get-follow-info.html)\n\nRetrieves information about all follower indices, including parameters and status for each follower index"]
    pub fn follow_info<'b>(&'a self, parts: CcrFollowInfoParts<'b>) -> CcrFollowInfo<'a, 'b> {
        CcrFollowInfo::new(self.transport(), parts)
    }
    #[doc = "[Ccr Follow Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-get-follow-stats.html)\n\nRetrieves follower stats. return shard-level stats about the following tasks associated with each shard for the specified indices."]
    pub fn follow_stats<'b>(&'a self, parts: CcrFollowStatsParts<'b>) -> CcrFollowStats<'a, 'b> {
        CcrFollowStats::new(self.transport(), parts)
    }
    #[doc = "[Ccr Forget Follower API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-post-forget-follower.html)\n\nRemoves the follower retention leases from the leader."]
    pub fn forget_follower<'b>(
        &'a self,
        parts: CcrForgetFollowerParts<'b>,
    ) -> CcrForgetFollower<'a, 'b, ()> {
        CcrForgetFollower::new(self.transport(), parts)
    }
    #[doc = "[Ccr Get Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-get-auto-follow-pattern.html)\n\nGets configured auto-follow patterns. Returns the specified auto-follow pattern collection."]
    pub fn get_auto_follow_pattern<'b>(
        &'a self,
        parts: CcrGetAutoFollowPatternParts<'b>,
    ) -> CcrGetAutoFollowPattern<'a, 'b> {
        CcrGetAutoFollowPattern::new(self.transport(), parts)
    }
    #[doc = "[Ccr Pause Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-pause-auto-follow-pattern.html)\n\nPauses an auto-follow pattern"]
    pub fn pause_auto_follow_pattern<'b>(
        &'a self,
        parts: CcrPauseAutoFollowPatternParts<'b>,
    ) -> CcrPauseAutoFollowPattern<'a, 'b, ()> {
        CcrPauseAutoFollowPattern::new(self.transport(), parts)
    }
    #[doc = "[Ccr Pause Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-post-pause-follow.html)\n\nPauses a follower index. The follower index will not fetch any additional operations from the leader index."]
    pub fn pause_follow<'b>(
        &'a self,
        parts: CcrPauseFollowParts<'b>,
    ) -> CcrPauseFollow<'a, 'b, ()> {
        CcrPauseFollow::new(self.transport(), parts)
    }
    #[doc = "[Ccr Put Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-put-auto-follow-pattern.html)\n\nCreates a new named collection of auto-follow patterns against a specified remote cluster. Newly created indices on the remote cluster matching any of the specified patterns will be automatically configured as follower indices."]
    pub fn put_auto_follow_pattern<'b>(
        &'a self,
        parts: CcrPutAutoFollowPatternParts<'b>,
    ) -> CcrPutAutoFollowPattern<'a, 'b, ()> {
        CcrPutAutoFollowPattern::new(self.transport(), parts)
    }
    #[doc = "[Ccr Resume Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-resume-auto-follow-pattern.html)\n\nResumes an auto-follow pattern that has been paused"]
    pub fn resume_auto_follow_pattern<'b>(
        &'a self,
        parts: CcrResumeAutoFollowPatternParts<'b>,
    ) -> CcrResumeAutoFollowPattern<'a, 'b, ()> {
        CcrResumeAutoFollowPattern::new(self.transport(), parts)
    }
    #[doc = "[Ccr Resume Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-post-resume-follow.html)\n\nResumes a follower index that has been paused"]
    pub fn resume_follow<'b>(
        &'a self,
        parts: CcrResumeFollowParts<'b>,
    ) -> CcrResumeFollow<'a, 'b, ()> {
        CcrResumeFollow::new(self.transport(), parts)
    }
    #[doc = "[Ccr Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-get-stats.html)\n\nGets all stats related to cross-cluster replication."]
    pub fn stats<'b>(&'a self) -> CcrStats<'a, 'b> {
        CcrStats::new(self.transport())
    }
    #[doc = "[Ccr Unfollow API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/ccr-post-unfollow.html)\n\nStops the following task associated with a follower index and removes index metadata and settings associated with cross-cluster replication."]
    pub fn unfollow<'b>(&'a self, parts: CcrUnfollowParts<'b>) -> CcrUnfollow<'a, 'b, ()> {
        CcrUnfollow::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Cross Cluster Replication APIs"]
    pub fn ccr(&self) -> Ccr {
        Ccr::new(self.transport())
    }
}
