use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct MonitoringBulkRequest<'a> {
    interval: &'a String,
    system_api_version: &'a String,
    system_id: &'a String,
}
pub struct MonitoringBulkRequestBuilder<'a> {
    interval: &'a String,
    system_api_version: &'a String,
    system_id: &'a String,
}
impl<'a> MonitoringBulkRequestBuilder<'a> {
    pub fn build(&self) -> MonitoringBulkRequest<'a> {
        MonitoringBulkRequest {
            interval: self.interval,
            system_api_version: self.system_api_version,
            system_id: self.system_id,
        }
    }
}
#[doc = "Monitoring APIs"]
pub struct MonitoringNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> MonitoringNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        MonitoringNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/es-monitoring.html"]
    pub fn bulk(&self, request: &MonitoringBulkRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_monitoring/bulk")
    }
}
impl ElasticsearchClient {
    #[doc = "Monitoring APIs"]
    pub fn monitoring(&self) -> MonitoringNamespaceClient {
        MonitoringNamespaceClient::new(self)
    }
}
