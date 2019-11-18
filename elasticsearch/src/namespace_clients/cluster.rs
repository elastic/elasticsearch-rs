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
#[derive(Clone, Debug)]
pub struct ClusterAllocationExplain<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    include_disk_info: Option<bool>,
    include_yes_decisions: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> ClusterAllocationExplain<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        ClusterAllocationExplain {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            include_disk_info: None,
            include_yes_decisions: None,
            pretty: None,
            source: None,
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
    #[doc = "Return information about disk usage and shard sizes (default: false)"]
    pub fn include_disk_info(mut self, include_disk_info: Option<bool>) -> Self {
        self.include_disk_info = include_disk_info;
        self
    }
    #[doc = "Return 'YES' decisions in explanation (default: false)"]
    pub fn include_yes_decisions(mut self, include_yes_decisions: Option<bool>) -> Self {
        self.include_yes_decisions = include_yes_decisions;
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
}
impl<B> Sender for ClusterAllocationExplain<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = std::borrow::Cow::Borrowed("/_cluster/allocation/explain");
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
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
                #[serde(rename = "include_disk_info", skip_serializing_if = "Option::is_none")]
                include_disk_info: Option<bool>,
                #[serde(
                    rename = "include_yes_decisions",
                    skip_serializing_if = "Option::is_none"
                )]
                include_yes_decisions: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Clone, Debug)]
