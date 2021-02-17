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

//! Searchable snapshot APIs
//!
//! [Searchable snapshots](https://www.elastic.co/guide/en/elasticsearch/reference/master/searchable-snapshots.html) let
//! you reduce your operating costs by using snapshots for resiliency rather than maintaining replica shards within a
//! cluster. This can result in significant cost savings for less frequently searched data.
//!

#![cfg(feature = "experimental-apis")]
#![doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Searchable Snapshots Clear Cache API"]
pub enum SearchableSnapshotsClearCacheParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
#[cfg(feature = "experimental-apis")]
impl<'b> SearchableSnapshotsClearCacheParts<'b> {
    #[doc = "Builds a relative URL path to the Searchable Snapshots Clear Cache API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchableSnapshotsClearCacheParts::None => "/_searchable_snapshots/cache/clear".into(),
            SearchableSnapshotsClearCacheParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(35usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_searchable_snapshots/cache/clear");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Searchable Snapshots Clear Cache API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/searchable-snapshots-apis.html)\n\nClear the cache of searchable snapshots."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct SearchableSnapshotsClearCache<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchableSnapshotsClearCacheParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<&'b [&'b str]>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b, B> SearchableSnapshotsClearCache<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchableSnapshotsClearCache] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchableSnapshotsClearCacheParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchableSnapshotsClearCache {
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
            index: None,
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
    pub fn body<T>(self, body: T) -> SearchableSnapshotsClearCache<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchableSnapshotsClearCache {
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
    #[doc = "Creates an asynchronous call to the Searchable Snapshots Clear Cache API that can be awaited"]
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
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                index: Option<&'b [&'b str]>,
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Searchable Snapshots Mount API"]
pub enum SearchableSnapshotsMountParts<'b> {
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'b str, &'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> SearchableSnapshotsMountParts<'b> {
    #[doc = "Builds a relative URL path to the Searchable Snapshots Mount API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchableSnapshotsMountParts::RepositorySnapshot(ref repository, ref snapshot) => {
                let encoded_repository: Cow<str> =
                    percent_encode(repository.as_bytes(), PARTS_ENCODED).into();
                let encoded_snapshot: Cow<str> =
                    percent_encode(snapshot.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    19usize + encoded_repository.len() + encoded_snapshot.len(),
                );
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.push_str("/");
                p.push_str(encoded_snapshot.as_ref());
                p.push_str("/_mount");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Searchable Snapshots Mount API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/searchable-snapshots-api-mount-snapshot.html)\n\nMount a snapshot as a searchable index."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct SearchableSnapshotsMount<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchableSnapshotsMountParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    storage: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b, B> SearchableSnapshotsMount<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchableSnapshotsMount] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchableSnapshotsMountParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchableSnapshotsMount {
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
            storage: None,
            wait_for_completion: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SearchableSnapshotsMount<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchableSnapshotsMount {
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
            storage: self.storage,
            wait_for_completion: self.wait_for_completion,
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
    #[doc = "Selects the kind of local storage used to accelerate searches. Experimental, and defaults to `full_copy`"]
    pub fn storage(mut self, storage: &'b str) -> Self {
        self.storage = Some(storage);
        self
    }
    #[doc = "Should this request wait until the operation has completed before returning"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Searchable Snapshots Mount API that can be awaited"]
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
                storage: Option<&'b str>,
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                storage: self.storage,
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Searchable Snapshots Repository Stats API"]
pub enum SearchableSnapshotsRepositoryStatsParts<'b> {
    #[doc = "Repository"]
    Repository(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> SearchableSnapshotsRepositoryStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Searchable Snapshots Repository Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchableSnapshotsRepositoryStatsParts::Repository(ref repository) => {
                let encoded_repository: Cow<str> =
                    percent_encode(repository.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_repository.len());
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.push_str("/_stats");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Searchable Snapshots Repository Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/searchable-snapshots-apis.html)\n\nDEPRECATED: This API is replaced by the Repositories Metering API."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct SearchableSnapshotsRepositoryStats<'a, 'b> {
    transport: &'a Transport,
    parts: SearchableSnapshotsRepositoryStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> SearchableSnapshotsRepositoryStats<'a, 'b> {
    #[doc = "Creates a new instance of [SearchableSnapshotsRepositoryStats] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: SearchableSnapshotsRepositoryStatsParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        SearchableSnapshotsRepositoryStats {
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
    #[doc = "Creates an asynchronous call to the Searchable Snapshots Repository Stats API that can be awaited"]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Searchable Snapshots Stats API"]
pub enum SearchableSnapshotsStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
#[cfg(feature = "experimental-apis")]
impl<'b> SearchableSnapshotsStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Searchable Snapshots Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchableSnapshotsStatsParts::None => "/_searchable_snapshots/stats".into(),
            SearchableSnapshotsStatsParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_searchable_snapshots/stats");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Searchable Snapshots Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/searchable-snapshots-apis.html)\n\nRetrieve various statistics about searchable snapshots."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct SearchableSnapshotsStats<'a, 'b> {
    transport: &'a Transport,
    parts: SearchableSnapshotsStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    level: Option<Level>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> SearchableSnapshotsStats<'a, 'b> {
    #[doc = "Creates a new instance of [SearchableSnapshotsStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchableSnapshotsStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchableSnapshotsStats {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            level: None,
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
    #[doc = "Creates an asynchronous call to the Searchable Snapshots Stats API that can be awaited"]
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
                level: Option<Level>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                level: self.level,
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
#[doc = "Namespace client for SearchableSnapshots APIs"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
pub struct SearchableSnapshots<'a> {
    transport: &'a Transport,
}
#[cfg(feature = "experimental-apis")]
impl<'a> SearchableSnapshots<'a> {
    #[doc = "Creates a new instance of [SearchableSnapshots]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Searchable Snapshots Clear Cache API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/searchable-snapshots-apis.html)\n\nClear the cache of searchable snapshots."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn clear_cache<'b>(
        &'a self,
        parts: SearchableSnapshotsClearCacheParts<'b>,
    ) -> SearchableSnapshotsClearCache<'a, 'b, ()> {
        SearchableSnapshotsClearCache::new(self.transport(), parts)
    }
    #[doc = "[Searchable Snapshots Mount API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/searchable-snapshots-api-mount-snapshot.html)\n\nMount a snapshot as a searchable index."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn mount<'b>(
        &'a self,
        parts: SearchableSnapshotsMountParts<'b>,
    ) -> SearchableSnapshotsMount<'a, 'b, ()> {
        SearchableSnapshotsMount::new(self.transport(), parts)
    }
    #[doc = "[Searchable Snapshots Repository Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/searchable-snapshots-apis.html)\n\nDEPRECATED: This API is replaced by the Repositories Metering API."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn repository_stats<'b>(
        &'a self,
        parts: SearchableSnapshotsRepositoryStatsParts<'b>,
    ) -> SearchableSnapshotsRepositoryStats<'a, 'b> {
        SearchableSnapshotsRepositoryStats::new(self.transport(), parts)
    }
    #[doc = "[Searchable Snapshots Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/searchable-snapshots-apis.html)\n\nRetrieve various statistics about searchable snapshots."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn stats<'b>(
        &'a self,
        parts: SearchableSnapshotsStatsParts<'b>,
    ) -> SearchableSnapshotsStats<'a, 'b> {
        SearchableSnapshotsStats::new(self.transport(), parts)
    }
}
#[cfg(feature = "experimental-apis")]
impl Elasticsearch {
    #[doc = "Creates a namespace client for SearchableSnapshots APIs"]
    pub fn searchable_snapshots(&self) -> SearchableSnapshots {
        SearchableSnapshots::new(self.transport())
    }
}
