// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
use crate::{
    client::{Elasticsearch, Sender},
    enums::*,
    error::ElasticsearchError,
    http_method::HttpMethod,
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Nodes Hot Threads API"]
pub enum NodesHotThreadsUrlParts {
    None,
    NodeId(Vec<String>),
}
impl NodesHotThreadsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            NodesHotThreadsUrlParts::None => "/_nodes/hot_threads".into(),
            NodesHotThreadsUrlParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let mut p = String::with_capacity(20usize + node_id_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/hot_threads");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Nodes Hot Threads API"]
pub struct NodesHotThreads {
    client: Elasticsearch,
    parts: NodesHotThreadsUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_idle_threads: Option<bool>,
    interval: Option<String>,
    pretty: Option<bool>,
    snapshots: Option<i64>,
    source: Option<String>,
    threads: Option<i64>,
    timeout: Option<String>,
    ty: Option<Type>,
}
impl NodesHotThreads {
    pub fn new(client: Elasticsearch, parts: NodesHotThreadsUrlParts) -> Self {
        NodesHotThreads {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            ignore_idle_threads: None,
            interval: None,
            pretty: None,
            snapshots: None,
            source: None,
            threads: None,
            timeout: None,
            ty: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Don't show threads that are in known-idle places, such as waiting on a socket select or pulling from an empty task queue (default: true)"]
    pub fn ignore_idle_threads(mut self, ignore_idle_threads: Option<bool>) -> Self {
        self.ignore_idle_threads = ignore_idle_threads;
        self
    }
    #[doc = "The interval for the second sampling of threads"]
    pub fn interval(mut self, interval: Option<String>) -> Self {
        self.interval = interval;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Number of samples of thread stacktrace (default: 10)"]
    pub fn snapshots(mut self, snapshots: Option<i64>) -> Self {
        self.snapshots = snapshots;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Specify the number of threads to provide information for (default: 3)"]
    pub fn threads(mut self, threads: Option<i64>) -> Self {
        self.threads = threads;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "The type to sample (default: cpu)"]
    pub fn ty(mut self, ty: Option<Type>) -> Self {
        self.ty = ty;
        self
    }
}
impl Sender for NodesHotThreads {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(
                    rename = "ignore_idle_threads",
                    skip_serializing_if = "Option::is_none"
                )]
                ignore_idle_threads: Option<bool>,
                #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
                interval: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "snapshots", skip_serializing_if = "Option::is_none")]
                snapshots: Option<i64>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "threads", skip_serializing_if = "Option::is_none")]
                threads: Option<i64>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
                #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
                ty: Option<Type>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Nodes Info API"]
pub enum NodesInfoUrlParts {
    None,
    NodeId(Vec<String>),
    Metric(Vec<String>),
    NodeIdMetric(Vec<String>, Vec<String>),
}
impl NodesInfoUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            NodesInfoUrlParts::None => "/_nodes".into(),
            NodesInfoUrlParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let mut p = String::with_capacity(8usize + node_id_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.into()
            }
            NodesInfoUrlParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(8usize + metric_str.len());
                p.push_str("/_nodes/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
            NodesInfoUrlParts::NodeIdMetric(ref node_id, ref metric) => {
                let node_id_str = node_id.join(",");
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(9usize + node_id_str.len() + metric_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Nodes Info API"]
pub struct NodesInfo {
    client: Elasticsearch,
    parts: NodesInfoUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl NodesInfo {
    pub fn new(client: Elasticsearch, parts: NodesInfoUrlParts) -> Self {
        NodesInfo {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: Option<bool>) -> Self {
        self.flat_settings = flat_settings;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for NodesInfo {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "flat_settings", skip_serializing_if = "Option::is_none")]
                flat_settings: Option<bool>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Nodes Reload Secure Settings API"]
pub enum NodesReloadSecureSettingsUrlParts {
    None,
    NodeId(Vec<String>),
}
impl NodesReloadSecureSettingsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            NodesReloadSecureSettingsUrlParts::None => "/_nodes/reload_secure_settings".into(),
            NodesReloadSecureSettingsUrlParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let mut p = String::with_capacity(31usize + node_id_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/reload_secure_settings");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Nodes Reload Secure Settings API"]
pub struct NodesReloadSecureSettings<B> {
    client: Elasticsearch,
    parts: NodesReloadSecureSettingsUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> NodesReloadSecureSettings<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: NodesReloadSecureSettingsUrlParts) -> Self {
        NodesReloadSecureSettings {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: Option<T>) -> NodesReloadSecureSettings<T>
    where
        T: Serialize,
    {
        NodesReloadSecureSettings {
            client: self.client,
            parts: self.parts,
            body,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl<B> Sender for NodesReloadSecureSettings<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Nodes Stats API"]
pub enum NodesStatsUrlParts {
    None,
    NodeId(Vec<String>),
    Metric(Vec<String>),
    NodeIdMetric(Vec<String>, Vec<String>),
    MetricIndexMetric(Vec<String>, Vec<String>),
    NodeIdMetricIndexMetric(Vec<String>, Vec<String>, Vec<String>),
}
impl NodesStatsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            NodesStatsUrlParts::None => "/_nodes/stats".into(),
            NodesStatsUrlParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let mut p = String::with_capacity(14usize + node_id_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/stats");
                p.into()
            }
            NodesStatsUrlParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(14usize + metric_str.len());
                p.push_str("/_nodes/stats/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
            NodesStatsUrlParts::NodeIdMetric(ref node_id, ref metric) => {
                let node_id_str = node_id.join(",");
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(15usize + node_id_str.len() + metric_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/stats/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
            NodesStatsUrlParts::MetricIndexMetric(ref metric, ref index_metric) => {
                let metric_str = metric.join(",");
                let index_metric_str = index_metric.join(",");
                let mut p =
                    String::with_capacity(15usize + metric_str.len() + index_metric_str.len());
                p.push_str("/_nodes/stats/");
                p.push_str(metric_str.as_ref());
                p.push_str("/");
                p.push_str(index_metric_str.as_ref());
                p.into()
            }
            NodesStatsUrlParts::NodeIdMetricIndexMetric(
                ref node_id,
                ref metric,
                ref index_metric,
            ) => {
                let node_id_str = node_id.join(",");
                let metric_str = metric.join(",");
                let index_metric_str = index_metric.join(",");
                let mut p = String::with_capacity(
                    16usize + node_id_str.len() + metric_str.len() + index_metric_str.len(),
                );
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/stats/");
                p.push_str(metric_str.as_ref());
                p.push_str("/");
                p.push_str(index_metric_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Nodes Stats API"]
pub struct NodesStats {
    client: Elasticsearch,
    parts: NodesStatsUrlParts,
    completion_fields: Option<Vec<String>>,
    error_trace: Option<bool>,
    fielddata_fields: Option<Vec<String>>,
    fields: Option<Vec<String>>,
    filter_path: Option<Vec<String>>,
    groups: Option<bool>,
    human: Option<bool>,
    include_segment_file_sizes: Option<bool>,
    level: Option<Level>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    types: Option<Vec<String>>,
}
impl NodesStats {
    pub fn new(client: Elasticsearch, parts: NodesStatsUrlParts) -> Self {
        NodesStats {
            client,
            parts,
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
            source: None,
            timeout: None,
            types: None,
        }
    }
    #[doc = "A comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards)"]
    pub fn completion_fields(mut self, completion_fields: Option<Vec<String>>) -> Self {
        self.completion_fields = completion_fields;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of fields for `fielddata` index metric (supports wildcards)"]
    pub fn fielddata_fields(mut self, fielddata_fields: Option<Vec<String>>) -> Self {
        self.fielddata_fields = fielddata_fields;
        self
    }
    #[doc = "A comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards)"]
    pub fn fields(mut self, fields: Option<Vec<String>>) -> Self {
        self.fields = fields;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "A comma-separated list of search groups for `search` index metric"]
    pub fn groups(mut self, groups: Option<bool>) -> Self {
        self.groups = groups;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested)"]
    pub fn include_segment_file_sizes(mut self, include_segment_file_sizes: Option<bool>) -> Self {
        self.include_segment_file_sizes = include_segment_file_sizes;
        self
    }
    #[doc = "Return indices stats aggregated at index, node or shard level"]
    pub fn level(mut self, level: Option<Level>) -> Self {
        self.level = level;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "A comma-separated list of document types for the `indexing` index metric"]
    pub fn types(mut self, types: Option<Vec<String>>) -> Self {
        self.types = types;
        self
    }
}
impl Sender for NodesStats {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "completion_fields",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                completion_fields: Option<Vec<String>>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "fielddata_fields",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                fielddata_fields: Option<Vec<String>>,
                #[serde(
                    rename = "fields",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                fields: Option<Vec<String>>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
                groups: Option<bool>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(
                    rename = "include_segment_file_sizes",
                    skip_serializing_if = "Option::is_none"
                )]
                include_segment_file_sizes: Option<bool>,
                #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
                level: Option<Level>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
                #[serde(
                    rename = "types",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                types: Option<Vec<String>>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Nodes Usage API"]
pub enum NodesUsageUrlParts {
    None,
    NodeId(Vec<String>),
    Metric(Vec<String>),
    NodeIdMetric(Vec<String>, Vec<String>),
}
impl NodesUsageUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            NodesUsageUrlParts::None => "/_nodes/usage".into(),
            NodesUsageUrlParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let mut p = String::with_capacity(14usize + node_id_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/usage");
                p.into()
            }
            NodesUsageUrlParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(14usize + metric_str.len());
                p.push_str("/_nodes/usage/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
            NodesUsageUrlParts::NodeIdMetric(ref node_id, ref metric) => {
                let node_id_str = node_id.join(",");
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(15usize + node_id_str.len() + metric_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/usage/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Nodes Usage API"]
pub struct NodesUsage {
    client: Elasticsearch,
    parts: NodesUsageUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl NodesUsage {
    pub fn new(client: Elasticsearch, parts: NodesUsageUrlParts) -> Self {
        NodesUsage {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for NodesUsage {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[doc = "Nodes APIs"]
pub struct Nodes {
    client: Elasticsearch,
}
impl Nodes {
    pub fn new(client: Elasticsearch) -> Self {
        Nodes { client }
    }
    #[doc = "Returns information about hot threads on each node in the cluster."]
    pub fn hot_threads(&self, parts: NodesHotThreadsUrlParts) -> NodesHotThreads {
        NodesHotThreads::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about nodes in the cluster."]
    pub fn info(&self, parts: NodesInfoUrlParts) -> NodesInfo {
        NodesInfo::new(self.client.clone(), parts)
    }
    #[doc = "Reloads secure settings."]
    pub fn reload_secure_settings(
        &self,
        parts: NodesReloadSecureSettingsUrlParts,
    ) -> NodesReloadSecureSettings<()> {
        NodesReloadSecureSettings::new(self.client.clone(), parts)
    }
    #[doc = "Returns statistical information about nodes in the cluster."]
    pub fn stats(&self, parts: NodesStatsUrlParts) -> NodesStats {
        NodesStats::new(self.client.clone(), parts)
    }
    #[doc = "Returns low-level information about REST actions usage on nodes."]
    pub fn usage(&self, parts: NodesUsageUrlParts) -> NodesUsage {
        NodesUsage::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Nodes APIs"]
    pub fn nodes(&self) -> Nodes {
        Nodes::new(self.clone())
    }
}