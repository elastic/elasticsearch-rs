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
#[allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
    params::*,
};
use serde::Serialize;
use serde_with;
use std::borrow::Cow;
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
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cluster Allocation Explain API](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-allocation-explain.html). Provides explanations for shard allocations in the cluster."]
pub struct ClusterAllocationExplain<'a, B> {
    client: Elasticsearch,
    parts: ClusterAllocationExplainParts,
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
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterAllocationExplain]"]
    pub fn new(client: Elasticsearch) -> Self {
        ClusterAllocationExplain {
            client,
            parts: ClusterAllocationExplainParts::None,
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
    pub fn body<T>(self, body: T) -> ClusterAllocationExplain<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterAllocationExplain {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous call to the Cluster Allocation Explain API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
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
                #[serde(rename = "include_disk_info")]
                include_disk_info: Option<bool>,
                #[serde(rename = "include_yes_decisions")]
                include_yes_decisions: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .client
            .send(method, &path, query_string.as_ref(), body)
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
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cluster Get Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html). Returns cluster settings."]
pub struct ClusterGetSettings<'a> {
    client: Elasticsearch,
    parts: ClusterGetSettingsParts,
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
    #[doc = "Creates a new instance of [ClusterGetSettings]"]
    pub fn new(client: Elasticsearch) -> Self {
        ClusterGetSettings {
            client,
            parts: ClusterGetSettingsParts::None,
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
    #[doc = "Creates an asynchronous call to the Cluster Get Settings API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
                #[serde(rename = "include_defaults")]
                include_defaults: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
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
#[doc = "API parts for the Cluster Health API"]
pub enum ClusterHealthParts<'a> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'a [&'a str]),
}
impl<'a> ClusterHealthParts<'a> {
    #[doc = "Builds a relative URL path to the Cluster Health API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterHealthParts::None => "/_cluster/health".into(),
            ClusterHealthParts::Index(ref index) => {
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
#[doc = "Builder for the [Cluster Health API](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-health.html). Returns basic information about the health of the cluster."]
pub struct ClusterHealth<'a> {
    client: Elasticsearch,
    parts: ClusterHealthParts<'a>,
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
    #[doc = "Creates a new instance of [ClusterHealth] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: ClusterHealthParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Cluster Health API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "level")]
                level: Option<Level>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'a str>,
                #[serde(rename = "wait_for_events")]
                wait_for_events: Option<WaitForEvents>,
                #[serde(rename = "wait_for_no_initializing_shards")]
                wait_for_no_initializing_shards: Option<bool>,
                #[serde(rename = "wait_for_no_relocating_shards")]
                wait_for_no_relocating_shards: Option<bool>,
                #[serde(rename = "wait_for_nodes")]
                wait_for_nodes: Option<&'a str>,
                #[serde(rename = "wait_for_status")]
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
            .client
            .send(method, &path, query_string.as_ref(), body)
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
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cluster Pending Tasks API](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-pending.html). Returns a list of any cluster-level changes (e.g. create index, update mapping,\nallocate or fail shard) which have not yet been executed."]
pub struct ClusterPendingTasks<'a> {
    client: Elasticsearch,
    parts: ClusterPendingTasksParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> ClusterPendingTasks<'a> {
    #[doc = "Creates a new instance of [ClusterPendingTasks]"]
    pub fn new(client: Elasticsearch) -> Self {
        ClusterPendingTasks {
            client,
            parts: ClusterPendingTasksParts::None,
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
    #[doc = "Creates an asynchronous call to the Cluster Pending Tasks API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .client
            .send(method, &path, query_string.as_ref(), body)
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
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cluster Put Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html). Updates the cluster settings."]
pub struct ClusterPutSettings<'a, B> {
    client: Elasticsearch,
    parts: ClusterPutSettingsParts,
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
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterPutSettings]"]
    pub fn new(client: Elasticsearch) -> Self {
        ClusterPutSettings {
            client,
            parts: ClusterPutSettingsParts::None,
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
    pub fn body<T>(self, body: T) -> ClusterPutSettings<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterPutSettings {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous call to the Cluster Put Settings API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
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
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cluster Remote Info API](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-remote-info.html). Returns the information about configured remote clusters."]
pub struct ClusterRemoteInfo<'a> {
    client: Elasticsearch,
    parts: ClusterRemoteInfoParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> ClusterRemoteInfo<'a> {
    #[doc = "Creates a new instance of [ClusterRemoteInfo]"]
    pub fn new(client: Elasticsearch) -> Self {
        ClusterRemoteInfo {
            client,
            parts: ClusterRemoteInfoParts::None,
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
    #[doc = "Creates an asynchronous call to the Cluster Remote Info API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
            .client
            .send(method, &path, query_string.as_ref(), body)
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
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cluster Reroute API](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-reroute.html). Allows to manually change the allocation of individual shards in the cluster."]
pub struct ClusterReroute<'a, B> {
    client: Elasticsearch,
    parts: ClusterRerouteParts,
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
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterReroute]"]
    pub fn new(client: Elasticsearch) -> Self {
        ClusterReroute {
            client,
            parts: ClusterRerouteParts::None,
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
    pub fn body<T>(self, body: T) -> ClusterReroute<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterReroute {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous call to the Cluster Reroute API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "dry_run")]
                dry_run: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "explain")]
                explain: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "metric", serialize_with = "crate::client::serialize_coll_qs")]
                metric: Option<&'a [&'a str]>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "retry_failed")]
                retry_failed: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Cluster State API"]
pub enum ClusterStateParts<'a> {
    #[doc = "No parts"]
    None,
    #[doc = "Metric"]
    Metric(&'a [&'a str]),
    #[doc = "Metric and Index"]
    MetricIndex(&'a [&'a str], &'a [&'a str]),
}
impl<'a> ClusterStateParts<'a> {
    #[doc = "Builds a relative URL path to the Cluster State API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterStateParts::None => "/_cluster/state".into(),
            ClusterStateParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(16usize + metric_str.len());
                p.push_str("/_cluster/state/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
            ClusterStateParts::MetricIndex(ref metric, ref index) => {
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
#[doc = "Builder for the [Cluster State API](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-state.html). Returns a comprehensive information about the state of the cluster."]
pub struct ClusterState<'a> {
    client: Elasticsearch,
    parts: ClusterStateParts<'a>,
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
    #[doc = "Creates a new instance of [ClusterState] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: ClusterStateParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Cluster State API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "wait_for_metadata_version")]
                wait_for_metadata_version: Option<i64>,
                #[serde(rename = "wait_for_timeout")]
                wait_for_timeout: Option<&'a str>,
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Cluster Stats API"]
pub enum ClusterStatsParts<'a> {
    #[doc = "No parts"]
    None,
    #[doc = "NodeId"]
    NodeId(&'a [&'a str]),
}
impl<'a> ClusterStatsParts<'a> {
    #[doc = "Builds a relative URL path to the Cluster Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterStatsParts::None => "/_cluster/stats".into(),
            ClusterStatsParts::NodeId(ref node_id) => {
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
#[doc = "Builder for the [Cluster Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-stats.html). Returns high-level overview of cluster statistics."]
pub struct ClusterStats<'a> {
    client: Elasticsearch,
    parts: ClusterStatsParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> ClusterStats<'a> {
    #[doc = "Creates a new instance of [ClusterStats] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: ClusterStatsParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Cluster Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "Namespace client for Cluster APIs"]
pub struct Cluster {
    client: Elasticsearch,
}
impl Cluster {
    #[doc = "Creates a new instance of [Cluster]"]
    pub fn new(client: Elasticsearch) -> Self {
        Self { client }
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
    pub fn health<'a>(&self, parts: ClusterHealthParts<'a>) -> ClusterHealth<'a> {
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
    pub fn state<'a>(&self, parts: ClusterStateParts<'a>) -> ClusterState<'a> {
        ClusterState::new(self.client.clone(), parts)
    }
    #[doc = "Returns high-level overview of cluster statistics."]
    pub fn stats<'a>(&self, parts: ClusterStatsParts<'a>) -> ClusterStats<'a> {
        ClusterStats::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Cluster APIs"]
    pub fn cluster(&self) -> Cluster {
        Cluster::new(self.clone())
    }
}