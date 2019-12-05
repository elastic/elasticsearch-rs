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
    client::Elasticsearch,
    enums::*,
    error::ElasticsearchError,
    request::{Body, HttpMethod, JsonBody, NdBody},
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Nodes Hot Threads API"]
pub enum NodesHotThreadsUrlParts<'a> {
    None,
    NodeId(&'a [&'a str]),
}
impl<'a> NodesHotThreadsUrlParts<'a> {
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
#[doc = "Builder for the [Nodes Hot Threads API](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-hot-threads.html). Returns information about hot threads on each node in the cluster."]
pub struct NodesHotThreads<'a> {
    client: Elasticsearch,
    parts: NodesHotThreadsUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_idle_threads: Option<bool>,
    interval: Option<&'a str>,
    pretty: Option<bool>,
    snapshots: Option<i64>,
    source: Option<&'a str>,
    threads: Option<i64>,
    timeout: Option<&'a str>,
    ty: Option<Type>,
}
impl<'a> NodesHotThreads<'a> {
    pub fn new(client: Elasticsearch, parts: NodesHotThreadsUrlParts<'a>) -> Self {
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
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn interval(mut self, interval: &'a str) -> Self {
        self.interval = Some(interval);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Number of samples of thread stacktrace (default: 10)"]
    pub fn snapshots(mut self, snapshots: i64) -> Self {
        self.snapshots = Some(snapshots);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specify the number of threads to provide information for (default: 3)"]
    pub fn threads(mut self, threads: i64) -> Self {
        self.threads = Some(threads);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "The type to sample (default: cpu)"]
    pub fn ty(mut self, ty: Type) -> Self {
        self.ty = Some(ty);
        self
    }
    #[doc = "Creates an asynchronous call to the Nodes Hot Threads API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_idle_threads")]
                ignore_idle_threads: Option<bool>,
                #[serde(rename = "interval")]
                interval: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "snapshots")]
                snapshots: Option<i64>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "threads")]
                threads: Option<i64>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Nodes Info API"]
