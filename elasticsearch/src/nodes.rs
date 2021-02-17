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

//! Node APIs
//!
//! Manage settings, perform operations, and retrieve information about the
//! [nodes in an Elasticsearch cluster](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster.html).

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
#[doc = "API parts for the Nodes Hot Threads API"]
pub enum NodesHotThreadsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "NodeId"]
    NodeId(&'b [&'b str]),
}
impl<'b> NodesHotThreadsParts<'b> {
    #[doc = "Builds a relative URL path to the Nodes Hot Threads API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NodesHotThreadsParts::None => "/_nodes/hot_threads".into(),
            NodesHotThreadsParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_node_id.len());
                p.push_str("/_nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/hot_threads");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Nodes Hot Threads API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-nodes-hot-threads.html)\n\nReturns information about hot threads on each node in the cluster."]
#[derive(Clone, Debug)]
pub struct NodesHotThreads<'a, 'b> {
    transport: &'a Transport,
    parts: NodesHotThreadsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_idle_threads: Option<bool>,
    interval: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    snapshots: Option<i64>,
    source: Option<&'b str>,
    threads: Option<i64>,
    timeout: Option<&'b str>,
    ty: Option<Type>,
}
impl<'a, 'b> NodesHotThreads<'a, 'b> {
    #[doc = "Creates a new instance of [NodesHotThreads] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: NodesHotThreadsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        NodesHotThreads {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            ignore_idle_threads: None,
            interval: None,
            pretty: None,
            request_timeout: None,
            snapshots: None,
            source: None,
            threads: None,
            timeout: None,
            ty: None,
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
    #[doc = "Don't show threads that are in known-idle places, such as waiting on a socket select or pulling from an empty task queue (default: true)"]
    pub fn ignore_idle_threads(mut self, ignore_idle_threads: bool) -> Self {
        self.ignore_idle_threads = Some(ignore_idle_threads);
        self
    }
    #[doc = "The interval for the second sampling of threads"]
    pub fn interval(mut self, interval: &'b str) -> Self {
        self.interval = Some(interval);
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
    #[doc = "Number of samples of thread stacktrace (default: 10)"]
    pub fn snapshots(mut self, snapshots: i64) -> Self {
        self.snapshots = Some(snapshots);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specify the number of threads to provide information for (default: 3)"]
    pub fn threads(mut self, threads: i64) -> Self {
        self.threads = Some(threads);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "The type to sample (default: cpu)"]
    pub fn ty(mut self, ty: Type) -> Self {
        self.ty = Some(ty);
        self
    }
    #[doc = "Creates an asynchronous call to the Nodes Hot Threads API that can be awaited"]
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
                ignore_idle_threads: Option<bool>,
                interval: Option<&'b str>,
                pretty: Option<bool>,
                snapshots: Option<i64>,
                source: Option<&'b str>,
                threads: Option<i64>,
                timeout: Option<&'b str>,
                #[serde(rename = "type")]
                ty: Option<Type>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                ignore_idle_threads: self.ignore_idle_threads,
                interval: self.interval,
                pretty: self.pretty,
                snapshots: self.snapshots,
                source: self.source,
                threads: self.threads,
                timeout: self.timeout,
                ty: self.ty,
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
#[doc = "API parts for the Nodes Info API"]
pub enum NodesInfoParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "NodeId"]
    NodeId(&'b [&'b str]),
    #[doc = "Metric"]
    Metric(&'b [&'b str]),
    #[doc = "NodeId and Metric"]
    NodeIdMetric(&'b [&'b str], &'b [&'b str]),
}
impl<'b> NodesInfoParts<'b> {
    #[doc = "Builds a relative URL path to the Nodes Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NodesInfoParts::None => "/_nodes".into(),
            NodesInfoParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(8usize + encoded_node_id.len());
                p.push_str("/_nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.into()
            }
            NodesInfoParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(8usize + encoded_metric.len());
                p.push_str("/_nodes/");
                p.push_str(encoded_metric.as_ref());
                p.into()
            }
            NodesInfoParts::NodeIdMetric(ref node_id, ref metric) => {
                let node_id_str = node_id.join(",");
                let metric_str = metric.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(9usize + encoded_node_id.len() + encoded_metric.len());
                p.push_str("/_nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/");
                p.push_str(encoded_metric.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Nodes Info API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-nodes-info.html)\n\nReturns information about nodes in the cluster."]
#[derive(Clone, Debug)]
pub struct NodesInfo<'a, 'b> {
    transport: &'a Transport,
    parts: NodesInfoParts<'b>,
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
impl<'a, 'b> NodesInfo<'a, 'b> {
    #[doc = "Creates a new instance of [NodesInfo] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: NodesInfoParts<'b>) -> Self {
        let headers = HeaderMap::new();
        NodesInfo {
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
    #[doc = "Creates an asynchronous call to the Nodes Info API that can be awaited"]
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Nodes Reload Secure Settings API"]
pub enum NodesReloadSecureSettingsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "NodeId"]
    NodeId(&'b [&'b str]),
}
impl<'b> NodesReloadSecureSettingsParts<'b> {
    #[doc = "Builds a relative URL path to the Nodes Reload Secure Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NodesReloadSecureSettingsParts::None => "/_nodes/reload_secure_settings".into(),
            NodesReloadSecureSettingsParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(31usize + encoded_node_id.len());
                p.push_str("/_nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/reload_secure_settings");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Nodes Reload Secure Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/secure-settings.html#reloadable-secure-settings)\n\nReloads secure settings."]
#[derive(Clone, Debug)]
pub struct NodesReloadSecureSettings<'a, 'b, B> {
    transport: &'a Transport,
    parts: NodesReloadSecureSettingsParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> NodesReloadSecureSettings<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [NodesReloadSecureSettings] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: NodesReloadSecureSettingsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        NodesReloadSecureSettings {
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
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> NodesReloadSecureSettings<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        NodesReloadSecureSettings {
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
    #[doc = "Creates an asynchronous call to the Nodes Reload Secure Settings API that can be awaited"]
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
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
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
#[doc = "API parts for the Nodes Stats API"]
pub enum NodesStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "NodeId"]
    NodeId(&'b [&'b str]),
    #[doc = "Metric"]
    Metric(&'b [&'b str]),
    #[doc = "NodeId and Metric"]
    NodeIdMetric(&'b [&'b str], &'b [&'b str]),
    #[doc = "Metric and IndexMetric"]
    MetricIndexMetric(&'b [&'b str], &'b [&'b str]),
    #[doc = "NodeId, Metric and IndexMetric"]
    NodeIdMetricIndexMetric(&'b [&'b str], &'b [&'b str], &'b [&'b str]),
}
impl<'b> NodesStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Nodes Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NodesStatsParts::None => "/_nodes/stats".into(),
            NodesStatsParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_node_id.len());
                p.push_str("/_nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats");
                p.into()
            }
            NodesStatsParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_metric.len());
                p.push_str("/_nodes/stats/");
                p.push_str(encoded_metric.as_ref());
                p.into()
            }
            NodesStatsParts::NodeIdMetric(ref node_id, ref metric) => {
                let node_id_str = node_id.join(",");
                let metric_str = metric.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(15usize + encoded_node_id.len() + encoded_metric.len());
                p.push_str("/_nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats/");
                p.push_str(encoded_metric.as_ref());
                p.into()
            }
            NodesStatsParts::MetricIndexMetric(ref metric, ref index_metric) => {
                let metric_str = metric.join(",");
                let index_metric_str = index_metric.join(",");
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_index_metric: Cow<str> =
                    percent_encode(index_metric_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    15usize + encoded_metric.len() + encoded_index_metric.len(),
                );
                p.push_str("/_nodes/stats/");
                p.push_str(encoded_metric.as_ref());
                p.push_str("/");
                p.push_str(encoded_index_metric.as_ref());
                p.into()
            }
            NodesStatsParts::NodeIdMetricIndexMetric(ref node_id, ref metric, ref index_metric) => {
                let node_id_str = node_id.join(",");
                let metric_str = metric.join(",");
                let index_metric_str = index_metric.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_index_metric: Cow<str> =
                    percent_encode(index_metric_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    16usize
                        + encoded_node_id.len()
                        + encoded_metric.len()
                        + encoded_index_metric.len(),
                );
                p.push_str("/_nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats/");
                p.push_str(encoded_metric.as_ref());
                p.push_str("/");
                p.push_str(encoded_index_metric.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Nodes Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-nodes-stats.html)\n\nReturns statistical information about nodes in the cluster."]
#[derive(Clone, Debug)]
pub struct NodesStats<'a, 'b> {
    transport: &'a Transport,
    parts: NodesStatsParts<'b>,
    completion_fields: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    fielddata_fields: Option<&'b [&'b str]>,
    fields: Option<&'b [&'b str]>,
    filter_path: Option<&'b [&'b str]>,
    groups: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    include_segment_file_sizes: Option<bool>,
    level: Option<Level>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    types: Option<&'b [&'b str]>,
}
impl<'a, 'b> NodesStats<'a, 'b> {
    #[doc = "Creates a new instance of [NodesStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: NodesStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        NodesStats {
            transport,
            parts,
            headers,
            completion_fields: None,
            error_trace: None,
            fielddata_fields: None,
            fields: None,
            filter_path: None,
            groups: None,
            human: None,
            include_segment_file_sizes: None,
            level: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
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
    #[doc = "A comma-separated list of search groups for `search` index metric"]
    pub fn groups(mut self, groups: bool) -> Self {
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
    #[doc = "Return indices stats aggregated at index, node or shard level"]
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of document types for the `indexing` index metric"]
    pub fn types(mut self, types: &'b [&'b str]) -> Self {
        self.types = Some(types);
        self
    }
    #[doc = "Creates an asynchronous call to the Nodes Stats API that can be awaited"]
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
                fielddata_fields: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                groups: Option<bool>,
                human: Option<bool>,
                include_segment_file_sizes: Option<bool>,
                level: Option<Level>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                types: Option<&'b [&'b str]>,
            }
            let query_params = QueryParams {
                completion_fields: self.completion_fields,
                error_trace: self.error_trace,
                fielddata_fields: self.fielddata_fields,
                fields: self.fields,
                filter_path: self.filter_path,
                groups: self.groups,
                human: self.human,
                include_segment_file_sizes: self.include_segment_file_sizes,
                level: self.level,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Nodes Usage API"]
pub enum NodesUsageParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "NodeId"]
    NodeId(&'b [&'b str]),
    #[doc = "Metric"]
    Metric(&'b [&'b str]),
    #[doc = "NodeId and Metric"]
    NodeIdMetric(&'b [&'b str], &'b [&'b str]),
}
impl<'b> NodesUsageParts<'b> {
    #[doc = "Builds a relative URL path to the Nodes Usage API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NodesUsageParts::None => "/_nodes/usage".into(),
            NodesUsageParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_node_id.len());
                p.push_str("/_nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/usage");
                p.into()
            }
            NodesUsageParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_metric.len());
                p.push_str("/_nodes/usage/");
                p.push_str(encoded_metric.as_ref());
                p.into()
            }
            NodesUsageParts::NodeIdMetric(ref node_id, ref metric) => {
                let node_id_str = node_id.join(",");
                let metric_str = metric.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(15usize + encoded_node_id.len() + encoded_metric.len());
                p.push_str("/_nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/usage/");
                p.push_str(encoded_metric.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Nodes Usage API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-nodes-usage.html)\n\nReturns low-level information about REST actions usage on nodes."]
#[derive(Clone, Debug)]
pub struct NodesUsage<'a, 'b> {
    transport: &'a Transport,
    parts: NodesUsageParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> NodesUsage<'a, 'b> {
    #[doc = "Creates a new instance of [NodesUsage] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: NodesUsageParts<'b>) -> Self {
        let headers = HeaderMap::new();
        NodesUsage {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
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
    #[doc = "Creates an asynchronous call to the Nodes Usage API that can be awaited"]
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
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
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
#[doc = "Namespace client for Nodes APIs"]
pub struct Nodes<'a> {
    transport: &'a Transport,
}
impl<'a> Nodes<'a> {
    #[doc = "Creates a new instance of [Nodes]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Nodes Hot Threads API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-nodes-hot-threads.html)\n\nReturns information about hot threads on each node in the cluster."]
    pub fn hot_threads<'b>(&'a self, parts: NodesHotThreadsParts<'b>) -> NodesHotThreads<'a, 'b> {
        NodesHotThreads::new(self.transport(), parts)
    }
    #[doc = "[Nodes Info API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-nodes-info.html)\n\nReturns information about nodes in the cluster."]
    pub fn info<'b>(&'a self, parts: NodesInfoParts<'b>) -> NodesInfo<'a, 'b> {
        NodesInfo::new(self.transport(), parts)
    }
    #[doc = "[Nodes Reload Secure Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/secure-settings.html#reloadable-secure-settings)\n\nReloads secure settings."]
    pub fn reload_secure_settings<'b>(
        &'a self,
        parts: NodesReloadSecureSettingsParts<'b>,
    ) -> NodesReloadSecureSettings<'a, 'b, ()> {
        NodesReloadSecureSettings::new(self.transport(), parts)
    }
    #[doc = "[Nodes Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-nodes-stats.html)\n\nReturns statistical information about nodes in the cluster."]
    pub fn stats<'b>(&'a self, parts: NodesStatsParts<'b>) -> NodesStats<'a, 'b> {
        NodesStats::new(self.transport(), parts)
    }
    #[doc = "[Nodes Usage API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/cluster-nodes-usage.html)\n\nReturns low-level information about REST actions usage on nodes."]
    pub fn usage<'b>(&'a self, parts: NodesUsageParts<'b>) -> NodesUsage<'a, 'b> {
        NodesUsage::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Nodes APIs"]
    pub fn nodes(&self) -> Nodes {
        Nodes::new(self.transport())
    }
}
