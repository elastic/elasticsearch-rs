use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
#[doc = "Nodes APIs"]
pub struct NodesNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl NodesNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        NodesNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-hot-threads.html"]
    pub fn hot_threads(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_nodes/hot_threads")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-info.html"]
    pub fn info(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_nodes")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/secure-settings.html#reloadable-secure-settings"]
    pub fn reload_secure_settings(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_nodes/reload_secure_settings")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html"]
    pub fn stats(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_nodes/stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-usage.html"]
    pub fn usage(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_nodes/usage")
    }
}
impl ElasticsearchClient {
    #[doc = "Nodes APIs"]
    pub fn nodes(&self) -> NodesNamespaceClient {
        NodesNamespaceClient::new(self)
    }
}