pub struct ClusterGetSettings {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    include_defaults: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl ClusterGetSettings {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterGetSettings {
            client,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            include_defaults: None,
            master_timeout: None,
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
    #[doc = "Whether to return all default clusters setting."]
    pub fn include_defaults(mut self, include_defaults: Option<bool>) -> Self {
        self.include_defaults = include_defaults;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
impl Sender for ClusterGetSettings {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = std::borrow::Cow::Borrowed("/_cluster/settings");
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
                #[serde(rename = "include_defaults", skip_serializing_if = "Option::is_none")]
                include_defaults: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<String>,
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Clone, Debug)]
pub struct ClusterHealth {
    client: Elasticsearch,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Option<Vec<String>>,
    level: Option<Level>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
    wait_for_events: Option<WaitForEvents>,
    wait_for_no_initializing_shards: Option<bool>,
    wait_for_no_relocating_shards: Option<bool>,
    wait_for_nodes: Option<String>,
    wait_for_status: Option<WaitForStatus>,
}
impl ClusterHealth {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterHealth {
            client,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            index: None,
            level: None,
            local: None,
            master_timeout: None,
            pretty: None,
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
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Limit the information returned to a specific index"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify the level of detail for returned information"]
    pub fn level(mut self, level: Option<Level>) -> Self {
        self.level = level;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "Wait until the specified number of shards is active"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
    #[doc = "Wait until all currently queued events with the given priority are processed"]
    pub fn wait_for_events(mut self, wait_for_events: Option<WaitForEvents>) -> Self {
        self.wait_for_events = wait_for_events;
        self
    }
    #[doc = "Whether to wait until there are no initializing shards in the cluster"]
    pub fn wait_for_no_initializing_shards(
        mut self,
        wait_for_no_initializing_shards: Option<bool>,
    ) -> Self {
        self.wait_for_no_initializing_shards = wait_for_no_initializing_shards;
        self
    }
    #[doc = "Whether to wait until there are no relocating shards in the cluster"]
    pub fn wait_for_no_relocating_shards(
        mut self,
        wait_for_no_relocating_shards: Option<bool>,
    ) -> Self {
        self.wait_for_no_relocating_shards = wait_for_no_relocating_shards;
        self
    }
    #[doc = "Wait until the specified number of nodes is available"]
    pub fn wait_for_nodes(mut self, wait_for_nodes: Option<String>) -> Self {
        self.wait_for_nodes = wait_for_nodes;
        self
    }
    #[doc = "Wait until cluster is in a specific state"]
    pub fn wait_for_status(mut self, wait_for_status: Option<WaitForStatus>) -> Self {
        self.wait_for_status = wait_for_status;
        self
    }
}
impl Sender for ClusterHealth {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(17usize + index_str.len());
                p.push_str("/_cluster/health/");
                p.push_str(index_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_cluster/health"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
                level: Option<Level>,
                #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
                local: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<String>,
                #[serde(rename = "wait_for_events", skip_serializing_if = "Option::is_none")]
                wait_for_events: Option<WaitForEvents>,
                #[serde(
                    rename = "wait_for_no_initializing_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_no_initializing_shards: Option<bool>,
                #[serde(
                    rename = "wait_for_no_relocating_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_no_relocating_shards: Option<bool>,
                #[serde(rename = "wait_for_nodes", skip_serializing_if = "Option::is_none")]
                wait_for_nodes: Option<String>,
                #[serde(rename = "wait_for_status", skip_serializing_if = "Option::is_none")]
                wait_for_status: Option<WaitForStatus>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Clone, Debug)]
pub struct ClusterPendingTasks {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl ClusterPendingTasks {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterPendingTasks {
            client,
            error_trace: None,
            filter_path: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            source: None,
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
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
}
impl Sender for ClusterPendingTasks {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = std::borrow::Cow::Borrowed("/_cluster/pending_tasks");
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
                #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
                local: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Clone, Debug)]
pub struct ClusterPutSettings<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> ClusterPutSettings<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        ClusterPutSettings {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            master_timeout: None,
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
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
impl<B> Sender for ClusterPutSettings<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = std::borrow::Cow::Borrowed("/_cluster/settings");
        let method = HttpMethod::Put;
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
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<String>,
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
                master_timeout: self.master_timeout,
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
#[derive(Clone, Debug)]
pub struct ClusterRemoteInfo {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl ClusterRemoteInfo {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterRemoteInfo {
            client,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
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
}
impl Sender for ClusterRemoteInfo {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = std::borrow::Cow::Borrowed("/_remote/info");
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
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Clone, Debug)]
pub struct ClusterReroute<B> {
    client: Elasticsearch,
    body: Option<B>,
    dry_run: Option<bool>,
    error_trace: Option<bool>,
    explain: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    metric: Option<Vec<String>>,
    pretty: Option<bool>,
    retry_failed: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> ClusterReroute<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        ClusterReroute {
            client,
            body: None,
            dry_run: None,
            error_trace: None,
            explain: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            metric: None,
            pretty: None,
            retry_failed: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Simulate the operation only and return the resulting state"]
    pub fn dry_run(mut self, dry_run: Option<bool>) -> Self {
        self.dry_run = dry_run;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Return an explanation of why the commands can or cannot be executed"]
    pub fn explain(mut self, explain: Option<bool>) -> Self {
        self.explain = explain;
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
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Limit the information returned to the specified metrics. Defaults to all but metadata"]
    pub fn metric(mut self, metric: Option<Vec<String>>) -> Self {
        self.metric = metric;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Retries allocation of shards that are blocked due to too many subsequent allocation failures"]
    pub fn retry_failed(mut self, retry_failed: Option<bool>) -> Self {
        self.retry_failed = retry_failed;
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
impl<B> Sender for ClusterReroute<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = std::borrow::Cow::Borrowed("/_cluster/reroute");
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "dry_run", skip_serializing_if = "Option::is_none")]
                dry_run: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "explain", skip_serializing_if = "Option::is_none")]
                explain: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<String>,
                #[serde(
                    rename = "metric",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                metric: Option<Vec<String>>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "retry_failed", skip_serializing_if = "Option::is_none")]
                retry_failed: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Clone, Debug)]
pub struct ClusterState {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    local: Option<bool>,
    master_timeout: Option<String>,
    metric: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
    wait_for_metadata_version: Option<i64>,
    wait_for_timeout: Option<String>,
}
impl ClusterState {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterState {
            client,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            local: None,
            master_timeout: None,
            metric: None,
            pretty: None,
            source: None,
            wait_for_metadata_version: None,
            wait_for_timeout: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names; use `_all` or empty string to perform the operation on all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Limit the information returned to the specified metrics"]
    pub fn metric(mut self, metric: Option<Vec<String>>) -> Self {
        self.metric = metric;
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
    #[doc = "Wait for the metadata version to be equal or greater than the specified metadata version"]
    pub fn wait_for_metadata_version(mut self, wait_for_metadata_version: Option<i64>) -> Self {
        self.wait_for_metadata_version = wait_for_metadata_version;
        self
    }
    #[doc = "The maximum time to wait for wait_for_metadata_version before timing out"]
    pub fn wait_for_timeout(mut self, wait_for_timeout: Option<String>) -> Self {
        self.wait_for_timeout = wait_for_timeout;
        self
    }
}
impl Sender for ClusterState {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match (&self.index, &self.metric) {
            (Some(index), Some(metric)) => {
                let metric_str = metric.join(",");
                let index_str = index.join(",");
                let mut p = String::with_capacity(17usize + metric_str.len() + index_str.len());
                p.push_str("/_cluster/state/");
                p.push_str(metric_str.as_ref());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (Some(index), None) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(21usize + index_str.len());
                p.push_str("/_cluster/state/_all/");
                p.push_str(index_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, Some(metric)) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(16usize + metric_str.len());
                p.push_str("/_cluster/state/");
                p.push_str(metric_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, None) => std::borrow::Cow::Borrowed("/_cluster/state"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
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
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
                local: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(
                    rename = "wait_for_metadata_version",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_metadata_version: Option<i64>,
                #[serde(rename = "wait_for_timeout", skip_serializing_if = "Option::is_none")]
                wait_for_timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Clone, Debug)]
pub struct ClusterStats {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    node_id: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl ClusterStats {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterStats {
            client,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
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
impl Sender for ClusterStats {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.node_id {
            Some(node_id) => {
                let node_id_str = node_id.join(",");
                let mut p = String::with_capacity(22usize + node_id_str.len());
                p.push_str("/_cluster/stats/nodes/");
                p.push_str(node_id_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_cluster/stats"),
        };
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
#[doc = "Cluster APIs"]
pub struct Cluster {
    client: Elasticsearch,
}
impl Cluster {
    pub fn new(client: Elasticsearch) -> Self {
        Cluster { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-allocation-explain.html"]
    pub fn allocation_explain<B>(&self) -> ClusterAllocationExplain<B>
    where
        B: Serialize,
    {
        ClusterAllocationExplain::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html"]
    pub fn get_settings(&self) -> ClusterGetSettings {
        ClusterGetSettings::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-health.html"]
    pub fn health(&self) -> ClusterHealth {
        ClusterHealth::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-pending.html"]
    pub fn pending_tasks(&self) -> ClusterPendingTasks {
        ClusterPendingTasks::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html"]
    pub fn put_settings<B>(&self) -> ClusterPutSettings<B>
    where
        B: Serialize,
    {
        ClusterPutSettings::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-remote-info.html"]
    pub fn remote_info(&self) -> ClusterRemoteInfo {
        ClusterRemoteInfo::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-reroute.html"]
    pub fn reroute<B>(&self) -> ClusterReroute<B>
    where
        B: Serialize,
    {
        ClusterReroute::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-state.html"]
    pub fn state(&self) -> ClusterState {
        ClusterState::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-stats.html"]
    pub fn stats(&self) -> ClusterStats {
        ClusterStats::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Cluster APIs"]
    pub fn cluster(&self) -> Cluster {
        Cluster::new(self.clone())
    }
}