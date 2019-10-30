// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
use super::super::client::Elasticsearch;
use super::super::enums::*;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::error::ElasticsearchError;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
#[derive(Default)]
pub struct MonitoringBulk {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    interval: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    system_api_version: Option<String>,
    system_id: Option<String>,
    ty: Option<String>,
}
impl MonitoringBulk {
    pub fn new(client: Elasticsearch) -> Self {
        MonitoringBulk {
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
    #[doc = "Collection interval (e.g., '10s' or '10000ms') of the payload"]
    pub fn interval(mut self, interval: Option<String>) -> Self {
        self.interval = interval;
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
impl Sender for MonitoringBulk {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_monitoring/bulk";
        let method = HttpMethod::Post;
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "interval")]
                interval: Option<String>,
                #[serde(rename = "system_api_version")]
                system_api_version: Option<String>,
                #[serde(rename = "system_id")]
                system_id: Option<String>,
            }
            let query_params = QueryParamsStruct {
                interval: self.interval,
                system_api_version: self.system_api_version,
                system_id: self.system_id,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(method, path, query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[doc = "Monitoring APIs"]
pub struct Monitoring {
    client: Elasticsearch,
}
impl Monitoring {
    pub fn new(client: Elasticsearch) -> Self {
        Monitoring { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/es-monitoring.html"]
    pub fn bulk(&self) -> MonitoringBulk {
        MonitoringBulk::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Monitoring APIs"]
    pub fn monitoring(&self) -> Monitoring {
        Monitoring::new(self.clone())
    }
}