pub enum NodesInfoUrlParts<'a> {
    None,
    NodeId(&'a [&'a str]),
    Metric(&'a [&'a str]),
    NodeIdMetric(&'a [&'a str], &'a [&'a str]),
}
impl<'a> NodesInfoUrlParts<'a> {
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
#[doc = "Builder for the [Nodes Info API](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-info.html). Returns information about nodes in the cluster."]
pub struct NodesInfo<'a> {
    client: Elasticsearch,
    parts: NodesInfoUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> NodesInfo<'a> {
    pub fn new(client: Elasticsearch, parts: NodesInfoUrlParts<'a>) -> Self {
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
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Nodes Info API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Nodes Reload Secure Settings API"]
pub enum NodesReloadSecureSettingsUrlParts<'a> {
    None,
    NodeId(&'a [&'a str]),
}
impl<'a> NodesReloadSecureSettingsUrlParts<'a> {
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
#[doc = "Builder for the [Nodes Reload Secure Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/master/secure-settings.html#reloadable-secure-settings). Reloads secure settings."]
pub struct NodesReloadSecureSettings<'a, B> {
    client: Elasticsearch,
    parts: NodesReloadSecureSettingsUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> NodesReloadSecureSettings<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: NodesReloadSecureSettingsUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> NodesReloadSecureSettings<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        NodesReloadSecureSettings {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Nodes Reload Secure Settings API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Nodes Stats API"]
pub enum NodesStatsUrlParts<'a> {
    None,
    NodeId(&'a [&'a str]),
    Metric(&'a [&'a str]),
    NodeIdMetric(&'a [&'a str], &'a [&'a str]),
    MetricIndexMetric(&'a [&'a str], &'a [&'a str]),
    NodeIdMetricIndexMetric(&'a [&'a str], &'a [&'a str], &'a [&'a str]),
}
impl<'a> NodesStatsUrlParts<'a> {
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
#[doc = "Builder for the [Nodes Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html). Returns statistical information about nodes in the cluster."]
pub struct NodesStats<'a> {
    client: Elasticsearch,
    parts: NodesStatsUrlParts<'a>,
    completion_fields: Option<&'a [&'a str]>,
    error_trace: Option<bool>,
    fielddata_fields: Option<&'a [&'a str]>,
    fields: Option<&'a [&'a str]>,
    filter_path: Option<&'a [&'a str]>,
    groups: Option<bool>,
    human: Option<bool>,
    include_segment_file_sizes: Option<bool>,
    level: Option<Level>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    types: Option<&'a [&'a str]>,
}
impl<'a> NodesStats<'a> {
    pub fn new(client: Elasticsearch, parts: NodesStatsUrlParts<'a>) -> Self {
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
    pub fn completion_fields(mut self, completion_fields: &'a [&'a str]) -> Self {
        self.completion_fields = Some(completion_fields);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of fields for `fielddata` index metric (supports wildcards)"]
    pub fn fielddata_fields(mut self, fielddata_fields: &'a [&'a str]) -> Self {
        self.fielddata_fields = Some(fielddata_fields);
        self
    }
    #[doc = "A comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards)"]
    pub fn fields(mut self, fields: &'a [&'a str]) -> Self {
        self.fields = Some(fields);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A comma-separated list of search groups for `search` index metric"]
    pub fn groups(mut self, groups: bool) -> Self {
        self.groups = Some(groups);
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of document types for the `indexing` index metric"]
    pub fn types(mut self, types: &'a [&'a str]) -> Self {
        self.types = Some(types);
        self
    }
    #[doc = "Creates an asynchronous call to the Nodes Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(
                    rename = "completion_fields",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                completion_fields: Option<&'a [&'a str]>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "fielddata_fields",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                fielddata_fields: Option<&'a [&'a str]>,
                #[serde(rename = "fields", serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'a [&'a str]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "groups")]
                groups: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "include_segment_file_sizes")]
                include_segment_file_sizes: Option<bool>,
                #[serde(rename = "level")]
                level: Option<Level>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "types", serialize_with = "crate::client::serialize_coll_qs")]
                types: Option<&'a [&'a str]>,
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Nodes Usage API"]
pub enum NodesUsageUrlParts<'a> {
    None,
    NodeId(&'a [&'a str]),
    Metric(&'a [&'a str]),
    NodeIdMetric(&'a [&'a str], &'a [&'a str]),
}
impl<'a> NodesUsageUrlParts<'a> {
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
#[doc = "Builder for the [Nodes Usage API](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-usage.html). Returns low-level information about REST actions usage on nodes."]
pub struct NodesUsage<'a> {
    client: Elasticsearch,
    parts: NodesUsageUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> NodesUsage<'a> {
    pub fn new(client: Elasticsearch, parts: NodesUsageUrlParts<'a>) -> Self {
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
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Nodes Usage API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Nodes APIs"]
pub struct Nodes {
    client: Elasticsearch,
}
impl Nodes {
    pub fn new(client: Elasticsearch) -> Self {
        Nodes { client }
    }
    #[doc = "Returns information about hot threads on each node in the cluster."]
    pub fn hot_threads<'a>(&self, parts: NodesHotThreadsUrlParts<'a>) -> NodesHotThreads<'a> {
        NodesHotThreads::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about nodes in the cluster."]
    pub fn info<'a>(&self, parts: NodesInfoUrlParts<'a>) -> NodesInfo<'a> {
        NodesInfo::new(self.client.clone(), parts)
    }
    #[doc = "Reloads secure settings."]
    pub fn reload_secure_settings<'a>(
        &self,
        parts: NodesReloadSecureSettingsUrlParts<'a>,
    ) -> NodesReloadSecureSettings<'a, ()> {
        NodesReloadSecureSettings::new(self.client.clone(), parts)
    }
    #[doc = "Returns statistical information about nodes in the cluster."]
    pub fn stats<'a>(&self, parts: NodesStatsUrlParts<'a>) -> NodesStats<'a> {
        NodesStats::new(self.client.clone(), parts)
    }
    #[doc = "Returns low-level information about REST actions usage on nodes."]
    pub fn usage<'a>(&self, parts: NodesUsageUrlParts<'a>) -> NodesUsage<'a> {
        NodesUsage::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Nodes APIs"]
    pub fn nodes(&self) -> Nodes {
        Nodes::new(self.clone())
    }
}
