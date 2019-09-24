

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct ClusterAllocationExplainBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    include_disk_info: Option<bool>,
    include_yes_decisions: Option<bool>,
}
impl ClusterAllocationExplainBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        ClusterAllocationExplainBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for ClusterAllocationExplainBuilder {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct ClusterGetSettingsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    flat_settings: Option<bool>,
    include_defaults: Option<bool>,
    master_timeout: Option<String>,
    timeout: Option<String>,
}
impl ClusterGetSettingsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        ClusterGetSettingsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for ClusterGetSettingsBuilder {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct ClusterHealthBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    expand_wildcards: Option<i32>,
    level: Option<i32>,
    local: Option<bool>,
    master_timeout: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
    wait_for_events: Option<i32>,
    wait_for_no_initializing_shards: Option<bool>,
    wait_for_no_relocating_shards: Option<bool>,
    wait_for_nodes: Option<String>,
    wait_for_status: Option<i32>,
}
impl ClusterHealthBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        ClusterHealthBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for ClusterHealthBuilder {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct ClusterPendingTasksBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    local: Option<bool>,
    master_timeout: Option<String>,
}
impl ClusterPendingTasksBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        ClusterPendingTasksBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for ClusterPendingTasksBuilder {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct ClusterPutSettingsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    flat_settings: Option<bool>,
    master_timeout: Option<String>,
    timeout: Option<String>,
}
impl ClusterPutSettingsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        ClusterPutSettingsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for ClusterPutSettingsBuilder {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct ClusterRemoteInfoBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl ClusterRemoteInfoBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        ClusterRemoteInfoBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for ClusterRemoteInfoBuilder {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct ClusterRerouteBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    dry_run: Option<bool>,
    explain: Option<bool>,
    master_timeout: Option<String>,
    metric: Option<Vec<String>>,
    retry_failed: Option<bool>,
    timeout: Option<String>,
}
impl ClusterRerouteBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        ClusterRerouteBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for ClusterRerouteBuilder {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct ClusterStateBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    flat_settings: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    wait_for_metadata_version: Option<i64>,
    wait_for_timeout: Option<String>,
}
impl ClusterStateBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        ClusterStateBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for ClusterStateBuilder {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct ClusterStatsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    flat_settings: Option<bool>,
    timeout: Option<String>,
}
impl ClusterStatsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        ClusterStatsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for ClusterStatsBuilder {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[doc = "Cluster APIs"]
pub struct ClusterClient {
    client: ElasticsearchClient,
}
impl ClusterClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        ClusterClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-allocation-explain.html"]
    pub fn allocation_explain(&self) -> ClusterAllocationExplainBuilder {
        ClusterAllocationExplainBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html"]
    pub fn get_settings(&self) -> ClusterGetSettingsBuilder {
        ClusterGetSettingsBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-health.html"]
    pub fn health(&self) -> ClusterHealthBuilder {
        ClusterHealthBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-pending.html"]
    pub fn pending_tasks(&self) -> ClusterPendingTasksBuilder {
        ClusterPendingTasksBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html"]
    pub fn put_settings(&self) -> ClusterPutSettingsBuilder {
        ClusterPutSettingsBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-remote-info.html"]
    pub fn remote_info(&self) -> ClusterRemoteInfoBuilder {
        ClusterRemoteInfoBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-reroute.html"]
    pub fn reroute(&self) -> ClusterRerouteBuilder {
        ClusterRerouteBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-state.html"]
    pub fn state(&self) -> ClusterStateBuilder {
        ClusterStateBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-stats.html"]
    pub fn stats(&self) -> ClusterStatsBuilder {
        ClusterStatsBuilder::new(self.client.clone())
    }
}
impl ElasticsearchClient {
    #[doc = "Cluster APIs"]
    pub fn cluster(&self) -> ClusterClient {
        ClusterClient::new(self.clone())
    }
}
