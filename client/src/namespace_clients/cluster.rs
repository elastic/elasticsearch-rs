use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
#[doc = "Cluster APIs"]
pub struct ClusterNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl ClusterNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        ClusterNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-allocation-explain.html"]
    pub fn allocation_explain(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_cluster/allocation/explain")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html"]
    pub fn get_settings(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cluster/settings")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-health.html"]
    pub fn health(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cluster/health")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-pending.html"]
    pub fn pending_tasks(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cluster/pending_tasks")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html"]
    pub fn put_settings(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_cluster/settings")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-remote-info.html"]
    pub fn remote_info(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_remote/info")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-reroute.html"]
    pub fn reroute(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_cluster/reroute")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-state.html"]
    pub fn state(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cluster/state")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-stats.html"]
    pub fn stats(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cluster/stats")
    }
}
impl ElasticsearchClient {
    #[doc = "Cluster APIs"]
    pub fn cluster(&self) -> ClusterNamespaceClient {
        ClusterNamespaceClient::new(self)
    }
}
