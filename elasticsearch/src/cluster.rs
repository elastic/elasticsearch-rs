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

//! Cluster APIs
//!
//! [Manage settings](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster.html),
//! perform operations, and retrieve information about an Elasticsearch cluster.

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
#[doc = "API parts for the Cluster Allocation Explain API"]
pub enum ClusterAllocationExplainParts {
    #[doc = "No parts"]
    None,
}
impl ClusterAllocationExplainParts {
    #[doc = "Builds a relative URL path to the Cluster Allocation Explain API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterAllocationExplainParts::None => "/_cluster/allocation/explain".into(),
        }
    }
}
#[doc = "Builder for the [Cluster Allocation Explain API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-allocation-explain.html)\n\nProvides explanations for shard allocations in the cluster."]
#[derive(Clone, Debug)]
pub struct ClusterAllocationExplain<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClusterAllocationExplainParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    include_disk_info: Option<bool>,
    include_yes_decisions: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ClusterAllocationExplain<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterAllocationExplain]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterAllocationExplain {
            transport,
            parts: ClusterAllocationExplainParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            include_disk_info: None,
            include_yes_decisions: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClusterAllocationExplain<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterAllocationExplain {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            include_disk_info: self.include_disk_info,
            include_yes_decisions: self.include_yes_decisions,
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
    #[doc = "Return information about disk usage and shard sizes (default: false)"]
    pub fn include_disk_info(mut self, include_disk_info: bool) -> Self {
        self.include_disk_info = Some(include_disk_info);
        self
    }
    #[doc = "Return 'YES' decisions in explanation (default: false)"]
    pub fn include_yes_decisions(mut self, include_yes_decisions: bool) -> Self {
        self.include_yes_decisions = Some(include_yes_decisions);
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
    #[doc = "Creates an asynchronous call to the Cluster Allocation Explain API that can be awaited"]
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
                include_disk_info: Option<bool>,
                include_yes_decisions: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                include_disk_info: self.include_disk_info,
                include_yes_decisions: self.include_yes_decisions,
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
#[doc = "API parts for the Cluster Delete Component Template API"]
pub enum ClusterDeleteComponentTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> ClusterDeleteComponentTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Delete Component Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterDeleteComponentTemplateParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_component_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Delete Component Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-component-template.html)\n\nDeletes a component template"]
#[derive(Clone, Debug)]
pub struct ClusterDeleteComponentTemplate<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterDeleteComponentTemplateParts<'b>,
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
impl<'a, 'b> ClusterDeleteComponentTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterDeleteComponentTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterDeleteComponentTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterDeleteComponentTemplate {
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
    #[doc = "Creates an asynchronous call to the Cluster Delete Component Template API that can be awaited"]
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
#[doc = "API parts for the Cluster Delete Voting Config Exclusions API"]
pub enum ClusterDeleteVotingConfigExclusionsParts {
    #[doc = "No parts"]
    None,
}
impl ClusterDeleteVotingConfigExclusionsParts {
    #[doc = "Builds a relative URL path to the Cluster Delete Voting Config Exclusions API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterDeleteVotingConfigExclusionsParts::None => {
                "/_cluster/voting_config_exclusions".into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Delete Voting Config Exclusions API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/voting-config-exclusions.html)\n\nClears cluster voting config exclusions."]
#[derive(Clone, Debug)]
pub struct ClusterDeleteVotingConfigExclusions<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterDeleteVotingConfigExclusionsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_removal: Option<bool>,
}
impl<'a, 'b> ClusterDeleteVotingConfigExclusions<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterDeleteVotingConfigExclusions]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterDeleteVotingConfigExclusions {
            transport,
            parts: ClusterDeleteVotingConfigExclusionsParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_for_removal: None,
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
    #[doc = "Specifies whether to wait for all excluded nodes to be removed from the cluster before clearing the voting configuration exclusions list."]
    pub fn wait_for_removal(mut self, wait_for_removal: bool) -> Self {
        self.wait_for_removal = Some(wait_for_removal);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Delete Voting Config Exclusions API that can be awaited"]
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
                wait_for_removal: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                wait_for_removal: self.wait_for_removal,
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
#[doc = "API parts for the Cluster Exists Component Template API"]
pub enum ClusterExistsComponentTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> ClusterExistsComponentTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Exists Component Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterExistsComponentTemplateParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_component_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Exists Component Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-component-template.html)\n\nReturns information about whether a particular component template exist"]
#[derive(Clone, Debug)]
pub struct ClusterExistsComponentTemplate<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterExistsComponentTemplateParts<'b>,
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
impl<'a, 'b> ClusterExistsComponentTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterExistsComponentTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterExistsComponentTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterExistsComponentTemplate {
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
    #[doc = "Creates an asynchronous call to the Cluster Exists Component Template API that can be awaited"]
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Cluster Get Component Template API"]
pub enum ClusterGetComponentTemplateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> ClusterGetComponentTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Get Component Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterGetComponentTemplateParts::None => "/_component_template".into(),
            ClusterGetComponentTemplateParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_component_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Get Component Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-component-template.html)\n\nReturns one or more component templates"]
#[derive(Clone, Debug)]
pub struct ClusterGetComponentTemplate<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterGetComponentTemplateParts<'b>,
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
impl<'a, 'b> ClusterGetComponentTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterGetComponentTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterGetComponentTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterGetComponentTemplate {
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
    #[doc = "Creates an asynchronous call to the Cluster Get Component Template API that can be awaited"]
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Cluster Get Settings API"]
pub enum ClusterGetSettingsParts {
    #[doc = "No parts"]
    None,
}
impl ClusterGetSettingsParts {
    #[doc = "Builds a relative URL path to the Cluster Get Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterGetSettingsParts::None => "/_cluster/settings".into(),
        }
    }
}
#[doc = "Builder for the [Cluster Get Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-update-settings.html)\n\nReturns cluster settings."]
#[derive(Clone, Debug)]
pub struct ClusterGetSettings<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterGetSettingsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    include_defaults: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> ClusterGetSettings<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterGetSettings]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterGetSettings {
            transport,
            parts: ClusterGetSettingsParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            include_defaults: None,
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
    #[doc = "Whether to return all default clusters setting."]
    pub fn include_defaults(mut self, include_defaults: bool) -> Self {
        self.include_defaults = Some(include_defaults);
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
    #[doc = "Creates an asynchronous call to the Cluster Get Settings API that can be awaited"]
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
                include_defaults: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                include_defaults: self.include_defaults,
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
#[doc = "API parts for the Cluster Health API"]
pub enum ClusterHealthParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> ClusterHealthParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Health API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterHealthParts::None => "/_cluster/health".into(),
            ClusterHealthParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_index.len());
                p.push_str("/_cluster/health/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Health API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-health.html)\n\nReturns basic information about the health of the cluster."]
#[derive(Clone, Debug)]
pub struct ClusterHealth<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterHealthParts<'b>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    level: Option<Level>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
    wait_for_events: Option<WaitForEvents>,
    wait_for_no_initializing_shards: Option<bool>,
    wait_for_no_relocating_shards: Option<bool>,
    wait_for_nodes: Option<&'b str>,
    wait_for_status: Option<WaitForStatus>,
}
impl<'a, 'b> ClusterHealth<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterHealth] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterHealthParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterHealth {
            transport,
            parts,
            headers,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            level: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
            wait_for_events: None,
            wait_for_no_initializing_shards: None,
            wait_for_no_relocating_shards: None,
            wait_for_nodes: None,
            wait_for_status: None,
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
    #[doc = "Specify the level of detail for returned information"]
    pub fn level(mut self, level: Level) -> Self {
        self.level = Some(level);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Wait until the specified number of shards is active"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Wait until all currently queued events with the given priority are processed"]
    pub fn wait_for_events(mut self, wait_for_events: WaitForEvents) -> Self {
        self.wait_for_events = Some(wait_for_events);
        self
    }
    #[doc = "Whether to wait until there are no initializing shards in the cluster"]
    pub fn wait_for_no_initializing_shards(
        mut self,
        wait_for_no_initializing_shards: bool,
    ) -> Self {
        self.wait_for_no_initializing_shards = Some(wait_for_no_initializing_shards);
        self
    }
    #[doc = "Whether to wait until there are no relocating shards in the cluster"]
    pub fn wait_for_no_relocating_shards(mut self, wait_for_no_relocating_shards: bool) -> Self {
        self.wait_for_no_relocating_shards = Some(wait_for_no_relocating_shards);
        self
    }
    #[doc = "Wait until the specified number of nodes is available"]
    pub fn wait_for_nodes(mut self, wait_for_nodes: &'b str) -> Self {
        self.wait_for_nodes = Some(wait_for_nodes);
        self
    }
    #[doc = "Wait until cluster is in a specific state"]
    pub fn wait_for_status(mut self, wait_for_status: WaitForStatus) -> Self {
        self.wait_for_status = Some(wait_for_status);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Health API that can be awaited"]
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
                level: Option<Level>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
                wait_for_events: Option<WaitForEvents>,
                wait_for_no_initializing_shards: Option<bool>,
                wait_for_no_relocating_shards: Option<bool>,
                wait_for_nodes: Option<&'b str>,
                wait_for_status: Option<WaitForStatus>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                level: self.level,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
                wait_for_events: self.wait_for_events,
                wait_for_no_initializing_shards: self.wait_for_no_initializing_shards,
                wait_for_no_relocating_shards: self.wait_for_no_relocating_shards,
                wait_for_nodes: self.wait_for_nodes,
                wait_for_status: self.wait_for_status,
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
#[doc = "API parts for the Cluster Pending Tasks API"]
pub enum ClusterPendingTasksParts {
    #[doc = "No parts"]
    None,
}
impl ClusterPendingTasksParts {
    #[doc = "Builds a relative URL path to the Cluster Pending Tasks API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterPendingTasksParts::None => "/_cluster/pending_tasks".into(),
        }
    }
}
#[doc = "Builder for the [Cluster Pending Tasks API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-pending.html)\n\nReturns a list of any cluster-level changes (e.g. create index, update mapping,\nallocate or fail shard) which have not yet been executed."]
#[derive(Clone, Debug)]
pub struct ClusterPendingTasks<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterPendingTasksParts,
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
impl<'a, 'b> ClusterPendingTasks<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterPendingTasks]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterPendingTasks {
            transport,
            parts: ClusterPendingTasksParts::None,
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
    #[doc = "Creates an asynchronous call to the Cluster Pending Tasks API that can be awaited"]
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Cluster Post Voting Config Exclusions API"]
pub enum ClusterPostVotingConfigExclusionsParts {
    #[doc = "No parts"]
    None,
}
impl ClusterPostVotingConfigExclusionsParts {
    #[doc = "Builds a relative URL path to the Cluster Post Voting Config Exclusions API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterPostVotingConfigExclusionsParts::None => {
                "/_cluster/voting_config_exclusions".into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Post Voting Config Exclusions API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/voting-config-exclusions.html)\n\nUpdates the cluster voting config exclusions by node ids or node names."]
#[derive(Clone, Debug)]
pub struct ClusterPostVotingConfigExclusions<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClusterPostVotingConfigExclusionsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    node_ids: Option<&'b str>,
    node_names: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> ClusterPostVotingConfigExclusions<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterPostVotingConfigExclusions]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterPostVotingConfigExclusions {
            transport,
            parts: ClusterPostVotingConfigExclusionsParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            node_ids: None,
            node_names: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClusterPostVotingConfigExclusions<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterPostVotingConfigExclusions {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            node_ids: self.node_ids,
            node_names: self.node_names,
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
    #[doc = "A comma-separated list of the persistent ids of the nodes to exclude from the voting configuration. If specified, you may not also specify ?node_names."]
    pub fn node_ids(mut self, node_ids: &'b str) -> Self {
        self.node_ids = Some(node_ids);
        self
    }
    #[doc = "A comma-separated list of the names of the nodes to exclude from the voting configuration. If specified, you may not also specify ?node_ids."]
    pub fn node_names(mut self, node_names: &'b str) -> Self {
        self.node_names = Some(node_names);
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
    #[doc = "Creates an asynchronous call to the Cluster Post Voting Config Exclusions API that can be awaited"]
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
                node_ids: Option<&'b str>,
                node_names: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                node_ids: self.node_ids,
                node_names: self.node_names,
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
#[doc = "API parts for the Cluster Put Component Template API"]
pub enum ClusterPutComponentTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> ClusterPutComponentTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Put Component Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterPutComponentTemplateParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_component_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Put Component Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-component-template.html)\n\nCreates or updates a component template"]
#[derive(Clone, Debug)]
pub struct ClusterPutComponentTemplate<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClusterPutComponentTemplateParts<'b>,
    body: Option<B>,
    create: Option<bool>,
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
impl<'a, 'b, B> ClusterPutComponentTemplate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterPutComponentTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterPutComponentTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterPutComponentTemplate {
            transport,
            parts,
            headers,
            body: None,
            create: None,
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
    pub fn body<T>(self, body: T) -> ClusterPutComponentTemplate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterPutComponentTemplate {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            create: self.create,
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Put Component Template API that can be awaited"]
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
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                create: self.create,
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
#[doc = "API parts for the Cluster Put Settings API"]
pub enum ClusterPutSettingsParts {
    #[doc = "No parts"]
    None,
}
impl ClusterPutSettingsParts {
    #[doc = "Builds a relative URL path to the Cluster Put Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterPutSettingsParts::None => "/_cluster/settings".into(),
        }
    }
}
#[doc = "Builder for the [Cluster Put Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-update-settings.html)\n\nUpdates the cluster settings."]
#[derive(Clone, Debug)]
pub struct ClusterPutSettings<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClusterPutSettingsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> ClusterPutSettings<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterPutSettings]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterPutSettings {
            transport,
            parts: ClusterPutSettingsParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClusterPutSettings<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterPutSettings {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            flat_settings: self.flat_settings,
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
    #[doc = "Creates an asynchronous call to the Cluster Put Settings API that can be awaited"]
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
                flat_settings: Option<bool>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
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
#[doc = "API parts for the Cluster Remote Info API"]
pub enum ClusterRemoteInfoParts {
    #[doc = "No parts"]
    None,
}
impl ClusterRemoteInfoParts {
    #[doc = "Builds a relative URL path to the Cluster Remote Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterRemoteInfoParts::None => "/_remote/info".into(),
        }
    }
}
#[doc = "Builder for the [Cluster Remote Info API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-remote-info.html)\n\nReturns the information about configured remote clusters."]
#[derive(Clone, Debug)]
pub struct ClusterRemoteInfo<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterRemoteInfoParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ClusterRemoteInfo<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterRemoteInfo]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterRemoteInfo {
            transport,
            parts: ClusterRemoteInfoParts::None,
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
    #[doc = "Creates an asynchronous call to the Cluster Remote Info API that can be awaited"]
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
#[doc = "API parts for the Cluster Reroute API"]
pub enum ClusterRerouteParts {
    #[doc = "No parts"]
    None,
}
impl ClusterRerouteParts {
    #[doc = "Builds a relative URL path to the Cluster Reroute API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterRerouteParts::None => "/_cluster/reroute".into(),
        }
    }
}
#[doc = "Builder for the [Cluster Reroute API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-reroute.html)\n\nAllows to manually change the allocation of individual shards in the cluster."]
#[derive(Clone, Debug)]
pub struct ClusterReroute<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClusterRerouteParts,
    body: Option<B>,
    dry_run: Option<bool>,
    error_trace: Option<bool>,
    explain: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    metric: Option<&'b [&'b str]>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    retry_failed: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> ClusterReroute<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterReroute]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterReroute {
            transport,
            parts: ClusterRerouteParts::None,
            headers,
            body: None,
            dry_run: None,
            error_trace: None,
            explain: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            metric: None,
            pretty: None,
            request_timeout: None,
            retry_failed: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClusterReroute<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterReroute {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            dry_run: self.dry_run,
            error_trace: self.error_trace,
            explain: self.explain,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            metric: self.metric,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            retry_failed: self.retry_failed,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Simulate the operation only and return the resulting state"]
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = Some(dry_run);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Return an explanation of why the commands can or cannot be executed"]
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
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Limit the information returned to the specified metrics. Defaults to all but metadata"]
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
    #[doc = "Retries allocation of shards that are blocked due to too many subsequent allocation failures"]
    pub fn retry_failed(mut self, retry_failed: bool) -> Self {
        self.retry_failed = Some(retry_failed);
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
    #[doc = "Creates an asynchronous call to the Cluster Reroute API that can be awaited"]
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
                explain: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                metric: Option<&'b [&'b str]>,
                pretty: Option<bool>,
                retry_failed: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                dry_run: self.dry_run,
                error_trace: self.error_trace,
                explain: self.explain,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                metric: self.metric,
                pretty: self.pretty,
                retry_failed: self.retry_failed,
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
#[doc = "API parts for the Cluster State API"]
pub enum ClusterStateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Metric"]
    Metric(&'b [&'b str]),
    #[doc = "Metric and Index"]
    MetricIndex(&'b [&'b str], &'b [&'b str]),
}
impl<'b> ClusterStateParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster State API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterStateParts::None => "/_cluster/state".into(),
            ClusterStateParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_metric.len());
                p.push_str("/_cluster/state/");
                p.push_str(encoded_metric.as_ref());
                p.into()
            }
            ClusterStateParts::MetricIndex(ref metric, ref index) => {
                let metric_str = metric.join(",");
                let index_str = index.join(",");
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(17usize + encoded_metric.len() + encoded_index.len());
                p.push_str("/_cluster/state/");
                p.push_str(encoded_metric.as_ref());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster State API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-state.html)\n\nReturns a comprehensive information about the state of the cluster."]
