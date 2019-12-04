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
    client::Elasticsearch, enums::*, error::ElasticsearchError, http_method::HttpMethod,
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cluster Allocation Explain API"]
pub enum ClusterAllocationExplainUrlParts {
    None,
}
impl ClusterAllocationExplainUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ClusterAllocationExplainUrlParts::None => "/_cluster/allocation/explain".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cluster Allocation Explain API"]
pub struct ClusterAllocationExplain<'a, B> {
    client: Elasticsearch,
    parts: ClusterAllocationExplainUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    include_disk_info: Option<bool>,
    include_yes_decisions: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> ClusterAllocationExplain<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        ClusterAllocationExplain {
            client,
            parts: ClusterAllocationExplainUrlParts::None,
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
    pub fn body<T>(self, body: T) -> ClusterAllocationExplain<'a, T>
    where
        T: Serialize,
    {
        ClusterAllocationExplain {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            include_disk_info: self.include_disk_info,
            include_yes_decisions: self.include_yes_decisions,
            pretty: self.pretty,
            source: self.source,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Cluster Allocation Explain API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
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
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cluster Get Settings API"]
pub enum ClusterGetSettingsUrlParts {
    None,
}
impl ClusterGetSettingsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ClusterGetSettingsUrlParts::None => "/_cluster/settings".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cluster Get Settings API"]
pub struct ClusterGetSettings<'a> {
    client: Elasticsearch,
    parts: ClusterGetSettingsUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    include_defaults: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> ClusterGetSettings<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterGetSettings {
            client,
            parts: ClusterGetSettingsUrlParts::None,
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
    #[doc = "Whether to return all default clusters setting."]
    pub fn include_defaults(mut self, include_defaults: bool) -> Self {
        self.include_defaults = Some(include_defaults);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous request to the Cluster Get Settings API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "flat_settings", skip_serializing_if = "Option::is_none")]
                flat_settings: Option<bool>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "include_defaults", skip_serializing_if = "Option::is_none")]
                include_defaults: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cluster Health API"]
pub enum ClusterHealthUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> ClusterHealthUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ClusterHealthUrlParts::None => "/_cluster/health".into(),
            ClusterHealthUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(17usize + index_str.len());
                p.push_str("/_cluster/health/");
                p.push_str(index_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cluster Health API"]
pub struct ClusterHealth<'a> {
    client: Elasticsearch,
    parts: ClusterHealthUrlParts<'a>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    level: Option<Level>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
    wait_for_events: Option<WaitForEvents>,
    wait_for_no_initializing_shards: Option<bool>,
    wait_for_no_relocating_shards: Option<bool>,
    wait_for_nodes: Option<&'a str>,
    wait_for_status: Option<WaitForStatus>,
}
impl<'a> ClusterHealth<'a> {
    pub fn new(client: Elasticsearch, parts: ClusterHealthUrlParts<'a>) -> Self {
        ClusterHealth {
            client,
            parts,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
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
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Wait until the specified number of shards is active"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
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
    pub fn wait_for_nodes(mut self, wait_for_nodes: &'a str) -> Self {
        self.wait_for_nodes = Some(wait_for_nodes);
        self
    }
    #[doc = "Wait until cluster is in a specific state"]
    pub fn wait_for_status(mut self, wait_for_status: WaitForStatus) -> Self {
        self.wait_for_status = Some(wait_for_status);
        self
    }
    #[doc = "Creates an asynchronous request to the Cluster Health API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
                level: Option<Level>,
                #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
                local: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<&'a str>,
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
                wait_for_nodes: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cluster Pending Tasks API"]
pub enum ClusterPendingTasksUrlParts {
    None,
}
impl ClusterPendingTasksUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ClusterPendingTasksUrlParts::None => "/_cluster/pending_tasks".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cluster Pending Tasks API"]
pub struct ClusterPendingTasks<'a> {
    client: Elasticsearch,
    parts: ClusterPendingTasksUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> ClusterPendingTasks<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterPendingTasks {
            client,
            parts: ClusterPendingTasksUrlParts::None,
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
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous request to the Cluster Pending Tasks API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
                local: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cluster Put Settings API"]
pub enum ClusterPutSettingsUrlParts {
    None,
}
impl ClusterPutSettingsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ClusterPutSettingsUrlParts::None => "/_cluster/settings".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cluster Put Settings API"]
pub struct ClusterPutSettings<'a, B> {
    client: Elasticsearch,
    parts: ClusterPutSettingsUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> ClusterPutSettings<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        ClusterPutSettings {
            client,
            parts: ClusterPutSettingsUrlParts::None,
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
    pub fn body<T>(self, body: T) -> ClusterPutSettings<'a, T>
    where
        T: Serialize,
    {
        ClusterPutSettings {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            flat_settings: self.flat_settings,
            human: self.human,
            master_timeout: self.master_timeout,
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
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous request to the Cluster Put Settings API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "flat_settings", skip_serializing_if = "Option::is_none")]
                flat_settings: Option<bool>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cluster Remote Info API"]
pub enum ClusterRemoteInfoUrlParts {
    None,
}
impl ClusterRemoteInfoUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ClusterRemoteInfoUrlParts::None => "/_remote/info".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cluster Remote Info API"]
pub struct ClusterRemoteInfo<'a> {
    client: Elasticsearch,
    parts: ClusterRemoteInfoUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> ClusterRemoteInfo<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterRemoteInfo {
            client,
            parts: ClusterRemoteInfoUrlParts::None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
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
    #[doc = "Creates an asynchronous request to the Cluster Remote Info API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cluster Reroute API"]
pub enum ClusterRerouteUrlParts {
    None,
}
impl ClusterRerouteUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ClusterRerouteUrlParts::None => "/_cluster/reroute".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cluster Reroute API"]
pub struct ClusterReroute<'a, B> {
    client: Elasticsearch,
    parts: ClusterRerouteUrlParts,
    body: Option<B>,
    dry_run: Option<bool>,
    error_trace: Option<bool>,
    explain: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    metric: Option<&'a [&'a str]>,
    pretty: Option<bool>,
    retry_failed: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> ClusterReroute<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        ClusterReroute {
            client,
            parts: ClusterRerouteUrlParts::None,
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
    pub fn body<T>(self, body: T) -> ClusterReroute<'a, T>
    where
        T: Serialize,
    {
        ClusterReroute {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Limit the information returned to the specified metrics. Defaults to all but metadata"]
    pub fn metric(mut self, metric: &'a [&'a str]) -> Self {
        self.metric = Some(metric);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Retries allocation of shards that are blocked due to too many subsequent allocation failures"]
    pub fn retry_failed(mut self, retry_failed: bool) -> Self {
        self.retry_failed = Some(retry_failed);
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
    #[doc = "Creates an asynchronous request to the Cluster Reroute API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "dry_run", skip_serializing_if = "Option::is_none")]
                dry_run: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "explain", skip_serializing_if = "Option::is_none")]
                explain: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<&'a str>,
                #[serde(
                    rename = "metric",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                metric: Option<&'a [&'a str]>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "retry_failed", skip_serializing_if = "Option::is_none")]
                retry_failed: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cluster State API"]
pub enum ClusterStateUrlParts<'a> {
    None,
    Metric(&'a [&'a str]),
    MetricIndex(&'a [&'a str], &'a [&'a str]),
}
impl<'a> ClusterStateUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ClusterStateUrlParts::None => "/_cluster/state".into(),
            ClusterStateUrlParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(16usize + metric_str.len());
                p.push_str("/_cluster/state/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
            ClusterStateUrlParts::MetricIndex(ref metric, ref index) => {
                let metric_str = metric.join(",");
                let index_str = index.join(",");
                let mut p = String::with_capacity(17usize + metric_str.len() + index_str.len());
                p.push_str("/_cluster/state/");
                p.push_str(metric_str.as_ref());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cluster State API"]
pub struct ClusterState<'a> {
    client: Elasticsearch,
    parts: ClusterStateUrlParts<'a>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    wait_for_metadata_version: Option<i64>,
    wait_for_timeout: Option<&'a str>,
}
impl<'a> ClusterState<'a> {
    pub fn new(client: Elasticsearch, parts: ClusterStateUrlParts<'a>) -> Self {
        ClusterState {
            client,
            parts,
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
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Wait for the metadata version to be equal or greater than the specified metadata version"]
    pub fn wait_for_metadata_version(mut self, wait_for_metadata_version: i64) -> Self {
        self.wait_for_metadata_version = Some(wait_for_metadata_version);
        self
    }
    #[doc = "The maximum time to wait for wait_for_metadata_version before timing out"]
    pub fn wait_for_timeout(mut self, wait_for_timeout: &'a str) -> Self {
        self.wait_for_timeout = Some(wait_for_timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Cluster State API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "flat_settings", skip_serializing_if = "Option::is_none")]
                flat_settings: Option<bool>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
                local: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(
                    rename = "wait_for_metadata_version",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_metadata_version: Option<i64>,
                #[serde(rename = "wait_for_timeout", skip_serializing_if = "Option::is_none")]
                wait_for_timeout: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cluster Stats API"]
pub enum ClusterStatsUrlParts<'a> {
    None,
    NodeId(&'a [&'a str]),
}
impl<'a> ClusterStatsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ClusterStatsUrlParts::None => "/_cluster/stats".into(),
            ClusterStatsUrlParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let mut p = String::with_capacity(22usize + node_id_str.len());
                p.push_str("/_cluster/stats/nodes/");
                p.push_str(node_id_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cluster Stats API"]
pub struct ClusterStats<'a> {
    client: Elasticsearch,
    parts: ClusterStatsUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> ClusterStats<'a> {
    pub fn new(client: Elasticsearch, parts: ClusterStatsUrlParts<'a>) -> Self {
        ClusterStats {
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
    #[doc = "Creates an asynchronous request to the Cluster Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "flat_settings", skip_serializing_if = "Option::is_none")]
                flat_settings: Option<bool>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
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
    #[doc = "Provides explanations for shard allocations in the cluster."]
    pub fn allocation_explain<'a>(&self) -> ClusterAllocationExplain<'a, ()> {
        ClusterAllocationExplain::new(self.client.clone())
    }
    #[doc = "Returns cluster settings."]
    pub fn get_settings<'a>(&self) -> ClusterGetSettings<'a> {
        ClusterGetSettings::new(self.client.clone())
    }
    #[doc = "Returns basic information about the health of the cluster."]
    pub fn health<'a>(&self, parts: ClusterHealthUrlParts<'a>) -> ClusterHealth<'a> {
        ClusterHealth::new(self.client.clone(), parts)
    }
    #[doc = "Returns a list of any cluster-level changes (e.g. create index, update mapping,\nallocate or fail shard) which have not yet been executed."]
    pub fn pending_tasks<'a>(&self) -> ClusterPendingTasks<'a> {
        ClusterPendingTasks::new(self.client.clone())
    }
    #[doc = "Updates the cluster settings."]
    pub fn put_settings<'a>(&self) -> ClusterPutSettings<'a, ()> {
        ClusterPutSettings::new(self.client.clone())
    }
    #[doc = "Returns the information about configured remote clusters."]
    pub fn remote_info<'a>(&self) -> ClusterRemoteInfo<'a> {
        ClusterRemoteInfo::new(self.client.clone())
    }
    #[doc = "Allows to manually change the allocation of individual shards in the cluster."]
    pub fn reroute<'a>(&self) -> ClusterReroute<'a, ()> {
        ClusterReroute::new(self.client.clone())
    }
    #[doc = "Returns a comprehensive information about the state of the cluster."]
    pub fn state<'a>(&self, parts: ClusterStateUrlParts<'a>) -> ClusterState<'a> {
        ClusterState::new(self.client.clone(), parts)
    }
    #[doc = "Returns high-level overview of cluster statistics."]
    pub fn stats<'a>(&self, parts: ClusterStatsUrlParts<'a>) -> ClusterStats<'a> {
        ClusterStats::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Cluster APIs"]
    pub fn cluster(&self) -> Cluster {
        Cluster::new(self.clone())
    }
}