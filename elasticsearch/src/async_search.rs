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

//! Async Search APIs
//!
//! [Async search APIs](https://www.elastic.co/guide/en/elasticsearch/reference/master/async-search.html)
//! let you asynchronously execute a search request, monitor its progress, and retrieve
//! partial results as they become available.

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
#[doc = "API parts for the Async Search Delete API"]
pub enum AsyncSearchDeleteParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> AsyncSearchDeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Async Search Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            AsyncSearchDeleteParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_id.len());
                p.push_str("/_async_search/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Async Search Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/async-search.html)\n\nDeletes an async search by ID. If the search is still running, the search request will be cancelled. Otherwise, the saved search results are deleted."]
#[derive(Clone, Debug)]
pub struct AsyncSearchDelete<'a, 'b> {
    transport: &'a Transport,
    parts: AsyncSearchDeleteParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> AsyncSearchDelete<'a, 'b> {
    #[doc = "Creates a new instance of [AsyncSearchDelete] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: AsyncSearchDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        AsyncSearchDelete {
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
    #[doc = "Creates an asynchronous call to the Async Search Delete API that can be awaited"]
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
#[doc = "API parts for the Async Search Get API"]
pub enum AsyncSearchGetParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> AsyncSearchGetParts<'b> {
    #[doc = "Builds a relative URL path to the Async Search Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            AsyncSearchGetParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_id.len());
                p.push_str("/_async_search/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Async Search Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/async-search.html)\n\nRetrieves the results of a previously submitted async search request given its ID."]
#[derive(Clone, Debug)]
pub struct AsyncSearchGet<'a, 'b> {
    transport: &'a Transport,
    parts: AsyncSearchGetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    keep_alive: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    typed_keys: Option<bool>,
    wait_for_completion_timeout: Option<&'b str>,
}
impl<'a, 'b> AsyncSearchGet<'a, 'b> {
    #[doc = "Creates a new instance of [AsyncSearchGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: AsyncSearchGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        AsyncSearchGet {
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
            typed_keys: None,
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
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "Specify the time that the request should block waiting for the final response"]
    pub fn wait_for_completion_timeout(mut self, wait_for_completion_timeout: &'b str) -> Self {
        self.wait_for_completion_timeout = Some(wait_for_completion_timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Async Search Get API that can be awaited"]
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
                typed_keys: Option<bool>,
                wait_for_completion_timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                keep_alive: self.keep_alive,
                pretty: self.pretty,
                source: self.source,
                typed_keys: self.typed_keys,
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
#[doc = "API parts for the Async Search Status API"]
pub enum AsyncSearchStatusParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> AsyncSearchStatusParts<'b> {
    #[doc = "Builds a relative URL path to the Async Search Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            AsyncSearchStatusParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_id.len());
                p.push_str("/_async_search/status/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Async Search Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/async-search.html)\n\nRetrieves the status of a previously submitted async search request given its ID."]
#[derive(Clone, Debug)]
pub struct AsyncSearchStatus<'a, 'b> {
    transport: &'a Transport,
    parts: AsyncSearchStatusParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> AsyncSearchStatus<'a, 'b> {
    #[doc = "Creates a new instance of [AsyncSearchStatus] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: AsyncSearchStatusParts<'b>) -> Self {
        let headers = HeaderMap::new();
        AsyncSearchStatus {
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
    #[doc = "Creates an asynchronous call to the Async Search Status API that can be awaited"]
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
#[doc = "API parts for the Async Search Submit API"]
pub enum AsyncSearchSubmitParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> AsyncSearchSubmitParts<'b> {
    #[doc = "Builds a relative URL path to the Async Search Submit API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            AsyncSearchSubmitParts::None => "/_async_search".into(),
            AsyncSearchSubmitParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_async_search");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Async Search Submit API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/async-search.html)\n\nExecutes a search request asynchronously."]
#[derive(Clone, Debug)]
pub struct AsyncSearchSubmit<'a, 'b, B> {
    transport: &'a Transport,
    parts: AsyncSearchSubmitParts<'b>,
    _source: Option<&'b [&'b str]>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    allow_no_indices: Option<bool>,
    allow_partial_search_results: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    batched_reduce_size: Option<i64>,
    body: Option<B>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    docvalue_fields: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    explain: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    keep_alive: Option<&'b str>,
    keep_on_completion: Option<bool>,
    lenient: Option<bool>,
    max_concurrent_shard_requests: Option<i64>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    request_cache: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    search_type: Option<SearchType>,
    seq_no_primary_term: Option<bool>,
    size: Option<i64>,
    sort: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    stats: Option<&'b [&'b str]>,
    stored_fields: Option<&'b [&'b str]>,
    suggest_field: Option<&'b str>,
    suggest_mode: Option<SuggestMode>,
    suggest_size: Option<i64>,
    suggest_text: Option<&'b str>,
    terminate_after: Option<i64>,
    timeout: Option<&'b str>,
    track_scores: Option<bool>,
    track_total_hits: Option<TrackTotalHits>,
    typed_keys: Option<bool>,
    version: Option<bool>,
    wait_for_completion_timeout: Option<&'b str>,
}
impl<'a, 'b, B> AsyncSearchSubmit<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [AsyncSearchSubmit] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: AsyncSearchSubmitParts<'b>) -> Self {
        let headers = HeaderMap::new();
        AsyncSearchSubmit {
            transport,
            parts,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            allow_no_indices: None,
            allow_partial_search_results: None,
            analyze_wildcard: None,
            analyzer: None,
            batched_reduce_size: None,
            body: None,
            default_operator: None,
            df: None,
            docvalue_fields: None,
            error_trace: None,
            expand_wildcards: None,
            explain: None,
            filter_path: None,
            from: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            keep_alive: None,
            keep_on_completion: None,
            lenient: None,
            max_concurrent_shard_requests: None,
            preference: None,
            pretty: None,
            q: None,
            request_cache: None,
            request_timeout: None,
            routing: None,
            search_type: None,
            seq_no_primary_term: None,
            size: None,
            sort: None,
            source: None,
            stats: None,
            stored_fields: None,
            suggest_field: None,
            suggest_mode: None,
            suggest_size: None,
            suggest_text: None,
            terminate_after: None,
            timeout: None,
            track_scores: None,
            track_total_hits: None,
            typed_keys: None,
            version: None,
            wait_for_completion_timeout: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: &'b [&'b str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Indicate if an error should be returned if there is a partial search failure or timeout"]
    pub fn allow_partial_search_results(mut self, allow_partial_search_results: bool) -> Self {
        self.allow_partial_search_results = Some(allow_partial_search_results);
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
    #[doc = "The number of shard results that should be reduced at once on the coordinating node. This value should be used as the granularity at which progress results will be made available."]
    pub fn batched_reduce_size(mut self, batched_reduce_size: i64) -> Self {
        self.batched_reduce_size = Some(batched_reduce_size);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> AsyncSearchSubmit<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        AsyncSearchSubmit {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            allow_no_indices: self.allow_no_indices,
            allow_partial_search_results: self.allow_partial_search_results,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            batched_reduce_size: self.batched_reduce_size,
            default_operator: self.default_operator,
            df: self.df,
            docvalue_fields: self.docvalue_fields,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            explain: self.explain,
            filter_path: self.filter_path,
            from: self.from,
            headers: self.headers,
            human: self.human,
            ignore_throttled: self.ignore_throttled,
            ignore_unavailable: self.ignore_unavailable,
            keep_alive: self.keep_alive,
            keep_on_completion: self.keep_on_completion,
            lenient: self.lenient,
            max_concurrent_shard_requests: self.max_concurrent_shard_requests,
            preference: self.preference,
            pretty: self.pretty,
            q: self.q,
            request_cache: self.request_cache,
            request_timeout: self.request_timeout,
            routing: self.routing,
            search_type: self.search_type,
            seq_no_primary_term: self.seq_no_primary_term,
            size: self.size,
            sort: self.sort,
            source: self.source,
            stats: self.stats,
            stored_fields: self.stored_fields,
            suggest_field: self.suggest_field,
            suggest_mode: self.suggest_mode,
            suggest_size: self.suggest_size,
            suggest_text: self.suggest_text,
            terminate_after: self.terminate_after,
            timeout: self.timeout,
            track_scores: self.track_scores,
            track_total_hits: self.track_total_hits,
            typed_keys: self.typed_keys,
            version: self.version,
            wait_for_completion_timeout: self.wait_for_completion_timeout,
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
    #[doc = "A comma-separated list of fields to return as the docvalue representation of a field for each hit"]
    pub fn docvalue_fields(mut self, docvalue_fields: &'b [&'b str]) -> Self {
        self.docvalue_fields = Some(docvalue_fields);
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
    #[doc = "Specify whether to return detailed information about score computation as part of a hit"]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Starting offset (default: 0)"]
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from);
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
    #[doc = "Whether specified concrete, expanded or aliased indices should be ignored when throttled"]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "The number of concurrent shard requests per node this search executes concurrently. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests"]
    pub fn max_concurrent_shard_requests(mut self, max_concurrent_shard_requests: i64) -> Self {
        self.max_concurrent_shard_requests = Some(max_concurrent_shard_requests);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
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
    #[doc = "Specify if request cache should be used for this request or not, defaults to true"]
    pub fn request_cache(mut self, request_cache: bool) -> Self {
        self.request_cache = Some(request_cache);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "Specify whether to return sequence number and primary term of the last modification of each hit"]
    pub fn seq_no_primary_term(mut self, seq_no_primary_term: bool) -> Self {
        self.seq_no_primary_term = Some(seq_no_primary_term);
        self
    }
    #[doc = "Number of hits to return (default: 10)"]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "A comma-separated list of <field>:<direction> pairs"]
    pub fn sort(mut self, sort: &'b [&'b str]) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specific 'tag' of the request for logging and statistical purposes"]
    pub fn stats(mut self, stats: &'b [&'b str]) -> Self {
        self.stats = Some(stats);
        self
    }
    #[doc = "A comma-separated list of stored fields to return as part of a hit"]
    pub fn stored_fields(mut self, stored_fields: &'b [&'b str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Specify which field to use for suggestions"]
    pub fn suggest_field(mut self, suggest_field: &'b str) -> Self {
        self.suggest_field = Some(suggest_field);
        self
    }
    #[doc = "Specify suggest mode"]
    pub fn suggest_mode(mut self, suggest_mode: SuggestMode) -> Self {
        self.suggest_mode = Some(suggest_mode);
        self
    }
    #[doc = "How many suggestions to return in response"]
    pub fn suggest_size(mut self, suggest_size: i64) -> Self {
        self.suggest_size = Some(suggest_size);
        self
    }
    #[doc = "The source text for which the suggestions should be returned"]
    pub fn suggest_text(mut self, suggest_text: &'b str) -> Self {
        self.suggest_text = Some(suggest_text);
        self
    }
    #[doc = "The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early."]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Whether to calculate and return scores even if they are not used for sorting"]
    pub fn track_scores(mut self, track_scores: bool) -> Self {
        self.track_scores = Some(track_scores);
        self
    }
    #[doc = "Indicate if the number of documents that match the query should be tracked"]
    pub fn track_total_hits<T: Into<TrackTotalHits>>(mut self, track_total_hits: T) -> Self {
        self.track_total_hits = Some(track_total_hits.into());
        self
    }
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "Specify whether to return document version as part of a hit"]
    pub fn version(mut self, version: bool) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specify the time that the request should block waiting for the final response"]
    pub fn wait_for_completion_timeout(mut self, wait_for_completion_timeout: &'b str) -> Self {
        self.wait_for_completion_timeout = Some(wait_for_completion_timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Async Search Submit API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                allow_no_indices: Option<bool>,
                allow_partial_search_results: Option<bool>,
                analyze_wildcard: Option<bool>,
                analyzer: Option<&'b str>,
                batched_reduce_size: Option<i64>,
                default_operator: Option<DefaultOperator>,
                df: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                docvalue_fields: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                explain: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                from: Option<i64>,
                human: Option<bool>,
                ignore_throttled: Option<bool>,
                ignore_unavailable: Option<bool>,
                keep_alive: Option<&'b str>,
                keep_on_completion: Option<bool>,
                lenient: Option<bool>,
                max_concurrent_shard_requests: Option<i64>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                q: Option<&'b str>,
                request_cache: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                search_type: Option<SearchType>,
                seq_no_primary_term: Option<bool>,
                size: Option<i64>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                sort: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                stats: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                stored_fields: Option<&'b [&'b str]>,
                suggest_field: Option<&'b str>,
                suggest_mode: Option<SuggestMode>,
                suggest_size: Option<i64>,
                suggest_text: Option<&'b str>,
                terminate_after: Option<i64>,
                timeout: Option<&'b str>,
                track_scores: Option<bool>,
                track_total_hits: Option<TrackTotalHits>,
                typed_keys: Option<bool>,
                version: Option<bool>,
                wait_for_completion_timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                allow_no_indices: self.allow_no_indices,
                allow_partial_search_results: self.allow_partial_search_results,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                batched_reduce_size: self.batched_reduce_size,
                default_operator: self.default_operator,
                df: self.df,
                docvalue_fields: self.docvalue_fields,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                explain: self.explain,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                keep_alive: self.keep_alive,
                keep_on_completion: self.keep_on_completion,
                lenient: self.lenient,
                max_concurrent_shard_requests: self.max_concurrent_shard_requests,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                request_cache: self.request_cache,
                routing: self.routing,
                search_type: self.search_type,
                seq_no_primary_term: self.seq_no_primary_term,
                size: self.size,
                sort: self.sort,
                source: self.source,
                stats: self.stats,
                stored_fields: self.stored_fields,
                suggest_field: self.suggest_field,
                suggest_mode: self.suggest_mode,
                suggest_size: self.suggest_size,
                suggest_text: self.suggest_text,
                terminate_after: self.terminate_after,
                timeout: self.timeout,
                track_scores: self.track_scores,
                track_total_hits: self.track_total_hits,
                typed_keys: self.typed_keys,
                version: self.version,
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
#[doc = "Namespace client for AsyncSearch APIs"]
pub struct AsyncSearch<'a> {
    transport: &'a Transport,
}
impl<'a> AsyncSearch<'a> {
    #[doc = "Creates a new instance of [AsyncSearch]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Async Search Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/async-search.html)\n\nDeletes an async search by ID. If the search is still running, the search request will be cancelled. Otherwise, the saved search results are deleted."]
    pub fn delete<'b>(&'a self, parts: AsyncSearchDeleteParts<'b>) -> AsyncSearchDelete<'a, 'b> {
        AsyncSearchDelete::new(self.transport(), parts)
    }
    #[doc = "[Async Search Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/async-search.html)\n\nRetrieves the results of a previously submitted async search request given its ID."]
    pub fn get<'b>(&'a self, parts: AsyncSearchGetParts<'b>) -> AsyncSearchGet<'a, 'b> {
        AsyncSearchGet::new(self.transport(), parts)
    }
    #[doc = "[Async Search Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/async-search.html)\n\nRetrieves the status of a previously submitted async search request given its ID."]
    pub fn status<'b>(&'a self, parts: AsyncSearchStatusParts<'b>) -> AsyncSearchStatus<'a, 'b> {
        AsyncSearchStatus::new(self.transport(), parts)
    }
    #[doc = "[Async Search Submit API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/async-search.html)\n\nExecutes a search request asynchronously."]
    pub fn submit<'b>(
        &'a self,
        parts: AsyncSearchSubmitParts<'b>,
    ) -> AsyncSearchSubmit<'a, 'b, ()> {
        AsyncSearchSubmit::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for AsyncSearch APIs"]
    pub fn async_search(&self) -> AsyncSearch {
        AsyncSearch::new(self.transport())
    }
}
