use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
#[doc = "Monitoring APIs"]
pub struct MonitoringNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl MonitoringNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        MonitoringNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/es-monitoring.html"]
    pub fn bulk(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_monitoring/bulk")
    }
}
impl ElasticsearchClient {
    #[doc = "Monitoring APIs"]
    pub fn monitoring(&self) -> MonitoringNamespaceClient {
        MonitoringNamespaceClient::new(self)
    }
}
