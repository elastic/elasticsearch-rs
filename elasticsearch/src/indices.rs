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

//! Index APIs
//!
//! [Manage individual indices](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices.html),
//! index settings, aliases, mappings, and index templates.

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
#[doc = "API parts for the Indices Add Block API"]
pub enum IndicesAddBlockParts<'b> {
    #[doc = "Index and Block"]
    IndexBlock(&'b [&'b str], &'b str),
}
impl<'b> IndicesAddBlockParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Add Block API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesAddBlockParts::IndexBlock(ref index, ref block) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_block: Cow<str> =
                    percent_encode(block.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(9usize + encoded_index.len() + encoded_block.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_block/");
                p.push_str(encoded_block.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Add Block API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/index-modules-blocks.html)\n\nAdds a block to an index."]
#[derive(Clone, Debug)]
pub struct IndicesAddBlock<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesAddBlockParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> IndicesAddBlock<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesAddBlock] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesAddBlockParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesAddBlock {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesAddBlock<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesAddBlock {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Creates an asynchronous call to the Indices Add Block API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Analyze API"]
pub enum IndicesAnalyzeParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IndicesAnalyzeParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Analyze API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesAnalyzeParts::None => "/_analyze".into(),
            IndicesAnalyzeParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_analyze");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Analyze API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-analyze.html)\n\nPerforms the analysis process on a text and return the tokens breakdown of the text."]
#[derive(Clone, Debug)]
pub struct IndicesAnalyze<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesAnalyzeParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    index: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesAnalyze<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesAnalyze] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesAnalyzeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesAnalyze {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            index: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesAnalyze<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesAnalyze {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            index: self.index,
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
    #[doc = "The name of the index to scope the operation"]
    pub fn index(mut self, index: &'b str) -> Self {
        self.index = Some(index);
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
    #[doc = "Creates an asynchronous call to the Indices Analyze API that can be awaited"]
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
                index: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                index: self.index,
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
#[doc = "API parts for the Indices Clear Cache API"]
pub enum IndicesClearCacheParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesClearCacheParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Clear Cache API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesClearCacheParts::None => "/_cache/clear".into(),
            IndicesClearCacheParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_cache/clear");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Clear Cache API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-clearcache.html)\n\nClears all or specific caches for one or more indices."]
#[derive(Clone, Debug)]
pub struct IndicesClearCache<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesClearCacheParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    fielddata: Option<bool>,
    fields: Option<&'b [&'b str]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<&'b [&'b str]>,
    pretty: Option<bool>,
    query: Option<bool>,
    request: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesClearCache<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesClearCache] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesClearCacheParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesClearCache {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            fielddata: None,
            fields: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            pretty: None,
            query: None,
            request: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesClearCache<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesClearCache {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            fielddata: self.fielddata,
            fields: self.fields,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            index: self.index,
            pretty: self.pretty,
            query: self.query,
            request: self.request,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "Clear field data"]
    pub fn fielddata(mut self, fielddata: bool) -> Self {
        self.fielddata = Some(fielddata);
        self
    }
    #[doc = "A comma-separated list of fields to clear when using the `fielddata` parameter (default: all)"]
    pub fn fields(mut self, fields: &'b [&'b str]) -> Self {
        self.fields = Some(fields);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "A comma-separated list of index name to limit the operation"]
    pub fn index(mut self, index: &'b [&'b str]) -> Self {
        self.index = Some(index);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Clear query caches"]
    pub fn query(mut self, query: bool) -> Self {
        self.query = Some(query);
        self
    }
    #[doc = "Clear request cache"]
    pub fn request(mut self, request: bool) -> Self {
        self.request = Some(request);
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
    #[doc = "Creates an asynchronous call to the Indices Clear Cache API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                fielddata: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                index: Option<&'b [&'b str]>,
                pretty: Option<bool>,
                query: Option<bool>,
                request: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                fielddata: self.fielddata,
                fields: self.fields,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                index: self.index,
                pretty: self.pretty,
                query: self.query,
                request: self.request,
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
#[doc = "API parts for the Indices Clone API"]
pub enum IndicesCloneParts<'b> {
    #[doc = "Index and Target"]
    IndexTarget(&'b str, &'b str),
}
impl<'b> IndicesCloneParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Clone API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesCloneParts::IndexTarget(ref index, ref target) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_target: Cow<str> =
                    percent_encode(target.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(9usize + encoded_index.len() + encoded_target.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_clone/");
                p.push_str(encoded_target.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Clone API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-clone-index.html)\n\nClones an index"]
#[derive(Clone, Debug)]
pub struct IndicesClone<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesCloneParts<'b>,
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
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesClone<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesClone] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesCloneParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesClone {
            transport,
            parts,
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
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesClone<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesClone {
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
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Set the number of active shards to wait for on the cloned index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Clone API that can be awaited"]
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
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Indices Close API"]
pub enum IndicesCloseParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesCloseParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Close API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesCloseParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(8usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_close");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Close API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-open-close.html)\n\nCloses an index."]
#[derive(Clone, Debug)]
pub struct IndicesClose<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesCloseParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesClose<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesClose] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesCloseParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesClose {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesClose<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesClose {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Sets the number of active shards to wait for before the operation returns. Set to `index-setting` to wait according to the index setting `index.write.wait_for_active_shards`, or `all` to wait for all shards, or an integer. Defaults to `0`."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Close API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Indices Create API"]
pub enum IndicesCreateParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IndicesCreateParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Create API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesCreateParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(1usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Create API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-create-index.html)\n\nCreates an index with optional settings and mappings."]
#[derive(Clone, Debug)]
pub struct IndicesCreate<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesCreateParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesCreate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesCreate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesCreateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesCreate {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            include_type_name: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesCreate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesCreate {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            include_type_name: self.include_type_name,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
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
    #[doc = "Whether a type should be expected in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Set the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Create API that can be awaited"]
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
                include_type_name: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                include_type_name: self.include_type_name,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Indices Create Data Stream API"]
pub enum IndicesCreateDataStreamParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> IndicesCreateDataStreamParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Create Data Stream API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesCreateDataStreamParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_name.len());
                p.push_str("/_data_stream/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Create Data Stream API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/data-streams.html)\n\nCreates a data stream"]
#[derive(Clone, Debug)]
pub struct IndicesCreateDataStream<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesCreateDataStreamParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesCreateDataStream<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesCreateDataStream] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesCreateDataStreamParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesCreateDataStream {
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
    pub fn body<T>(self, body: T) -> IndicesCreateDataStream<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesCreateDataStream {
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
    #[doc = "Creates an asynchronous call to the Indices Create Data Stream API that can be awaited"]
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
#[doc = "API parts for the Indices Data Streams Stats API"]
pub enum IndicesDataStreamsStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> IndicesDataStreamsStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Data Streams Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesDataStreamsStatsParts::None => "/_data_stream/_stats".into(),
            IndicesDataStreamsStatsParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_data_stream/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/_stats");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Data Streams Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/data-streams.html)\n\nProvides statistics on operations happening in a data stream."]
#[derive(Clone, Debug)]
pub struct IndicesDataStreamsStats<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesDataStreamsStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesDataStreamsStats<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesDataStreamsStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesDataStreamsStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesDataStreamsStats {
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
    #[doc = "Creates an asynchronous call to the Indices Data Streams Stats API that can be awaited"]
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
#[doc = "API parts for the Indices Delete API"]
pub enum IndicesDeleteParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesDeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesDeleteParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(1usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-delete-index.html)\n\nDeletes an index."]
#[derive(Clone, Debug)]
pub struct IndicesDelete<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesDeleteParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> IndicesDelete<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesDelete] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesDelete {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Ignore if a wildcard expression resolves to no concrete indices (default: false)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether wildcard expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Ignore unavailable indexes (default: false)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Creates an asynchronous call to the Indices Delete API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Indices Delete Alias API"]
pub enum IndicesDeleteAliasParts<'b> {
    #[doc = "Index and Name"]
    IndexName(&'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesDeleteAliasParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Delete Alias API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesDeleteAliasParts::IndexName(ref index, ref name) => {
                let index_str = index.join(",");
                let name_str = name.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(9usize + encoded_index.len() + encoded_name.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_alias/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Delete Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-aliases.html)\n\nDeletes an alias."]
#[derive(Clone, Debug)]
pub struct IndicesDeleteAlias<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesDeleteAliasParts<'b>,
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
impl<'a, 'b> IndicesDeleteAlias<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesDeleteAlias] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesDeleteAliasParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesDeleteAlias {
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
            timeout: None,
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
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Explicit timestamp for the document"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Delete Alias API that can be awaited"]
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
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Delete Data Stream API"]
pub enum IndicesDeleteDataStreamParts<'b> {
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> IndicesDeleteDataStreamParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Delete Data Stream API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesDeleteDataStreamParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_name.len());
                p.push_str("/_data_stream/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Delete Data Stream API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/data-streams.html)\n\nDeletes a data stream."]
#[derive(Clone, Debug)]
pub struct IndicesDeleteDataStream<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesDeleteDataStreamParts<'b>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesDeleteDataStream<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesDeleteDataStream] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesDeleteDataStreamParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesDeleteDataStream {
            transport,
            parts,
            headers,
            error_trace: None,
            expand_wildcards: None,
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
    #[doc = "Whether wildcard expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Creates an asynchronous call to the Indices Delete Data Stream API that can be awaited"]
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
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
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
#[doc = "API parts for the Indices Delete Index Template API"]
pub enum IndicesDeleteIndexTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> IndicesDeleteIndexTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Delete Index Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesDeleteIndexTemplateParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_name.len());
                p.push_str("/_index_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Delete Index Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nDeletes an index template."]
#[derive(Clone, Debug)]
pub struct IndicesDeleteIndexTemplate<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesDeleteIndexTemplateParts<'b>,
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
impl<'a, 'b> IndicesDeleteIndexTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesDeleteIndexTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesDeleteIndexTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesDeleteIndexTemplate {
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
            timeout: None,
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
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Creates an asynchronous call to the Indices Delete Index Template API that can be awaited"]
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
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Delete Template API"]
pub enum IndicesDeleteTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> IndicesDeleteTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Delete Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesDeleteTemplateParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_name.len());
                p.push_str("/_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Delete Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nDeletes an index template."]
#[derive(Clone, Debug)]
pub struct IndicesDeleteTemplate<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesDeleteTemplateParts<'b>,
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
impl<'a, 'b> IndicesDeleteTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesDeleteTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesDeleteTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesDeleteTemplate {
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
            timeout: None,
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
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Creates an asynchronous call to the Indices Delete Template API that can be awaited"]
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
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Exists API"]
pub enum IndicesExistsParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesExistsParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Exists API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesExistsParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(1usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Exists API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-exists.html)\n\nReturns information about whether a particular index exists."]
#[derive(Clone, Debug)]
pub struct IndicesExists<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesExistsParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesExists<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesExists] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesExistsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesExists {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            ignore_unavailable: None,
            include_defaults: None,
            local: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Ignore if a wildcard expression resolves to no concrete indices (default: false)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether wildcard expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "Ignore unavailable indexes (default: false)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether to return all default setting for each of the indices."]
    pub fn include_defaults(mut self, include_defaults: bool) -> Self {
        self.include_defaults = Some(include_defaults);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Exists API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                flat_settings: Option<bool>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                include_defaults: Option<bool>,
                local: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_defaults: self.include_defaults,
                local: self.local,
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
#[doc = "API parts for the Indices Exists Alias API"]
pub enum IndicesExistsAliasParts<'b> {
    #[doc = "Name"]
    Name(&'b [&'b str]),
    #[doc = "Index and Name"]
    IndexName(&'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesExistsAliasParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Exists Alias API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesExistsAliasParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(8usize + encoded_name.len());
                p.push_str("/_alias/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
            IndicesExistsAliasParts::IndexName(ref index, ref name) => {
                let index_str = index.join(",");
                let name_str = name.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(9usize + encoded_index.len() + encoded_name.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_alias/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Exists Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-aliases.html)\n\nReturns information about whether a particular alias exists."]
#[derive(Clone, Debug)]
pub struct IndicesExistsAlias<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesExistsAliasParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesExistsAlias<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesExistsAlias] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesExistsAliasParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesExistsAlias {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            local: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Exists Alias API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                local: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
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
#[doc = "API parts for the Indices Exists Index Template API"]
pub enum IndicesExistsIndexTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> IndicesExistsIndexTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Exists Index Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesExistsIndexTemplateParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_name.len());
                p.push_str("/_index_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Exists Index Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nReturns information about whether a particular index template exists."]
#[derive(Clone, Debug)]
pub struct IndicesExistsIndexTemplate<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesExistsIndexTemplateParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesExistsIndexTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesExistsIndexTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesExistsIndexTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesExistsIndexTemplate {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            local: None,
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
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Exists Index Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                flat_settings: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                local: self.local,
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Exists Template API"]
pub enum IndicesExistsTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> IndicesExistsTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Exists Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesExistsTemplateParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_name.len());
                p.push_str("/_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Exists Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nReturns information about whether a particular index template exists."]
#[derive(Clone, Debug)]
pub struct IndicesExistsTemplate<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesExistsTemplateParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesExistsTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesExistsTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesExistsTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesExistsTemplate {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            local: None,
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
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Exists Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                flat_settings: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                local: self.local,
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Exists Type API"]
pub enum IndicesExistsTypeParts<'b> {
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesExistsTypeParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Exists Type API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesExistsTypeParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_ty: Cow<str> = percent_encode(ty_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_index.len() + encoded_ty.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_mapping/");
                p.push_str(encoded_ty.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Exists Type API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-types-exists.html)\n\nReturns information about whether a particular document type exists. (DEPRECATED)"]
#[derive(Clone, Debug)]
pub struct IndicesExistsType<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesExistsTypeParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesExistsType<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesExistsType] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesExistsTypeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesExistsType {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            local: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Exists Type API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                local: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
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
#[doc = "API parts for the Indices Flush API"]
pub enum IndicesFlushParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesFlushParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Flush API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesFlushParts::None => "/_flush".into(),
            IndicesFlushParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(8usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_flush");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Flush API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-flush.html)\n\nPerforms the flush operation on one or more indices."]
#[derive(Clone, Debug)]
pub struct IndicesFlush<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesFlushParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    force: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_if_ongoing: Option<bool>,
}
impl<'a, 'b, B> IndicesFlush<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesFlush] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesFlushParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesFlush {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            force: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_if_ongoing: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesFlush<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesFlush {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            force: self.force,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            wait_if_ongoing: self.wait_if_ongoing,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Whether a flush should be forced even if it is not necessarily needed ie. if no changes will be committed to the index. This is useful if transaction log IDs should be incremented even if no uncommitted changes are present. (This setting can be considered as internal)"]
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "If set to true the flush operation will block until the flush can be executed if another flush operation is already executing. The default is true. If set to false the flush will be skipped iff if another flush operation is already running."]
    pub fn wait_if_ongoing(mut self, wait_if_ongoing: bool) -> Self {
        self.wait_if_ongoing = Some(wait_if_ongoing);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Flush API that can be awaited"]
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
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                force: Option<bool>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                wait_if_ongoing: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                force: self.force,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                pretty: self.pretty,
                source: self.source,
                wait_if_ongoing: self.wait_if_ongoing,
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
#[doc = "API parts for the Indices Flush Synced API"]
pub enum IndicesFlushSyncedParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesFlushSyncedParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Flush Synced API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesFlushSyncedParts::None => "/_flush/synced".into(),
            IndicesFlushSyncedParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_flush/synced");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Flush Synced API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-synced-flush-api.html)\n\nPerforms a synced flush operation on one or more indices. Synced flush is deprecated and will be removed in 8.0. Use flush instead"]
#[derive(Clone, Debug)]
pub struct IndicesFlushSynced<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesFlushSyncedParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesFlushSynced<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesFlushSynced] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesFlushSyncedParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesFlushSynced {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesFlushSynced<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesFlushSynced {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "Creates an asynchronous call to the Indices Flush Synced API that can be awaited"]
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
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
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
#[doc = "API parts for the Indices Forcemerge API"]
pub enum IndicesForcemergeParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesForcemergeParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Forcemerge API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesForcemergeParts::None => "/_forcemerge".into(),
            IndicesForcemergeParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_forcemerge");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Forcemerge API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-forcemerge.html)\n\nPerforms the force merge operation on one or more indices."]
#[derive(Clone, Debug)]
pub struct IndicesForcemerge<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesForcemergeParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    flush: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    max_num_segments: Option<i64>,
    only_expunge_deletes: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesForcemerge<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesForcemerge] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesForcemergeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesForcemerge {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flush: None,
            human: None,
            ignore_unavailable: None,
            max_num_segments: None,
            only_expunge_deletes: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesForcemerge<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesForcemerge {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            flush: self.flush,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            max_num_segments: self.max_num_segments,
            only_expunge_deletes: self.only_expunge_deletes,
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Specify whether the index should be flushed after performing the operation (default: true)"]
    pub fn flush(mut self, flush: bool) -> Self {
        self.flush = Some(flush);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "The number of segments the index should be merged into (default: dynamic)"]
    pub fn max_num_segments(mut self, max_num_segments: i64) -> Self {
        self.max_num_segments = Some(max_num_segments);
        self
    }
    #[doc = "Specify whether the operation should only expunge deleted documents"]
    pub fn only_expunge_deletes(mut self, only_expunge_deletes: bool) -> Self {
        self.only_expunge_deletes = Some(only_expunge_deletes);
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
    #[doc = "Creates an asynchronous call to the Indices Forcemerge API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                flush: Option<bool>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                max_num_segments: Option<i64>,
                only_expunge_deletes: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                flush: self.flush,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                max_num_segments: self.max_num_segments,
                only_expunge_deletes: self.only_expunge_deletes,
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
#[doc = "API parts for the Indices Freeze API"]
pub enum IndicesFreezeParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IndicesFreezeParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Freeze API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesFreezeParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(9usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_freeze");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Freeze API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/freeze-index-api.html)\n\nFreezes an index. A frozen index has almost no overhead on the cluster (except for maintaining its metadata in memory) and is read-only."]
#[derive(Clone, Debug)]
pub struct IndicesFreeze<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesFreezeParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesFreeze<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesFreeze] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesFreezeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesFreeze {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesFreeze<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesFreeze {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Freeze API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Indices Get API"]
pub enum IndicesGetParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesGetParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(1usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-get-index.html)\n\nReturns information about one or more indices."]
#[derive(Clone, Debug)]
pub struct IndicesGet<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesGetParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGet<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGet {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            ignore_unavailable: None,
            include_defaults: None,
            include_type_name: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Ignore if a wildcard expression resolves to no concrete indices (default: false)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether wildcard expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "Ignore unavailable indexes (default: false)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether to return all default setting for each of the indices."]
    pub fn include_defaults(mut self, include_defaults: bool) -> Self {
        self.include_defaults = Some(include_defaults);
        self
    }
    #[doc = "Whether to add the type name to the response (default: false)"]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Creates an asynchronous call to the Indices Get API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                flat_settings: Option<bool>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                include_defaults: Option<bool>,
                include_type_name: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_defaults: self.include_defaults,
                include_type_name: self.include_type_name,
                local: self.local,
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Get Alias API"]
pub enum IndicesGetAliasParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b [&'b str]),
    #[doc = "Index and Name"]
    IndexName(&'b [&'b str], &'b [&'b str]),
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesGetAliasParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Get Alias API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetAliasParts::None => "/_alias".into(),
            IndicesGetAliasParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(8usize + encoded_name.len());
                p.push_str("/_alias/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
            IndicesGetAliasParts::IndexName(ref index, ref name) => {
                let index_str = index.join(",");
                let name_str = name.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(9usize + encoded_index.len() + encoded_name.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_alias/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
            IndicesGetAliasParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(8usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_alias");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Get Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-aliases.html)\n\nReturns an alias."]
#[derive(Clone, Debug)]
pub struct IndicesGetAlias<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesGetAliasParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetAlias<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetAlias] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesGetAliasParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetAlias {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            local: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Get Alias API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                local: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
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
#[doc = "API parts for the Indices Get Data Stream API"]
pub enum IndicesGetDataStreamParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> IndicesGetDataStreamParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Get Data Stream API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetDataStreamParts::None => "/_data_stream".into(),
            IndicesGetDataStreamParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_name.len());
                p.push_str("/_data_stream/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Get Data Stream API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/data-streams.html)\n\nReturns data streams."]
#[derive(Clone, Debug)]
pub struct IndicesGetDataStream<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesGetDataStreamParts<'b>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetDataStream<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetDataStream] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesGetDataStreamParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetDataStream {
            transport,
            parts,
            headers,
            error_trace: None,
            expand_wildcards: None,
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
    #[doc = "Whether wildcard expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Creates an asynchronous call to the Indices Get Data Stream API that can be awaited"]
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
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
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
#[doc = "API parts for the Indices Get Field Mapping API"]
pub enum IndicesGetFieldMappingParts<'b> {
    #[doc = "Fields"]
    Fields(&'b [&'b str]),
    #[doc = "Index and Fields"]
    IndexFields(&'b [&'b str], &'b [&'b str]),
    #[doc = "Type and Fields"]
    TypeFields(&'b [&'b str], &'b [&'b str]),
    #[doc = "Index, Type and Fields"]
    IndexTypeFields(&'b [&'b str], &'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesGetFieldMappingParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Get Field Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetFieldMappingParts::Fields(ref fields) => {
                let fields_str = fields.join(",");
                let encoded_fields: Cow<str> =
                    percent_encode(fields_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_fields.len());
                p.push_str("/_mapping/field/");
                p.push_str(encoded_fields.as_ref());
                p.into()
            }
            IndicesGetFieldMappingParts::IndexFields(ref index, ref fields) => {
                let index_str = index.join(",");
                let fields_str = fields.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_fields: Cow<str> =
                    percent_encode(fields_str.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(17usize + encoded_index.len() + encoded_fields.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_mapping/field/");
                p.push_str(encoded_fields.as_ref());
                p.into()
            }
            IndicesGetFieldMappingParts::TypeFields(ref ty, ref fields) => {
                let ty_str = ty.join(",");
                let fields_str = fields.join(",");
                let encoded_ty: Cow<str> = percent_encode(ty_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_fields: Cow<str> =
                    percent_encode(fields_str.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(17usize + encoded_ty.len() + encoded_fields.len());
                p.push_str("/_mapping/");
                p.push_str(encoded_ty.as_ref());
                p.push_str("/field/");
                p.push_str(encoded_fields.as_ref());
                p.into()
            }
            IndicesGetFieldMappingParts::IndexTypeFields(ref index, ref ty, ref fields) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let fields_str = fields.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_ty: Cow<str> = percent_encode(ty_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_fields: Cow<str> =
                    percent_encode(fields_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    18usize + encoded_index.len() + encoded_ty.len() + encoded_fields.len(),
                );
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_mapping/");
                p.push_str(encoded_ty.as_ref());
                p.push_str("/field/");
                p.push_str(encoded_fields.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Get Field Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-get-field-mapping.html)\n\nReturns mapping for one or more fields."]
#[derive(Clone, Debug)]
pub struct IndicesGetFieldMapping<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesGetFieldMappingParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetFieldMapping<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetFieldMapping] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesGetFieldMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetFieldMapping {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            include_defaults: None,
            include_type_name: None,
            local: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether the default mapping values should be returned as well"]
    pub fn include_defaults(mut self, include_defaults: bool) -> Self {
        self.include_defaults = Some(include_defaults);
        self
    }
    #[doc = "Whether a type should be returned in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Get Field Mapping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                include_defaults: Option<bool>,
                include_type_name: Option<bool>,
                local: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_defaults: self.include_defaults,
                include_type_name: self.include_type_name,
                local: self.local,
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
#[doc = "API parts for the Indices Get Index Template API"]
pub enum IndicesGetIndexTemplateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> IndicesGetIndexTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Get Index Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetIndexTemplateParts::None => "/_index_template".into(),
            IndicesGetIndexTemplateParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_name.len());
                p.push_str("/_index_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Get Index Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nReturns an index template."]
#[derive(Clone, Debug)]
pub struct IndicesGetIndexTemplate<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesGetIndexTemplateParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetIndexTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetIndexTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesGetIndexTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetIndexTemplate {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            local: None,
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
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Get Index Template API that can be awaited"]
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
                flat_settings: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                local: self.local,
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Get Mapping API"]
pub enum IndicesGetMappingParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Type"]
    Type(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesGetMappingParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Get Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetMappingParts::None => "/_mapping".into(),
            IndicesGetMappingParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_mapping");
                p.into()
            }
            IndicesGetMappingParts::Type(ref ty) => {
                let ty_str = ty.join(",");
                let encoded_ty: Cow<str> = percent_encode(ty_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_ty.len());
                p.push_str("/_mapping/");
                p.push_str(encoded_ty.as_ref());
                p.into()
            }
            IndicesGetMappingParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_ty: Cow<str> = percent_encode(ty_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_index.len() + encoded_ty.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_mapping/");
                p.push_str(encoded_ty.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Get Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-get-mapping.html)\n\nReturns mappings for one or more indices."]
#[derive(Clone, Debug)]
pub struct IndicesGetMapping<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesGetMappingParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetMapping<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetMapping] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesGetMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetMapping {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            include_type_name: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether to add the type name to the response (default: false)"]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Creates an asynchronous call to the Indices Get Mapping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                include_type_name: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_type_name: self.include_type_name,
                local: self.local,
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Get Settings API"]
pub enum IndicesGetSettingsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Name"]
    IndexName(&'b [&'b str], &'b [&'b str]),
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> IndicesGetSettingsParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Get Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetSettingsParts::None => "/_settings".into(),
            IndicesGetSettingsParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_settings");
                p.into()
            }
            IndicesGetSettingsParts::IndexName(ref index, ref name) => {
                let index_str = index.join(",");
                let name_str = name.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(12usize + encoded_index.len() + encoded_name.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_settings/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
            IndicesGetSettingsParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_name.len());
                p.push_str("/_settings/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Get Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-get-settings.html)\n\nReturns settings for one or more indices."]
#[derive(Clone, Debug)]
pub struct IndicesGetSettings<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesGetSettingsParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetSettings<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetSettings] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesGetSettingsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetSettings {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            ignore_unavailable: None,
            include_defaults: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether to return all default setting for each of the indices."]
    pub fn include_defaults(mut self, include_defaults: bool) -> Self {
        self.include_defaults = Some(include_defaults);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Creates an asynchronous call to the Indices Get Settings API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                flat_settings: Option<bool>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                include_defaults: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_defaults: self.include_defaults,
                local: self.local,
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Get Template API"]
pub enum IndicesGetTemplateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> IndicesGetTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Get Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetTemplateParts::None => "/_template".into(),
            IndicesGetTemplateParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_name.len());
                p.push_str("/_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Get Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nReturns an index template."]
#[derive(Clone, Debug)]
pub struct IndicesGetTemplate<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesGetTemplateParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesGetTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetTemplate {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            include_type_name: None,
            local: None,
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
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "Whether a type should be returned in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Get Template API that can be awaited"]
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
                flat_settings: Option<bool>,
                human: Option<bool>,
                include_type_name: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                include_type_name: self.include_type_name,
                local: self.local,
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Get Upgrade API"]
pub enum IndicesGetUpgradeParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesGetUpgradeParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Get Upgrade API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetUpgradeParts::None => "/_upgrade".into(),
            IndicesGetUpgradeParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_upgrade");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Get Upgrade API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-upgrade.html)\n\nDEPRECATED Returns a progress status of current upgrade."]
#[derive(Clone, Debug)]
pub struct IndicesGetUpgrade<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesGetUpgradeParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetUpgrade<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetUpgrade] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesGetUpgradeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetUpgrade {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "Creates an asynchronous call to the Indices Get Upgrade API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
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
#[doc = "API parts for the Indices Migrate To Data Stream API"]
pub enum IndicesMigrateToDataStreamParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> IndicesMigrateToDataStreamParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Migrate To Data Stream API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesMigrateToDataStreamParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_name.len());
                p.push_str("/_data_stream/_migrate/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Migrate To Data Stream API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/data-streams.html)\n\nMigrates an alias to a data stream"]
#[derive(Clone, Debug)]
pub struct IndicesMigrateToDataStream<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesMigrateToDataStreamParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesMigrateToDataStream<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesMigrateToDataStream] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesMigrateToDataStreamParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesMigrateToDataStream {
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
    pub fn body<T>(self, body: T) -> IndicesMigrateToDataStream<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesMigrateToDataStream {
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
    #[doc = "Creates an asynchronous call to the Indices Migrate To Data Stream API that can be awaited"]
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
#[doc = "API parts for the Indices Open API"]
pub enum IndicesOpenParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesOpenParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Open API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesOpenParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(7usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_open");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Open API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-open-close.html)\n\nOpens an index."]
#[derive(Clone, Debug)]
pub struct IndicesOpen<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesOpenParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesOpen<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesOpen] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesOpenParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesOpen {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesOpen<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesOpen {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Open API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Indices Promote Data Stream API"]
pub enum IndicesPromoteDataStreamParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> IndicesPromoteDataStreamParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Promote Data Stream API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesPromoteDataStreamParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_name.len());
                p.push_str("/_data_stream/_promote/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Promote Data Stream API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/data-streams.html)\n\nPromotes a data stream from a replicated data stream managed by CCR to a regular data stream"]
#[derive(Clone, Debug)]
pub struct IndicesPromoteDataStream<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesPromoteDataStreamParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesPromoteDataStream<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesPromoteDataStream] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesPromoteDataStreamParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesPromoteDataStream {
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
    pub fn body<T>(self, body: T) -> IndicesPromoteDataStream<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesPromoteDataStream {
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
    #[doc = "Creates an asynchronous call to the Indices Promote Data Stream API that can be awaited"]
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
#[doc = "API parts for the Indices Put Alias API"]
pub enum IndicesPutAliasParts<'b> {
    #[doc = "Index and Name"]
    IndexName(&'b [&'b str], &'b str),
}
impl<'b> IndicesPutAliasParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Put Alias API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesPutAliasParts::IndexName(ref index, ref name) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(9usize + encoded_index.len() + encoded_name.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_alias/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Put Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-aliases.html)\n\nCreates or updates an alias."]
#[derive(Clone, Debug)]
pub struct IndicesPutAlias<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesPutAliasParts<'b>,
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
impl<'a, 'b, B> IndicesPutAlias<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesPutAlias] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesPutAliasParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesPutAlias {
            transport,
            parts,
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
    pub fn body<T>(self, body: T) -> IndicesPutAlias<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesPutAlias {
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
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Explicit timestamp for the document"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Put Alias API that can be awaited"]
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Put Index Template API"]
pub enum IndicesPutIndexTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> IndicesPutIndexTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Put Index Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesPutIndexTemplateParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_name.len());
                p.push_str("/_index_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Put Index Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nCreates or updates an index template."]
#[derive(Clone, Debug)]
pub struct IndicesPutIndexTemplate<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesPutIndexTemplateParts<'b>,
    body: Option<B>,
    cause: Option<&'b str>,
    create: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesPutIndexTemplate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesPutIndexTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesPutIndexTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesPutIndexTemplate {
            transport,
            parts,
            headers,
            body: None,
            cause: None,
            create: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesPutIndexTemplate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesPutIndexTemplate {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            cause: self.cause,
            create: self.create,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "User defined reason for creating/updating the index template"]
    pub fn cause(mut self, cause: &'b str) -> Self {
        self.cause = Some(cause);
        self
    }
    #[doc = "Whether the index template should only be added if new or can also replace an existing one"]
    pub fn create(mut self, create: bool) -> Self {
        self.create = Some(create);
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
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Creates an asynchronous call to the Indices Put Index Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cause: Option<&'b str>,
                create: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                cause: self.cause,
                create: self.create,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
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
#[doc = "API parts for the Indices Put Mapping API"]
pub enum IndicesPutMappingParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b str),
    #[doc = "Type"]
    Type(&'b str),
}
impl<'b> IndicesPutMappingParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Put Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesPutMappingParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_mapping");
                p.into()
            }
            IndicesPutMappingParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_ty: Cow<str> = percent_encode(ty.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_index.len() + encoded_ty.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/");
                p.push_str(encoded_ty.as_ref());
                p.push_str("/_mapping");
                p.into()
            }
            IndicesPutMappingParts::Type(ref ty) => {
                let encoded_ty: Cow<str> = percent_encode(ty.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_ty.len());
                p.push_str("/_mappings/");
                p.push_str(encoded_ty.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-put-mapping.html)\n\nUpdates the index mappings."]
#[derive(Clone, Debug)]
pub struct IndicesPutMapping<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesPutMappingParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    write_index_only: Option<bool>,
}
impl<'a, 'b, B> IndicesPutMapping<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesPutMapping] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesPutMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesPutMapping {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            include_type_name: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            write_index_only: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesPutMapping<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesPutMapping {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            include_type_name: self.include_type_name,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
            write_index_only: self.write_index_only,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether a type should be expected in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "When true, applies mappings only to the write index of an alias or data stream"]
    pub fn write_index_only(mut self, write_index_only: bool) -> Self {
        self.write_index_only = Some(write_index_only);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Put Mapping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                include_type_name: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                write_index_only: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_type_name: self.include_type_name,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                write_index_only: self.write_index_only,
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
#[doc = "API parts for the Indices Put Settings API"]
pub enum IndicesPutSettingsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesPutSettingsParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Put Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesPutSettingsParts::None => "/_settings".into(),
            IndicesPutSettingsParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_settings");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Put Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-update-settings.html)\n\nUpdates the index settings."]
#[derive(Clone, Debug)]
pub struct IndicesPutSettings<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesPutSettingsParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    preserve_existing: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> IndicesPutSettings<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesPutSettings] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesPutSettingsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesPutSettings {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            preserve_existing: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesPutSettings<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesPutSettings {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            flat_settings: self.flat_settings,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            preserve_existing: self.preserve_existing,
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to update existing settings. If set to `true` existing settings on an index remain unchanged, the default is `false`"]
    pub fn preserve_existing(mut self, preserve_existing: bool) -> Self {
        self.preserve_existing = Some(preserve_existing);
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
    #[doc = "Creates an asynchronous call to the Indices Put Settings API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                flat_settings: Option<bool>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                master_timeout: Option<&'b str>,
                preserve_existing: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                preserve_existing: self.preserve_existing,
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Put Template API"]
pub enum IndicesPutTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> IndicesPutTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Put Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesPutTemplateParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_name.len());
                p.push_str("/_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Put Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nCreates or updates an index template."]
#[derive(Clone, Debug)]
pub struct IndicesPutTemplate<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesPutTemplateParts<'b>,
    body: Option<B>,
    create: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<&'b str>,
    order: Option<i64>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesPutTemplate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesPutTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesPutTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesPutTemplate {
            transport,
            parts,
            headers,
            body: None,
            create: None,
            error_trace: None,
            filter_path: None,
            human: None,
            include_type_name: None,
            master_timeout: None,
            order: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesPutTemplate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesPutTemplate {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            create: self.create,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            include_type_name: self.include_type_name,
            master_timeout: self.master_timeout,
            order: self.order,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "Whether the index template should only be added if new or can also replace an existing one"]
    pub fn create(mut self, create: bool) -> Self {
        self.create = Some(create);
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
    #[doc = "Whether a type should be returned in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "The order for this template when merging multiple matching ones (higher numbers are merged later, overriding the lower numbers)"]
    pub fn order(mut self, order: i64) -> Self {
        self.order = Some(order);
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
    #[doc = "Creates an asynchronous call to the Indices Put Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                create: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                include_type_name: Option<bool>,
                master_timeout: Option<&'b str>,
                order: Option<i64>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                create: self.create,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                include_type_name: self.include_type_name,
                master_timeout: self.master_timeout,
                order: self.order,
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
#[doc = "API parts for the Indices Recovery API"]
pub enum IndicesRecoveryParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesRecoveryParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Recovery API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesRecoveryParts::None => "/_recovery".into(),
            IndicesRecoveryParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_recovery");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Recovery API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-recovery.html)\n\nReturns information about ongoing index shard recoveries."]
#[derive(Clone, Debug)]
pub struct IndicesRecovery<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesRecoveryParts<'b>,
    active_only: Option<bool>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesRecovery<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesRecovery] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesRecoveryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesRecovery {
            transport,
            parts,
            headers,
            active_only: None,
            detailed: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Display only those recoveries that are currently on-going"]
    pub fn active_only(mut self, active_only: bool) -> Self {
        self.active_only = Some(active_only);
        self
    }
    #[doc = "Whether to display detailed information about shard recovery"]
    pub fn detailed(mut self, detailed: bool) -> Self {
        self.detailed = Some(detailed);
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
    #[doc = "Creates an asynchronous call to the Indices Recovery API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                active_only: Option<bool>,
                detailed: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                active_only: self.active_only,
                detailed: self.detailed,
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
#[doc = "API parts for the Indices Refresh API"]
pub enum IndicesRefreshParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesRefreshParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Refresh API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesRefreshParts::None => "/_refresh".into(),
            IndicesRefreshParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_refresh");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Refresh API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-refresh.html)\n\nPerforms the refresh operation in one or more indices."]
#[derive(Clone, Debug)]
pub struct IndicesRefresh<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesRefreshParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesRefresh<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesRefresh] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesRefreshParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesRefresh {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesRefresh<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesRefresh {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "Creates an asynchronous call to the Indices Refresh API that can be awaited"]
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
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
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
#[doc = "API parts for the Indices Reload Search Analyzers API"]
pub enum IndicesReloadSearchAnalyzersParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesReloadSearchAnalyzersParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Reload Search Analyzers API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesReloadSearchAnalyzersParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(26usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_reload_search_analyzers");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Reload Search Analyzers API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-reload-analyzers.html)\n\nReloads an index's search analyzers and their resources."]
#[derive(Clone, Debug)]
pub struct IndicesReloadSearchAnalyzers<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesReloadSearchAnalyzersParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesReloadSearchAnalyzers<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesReloadSearchAnalyzers] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesReloadSearchAnalyzersParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesReloadSearchAnalyzers {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesReloadSearchAnalyzers<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesReloadSearchAnalyzers {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "Creates an asynchronous call to the Indices Reload Search Analyzers API that can be awaited"]
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
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Resolve Index API"]
pub enum IndicesResolveIndexParts<'b> {
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
#[cfg(feature = "experimental-apis")]
impl<'b> IndicesResolveIndexParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Resolve Index API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesResolveIndexParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_name.len());
                p.push_str("/_resolve/index/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Resolve Index API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-resolve-index-api.html)\n\nReturns information about any matching indices, aliases, and data streams"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct IndicesResolveIndex<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesResolveIndexParts<'b>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> IndicesResolveIndex<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesResolveIndex] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesResolveIndexParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesResolveIndex {
            transport,
            parts,
            headers,
            error_trace: None,
            expand_wildcards: None,
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
    #[doc = "Whether wildcard expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Creates an asynchronous call to the Indices Resolve Index API that can be awaited"]
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
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
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
#[doc = "API parts for the Indices Rollover API"]
pub enum IndicesRolloverParts<'b> {
    #[doc = "Alias"]
    Alias(&'b str),
    #[doc = "Alias and NewIndex"]
    AliasNewIndex(&'b str, &'b str),
}
impl<'b> IndicesRolloverParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Rollover API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesRolloverParts::Alias(ref alias) => {
                let encoded_alias: Cow<str> =
                    percent_encode(alias.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_alias.len());
                p.push_str("/");
                p.push_str(encoded_alias.as_ref());
                p.push_str("/_rollover");
                p.into()
            }
            IndicesRolloverParts::AliasNewIndex(ref alias, ref new_index) => {
                let encoded_alias: Cow<str> =
                    percent_encode(alias.as_bytes(), PARTS_ENCODED).into();
                let encoded_new_index: Cow<str> =
                    percent_encode(new_index.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(12usize + encoded_alias.len() + encoded_new_index.len());
                p.push_str("/");
                p.push_str(encoded_alias.as_ref());
                p.push_str("/_rollover/");
                p.push_str(encoded_new_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Rollover API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-rollover-index.html)\n\nUpdates an alias to point to a new index when the existing index\nis considered to be too large or too old."]
#[derive(Clone, Debug)]
pub struct IndicesRollover<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesRolloverParts<'b>,
    body: Option<B>,
    dry_run: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesRollover<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesRollover] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesRolloverParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesRollover {
            transport,
            parts,
            headers,
            body: None,
            dry_run: None,
            error_trace: None,
            filter_path: None,
            human: None,
            include_type_name: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesRollover<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesRollover {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            dry_run: self.dry_run,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            include_type_name: self.include_type_name,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "If set to true the rollover action will only be validated but not actually performed even if a condition matches. The default is false"]
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = Some(dry_run);
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
    #[doc = "Whether a type should be included in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Set the number of active shards to wait for on the newly created rollover index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Rollover API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                dry_run: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                include_type_name: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                dry_run: self.dry_run,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                include_type_name: self.include_type_name,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Indices Segments API"]
pub enum IndicesSegmentsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesSegmentsParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Segments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesSegmentsParts::None => "/_segments".into(),
            IndicesSegmentsParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_segments");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Segments API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-segments.html)\n\nProvides low-level information about segments in a Lucene index."]
#[derive(Clone, Debug)]
pub struct IndicesSegments<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesSegmentsParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    verbose: Option<bool>,
}
impl<'a, 'b> IndicesSegments<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesSegments] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesSegmentsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesSegments {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            request_timeout: None,
            source: None,
            verbose: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "Includes detailed memory usage by Lucene."]
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = Some(verbose);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Segments API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                verbose: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                pretty: self.pretty,
                source: self.source,
                verbose: self.verbose,
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
#[doc = "API parts for the Indices Shard Stores API"]
pub enum IndicesShardStoresParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesShardStoresParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Shard Stores API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesShardStoresParts::None => "/_shard_stores".into(),
            IndicesShardStoresParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_shard_stores");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Shard Stores API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-shards-stores.html)\n\nProvides store information for shard copies of indices."]
#[derive(Clone, Debug)]
pub struct IndicesShardStores<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesShardStoresParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    status: Option<&'b [&'b str]>,
}
impl<'a, 'b> IndicesShardStores<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesShardStores] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesShardStoresParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesShardStores {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            request_timeout: None,
            source: None,
            status: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "A comma-separated list of statuses used to filter on shards to get store information for"]
    pub fn status(mut self, status: &'b [&'b str]) -> Self {
        self.status = Some(status);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Shard Stores API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                status: Option<&'b [&'b str]>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                pretty: self.pretty,
                source: self.source,
                status: self.status,
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
#[doc = "API parts for the Indices Shrink API"]
pub enum IndicesShrinkParts<'b> {
    #[doc = "Index and Target"]
    IndexTarget(&'b str, &'b str),
}
impl<'b> IndicesShrinkParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Shrink API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesShrinkParts::IndexTarget(ref index, ref target) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_target: Cow<str> =
                    percent_encode(target.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(10usize + encoded_index.len() + encoded_target.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_shrink/");
                p.push_str(encoded_target.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Shrink API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-shrink-index.html)\n\nAllow to shrink an existing index into a new index with fewer primary shards."]
#[derive(Clone, Debug)]
pub struct IndicesShrink<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesShrinkParts<'b>,
    body: Option<B>,
    copy_settings: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesShrink<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesShrink] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesShrinkParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesShrink {
            transport,
            parts,
            headers,
            body: None,
            copy_settings: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesShrink<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesShrink {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            copy_settings: self.copy_settings,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "whether or not to copy settings from the source index (defaults to false)"]
    pub fn copy_settings(mut self, copy_settings: bool) -> Self {
        self.copy_settings = Some(copy_settings);
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
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Set the number of active shards to wait for on the shrunken index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Shrink API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                copy_settings: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                copy_settings: self.copy_settings,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Indices Simulate Index Template API"]
pub enum IndicesSimulateIndexTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> IndicesSimulateIndexTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Simulate Index Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesSimulateIndexTemplateParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_name.len());
                p.push_str("/_index_template/_simulate_index/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Simulate Index Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nSimulate matching the given index name against the index templates in the system"]
#[derive(Clone, Debug)]
pub struct IndicesSimulateIndexTemplate<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesSimulateIndexTemplateParts<'b>,
    body: Option<B>,
    cause: Option<&'b str>,
    create: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesSimulateIndexTemplate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesSimulateIndexTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesSimulateIndexTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesSimulateIndexTemplate {
            transport,
            parts,
            headers,
            body: None,
            cause: None,
            create: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesSimulateIndexTemplate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesSimulateIndexTemplate {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            cause: self.cause,
            create: self.create,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "User defined reason for dry-run creating the new template for simulation purposes"]
    pub fn cause(mut self, cause: &'b str) -> Self {
        self.cause = Some(cause);
        self
    }
    #[doc = "Whether the index template we optionally defined in the body should only be dry-run added if new or can also replace an existing one"]
    pub fn create(mut self, create: bool) -> Self {
        self.create = Some(create);
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
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Creates an asynchronous call to the Indices Simulate Index Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cause: Option<&'b str>,
                create: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                cause: self.cause,
                create: self.create,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
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
#[doc = "API parts for the Indices Simulate Template API"]
pub enum IndicesSimulateTemplateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> IndicesSimulateTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Simulate Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesSimulateTemplateParts::None => "/_index_template/_simulate".into(),
            IndicesSimulateTemplateParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_name.len());
                p.push_str("/_index_template/_simulate/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Simulate Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nSimulate resolving the given template name or body"]
#[derive(Clone, Debug)]
pub struct IndicesSimulateTemplate<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesSimulateTemplateParts<'b>,
    body: Option<B>,
    cause: Option<&'b str>,
    create: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesSimulateTemplate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesSimulateTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesSimulateTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesSimulateTemplate {
            transport,
            parts,
            headers,
            body: None,
            cause: None,
            create: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesSimulateTemplate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesSimulateTemplate {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            cause: self.cause,
            create: self.create,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "User defined reason for dry-run creating the new template for simulation purposes"]
    pub fn cause(mut self, cause: &'b str) -> Self {
        self.cause = Some(cause);
        self
    }
    #[doc = "Whether the index template we optionally defined in the body should only be dry-run added if new or can also replace an existing one"]
    pub fn create(mut self, create: bool) -> Self {
        self.create = Some(create);
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
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Creates an asynchronous call to the Indices Simulate Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cause: Option<&'b str>,
                create: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                cause: self.cause,
                create: self.create,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
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
#[doc = "API parts for the Indices Split API"]
pub enum IndicesSplitParts<'b> {
    #[doc = "Index and Target"]
    IndexTarget(&'b str, &'b str),
}
impl<'b> IndicesSplitParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Split API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesSplitParts::IndexTarget(ref index, ref target) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_target: Cow<str> =
                    percent_encode(target.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(9usize + encoded_index.len() + encoded_target.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_split/");
                p.push_str(encoded_target.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Split API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-split-index.html)\n\nAllows you to split an existing index into a new index with more primary shards."]
#[derive(Clone, Debug)]
pub struct IndicesSplit<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesSplitParts<'b>,
    body: Option<B>,
    copy_settings: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesSplit<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesSplit] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesSplitParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesSplit {
            transport,
            parts,
            headers,
            body: None,
            copy_settings: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesSplit<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesSplit {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            copy_settings: self.copy_settings,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "whether or not to copy settings from the source index (defaults to false)"]
    pub fn copy_settings(mut self, copy_settings: bool) -> Self {
        self.copy_settings = Some(copy_settings);
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
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Set the number of active shards to wait for on the shrunken index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Split API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                copy_settings: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                copy_settings: self.copy_settings,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Indices Stats API"]
pub enum IndicesStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Metric"]
    Metric(&'b [&'b str]),
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Metric"]
    IndexMetric(&'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesStatsParts::None => "/_stats".into(),
            IndicesStatsParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(8usize + encoded_metric.len());
                p.push_str("/_stats/");
                p.push_str(encoded_metric.as_ref());
                p.into()
            }
            IndicesStatsParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(8usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_stats");
                p.into()
            }
            IndicesStatsParts::IndexMetric(ref index, ref metric) => {
                let index_str = index.join(",");
                let metric_str = metric.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(9usize + encoded_index.len() + encoded_metric.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_stats/");
                p.push_str(encoded_metric.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-stats.html)\n\nProvides statistics on operations happening in an index."]
#[derive(Clone, Debug)]
pub struct IndicesStats<'a, 'b> {
    transport: &'a Transport,
    parts: IndicesStatsParts<'b>,
    completion_fields: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    fielddata_fields: Option<&'b [&'b str]>,
    fields: Option<&'b [&'b str]>,
    filter_path: Option<&'b [&'b str]>,
    forbid_closed_indices: Option<bool>,
    groups: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    include_segment_file_sizes: Option<bool>,
    include_unloaded_segments: Option<bool>,
    level: Option<Level>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    types: Option<&'b [&'b str]>,
}
impl<'a, 'b> IndicesStats<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesStats {
            transport,
            parts,
            headers,
            completion_fields: None,
            error_trace: None,
            expand_wildcards: None,
            fielddata_fields: None,
            fields: None,
            filter_path: None,
            forbid_closed_indices: None,
            groups: None,
            human: None,
            include_segment_file_sizes: None,
            include_unloaded_segments: None,
            level: None,
            pretty: None,
            request_timeout: None,
            source: None,
            types: None,
        }
    }
    #[doc = "A comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards)"]
    pub fn completion_fields(mut self, completion_fields: &'b [&'b str]) -> Self {
        self.completion_fields = Some(completion_fields);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of fields for `fielddata` index metric (supports wildcards)"]
    pub fn fielddata_fields(mut self, fielddata_fields: &'b [&'b str]) -> Self {
        self.fielddata_fields = Some(fielddata_fields);
        self
    }
    #[doc = "A comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards)"]
    pub fn fields(mut self, fields: &'b [&'b str]) -> Self {
        self.fields = Some(fields);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "If set to false stats will also collected from closed indices if explicitly specified or if expand_wildcards expands to closed indices"]
    pub fn forbid_closed_indices(mut self, forbid_closed_indices: bool) -> Self {
        self.forbid_closed_indices = Some(forbid_closed_indices);
        self
    }
    #[doc = "A comma-separated list of search groups for `search` index metric"]
    pub fn groups(mut self, groups: &'b [&'b str]) -> Self {
        self.groups = Some(groups);
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
    #[doc = "Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested)"]
    pub fn include_segment_file_sizes(mut self, include_segment_file_sizes: bool) -> Self {
        self.include_segment_file_sizes = Some(include_segment_file_sizes);
        self
    }
    #[doc = "If set to true segment stats will include stats for segments that are not currently loaded into memory"]
    pub fn include_unloaded_segments(mut self, include_unloaded_segments: bool) -> Self {
        self.include_unloaded_segments = Some(include_unloaded_segments);
        self
    }
    #[doc = "Return stats aggregated at cluster, index or shard level"]
    pub fn level(mut self, level: Level) -> Self {
        self.level = Some(level);
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
    #[doc = "A comma-separated list of document types for the `indexing` index metric"]
    pub fn types(mut self, types: &'b [&'b str]) -> Self {
        self.types = Some(types);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                completion_fields: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                fielddata_fields: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                forbid_closed_indices: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                groups: Option<&'b [&'b str]>,
                human: Option<bool>,
                include_segment_file_sizes: Option<bool>,
                include_unloaded_segments: Option<bool>,
                level: Option<Level>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                types: Option<&'b [&'b str]>,
            }
            let query_params = QueryParams {
                completion_fields: self.completion_fields,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                fielddata_fields: self.fielddata_fields,
                fields: self.fields,
                filter_path: self.filter_path,
                forbid_closed_indices: self.forbid_closed_indices,
                groups: self.groups,
                human: self.human,
                include_segment_file_sizes: self.include_segment_file_sizes,
                include_unloaded_segments: self.include_unloaded_segments,
                level: self.level,
                pretty: self.pretty,
                source: self.source,
                types: self.types,
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
#[doc = "API parts for the Indices Unfreeze API"]
pub enum IndicesUnfreezeParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IndicesUnfreezeParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Unfreeze API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesUnfreezeParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_unfreeze");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Unfreeze API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/unfreeze-index-api.html)\n\nUnfreezes an index. When a frozen index is unfrozen, the index goes through the normal recovery process and becomes writeable again."]
#[derive(Clone, Debug)]
pub struct IndicesUnfreeze<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesUnfreezeParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesUnfreeze<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesUnfreeze] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesUnfreezeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesUnfreeze {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesUnfreeze<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesUnfreeze {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Unfreeze API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Indices Update Aliases API"]
pub enum IndicesUpdateAliasesParts {
    #[doc = "No parts"]
    None,
}
impl IndicesUpdateAliasesParts {
    #[doc = "Builds a relative URL path to the Indices Update Aliases API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesUpdateAliasesParts::None => "/_aliases".into(),
        }
    }
}
#[doc = "Builder for the [Indices Update Aliases API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-aliases.html)\n\nUpdates index aliases."]
#[derive(Clone, Debug)]
pub struct IndicesUpdateAliases<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesUpdateAliasesParts,
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
impl<'a, 'b, B> IndicesUpdateAliases<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesUpdateAliases]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        IndicesUpdateAliases {
            transport,
            parts: IndicesUpdateAliasesParts::None,
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
    pub fn body<T>(self, body: T) -> IndicesUpdateAliases<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesUpdateAliases {
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
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Request timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Update Aliases API that can be awaited"]
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Upgrade API"]
pub enum IndicesUpgradeParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesUpgradeParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Upgrade API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesUpgradeParts::None => "/_upgrade".into(),
            IndicesUpgradeParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_upgrade");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Upgrade API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-upgrade.html)\n\nDEPRECATED Upgrades to the current version of Lucene."]
#[derive(Clone, Debug)]
pub struct IndicesUpgrade<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesUpgradeParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    only_ancient_segments: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> IndicesUpgrade<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesUpgrade] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesUpgradeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesUpgrade {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            only_ancient_segments: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_for_completion: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesUpgrade<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesUpgrade {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            only_ancient_segments: self.only_ancient_segments,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            wait_for_completion: self.wait_for_completion,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "If true, only ancient (an older Lucene major release) segments will be upgraded"]
    pub fn only_ancient_segments(mut self, only_ancient_segments: bool) -> Self {
        self.only_ancient_segments = Some(only_ancient_segments);
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
    #[doc = "Specify whether the request should block until the all segments are upgraded (default: false)"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Upgrade API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                only_ancient_segments: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                only_ancient_segments: self.only_ancient_segments,
                pretty: self.pretty,
                source: self.source,
                wait_for_completion: self.wait_for_completion,
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
#[doc = "API parts for the Indices Validate Query API"]
pub enum IndicesValidateQueryParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesValidateQueryParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Validate Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesValidateQueryParts::None => "/_validate/query".into(),
            IndicesValidateQueryParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_validate/query");
                p.into()
            }
            IndicesValidateQueryParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_ty: Cow<str> = percent_encode(ty_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_index.len() + encoded_ty.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/");
                p.push_str(encoded_ty.as_ref());
                p.push_str("/_validate/query");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Indices Validate Query API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/search-validate.html)\n\nAllows a user to validate a potentially expensive query without executing it."]
#[derive(Clone, Debug)]
pub struct IndicesValidateQuery<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndicesValidateQueryParts<'b>,
    all_shards: Option<bool>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    body: Option<B>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    explain: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    request_timeout: Option<Duration>,
    rewrite: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesValidateQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesValidateQuery] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndicesValidateQueryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesValidateQuery {
            transport,
            parts,
            headers,
            all_shards: None,
            allow_no_indices: None,
            analyze_wildcard: None,
            analyzer: None,
            body: None,
            default_operator: None,
            df: None,
            error_trace: None,
            expand_wildcards: None,
            explain: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            lenient: None,
            pretty: None,
            q: None,
            request_timeout: None,
            rewrite: None,
            source: None,
        }
    }
    #[doc = "Execute validation on all shards instead of one random shard per index"]
    pub fn all_shards(mut self, all_shards: bool) -> Self {
        self.all_shards = Some(all_shards);
        self
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "The analyzer to use for the query string"]
    pub fn analyzer(mut self, analyzer: &'b str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesValidateQuery<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesValidateQuery {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            all_shards: self.all_shards,
            allow_no_indices: self.allow_no_indices,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            default_operator: self.default_operator,
            df: self.df,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            explain: self.explain,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            lenient: self.lenient,
            pretty: self.pretty,
            q: self.q,
            request_timeout: self.request_timeout,
            rewrite: self.rewrite,
            source: self.source,
        }
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: &'b str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "Return detailed information about the error"]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: &'b str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "Provide a more detailed explanation showing the actual Lucene query that will be executed."]
    pub fn rewrite(mut self, rewrite: bool) -> Self {
        self.rewrite = Some(rewrite);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Validate Query API that can be awaited"]
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
                all_shards: Option<bool>,
                allow_no_indices: Option<bool>,
                analyze_wildcard: Option<bool>,
                analyzer: Option<&'b str>,
                default_operator: Option<DefaultOperator>,
                df: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                explain: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                lenient: Option<bool>,
                pretty: Option<bool>,
                q: Option<&'b str>,
                rewrite: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                all_shards: self.all_shards,
                allow_no_indices: self.allow_no_indices,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                explain: self.explain,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                pretty: self.pretty,
                q: self.q,
                rewrite: self.rewrite,
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
#[doc = "Namespace client for Indices APIs"]
pub struct Indices<'a> {
    transport: &'a Transport,
}
impl<'a> Indices<'a> {
    #[doc = "Creates a new instance of [Indices]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Indices Add Block API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/index-modules-blocks.html)\n\nAdds a block to an index."]
    pub fn add_block<'b>(&'a self, parts: IndicesAddBlockParts<'b>) -> IndicesAddBlock<'a, 'b, ()> {
        IndicesAddBlock::new(self.transport(), parts)
    }
    #[doc = "[Indices Analyze API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-analyze.html)\n\nPerforms the analysis process on a text and return the tokens breakdown of the text."]
    pub fn analyze<'b>(&'a self, parts: IndicesAnalyzeParts<'b>) -> IndicesAnalyze<'a, 'b, ()> {
        IndicesAnalyze::new(self.transport(), parts)
    }
    #[doc = "[Indices Clear Cache API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-clearcache.html)\n\nClears all or specific caches for one or more indices."]
    pub fn clear_cache<'b>(
        &'a self,
        parts: IndicesClearCacheParts<'b>,
    ) -> IndicesClearCache<'a, 'b, ()> {
        IndicesClearCache::new(self.transport(), parts)
    }
    #[doc = "[Indices Clone API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-clone-index.html)\n\nClones an index"]
    pub fn clone<'b>(&'a self, parts: IndicesCloneParts<'b>) -> IndicesClone<'a, 'b, ()> {
        IndicesClone::new(self.transport(), parts)
    }
    #[doc = "[Indices Close API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-open-close.html)\n\nCloses an index."]
    pub fn close<'b>(&'a self, parts: IndicesCloseParts<'b>) -> IndicesClose<'a, 'b, ()> {
        IndicesClose::new(self.transport(), parts)
    }
    #[doc = "[Indices Create API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-create-index.html)\n\nCreates an index with optional settings and mappings.\n\n# Examples\n\nCreate an index with a mapping\n\n```rust,no_run\n# use elasticsearch::{Elasticsearch, Error, indices::IndicesCreateParts};\n# use serde_json::{json, Value};\n# async fn doc() -> Result<(), Box<dyn std::error::Error>> {\nlet client = Elasticsearch::default();\nlet response = client\n    .indices()\n    .create(IndicesCreateParts::Index(\"test_index\"))\n    .body(json!({\n        \"mappings\" : {\n            \"properties\" : {\n                \"field1\" : { \"type\" : \"text\" }\n            }\n        }\n    }))\n    .send()\n    .await?;\n    \n# Ok(())\n# }\n```"]
    pub fn create<'b>(&'a self, parts: IndicesCreateParts<'b>) -> IndicesCreate<'a, 'b, ()> {
        IndicesCreate::new(self.transport(), parts)
    }
    #[doc = "[Indices Create Data Stream API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/data-streams.html)\n\nCreates a data stream"]
    pub fn create_data_stream<'b>(
        &'a self,
        parts: IndicesCreateDataStreamParts<'b>,
    ) -> IndicesCreateDataStream<'a, 'b, ()> {
        IndicesCreateDataStream::new(self.transport(), parts)
    }
    #[doc = "[Indices Data Streams Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/data-streams.html)\n\nProvides statistics on operations happening in a data stream."]
    pub fn data_streams_stats<'b>(
        &'a self,
        parts: IndicesDataStreamsStatsParts<'b>,
    ) -> IndicesDataStreamsStats<'a, 'b> {
        IndicesDataStreamsStats::new(self.transport(), parts)
    }
    #[doc = "[Indices Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-delete-index.html)\n\nDeletes an index."]
    pub fn delete<'b>(&'a self, parts: IndicesDeleteParts<'b>) -> IndicesDelete<'a, 'b> {
        IndicesDelete::new(self.transport(), parts)
    }
    #[doc = "[Indices Delete Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-aliases.html)\n\nDeletes an alias."]
    pub fn delete_alias<'b>(
        &'a self,
        parts: IndicesDeleteAliasParts<'b>,
    ) -> IndicesDeleteAlias<'a, 'b> {
        IndicesDeleteAlias::new(self.transport(), parts)
    }
    #[doc = "[Indices Delete Data Stream API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/data-streams.html)\n\nDeletes a data stream."]
    pub fn delete_data_stream<'b>(
        &'a self,
        parts: IndicesDeleteDataStreamParts<'b>,
    ) -> IndicesDeleteDataStream<'a, 'b> {
        IndicesDeleteDataStream::new(self.transport(), parts)
    }
    #[doc = "[Indices Delete Index Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nDeletes an index template."]
    pub fn delete_index_template<'b>(
        &'a self,
        parts: IndicesDeleteIndexTemplateParts<'b>,
    ) -> IndicesDeleteIndexTemplate<'a, 'b> {
        IndicesDeleteIndexTemplate::new(self.transport(), parts)
    }
    #[doc = "[Indices Delete Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nDeletes an index template."]
    pub fn delete_template<'b>(
        &'a self,
        parts: IndicesDeleteTemplateParts<'b>,
    ) -> IndicesDeleteTemplate<'a, 'b> {
        IndicesDeleteTemplate::new(self.transport(), parts)
    }
    #[doc = "[Indices Exists API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-exists.html)\n\nReturns information about whether a particular index exists."]
    pub fn exists<'b>(&'a self, parts: IndicesExistsParts<'b>) -> IndicesExists<'a, 'b> {
        IndicesExists::new(self.transport(), parts)
    }
    #[doc = "[Indices Exists Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-aliases.html)\n\nReturns information about whether a particular alias exists."]
    pub fn exists_alias<'b>(
        &'a self,
        parts: IndicesExistsAliasParts<'b>,
    ) -> IndicesExistsAlias<'a, 'b> {
        IndicesExistsAlias::new(self.transport(), parts)
    }
    #[doc = "[Indices Exists Index Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nReturns information about whether a particular index template exists."]
    pub fn exists_index_template<'b>(
        &'a self,
        parts: IndicesExistsIndexTemplateParts<'b>,
    ) -> IndicesExistsIndexTemplate<'a, 'b> {
        IndicesExistsIndexTemplate::new(self.transport(), parts)
    }
    #[doc = "[Indices Exists Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nReturns information about whether a particular index template exists."]
    pub fn exists_template<'b>(
        &'a self,
        parts: IndicesExistsTemplateParts<'b>,
    ) -> IndicesExistsTemplate<'a, 'b> {
        IndicesExistsTemplate::new(self.transport(), parts)
    }
    #[doc = "[Indices Exists Type API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-types-exists.html)\n\nReturns information about whether a particular document type exists. (DEPRECATED)"]
    pub fn exists_type<'b>(
        &'a self,
        parts: IndicesExistsTypeParts<'b>,
    ) -> IndicesExistsType<'a, 'b> {
        IndicesExistsType::new(self.transport(), parts)
    }
    #[doc = "[Indices Flush API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-flush.html)\n\nPerforms the flush operation on one or more indices."]
    pub fn flush<'b>(&'a self, parts: IndicesFlushParts<'b>) -> IndicesFlush<'a, 'b, ()> {
        IndicesFlush::new(self.transport(), parts)
    }
    #[doc = "[Indices Flush Synced API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-synced-flush-api.html)\n\nPerforms a synced flush operation on one or more indices. Synced flush is deprecated and will be removed in 8.0. Use flush instead"]
    pub fn flush_synced<'b>(
        &'a self,
        parts: IndicesFlushSyncedParts<'b>,
    ) -> IndicesFlushSynced<'a, 'b, ()> {
        IndicesFlushSynced::new(self.transport(), parts)
    }
    #[doc = "[Indices Forcemerge API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-forcemerge.html)\n\nPerforms the force merge operation on one or more indices."]
    pub fn forcemerge<'b>(
        &'a self,
        parts: IndicesForcemergeParts<'b>,
    ) -> IndicesForcemerge<'a, 'b, ()> {
        IndicesForcemerge::new(self.transport(), parts)
    }
    #[doc = "[Indices Freeze API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/freeze-index-api.html)\n\nFreezes an index. A frozen index has almost no overhead on the cluster (except for maintaining its metadata in memory) and is read-only."]
    pub fn freeze<'b>(&'a self, parts: IndicesFreezeParts<'b>) -> IndicesFreeze<'a, 'b, ()> {
        IndicesFreeze::new(self.transport(), parts)
    }
    #[doc = "[Indices Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-get-index.html)\n\nReturns information about one or more indices."]
    pub fn get<'b>(&'a self, parts: IndicesGetParts<'b>) -> IndicesGet<'a, 'b> {
        IndicesGet::new(self.transport(), parts)
    }
    #[doc = "[Indices Get Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-aliases.html)\n\nReturns an alias."]
    pub fn get_alias<'b>(&'a self, parts: IndicesGetAliasParts<'b>) -> IndicesGetAlias<'a, 'b> {
        IndicesGetAlias::new(self.transport(), parts)
    }
    #[doc = "[Indices Get Data Stream API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/data-streams.html)\n\nReturns data streams."]
    pub fn get_data_stream<'b>(
        &'a self,
        parts: IndicesGetDataStreamParts<'b>,
    ) -> IndicesGetDataStream<'a, 'b> {
        IndicesGetDataStream::new(self.transport(), parts)
    }
    #[doc = "[Indices Get Field Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-get-field-mapping.html)\n\nReturns mapping for one or more fields."]
    pub fn get_field_mapping<'b>(
        &'a self,
        parts: IndicesGetFieldMappingParts<'b>,
    ) -> IndicesGetFieldMapping<'a, 'b> {
        IndicesGetFieldMapping::new(self.transport(), parts)
    }
    #[doc = "[Indices Get Index Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nReturns an index template."]
    pub fn get_index_template<'b>(
        &'a self,
        parts: IndicesGetIndexTemplateParts<'b>,
    ) -> IndicesGetIndexTemplate<'a, 'b> {
        IndicesGetIndexTemplate::new(self.transport(), parts)
    }
    #[doc = "[Indices Get Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-get-mapping.html)\n\nReturns mappings for one or more indices."]
    pub fn get_mapping<'b>(
        &'a self,
        parts: IndicesGetMappingParts<'b>,
    ) -> IndicesGetMapping<'a, 'b> {
        IndicesGetMapping::new(self.transport(), parts)
    }
    #[doc = "[Indices Get Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-get-settings.html)\n\nReturns settings for one or more indices."]
    pub fn get_settings<'b>(
        &'a self,
        parts: IndicesGetSettingsParts<'b>,
    ) -> IndicesGetSettings<'a, 'b> {
        IndicesGetSettings::new(self.transport(), parts)
    }
    #[doc = "[Indices Get Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nReturns an index template."]
    pub fn get_template<'b>(
        &'a self,
        parts: IndicesGetTemplateParts<'b>,
    ) -> IndicesGetTemplate<'a, 'b> {
        IndicesGetTemplate::new(self.transport(), parts)
    }
    #[doc = "[Indices Get Upgrade API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-upgrade.html)\n\nDEPRECATED Returns a progress status of current upgrade."]
    pub fn get_upgrade<'b>(
        &'a self,
        parts: IndicesGetUpgradeParts<'b>,
    ) -> IndicesGetUpgrade<'a, 'b> {
        IndicesGetUpgrade::new(self.transport(), parts)
    }
    #[doc = "[Indices Migrate To Data Stream API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/data-streams.html)\n\nMigrates an alias to a data stream"]
    pub fn migrate_to_data_stream<'b>(
        &'a self,
        parts: IndicesMigrateToDataStreamParts<'b>,
    ) -> IndicesMigrateToDataStream<'a, 'b, ()> {
        IndicesMigrateToDataStream::new(self.transport(), parts)
    }
    #[doc = "[Indices Open API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-open-close.html)\n\nOpens an index."]
    pub fn open<'b>(&'a self, parts: IndicesOpenParts<'b>) -> IndicesOpen<'a, 'b, ()> {
        IndicesOpen::new(self.transport(), parts)
    }
    #[doc = "[Indices Promote Data Stream API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/data-streams.html)\n\nPromotes a data stream from a replicated data stream managed by CCR to a regular data stream"]
    pub fn promote_data_stream<'b>(
        &'a self,
        parts: IndicesPromoteDataStreamParts<'b>,
    ) -> IndicesPromoteDataStream<'a, 'b, ()> {
        IndicesPromoteDataStream::new(self.transport(), parts)
    }
    #[doc = "[Indices Put Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-aliases.html)\n\nCreates or updates an alias."]
    pub fn put_alias<'b>(&'a self, parts: IndicesPutAliasParts<'b>) -> IndicesPutAlias<'a, 'b, ()> {
        IndicesPutAlias::new(self.transport(), parts)
    }
    #[doc = "[Indices Put Index Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nCreates or updates an index template."]
    pub fn put_index_template<'b>(
        &'a self,
        parts: IndicesPutIndexTemplateParts<'b>,
    ) -> IndicesPutIndexTemplate<'a, 'b, ()> {
        IndicesPutIndexTemplate::new(self.transport(), parts)
    }
    #[doc = "[Indices Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-put-mapping.html)\n\nUpdates the index mappings.\n\n# Examples\n\nPut a mapping into an existing index, assuming the index does not have a mapping, \nor that any properties specified do not conflict with existing properties\n\n```rust,no_run\n# use elasticsearch::{Elasticsearch, Error, indices::IndicesPutMappingParts};\n# use serde_json::{json, Value};\n# async fn doc() -> Result<(), Box<dyn std::error::Error>> {\nlet client = Elasticsearch::default();\nlet response = client\n    .indices()\n    .put_mapping(IndicesPutMappingParts::Index(&[\"test_index\"]))\n    .body(json!({\n        \"properties\" : {\n            \"field1\" : { \"type\" : \"text\" }\n        }\n    }))\n    .send()\n    .await?;\n    \n# Ok(())\n# }\n```"]
    pub fn put_mapping<'b>(
        &'a self,
        parts: IndicesPutMappingParts<'b>,
    ) -> IndicesPutMapping<'a, 'b, ()> {
        IndicesPutMapping::new(self.transport(), parts)
    }
    #[doc = "[Indices Put Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-update-settings.html)\n\nUpdates the index settings."]
    pub fn put_settings<'b>(
        &'a self,
        parts: IndicesPutSettingsParts<'b>,
    ) -> IndicesPutSettings<'a, 'b, ()> {
        IndicesPutSettings::new(self.transport(), parts)
    }
    #[doc = "[Indices Put Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nCreates or updates an index template."]
    pub fn put_template<'b>(
        &'a self,
        parts: IndicesPutTemplateParts<'b>,
    ) -> IndicesPutTemplate<'a, 'b, ()> {
        IndicesPutTemplate::new(self.transport(), parts)
    }
    #[doc = "[Indices Recovery API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-recovery.html)\n\nReturns information about ongoing index shard recoveries."]
    pub fn recovery<'b>(&'a self, parts: IndicesRecoveryParts<'b>) -> IndicesRecovery<'a, 'b> {
        IndicesRecovery::new(self.transport(), parts)
    }
    #[doc = "[Indices Refresh API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-refresh.html)\n\nPerforms the refresh operation in one or more indices."]
    pub fn refresh<'b>(&'a self, parts: IndicesRefreshParts<'b>) -> IndicesRefresh<'a, 'b, ()> {
        IndicesRefresh::new(self.transport(), parts)
    }
    #[doc = "[Indices Reload Search Analyzers API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-reload-analyzers.html)\n\nReloads an index's search analyzers and their resources."]
    pub fn reload_search_analyzers<'b>(
        &'a self,
        parts: IndicesReloadSearchAnalyzersParts<'b>,
    ) -> IndicesReloadSearchAnalyzers<'a, 'b, ()> {
        IndicesReloadSearchAnalyzers::new(self.transport(), parts)
    }
    #[doc = "[Indices Resolve Index API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-resolve-index-api.html)\n\nReturns information about any matching indices, aliases, and data streams"]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn resolve_index<'b>(
        &'a self,
        parts: IndicesResolveIndexParts<'b>,
    ) -> IndicesResolveIndex<'a, 'b> {
        IndicesResolveIndex::new(self.transport(), parts)
    }
    #[doc = "[Indices Rollover API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-rollover-index.html)\n\nUpdates an alias to point to a new index when the existing index\nis considered to be too large or too old."]
    pub fn rollover<'b>(&'a self, parts: IndicesRolloverParts<'b>) -> IndicesRollover<'a, 'b, ()> {
        IndicesRollover::new(self.transport(), parts)
    }
    #[doc = "[Indices Segments API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-segments.html)\n\nProvides low-level information about segments in a Lucene index."]
    pub fn segments<'b>(&'a self, parts: IndicesSegmentsParts<'b>) -> IndicesSegments<'a, 'b> {
        IndicesSegments::new(self.transport(), parts)
    }
    #[doc = "[Indices Shard Stores API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-shards-stores.html)\n\nProvides store information for shard copies of indices."]
    pub fn shard_stores<'b>(
        &'a self,
        parts: IndicesShardStoresParts<'b>,
    ) -> IndicesShardStores<'a, 'b> {
        IndicesShardStores::new(self.transport(), parts)
    }
    #[doc = "[Indices Shrink API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-shrink-index.html)\n\nAllow to shrink an existing index into a new index with fewer primary shards."]
    pub fn shrink<'b>(&'a self, parts: IndicesShrinkParts<'b>) -> IndicesShrink<'a, 'b, ()> {
        IndicesShrink::new(self.transport(), parts)
    }
    #[doc = "[Indices Simulate Index Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nSimulate matching the given index name against the index templates in the system"]
    pub fn simulate_index_template<'b>(
        &'a self,
        parts: IndicesSimulateIndexTemplateParts<'b>,
    ) -> IndicesSimulateIndexTemplate<'a, 'b, ()> {
        IndicesSimulateIndexTemplate::new(self.transport(), parts)
    }
    #[doc = "[Indices Simulate Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-templates.html)\n\nSimulate resolving the given template name or body"]
    pub fn simulate_template<'b>(
        &'a self,
        parts: IndicesSimulateTemplateParts<'b>,
    ) -> IndicesSimulateTemplate<'a, 'b, ()> {
        IndicesSimulateTemplate::new(self.transport(), parts)
    }
    #[doc = "[Indices Split API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-split-index.html)\n\nAllows you to split an existing index into a new index with more primary shards."]
    pub fn split<'b>(&'a self, parts: IndicesSplitParts<'b>) -> IndicesSplit<'a, 'b, ()> {
        IndicesSplit::new(self.transport(), parts)
    }
    #[doc = "[Indices Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-stats.html)\n\nProvides statistics on operations happening in an index."]
    pub fn stats<'b>(&'a self, parts: IndicesStatsParts<'b>) -> IndicesStats<'a, 'b> {
        IndicesStats::new(self.transport(), parts)
    }
    #[doc = "[Indices Unfreeze API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/unfreeze-index-api.html)\n\nUnfreezes an index. When a frozen index is unfrozen, the index goes through the normal recovery process and becomes writeable again."]
    pub fn unfreeze<'b>(&'a self, parts: IndicesUnfreezeParts<'b>) -> IndicesUnfreeze<'a, 'b, ()> {
        IndicesUnfreeze::new(self.transport(), parts)
    }
    #[doc = "[Indices Update Aliases API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-aliases.html)\n\nUpdates index aliases."]
    pub fn update_aliases<'b>(&'a self) -> IndicesUpdateAliases<'a, 'b, ()> {
        IndicesUpdateAliases::new(self.transport())
    }
    #[doc = "[Indices Upgrade API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-upgrade.html)\n\nDEPRECATED Upgrades to the current version of Lucene."]
    pub fn upgrade<'b>(&'a self, parts: IndicesUpgradeParts<'b>) -> IndicesUpgrade<'a, 'b, ()> {
        IndicesUpgrade::new(self.transport(), parts)
    }
    #[doc = "[Indices Validate Query API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/search-validate.html)\n\nAllows a user to validate a potentially expensive query without executing it."]
    pub fn validate_query<'b>(
        &'a self,
        parts: IndicesValidateQueryParts<'b>,
    ) -> IndicesValidateQuery<'a, 'b, ()> {
        IndicesValidateQuery::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Indices APIs"]
    pub fn indices(&self) -> Indices {
        Indices::new(self.transport())
    }
}
