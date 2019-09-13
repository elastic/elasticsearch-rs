

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct ClusterAllocationExplainRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    include_disk_info: Option<&'a bool>,
    include_yes_decisions: Option<&'a bool>,
}
impl<'a> ClusterAllocationExplainRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        ClusterAllocationExplainRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for ClusterAllocationExplainRequestBuilder<'a> {
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
pub struct ClusterGetSettingsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    flat_settings: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    master_timeout: &'a str,
    timeout: &'a str,
}
impl<'a> ClusterGetSettingsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        ClusterGetSettingsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for ClusterGetSettingsRequestBuilder<'a> {
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
pub struct ClusterHealthRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    expand_wildcards: Option<&'a i32>,
    level: Option<&'a i32>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    timeout: &'a str,
    wait_for_active_shards: &'a str,
    wait_for_events: Option<&'a i32>,
    wait_for_no_initializing_shards: Option<&'a bool>,
    wait_for_no_relocating_shards: Option<&'a bool>,
    wait_for_nodes: &'a str,
    wait_for_status: Option<&'a i32>,
}
impl<'a> ClusterHealthRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        ClusterHealthRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for ClusterHealthRequestBuilder<'a> {
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
pub struct ClusterPendingTasksRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    local: Option<&'a bool>,
    master_timeout: &'a str,
}
impl<'a> ClusterPendingTasksRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        ClusterPendingTasksRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for ClusterPendingTasksRequestBuilder<'a> {
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
pub struct ClusterPutSettingsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    flat_settings: Option<&'a bool>,
    master_timeout: &'a str,
    timeout: &'a str,
}
impl<'a> ClusterPutSettingsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        ClusterPutSettingsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for ClusterPutSettingsRequestBuilder<'a> {
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
pub struct ClusterRemoteInfoRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> ClusterRemoteInfoRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        ClusterRemoteInfoRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for ClusterRemoteInfoRequestBuilder<'a> {
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
pub struct ClusterRerouteRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    dry_run: Option<&'a bool>,
    explain: Option<&'a bool>,
    master_timeout: &'a str,
    metric: Option<&'a Vec<String>>,
    retry_failed: Option<&'a bool>,
    timeout: &'a str,
}
impl<'a> ClusterRerouteRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        ClusterRerouteRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for ClusterRerouteRequestBuilder<'a> {
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
pub struct ClusterStateRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    wait_for_metadata_version: Option<&'a i64>,
    wait_for_timeout: &'a str,
}
impl<'a> ClusterStateRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        ClusterStateRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for ClusterStateRequestBuilder<'a> {
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
pub struct ClusterStatsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    flat_settings: Option<&'a bool>,
    timeout: &'a str,
}
impl<'a> ClusterStatsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        ClusterStatsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for ClusterStatsRequestBuilder<'a> {
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
pub struct ClusterNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> ClusterNamespaceClient<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        ClusterNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-allocation-explain.html"]
    pub fn allocation_explain(&self) -> ClusterAllocationExplainRequestBuilder {
        ClusterAllocationExplainRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html"]
    pub fn get_settings(&self) -> ClusterGetSettingsRequestBuilder {
        ClusterGetSettingsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-health.html"]
    pub fn health(&self) -> ClusterHealthRequestBuilder {
        ClusterHealthRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-pending.html"]
    pub fn pending_tasks(&self) -> ClusterPendingTasksRequestBuilder {
        ClusterPendingTasksRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html"]
    pub fn put_settings(&self) -> ClusterPutSettingsRequestBuilder {
        ClusterPutSettingsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-remote-info.html"]
    pub fn remote_info(&self) -> ClusterRemoteInfoRequestBuilder {
        ClusterRemoteInfoRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-reroute.html"]
    pub fn reroute(&self) -> ClusterRerouteRequestBuilder {
        ClusterRerouteRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-state.html"]
    pub fn state(&self) -> ClusterStateRequestBuilder {
        ClusterStateRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-stats.html"]
    pub fn stats(&self) -> ClusterStatsRequestBuilder {
        ClusterStatsRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Cluster APIs"]
    pub fn cluster(&self) -> ClusterNamespaceClient {
        ClusterNamespaceClient::new(self)
    }
}
