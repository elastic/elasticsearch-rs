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
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Collection interval (e.g., '10s' or '10000ms') of the payload"]
    pub fn interval(mut self, interval: Option<String>) -> Self {
        self.interval = interval;
        self
    }
    #[doc = "API Version of the monitored system"]
    pub fn system_api_version(mut self, system_api_version: Option<String>) -> Self {
        self.system_api_version = system_api_version;
        self
    }
    #[doc = "Identifier of the monitored system"]
    pub fn system_id(mut self, system_id: Option<String>) -> Self {
        self.system_id = system_id;
        self
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
        MonitoringBulkBuilder::new(self.client.clone())
    }
}
impl ElasticsearchClient {
    #[doc = "Monitoring APIs"]
    pub fn monitoring(&self) -> MonitoringClient {
        MonitoringClient::new(self.clone())
    }
}
