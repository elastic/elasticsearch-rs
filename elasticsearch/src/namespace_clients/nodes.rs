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
pub struct NodesHotThreads {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_idle_threads: Option<bool>,
    interval: Option<String>,
    node_id: Option<Vec<String>>,
    pretty: Option<bool>,
    snapshots: Option<i64>,
    source: Option<String>,
    threads: Option<i64>,
    timeout: Option<String>,
    ty: Option<Type>,
}
impl NodesHotThreads {
    pub fn new(client: Elasticsearch) -> Self {
        NodesHotThreads {
            client,
            error_trace: None,
            filter_path: None,
            human: None,
            ignore_idle_threads: None,
            interval: None,
            node_id: None,
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
    #[doc = "A comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes"]
    pub fn node_id(mut self, node_id: Option<Vec<String>>) -> Self {
        self.node_id = node_id;
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
        let path = match &self.node_id {
            Some(node_id) => {
                let node_id_str = node_id.join(",");
                let mut p = String::with_capacity(20usize + node_id_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/hot_threads");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_nodes/hot_threads"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_idle_threads")]
                ignore_idle_threads: Option<bool>,
                #[serde(rename = "interval")]
                interval: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "snapshots")]
                snapshots: Option<i64>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "threads")]
                threads: Option<i64>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "type")]
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
pub struct NodesInfo {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    metric: Option<Vec<String>>,
    node_id: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl NodesInfo {
    pub fn new(client: Elasticsearch) -> Self {
        NodesInfo {
            client,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            metric: None,
            node_id: None,
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
    #[doc = "A comma-separated list of metrics you wish returned. Leave empty to return all."]
    pub fn metric(mut self, metric: Option<Vec<String>>) -> Self {
        self.metric = metric;
        self
    }
    #[doc = "A comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes"]
    pub fn node_id(mut self, node_id: Option<Vec<String>>) -> Self {
        self.node_id = node_id;
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
        let path = match (&self.metric, &self.node_id) {
            (Some(metric), Some(node_id)) => {
                let node_id_str = node_id.join(",");
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(9usize + node_id_str.len() + metric_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/");
                p.push_str(metric_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, Some(node_id)) => {
                let node_id_str = node_id.join(",");
                let mut p = String::with_capacity(8usize + node_id_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (Some(metric), None) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(8usize + metric_str.len());
                p.push_str("/_nodes/");
                p.push_str(metric_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, None) => std::borrow::Cow::Borrowed("/_nodes"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
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
pub struct NodesReloadSecureSettings<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    node_id: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> NodesReloadSecureSettings<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        NodesReloadSecureSettings {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            node_id: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
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
    #[doc = "A comma-separated list of node IDs to span the reload/reinit call. Should stay empty because reloading usually involves all cluster nodes."]
    pub fn node_id(mut self, node_id: Option<Vec<String>>) -> Self {
        self.node_id = node_id;
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
        let path = match &self.node_id {
            Some(node_id) => {
                let node_id_str = node_id.join(",");
                let mut p = String::with_capacity(31usize + node_id_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/reload_secure_settings");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_nodes/reload_secure_settings"),
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
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
pub struct NodesStats {
    client: Elasticsearch,
    completion_fields: Option<Vec<String>>,
    error_trace: Option<bool>,
    fielddata_fields: Option<Vec<String>>,
    fields: Option<Vec<String>>,
    filter_path: Option<Vec<String>>,
    groups: Option<bool>,
    human: Option<bool>,
    include_segment_file_sizes: Option<bool>,
    index_metric: Option<Vec<String>>,
    level: Option<Level>,
    metric: Option<Vec<String>>,
    node_id: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    types: Option<Vec<String>>,
}
impl NodesStats {
    pub fn new(client: Elasticsearch) -> Self {
        NodesStats {
            client,
            completion_fields: None,
            error_trace: None,
            fielddata_fields: None,
            fields: None,
            filter_path: None,
            groups: None,
            human: None,
            include_segment_file_sizes: None,
            index_metric: None,
            level: None,
            metric: None,
            node_id: None,
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
    #[doc = "Limit the information returned for `indices` metric to the specific index metrics. Isn't used if `indices` (or `all`) metric isn't specified."]
    pub fn index_metric(mut self, index_metric: Option<Vec<String>>) -> Self {
        self.index_metric = index_metric;
        self
    }
    #[doc = "Return indices stats aggregated at index, node or shard level"]
    pub fn level(mut self, level: Option<Level>) -> Self {
        self.level = level;
        self
    }
    #[doc = "Limit the information returned to the specified metrics"]
    pub fn metric(mut self, metric: Option<Vec<String>>) -> Self {
        self.metric = metric;
        self
    }
    #[doc = "A comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes"]
    pub fn node_id(mut self, node_id: Option<Vec<String>>) -> Self {
        self.node_id = node_id;
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
        let path = match (&self.index_metric, &self.metric, &self.node_id) {
            (Some(index_metric), Some(metric), Some(node_id)) => {
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
                std::borrow::Cow::Owned(p)
            }
            (None, Some(metric), Some(node_id)) => {
                let node_id_str = node_id.join(",");
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(15usize + node_id_str.len() + metric_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/stats/");
                p.push_str(metric_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (Some(index_metric), Some(metric), None) => {
                let metric_str = metric.join(",");
                let index_metric_str = index_metric.join(",");
                let mut p =
                    String::with_capacity(15usize + metric_str.len() + index_metric_str.len());
                p.push_str("/_nodes/stats/");
                p.push_str(metric_str.as_ref());
                p.push_str("/");
                p.push_str(index_metric_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, None, Some(node_id)) => {
                let node_id_str = node_id.join(",");
                let mut p = String::with_capacity(14usize + node_id_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/stats");
                std::borrow::Cow::Owned(p)
            }
            (None, Some(metric), None) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(14usize + metric_str.len());
                p.push_str("/_nodes/stats/");
                p.push_str(metric_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (Some(index_metric), None, Some(node_id)) => {
                let node_id_str = node_id.join(",");
                let index_metric_str = index_metric.join(",");
                let mut p =
                    String::with_capacity(19usize + node_id_str.len() + index_metric_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/stats/all/");
                p.push_str(index_metric_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (Some(index_metric), None, None) => {
                let index_metric_str = index_metric.join(",");
                let mut p = String::with_capacity(18usize + index_metric_str.len());
                p.push_str("/_nodes/stats/all/");
                p.push_str(index_metric_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, None, None) => std::borrow::Cow::Borrowed("/_nodes/stats"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "completion_fields",
                    serialize_with = "crate::client::serialize_vec_qs"
                )]
                completion_fields: Option<Vec<String>>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "fielddata_fields",
                    serialize_with = "crate::client::serialize_vec_qs"
                )]
                fielddata_fields: Option<Vec<String>>,
                #[serde(rename = "fields", serialize_with = "crate::client::serialize_vec_qs")]
                fields: Option<Vec<String>>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs"
                )]
                filter_path: Option<Vec<String>>,
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
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "types", serialize_with = "crate::client::serialize_vec_qs")]
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
pub struct NodesUsage {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    metric: Option<Vec<String>>,
    node_id: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl NodesUsage {
    pub fn new(client: Elasticsearch) -> Self {
        NodesUsage {
            client,
            error_trace: None,
            filter_path: None,
            human: None,
            metric: None,
            node_id: None,
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
    #[doc = "Limit the information returned to the specified metrics"]
    pub fn metric(mut self, metric: Option<Vec<String>>) -> Self {
        self.metric = metric;
        self
    }
    #[doc = "A comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes"]
    pub fn node_id(mut self, node_id: Option<Vec<String>>) -> Self {
        self.node_id = node_id;
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
        let path = match (&self.metric, &self.node_id) {
            (Some(metric), Some(node_id)) => {
                let node_id_str = node_id.join(",");
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(15usize + node_id_str.len() + metric_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/usage/");
                p.push_str(metric_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, Some(node_id)) => {
                let node_id_str = node_id.join(",");
                let mut p = String::with_capacity(14usize + node_id_str.len());
                p.push_str("/_nodes/");
                p.push_str(node_id_str.as_ref());
                p.push_str("/usage");
                std::borrow::Cow::Owned(p)
            }
            (Some(metric), None) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(14usize + metric_str.len());
                p.push_str("/_nodes/usage/");
                p.push_str(metric_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, None) => std::borrow::Cow::Borrowed("/_nodes/usage"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
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
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-hot-threads.html"]
    pub fn hot_threads(&self) -> NodesHotThreads {
        NodesHotThreads::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-info.html"]
    pub fn info(&self) -> NodesInfo {
        NodesInfo::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/secure-settings.html#reloadable-secure-settings"]
    pub fn reload_secure_settings<B>(&self) -> NodesReloadSecureSettings<B>
    where
        B: Serialize,
    {
        NodesReloadSecureSettings::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html"]
    pub fn stats(&self) -> NodesStats {
        NodesStats::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-usage.html"]
    pub fn usage(&self) -> NodesUsage {
        NodesUsage::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Nodes APIs"]
    pub fn nodes(&self) -> Nodes {
        Nodes::new(self.clone())
    }
}