#[derive(Clone, Debug)]
pub struct ClusterState<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterStateParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_metadata_version: Option<i64>,
    wait_for_timeout: Option<&'b str>,
}
impl<'a, 'b> ClusterState<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterState] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterStateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterState {
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
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_for_metadata_version: None,
            wait_for_timeout: None,
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
    #[doc = "Wait for the metadata version to be equal or greater than the specified metadata version"]
    pub fn wait_for_metadata_version(mut self, wait_for_metadata_version: i64) -> Self {
        self.wait_for_metadata_version = Some(wait_for_metadata_version);
        self
    }
    #[doc = "The maximum time to wait for wait_for_metadata_version before timing out"]
    pub fn wait_for_timeout(mut self, wait_for_timeout: &'b str) -> Self {
        self.wait_for_timeout = Some(wait_for_timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster State API that can be awaited"]
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
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                wait_for_metadata_version: Option<i64>,
                wait_for_timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                wait_for_metadata_version: self.wait_for_metadata_version,
                wait_for_timeout: self.wait_for_timeout,
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
#[doc = "API parts for the Cluster Stats API"]
pub enum ClusterStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "NodeId"]
    NodeId(&'b [&'b str]),
}
impl<'b> ClusterStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterStatsParts::None => "/_cluster/stats".into(),
            ClusterStatsParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_node_id.len());
                p.push_str("/_cluster/stats/nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-stats.html)\n\nReturns high-level overview of cluster statistics."]
