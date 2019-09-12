use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct ClusterAllocationExplainRequest<'a> {
    include_disk_info: Option<&'a bool>,
    include_yes_decisions: Option<&'a bool>,
}
pub struct ClusterAllocationExplainRequestBuilder<'a> {
    include_disk_info: Option<&'a bool>,
    include_yes_decisions: Option<&'a bool>,
}
impl<'a> ClusterAllocationExplainRequestBuilder<'a> {
    pub fn build(&self) -> ClusterAllocationExplainRequest<'a> {
        ClusterAllocationExplainRequest {
            include_disk_info: self.include_disk_info,
            include_yes_decisions: self.include_yes_decisions,
        }
    }
}
pub struct ClusterGetSettingsRequest<'a> {
    flat_settings: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct ClusterGetSettingsRequestBuilder<'a> {
    flat_settings: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> ClusterGetSettingsRequestBuilder<'a> {
    pub fn build(&self) -> ClusterGetSettingsRequest<'a> {
        ClusterGetSettingsRequest {
            flat_settings: self.flat_settings,
            include_defaults: self.include_defaults,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
pub struct ClusterHealthRequest<'a> {
    expand_wildcards: Option<&'a i32>,
    level: Option<&'a i32>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
    wait_for_events: Option<&'a i32>,
    wait_for_no_initializing_shards: Option<&'a bool>,
    wait_for_no_relocating_shards: Option<&'a bool>,
    wait_for_nodes: &'a String,
    wait_for_status: Option<&'a i32>,
}
pub struct ClusterHealthRequestBuilder<'a> {
    expand_wildcards: Option<&'a i32>,
    level: Option<&'a i32>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
    wait_for_events: Option<&'a i32>,
    wait_for_no_initializing_shards: Option<&'a bool>,
    wait_for_no_relocating_shards: Option<&'a bool>,
    wait_for_nodes: &'a String,
    wait_for_status: Option<&'a i32>,
}
impl<'a> ClusterHealthRequestBuilder<'a> {
    pub fn build(&self) -> ClusterHealthRequest<'a> {
        ClusterHealthRequest {
            expand_wildcards: self.expand_wildcards,
            level: self.level,
            local: self.local,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
            wait_for_events: self.wait_for_events,
            wait_for_no_initializing_shards: self.wait_for_no_initializing_shards,
            wait_for_no_relocating_shards: self.wait_for_no_relocating_shards,
            wait_for_nodes: self.wait_for_nodes,
            wait_for_status: self.wait_for_status,
        }
    }
}
pub struct ClusterPendingTasksRequest<'a> {
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
pub struct ClusterPendingTasksRequestBuilder<'a> {
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
impl<'a> ClusterPendingTasksRequestBuilder<'a> {
    pub fn build(&self) -> ClusterPendingTasksRequest<'a> {
        ClusterPendingTasksRequest {
            local: self.local,
            master_timeout: self.master_timeout,
        }
    }
}
pub struct ClusterPutSettingsRequest<'a> {
    flat_settings: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct ClusterPutSettingsRequestBuilder<'a> {
    flat_settings: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> ClusterPutSettingsRequestBuilder<'a> {
    pub fn build(&self) -> ClusterPutSettingsRequest<'a> {
        ClusterPutSettingsRequest {
            flat_settings: self.flat_settings,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
pub struct ClusterRemoteInfoRequest<'a> {}
pub struct ClusterRemoteInfoRequestBuilder<'a> {}
impl<'a> ClusterRemoteInfoRequestBuilder<'a> {
    pub fn build(&self) -> ClusterRemoteInfoRequest<'a> {
        ClusterRemoteInfoRequest {}
    }
}
pub struct ClusterRerouteRequest<'a> {
    dry_run: Option<&'a bool>,
    explain: Option<&'a bool>,
    master_timeout: &'a String,
    metric: &'a Vec<String>,
    retry_failed: Option<&'a bool>,
    timeout: &'a String,
}
pub struct ClusterRerouteRequestBuilder<'a> {
    dry_run: Option<&'a bool>,
    explain: Option<&'a bool>,
    master_timeout: &'a String,
    metric: &'a Vec<String>,
    retry_failed: Option<&'a bool>,
    timeout: &'a String,
}
impl<'a> ClusterRerouteRequestBuilder<'a> {
    pub fn build(&self) -> ClusterRerouteRequest<'a> {
        ClusterRerouteRequest {
            dry_run: self.dry_run,
            explain: self.explain,
            master_timeout: self.master_timeout,
            metric: self.metric,
            retry_failed: self.retry_failed,
            timeout: self.timeout,
        }
    }
}
pub struct ClusterStateRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    wait_for_metadata_version: Option<&'a i64>,
    wait_for_timeout: &'a String,
}
pub struct ClusterStateRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    wait_for_metadata_version: Option<&'a i64>,
    wait_for_timeout: &'a String,
}
impl<'a> ClusterStateRequestBuilder<'a> {
    pub fn build(&self) -> ClusterStateRequest<'a> {
        ClusterStateRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            flat_settings: self.flat_settings,
            ignore_unavailable: self.ignore_unavailable,
            local: self.local,
            master_timeout: self.master_timeout,
            wait_for_metadata_version: self.wait_for_metadata_version,
            wait_for_timeout: self.wait_for_timeout,
        }
    }
}
pub struct ClusterStatsRequest<'a> {
    flat_settings: Option<&'a bool>,
    timeout: &'a String,
}
pub struct ClusterStatsRequestBuilder<'a> {
    flat_settings: Option<&'a bool>,
    timeout: &'a String,
}
impl<'a> ClusterStatsRequestBuilder<'a> {
    pub fn build(&self) -> ClusterStatsRequest<'a> {
        ClusterStatsRequest {
            flat_settings: self.flat_settings,
            timeout: self.timeout,
        }
    }
}
#[doc = "Cluster APIs"]
pub struct ClusterNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> ClusterNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        ClusterNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-allocation-explain.html"]
    pub fn allocation_explain(
        &self,
        request: &ClusterAllocationExplainRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_cluster/allocation/explain")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html"]
    pub fn get_settings(&self, request: &ClusterGetSettingsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cluster/settings")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-health.html"]
    pub fn health(&self, request: &ClusterHealthRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cluster/health")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-pending.html"]
    pub fn pending_tasks(&self, request: &ClusterPendingTasksRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cluster/pending_tasks")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html"]
    pub fn put_settings(&self, request: &ClusterPutSettingsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_cluster/settings")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-remote-info.html"]
    pub fn remote_info(&self, request: &ClusterRemoteInfoRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_remote/info")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-reroute.html"]
    pub fn reroute(&self, request: &ClusterRerouteRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_cluster/reroute")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-state.html"]
    pub fn state(&self, request: &ClusterStateRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cluster/state")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-stats.html"]
    pub fn stats(&self, request: &ClusterStatsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cluster/stats")
    }
}
impl ElasticsearchClient {
    #[doc = "Cluster APIs"]
    pub fn cluster(&self) -> ClusterNamespaceClient {
        ClusterNamespaceClient::new(self)
    }
}
