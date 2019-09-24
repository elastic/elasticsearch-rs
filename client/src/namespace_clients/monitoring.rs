

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct MonitoringBulkBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    interval: Option<String>,
    system_api_version: Option<String>,
    system_id: Option<String>,
}
impl MonitoringBulkBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MonitoringBulkBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MonitoringBulkBuilder {
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
#[doc = "Monitoring APIs"]
pub struct MonitoringClient {
    client: ElasticsearchClient,
}
impl MonitoringClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        MonitoringClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/es-monitoring.html"]
    pub fn bulk(&self) -> MonitoringBulkBuilder {
        MonitoringBulkBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Monitoring APIs"]
    pub fn monitoring(&self) -> MonitoringClient {
        MonitoringClient::new(self.clone())
    }
}