#[derive(Clone, Debug)]
pub struct ClusterStats<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> ClusterStats<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterStats {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
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
    #[doc = "Creates an asynchronous call to the Cluster Stats API that can be awaited"]
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
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
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
#[doc = "Namespace client for Cluster APIs"]
pub struct Cluster<'a> {
    transport: &'a Transport,
}
impl<'a> Cluster<'a> {
    #[doc = "Creates a new instance of [Cluster]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Cluster Allocation Explain API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-allocation-explain.html)\n\nProvides explanations for shard allocations in the cluster."]
    pub fn allocation_explain<'b>(&'a self) -> ClusterAllocationExplain<'a, 'b, ()> {
        ClusterAllocationExplain::new(self.transport())
    }
    #[doc = "[Cluster Delete Component Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-component-template.html)\n\nDeletes a component template"]
    pub fn delete_component_template<'b>(
        &'a self,
        parts: ClusterDeleteComponentTemplateParts<'b>,
    ) -> ClusterDeleteComponentTemplate<'a, 'b> {
        ClusterDeleteComponentTemplate::new(self.transport(), parts)
    }
    #[doc = "[Cluster Delete Voting Config Exclusions API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/voting-config-exclusions.html)\n\nClears cluster voting config exclusions."]
    pub fn delete_voting_config_exclusions<'b>(
        &'a self,
    ) -> ClusterDeleteVotingConfigExclusions<'a, 'b> {
        ClusterDeleteVotingConfigExclusions::new(self.transport())
    }
    #[doc = "[Cluster Exists Component Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-component-template.html)\n\nReturns information about whether a particular component template exist"]
    pub fn exists_component_template<'b>(
        &'a self,
        parts: ClusterExistsComponentTemplateParts<'b>,
    ) -> ClusterExistsComponentTemplate<'a, 'b> {
        ClusterExistsComponentTemplate::new(self.transport(), parts)
    }
    #[doc = "[Cluster Get Component Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-component-template.html)\n\nReturns one or more component templates"]
    pub fn get_component_template<'b>(
        &'a self,
        parts: ClusterGetComponentTemplateParts<'b>,
    ) -> ClusterGetComponentTemplate<'a, 'b> {
        ClusterGetComponentTemplate::new(self.transport(), parts)
    }
    #[doc = "[Cluster Get Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-update-settings.html)\n\nReturns cluster settings."]
    pub fn get_settings<'b>(&'a self) -> ClusterGetSettings<'a, 'b> {
        ClusterGetSettings::new(self.transport())
    }
    #[doc = "[Cluster Health API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-health.html)\n\nReturns basic information about the health of the cluster."]
    pub fn health<'b>(&'a self, parts: ClusterHealthParts<'b>) -> ClusterHealth<'a, 'b> {
        ClusterHealth::new(self.transport(), parts)
    }
    #[doc = "[Cluster Pending Tasks API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-pending.html)\n\nReturns a list of any cluster-level changes (e.g. create index, update mapping,\nallocate or fail shard) which have not yet been executed."]
    pub fn pending_tasks<'b>(&'a self) -> ClusterPendingTasks<'a, 'b> {
        ClusterPendingTasks::new(self.transport())
    }
    #[doc = "[Cluster Post Voting Config Exclusions API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/voting-config-exclusions.html)\n\nUpdates the cluster voting config exclusions by node ids or node names."]
    pub fn post_voting_config_exclusions<'b>(
        &'a self,
    ) -> ClusterPostVotingConfigExclusions<'a, 'b, ()> {
        ClusterPostVotingConfigExclusions::new(self.transport())
    }
    #[doc = "[Cluster Put Component Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/indices-component-template.html)\n\nCreates or updates a component template"]
    pub fn put_component_template<'b>(
        &'a self,
        parts: ClusterPutComponentTemplateParts<'b>,
    ) -> ClusterPutComponentTemplate<'a, 'b, ()> {
        ClusterPutComponentTemplate::new(self.transport(), parts)
    }
    #[doc = "[Cluster Put Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-update-settings.html)\n\nUpdates the cluster settings."]
    pub fn put_settings<'b>(&'a self) -> ClusterPutSettings<'a, 'b, ()> {
        ClusterPutSettings::new(self.transport())
    }
    #[doc = "[Cluster Remote Info API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-remote-info.html)\n\nReturns the information about configured remote clusters."]
    pub fn remote_info<'b>(&'a self) -> ClusterRemoteInfo<'a, 'b> {
        ClusterRemoteInfo::new(self.transport())
    }
    #[doc = "[Cluster Reroute API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-reroute.html)\n\nAllows to manually change the allocation of individual shards in the cluster."]
    pub fn reroute<'b>(&'a self) -> ClusterReroute<'a, 'b, ()> {
        ClusterReroute::new(self.transport())
    }
    #[doc = "[Cluster State API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-state.html)\n\nReturns a comprehensive information about the state of the cluster."]
    pub fn state<'b>(&'a self, parts: ClusterStateParts<'b>) -> ClusterState<'a, 'b> {
        ClusterState::new(self.transport(), parts)
    }
    #[doc = "[Cluster Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-stats.html)\n\nReturns high-level overview of cluster statistics."]
    pub fn stats<'b>(&'a self, parts: ClusterStatsParts<'b>) -> ClusterStats<'a, 'b> {
        ClusterStats::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Cluster APIs"]
    pub fn cluster(&self) -> Cluster {
        Cluster::new(self.transport())
    }
}
