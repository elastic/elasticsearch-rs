

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[Default]
pub struct MonitoringBulkRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    interval: &'a str,
    system_api_version: &'a str,
    system_id: &'a str,
}
impl<'a> MonitoringBulkRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        MonitoringBulkRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MonitoringBulkRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
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
    pub fn bulk(&self) -> MonitoringBulkRequestBuilder {
        MonitoringBulkRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Monitoring APIs"]
    pub fn monitoring(&self) -> MonitoringNamespaceClient {
        MonitoringNamespaceClient::new(self)
    }
}
