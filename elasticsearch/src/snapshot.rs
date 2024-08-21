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

//! Snapshot APIs
//!
//! [Manage snapshots taken from a running Elasticsearch cluster](https://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html).
//! A snapshot is a backup of individual
//! indices or the entire cluster, stored in a repository on a shared filesystem or a remote repository
//! on S3, HDFS, Azure, Google Cloud storage, and more.

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
#[doc = "API parts for the Snapshot Cleanup Repository API"]
pub enum SnapshotCleanupRepositoryParts<'b> {
    #[doc = "Repository"]
    Repository(&'b str),
}
impl<'b> SnapshotCleanupRepositoryParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Cleanup Repository API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotCleanupRepositoryParts::Repository(repository) => {
                let encoded_repository: Cow<str> =
                    percent_encode(repository.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_repository.len());
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.push_str("/_cleanup");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Snapshot Cleanup Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/clean-up-snapshot-repo-api.html)\n\nRemoves stale data from repository."]
#[derive(Clone, Debug)]
pub struct SnapshotCleanupRepository<'a, 'b, B> {
    transport: &'a Transport,
    parts: SnapshotCleanupRepositoryParts<'b>,
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
impl<'a, 'b, B> SnapshotCleanupRepository<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotCleanupRepository] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SnapshotCleanupRepositoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotCleanupRepository {
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
    pub fn body<T>(self, body: T) -> SnapshotCleanupRepository<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SnapshotCleanupRepository {
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
    #[doc = "Creates an asynchronous call to the Snapshot Cleanup Repository API that can be awaited"]
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
#[doc = "API parts for the Snapshot Clone API"]
pub enum SnapshotCloneParts<'b> {
    #[doc = "Repository, Snapshot and TargetSnapshot"]
    RepositorySnapshotTargetSnapshot(&'b str, &'b str, &'b str),
}
impl<'b> SnapshotCloneParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Clone API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotCloneParts::RepositorySnapshotTargetSnapshot(
                repository,
                snapshot,
                target_snapshot,
            ) => {
                let encoded_repository: Cow<str> =
                    percent_encode(repository.as_bytes(), PARTS_ENCODED).into();
                let encoded_snapshot: Cow<str> =
                    percent_encode(snapshot.as_bytes(), PARTS_ENCODED).into();
                let encoded_target_snapshot: Cow<str> =
                    percent_encode(target_snapshot.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    20usize
                        + encoded_repository.len()
                        + encoded_snapshot.len()
                        + encoded_target_snapshot.len(),
                );
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.push('/');
                p.push_str(encoded_snapshot.as_ref());
                p.push_str("/_clone/");
                p.push_str(encoded_target_snapshot.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Snapshot Clone API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nClones indices from one snapshot into another snapshot in the same repository."]
#[derive(Clone, Debug)]
pub struct SnapshotClone<'a, 'b, B> {
    transport: &'a Transport,
    parts: SnapshotCloneParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SnapshotClone<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotClone] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SnapshotCloneParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotClone {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SnapshotClone<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SnapshotClone {
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
    #[doc = "Creates an asynchronous call to the Snapshot Clone API that can be awaited"]
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Snapshot Create API"]
pub enum SnapshotCreateParts<'b> {
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'b str, &'b str),
}
impl<'b> SnapshotCreateParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Create API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotCreateParts::RepositorySnapshot(repository, snapshot) => {
                let encoded_repository: Cow<str> =
                    percent_encode(repository.as_bytes(), PARTS_ENCODED).into();
                let encoded_snapshot: Cow<str> =
                    percent_encode(snapshot.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    12usize + encoded_repository.len() + encoded_snapshot.len(),
                );
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.push('/');
                p.push_str(encoded_snapshot.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Snapshot Create API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nCreates a snapshot in a repository."]
#[derive(Clone, Debug)]
pub struct SnapshotCreate<'a, 'b, B> {
    transport: &'a Transport,
    parts: SnapshotCreateParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> SnapshotCreate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotCreate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SnapshotCreateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotCreate {
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
            wait_for_completion: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SnapshotCreate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SnapshotCreate {
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
    #[doc = "Should this request wait until the operation has completed before returning"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Create API that can be awaited"]
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
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Snapshot Create Repository API"]
pub enum SnapshotCreateRepositoryParts<'b> {
    #[doc = "Repository"]
    Repository(&'b str),
}
impl<'b> SnapshotCreateRepositoryParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Create Repository API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotCreateRepositoryParts::Repository(repository) => {
                let encoded_repository: Cow<str> =
                    percent_encode(repository.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_repository.len());
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Snapshot Create Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nCreates a repository."]
#[derive(Clone, Debug)]
pub struct SnapshotCreateRepository<'a, 'b, B> {
    transport: &'a Transport,
    parts: SnapshotCreateRepositoryParts<'b>,
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
    verify: Option<bool>,
}
impl<'a, 'b, B> SnapshotCreateRepository<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotCreateRepository] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SnapshotCreateRepositoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotCreateRepository {
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
            verify: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SnapshotCreateRepository<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SnapshotCreateRepository {
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
            verify: self.verify,
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
    #[doc = "Whether to verify the repository after creation"]
    pub fn verify(mut self, verify: bool) -> Self {
        self.verify = Some(verify);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Create Repository API that can be awaited"]
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
                verify: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                verify: self.verify,
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
#[doc = "API parts for the Snapshot Delete API"]
pub enum SnapshotDeleteParts<'b> {
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'b str, &'b [&'b str]),
}
impl<'b> SnapshotDeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotDeleteParts::RepositorySnapshot(repository, snapshot) => {
                let snapshot_str = snapshot.join(",");
                let encoded_repository: Cow<str> =
                    percent_encode(repository.as_bytes(), PARTS_ENCODED).into();
                let encoded_snapshot: Cow<str> =
                    percent_encode(snapshot_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    12usize + encoded_repository.len() + encoded_snapshot.len(),
                );
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.push('/');
                p.push_str(encoded_snapshot.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Snapshot Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nDeletes one or more snapshots."]
#[derive(Clone, Debug)]
pub struct SnapshotDelete<'a, 'b> {
    transport: &'a Transport,
    parts: SnapshotDeleteParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b> SnapshotDelete<'a, 'b> {
    #[doc = "Creates a new instance of [SnapshotDelete] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SnapshotDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotDelete {
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
            wait_for_completion: None,
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
    #[doc = "Should this request wait until the operation has completed before returning"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Delete API that can be awaited"]
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
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                wait_for_completion: self.wait_for_completion,
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
#[doc = "API parts for the Snapshot Delete Repository API"]
pub enum SnapshotDeleteRepositoryParts<'b> {
    #[doc = "Repository"]
    Repository(&'b [&'b str]),
}
impl<'b> SnapshotDeleteRepositoryParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Delete Repository API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotDeleteRepositoryParts::Repository(repository) => {
                let repository_str = repository.join(",");
                let encoded_repository: Cow<str> =
                    percent_encode(repository_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_repository.len());
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Snapshot Delete Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nDeletes a repository."]
#[derive(Clone, Debug)]
pub struct SnapshotDeleteRepository<'a, 'b> {
    transport: &'a Transport,
    parts: SnapshotDeleteRepositoryParts<'b>,
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
impl<'a, 'b> SnapshotDeleteRepository<'a, 'b> {
    #[doc = "Creates a new instance of [SnapshotDeleteRepository] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SnapshotDeleteRepositoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotDeleteRepository {
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
    #[doc = "Creates an asynchronous call to the Snapshot Delete Repository API that can be awaited"]
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Snapshot Get API"]
pub enum SnapshotGetParts<'b> {
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'b str, &'b [&'b str]),
}
impl<'b> SnapshotGetParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotGetParts::RepositorySnapshot(repository, snapshot) => {
                let snapshot_str = snapshot.join(",");
                let encoded_repository: Cow<str> =
                    percent_encode(repository.as_bytes(), PARTS_ENCODED).into();
                let encoded_snapshot: Cow<str> =
                    percent_encode(snapshot_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    12usize + encoded_repository.len() + encoded_snapshot.len(),
                );
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.push('/');
                p.push_str(encoded_snapshot.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Snapshot Get API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nReturns information about a snapshot."]
#[derive(Clone, Debug)]
pub struct SnapshotGet<'a, 'b> {
    transport: &'a Transport,
    parts: SnapshotGetParts<'b>,
    after: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from_sort_value: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_repository: Option<bool>,
    index_details: Option<bool>,
    index_names: Option<bool>,
    master_timeout: Option<&'b str>,
    offset: Option<&'b str>,
    order: Option<Order>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    size: Option<&'b str>,
    slm_policy_filter: Option<&'b str>,
    sort: Option<Sort>,
    source: Option<&'b str>,
    verbose: Option<bool>,
}
impl<'a, 'b> SnapshotGet<'a, 'b> {
    #[doc = "Creates a new instance of [SnapshotGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SnapshotGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotGet {
            transport,
            parts,
            headers,
            after: None,
            error_trace: None,
            filter_path: None,
            from_sort_value: None,
            human: None,
            ignore_unavailable: None,
            include_repository: None,
            index_details: None,
            index_names: None,
            master_timeout: None,
            offset: None,
            order: None,
            pretty: None,
            request_timeout: None,
            size: None,
            slm_policy_filter: None,
            sort: None,
            source: None,
            verbose: None,
        }
    }
    #[doc = "Offset identifier to start pagination from as returned by the 'next' field in the response body."]
    pub fn after(mut self, after: &'b str) -> Self {
        self.after = Some(after);
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
    #[doc = "Value of the current sort column at which to start retrieval."]
    pub fn from_sort_value(mut self, from_sort_value: &'b str) -> Self {
        self.from_sort_value = Some(from_sort_value);
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
    #[doc = "Whether to ignore unavailable snapshots, defaults to false which means a SnapshotMissingException is thrown"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether to include the repository name in the snapshot info. Defaults to true."]
    pub fn include_repository(mut self, include_repository: bool) -> Self {
        self.include_repository = Some(include_repository);
        self
    }
    #[doc = "Whether to include details of each index in the snapshot, if those details are available. Defaults to false."]
    pub fn index_details(mut self, index_details: bool) -> Self {
        self.index_details = Some(index_details);
        self
    }
    #[doc = "Whether to include the name of each index in the snapshot. Defaults to true."]
    pub fn index_names(mut self, index_names: bool) -> Self {
        self.index_names = Some(index_names);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Numeric offset to start pagination based on the snapshots matching the request. Defaults to 0"]
    pub fn offset(mut self, offset: &'b str) -> Self {
        self.offset = Some(offset);
        self
    }
    #[doc = "Sort order"]
    pub fn order(mut self, order: Order) -> Self {
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
    #[doc = "Maximum number of snapshots to return. Defaults to 0 which means return all that match without limit."]
    pub fn size(mut self, size: &'b str) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "Filter snapshots by a comma-separated list of SLM policy names that snapshots belong to. Accepts wildcards. Use the special pattern '_none' to match snapshots without an SLM policy"]
    pub fn slm_policy_filter(mut self, slm_policy_filter: &'b str) -> Self {
        self.slm_policy_filter = Some(slm_policy_filter);
        self
    }
    #[doc = "Allows setting a sort order for the result. Defaults to start_time"]
    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Whether to show verbose snapshot info or only show the basic info found in the repository index blob"]
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = Some(verbose);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Get API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                after: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                from_sort_value: Option<&'b str>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                include_repository: Option<bool>,
                index_details: Option<bool>,
                index_names: Option<bool>,
                master_timeout: Option<&'b str>,
                offset: Option<&'b str>,
                order: Option<Order>,
                pretty: Option<bool>,
                size: Option<&'b str>,
                slm_policy_filter: Option<&'b str>,
                sort: Option<Sort>,
                source: Option<&'b str>,
                verbose: Option<bool>,
            }
            let query_params = QueryParams {
                after: self.after,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from_sort_value: self.from_sort_value,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_repository: self.include_repository,
                index_details: self.index_details,
                index_names: self.index_names,
                master_timeout: self.master_timeout,
                offset: self.offset,
                order: self.order,
                pretty: self.pretty,
                size: self.size,
                slm_policy_filter: self.slm_policy_filter,
                sort: self.sort,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Snapshot Get Repository API"]
pub enum SnapshotGetRepositoryParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Repository"]
    Repository(&'b [&'b str]),
}
impl<'b> SnapshotGetRepositoryParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Get Repository API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotGetRepositoryParts::None => "/_snapshot".into(),
            SnapshotGetRepositoryParts::Repository(repository) => {
                let repository_str = repository.join(",");
                let encoded_repository: Cow<str> =
                    percent_encode(repository_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_repository.len());
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Snapshot Get Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nReturns information about a repository."]
#[derive(Clone, Debug)]
pub struct SnapshotGetRepository<'a, 'b> {
    transport: &'a Transport,
    parts: SnapshotGetRepositoryParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SnapshotGetRepository<'a, 'b> {
    #[doc = "Creates a new instance of [SnapshotGetRepository] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SnapshotGetRepositoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotGetRepository {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
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
    #[doc = "Creates an asynchronous call to the Snapshot Get Repository API that can be awaited"]
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
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Snapshot Repository Analyze API"]
pub enum SnapshotRepositoryAnalyzeParts<'b> {
    #[doc = "Repository"]
    Repository(&'b str),
}
impl<'b> SnapshotRepositoryAnalyzeParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Repository Analyze API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotRepositoryAnalyzeParts::Repository(repository) => {
                let encoded_repository: Cow<str> =
                    percent_encode(repository.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_repository.len());
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.push_str("/_analyze");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Snapshot Repository Analyze API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nAnalyzes a repository for correctness and performance"]
#[derive(Clone, Debug)]
pub struct SnapshotRepositoryAnalyze<'a, 'b, B> {
    transport: &'a Transport,
    parts: SnapshotRepositoryAnalyzeParts<'b>,
    blob_count: Option<i64>,
    body: Option<B>,
    concurrency: Option<i64>,
    detailed: Option<bool>,
    early_read_node_count: Option<i64>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    max_blob_size: Option<&'b str>,
    max_total_data_size: Option<&'b str>,
    pretty: Option<bool>,
    rare_action_probability: Option<i64>,
    rarely_abort_writes: Option<bool>,
    read_node_count: Option<i64>,
    request_timeout: Option<Duration>,
    seed: Option<i64>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> SnapshotRepositoryAnalyze<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotRepositoryAnalyze] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SnapshotRepositoryAnalyzeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotRepositoryAnalyze {
            transport,
            parts,
            headers,
            blob_count: None,
            body: None,
            concurrency: None,
            detailed: None,
            early_read_node_count: None,
            error_trace: None,
            filter_path: None,
            human: None,
            max_blob_size: None,
            max_total_data_size: None,
            pretty: None,
            rare_action_probability: None,
            rarely_abort_writes: None,
            read_node_count: None,
            request_timeout: None,
            seed: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Number of blobs to create during the test. Defaults to 100."]
    pub fn blob_count(mut self, blob_count: i64) -> Self {
        self.blob_count = Some(blob_count);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SnapshotRepositoryAnalyze<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SnapshotRepositoryAnalyze {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            blob_count: self.blob_count,
            concurrency: self.concurrency,
            detailed: self.detailed,
            early_read_node_count: self.early_read_node_count,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            max_blob_size: self.max_blob_size,
            max_total_data_size: self.max_total_data_size,
            pretty: self.pretty,
            rare_action_probability: self.rare_action_probability,
            rarely_abort_writes: self.rarely_abort_writes,
            read_node_count: self.read_node_count,
            request_timeout: self.request_timeout,
            seed: self.seed,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Number of operations to run concurrently during the test. Defaults to 10."]
    pub fn concurrency(mut self, concurrency: i64) -> Self {
        self.concurrency = Some(concurrency);
        self
    }
    #[doc = "Whether to return detailed results or a summary. Defaults to 'false' so that only the summary is returned."]
    pub fn detailed(mut self, detailed: bool) -> Self {
        self.detailed = Some(detailed);
        self
    }
    #[doc = "Number of nodes on which to perform an early read on a blob, i.e. before writing has completed. Early reads are rare actions so the 'rare_action_probability' parameter is also relevant. Defaults to 2."]
    pub fn early_read_node_count(mut self, early_read_node_count: i64) -> Self {
        self.early_read_node_count = Some(early_read_node_count);
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
    #[doc = "Maximum size of a blob to create during the test, e.g '1gb' or '100mb'. Defaults to '10mb'."]
    pub fn max_blob_size(mut self, max_blob_size: &'b str) -> Self {
        self.max_blob_size = Some(max_blob_size);
        self
    }
    #[doc = "Maximum total size of all blobs to create during the test, e.g '1tb' or '100gb'. Defaults to '1gb'."]
    pub fn max_total_data_size(mut self, max_total_data_size: &'b str) -> Self {
        self.max_total_data_size = Some(max_total_data_size);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Probability of taking a rare action such as an early read or an overwrite. Defaults to 0.02."]
    pub fn rare_action_probability(mut self, rare_action_probability: i64) -> Self {
        self.rare_action_probability = Some(rare_action_probability);
        self
    }
    #[doc = "Whether to rarely abort writes before they complete. Defaults to 'true'."]
    pub fn rarely_abort_writes(mut self, rarely_abort_writes: bool) -> Self {
        self.rarely_abort_writes = Some(rarely_abort_writes);
        self
    }
    #[doc = "Number of nodes on which to read a blob after writing. Defaults to 10."]
    pub fn read_node_count(mut self, read_node_count: i64) -> Self {
        self.read_node_count = Some(read_node_count);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "Seed for the random number generator used to create the test workload. Defaults to a random value."]
    pub fn seed(mut self, seed: i64) -> Self {
        self.seed = Some(seed);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout. Defaults to '30s'."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Repository Analyze API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                blob_count: Option<i64>,
                concurrency: Option<i64>,
                detailed: Option<bool>,
                early_read_node_count: Option<i64>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                max_blob_size: Option<&'b str>,
                max_total_data_size: Option<&'b str>,
                pretty: Option<bool>,
                rare_action_probability: Option<i64>,
                rarely_abort_writes: Option<bool>,
                read_node_count: Option<i64>,
                seed: Option<i64>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                blob_count: self.blob_count,
                concurrency: self.concurrency,
                detailed: self.detailed,
                early_read_node_count: self.early_read_node_count,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                max_blob_size: self.max_blob_size,
                max_total_data_size: self.max_total_data_size,
                pretty: self.pretty,
                rare_action_probability: self.rare_action_probability,
                rarely_abort_writes: self.rarely_abort_writes,
                read_node_count: self.read_node_count,
                seed: self.seed,
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
#[doc = "API parts for the Snapshot Restore API"]
pub enum SnapshotRestoreParts<'b> {
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'b str, &'b str),
}
impl<'b> SnapshotRestoreParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Restore API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotRestoreParts::RepositorySnapshot(repository, snapshot) => {
                let encoded_repository: Cow<str> =
                    percent_encode(repository.as_bytes(), PARTS_ENCODED).into();
                let encoded_snapshot: Cow<str> =
                    percent_encode(snapshot.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    21usize + encoded_repository.len() + encoded_snapshot.len(),
                );
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.push('/');
                p.push_str(encoded_snapshot.as_ref());
                p.push_str("/_restore");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Snapshot Restore API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nRestores a snapshot."]
#[derive(Clone, Debug)]
pub struct SnapshotRestore<'a, 'b, B> {
    transport: &'a Transport,
    parts: SnapshotRestoreParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> SnapshotRestore<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotRestore] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SnapshotRestoreParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotRestore {
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
            wait_for_completion: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SnapshotRestore<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SnapshotRestore {
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
    #[doc = "Should this request wait until the operation has completed before returning"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Restore API that can be awaited"]
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
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Snapshot Status API"]
pub enum SnapshotStatusParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Repository"]
    Repository(&'b str),
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'b str, &'b [&'b str]),
}
impl<'b> SnapshotStatusParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotStatusParts::None => "/_snapshot/_status".into(),
            SnapshotStatusParts::Repository(repository) => {
                let encoded_repository: Cow<str> =
                    percent_encode(repository.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_repository.len());
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.push_str("/_status");
                p.into()
            }
            SnapshotStatusParts::RepositorySnapshot(repository, snapshot) => {
                let snapshot_str = snapshot.join(",");
                let encoded_repository: Cow<str> =
                    percent_encode(repository.as_bytes(), PARTS_ENCODED).into();
                let encoded_snapshot: Cow<str> =
                    percent_encode(snapshot_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    20usize + encoded_repository.len() + encoded_snapshot.len(),
                );
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.push('/');
                p.push_str(encoded_snapshot.as_ref());
                p.push_str("/_status");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Snapshot Status API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nReturns information about the status of a snapshot."]
#[derive(Clone, Debug)]
pub struct SnapshotStatus<'a, 'b> {
    transport: &'a Transport,
    parts: SnapshotStatusParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SnapshotStatus<'a, 'b> {
    #[doc = "Creates a new instance of [SnapshotStatus] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SnapshotStatusParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotStatus {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
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
    #[doc = "Whether to ignore unavailable snapshots, defaults to false which means a SnapshotMissingException is thrown"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "Creates an asynchronous call to the Snapshot Status API that can be awaited"]
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
                ignore_unavailable: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
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
#[doc = "API parts for the Snapshot Verify Repository API"]
pub enum SnapshotVerifyRepositoryParts<'b> {
    #[doc = "Repository"]
    Repository(&'b str),
}
impl<'b> SnapshotVerifyRepositoryParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Verify Repository API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotVerifyRepositoryParts::Repository(repository) => {
                let encoded_repository: Cow<str> =
                    percent_encode(repository.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_repository.len());
                p.push_str("/_snapshot/");
                p.push_str(encoded_repository.as_ref());
                p.push_str("/_verify");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Snapshot Verify Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nVerifies a repository."]
#[derive(Clone, Debug)]
pub struct SnapshotVerifyRepository<'a, 'b, B> {
    transport: &'a Transport,
    parts: SnapshotVerifyRepositoryParts<'b>,
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
impl<'a, 'b, B> SnapshotVerifyRepository<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotVerifyRepository] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SnapshotVerifyRepositoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotVerifyRepository {
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
    pub fn body<T>(self, body: T) -> SnapshotVerifyRepository<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SnapshotVerifyRepository {
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
    #[doc = "Creates an asynchronous call to the Snapshot Verify Repository API that can be awaited"]
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
#[doc = "Namespace client for Snapshot APIs"]
pub struct Snapshot<'a> {
    transport: &'a Transport,
}
impl<'a> Snapshot<'a> {
    #[doc = "Creates a new instance of [Snapshot]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Snapshot Cleanup Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/clean-up-snapshot-repo-api.html)\n\nRemoves stale data from repository."]
    pub fn cleanup_repository<'b>(
        &'a self,
        parts: SnapshotCleanupRepositoryParts<'b>,
    ) -> SnapshotCleanupRepository<'a, 'b, ()> {
        SnapshotCleanupRepository::new(self.transport(), parts)
    }
    #[doc = "[Snapshot Clone API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nClones indices from one snapshot into another snapshot in the same repository."]
    pub fn clone<'b>(&'a self, parts: SnapshotCloneParts<'b>) -> SnapshotClone<'a, 'b, ()> {
        SnapshotClone::new(self.transport(), parts)
    }
    #[doc = "[Snapshot Create API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nCreates a snapshot in a repository."]
    pub fn create<'b>(&'a self, parts: SnapshotCreateParts<'b>) -> SnapshotCreate<'a, 'b, ()> {
        SnapshotCreate::new(self.transport(), parts)
    }
    #[doc = "[Snapshot Create Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nCreates a repository."]
    pub fn create_repository<'b>(
        &'a self,
        parts: SnapshotCreateRepositoryParts<'b>,
    ) -> SnapshotCreateRepository<'a, 'b, ()> {
        SnapshotCreateRepository::new(self.transport(), parts)
    }
    #[doc = "[Snapshot Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nDeletes one or more snapshots."]
    pub fn delete<'b>(&'a self, parts: SnapshotDeleteParts<'b>) -> SnapshotDelete<'a, 'b> {
        SnapshotDelete::new(self.transport(), parts)
    }
    #[doc = "[Snapshot Delete Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nDeletes a repository."]
    pub fn delete_repository<'b>(
        &'a self,
        parts: SnapshotDeleteRepositoryParts<'b>,
    ) -> SnapshotDeleteRepository<'a, 'b> {
        SnapshotDeleteRepository::new(self.transport(), parts)
    }
    #[doc = "[Snapshot Get API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nReturns information about a snapshot."]
    pub fn get<'b>(&'a self, parts: SnapshotGetParts<'b>) -> SnapshotGet<'a, 'b> {
        SnapshotGet::new(self.transport(), parts)
    }
    #[doc = "[Snapshot Get Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nReturns information about a repository."]
    pub fn get_repository<'b>(
        &'a self,
        parts: SnapshotGetRepositoryParts<'b>,
    ) -> SnapshotGetRepository<'a, 'b> {
        SnapshotGetRepository::new(self.transport(), parts)
    }
    #[doc = "[Snapshot Repository Analyze API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nAnalyzes a repository for correctness and performance"]
    pub fn repository_analyze<'b>(
        &'a self,
        parts: SnapshotRepositoryAnalyzeParts<'b>,
    ) -> SnapshotRepositoryAnalyze<'a, 'b, ()> {
        SnapshotRepositoryAnalyze::new(self.transport(), parts)
    }
    #[doc = "[Snapshot Restore API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nRestores a snapshot."]
    pub fn restore<'b>(&'a self, parts: SnapshotRestoreParts<'b>) -> SnapshotRestore<'a, 'b, ()> {
        SnapshotRestore::new(self.transport(), parts)
    }
    #[doc = "[Snapshot Status API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nReturns information about the status of a snapshot."]
    pub fn status<'b>(&'a self, parts: SnapshotStatusParts<'b>) -> SnapshotStatus<'a, 'b> {
        SnapshotStatus::new(self.transport(), parts)
    }
    #[doc = "[Snapshot Verify Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/modules-snapshots.html)\n\nVerifies a repository."]
    pub fn verify_repository<'b>(
        &'a self,
        parts: SnapshotVerifyRepositoryParts<'b>,
    ) -> SnapshotVerifyRepository<'a, 'b, ()> {
        SnapshotVerifyRepository::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Snapshot APIs"]
    pub fn snapshot(&self) -> Snapshot {
        Snapshot::new(self.transport())
    }
}
