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
use crate::{
    client::Elasticsearch, enums::*, error::ElasticsearchError, http_method::HttpMethod,
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Monitoring Bulk API"]
pub enum MonitoringBulkUrlParts {
    None,
    Type(String),
}
impl MonitoringBulkUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MonitoringBulkUrlParts::None => "/_monitoring/bulk".into(),
            MonitoringBulkUrlParts::Type(ref ty) => {
                let mut p = String::with_capacity(18usize + ty.len());
                p.push_str("/_monitoring/");
                p.push_str(ty.as_ref());
                p.push_str("/bulk");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Monitoring Bulk API"]
pub struct MonitoringBulk<B> {
    client: Elasticsearch,
    parts: MonitoringBulkUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    interval: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    system_api_version: Option<String>,
    system_id: Option<String>,
}
impl<B> MonitoringBulk<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MonitoringBulkUrlParts) -> Self {
        MonitoringBulk {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            interval: None,
            pretty: None,
            source: None,
            system_api_version: None,
            system_id: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MonitoringBulk<T>
    where
        T: Serialize,
    {
        MonitoringBulk {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            interval: self.interval,
            pretty: self.pretty,
            source: self.source,
            system_api_version: self.system_api_version,
            system_id: self.system_id,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Collection interval (e.g., '10s' or '10000ms') of the payload"]
    pub fn interval(mut self, interval: String) -> Self {
        self.interval = Some(interval);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "API Version of the monitored system"]
    pub fn system_api_version(mut self, system_api_version: String) -> Self {
        self.system_api_version = Some(system_api_version);
        self
    }
    #[doc = "Identifier of the monitored system"]
    pub fn system_id(mut self, system_id: String) -> Self {
        self.system_id = Some(system_id);
        self
    }
    #[doc = "Creates an asynchronous request to the Monitoring Bulk API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
                interval: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "system_api_version", skip_serializing_if = "Option::is_none")]
                system_api_version: Option<String>,
                #[serde(rename = "system_id", skip_serializing_if = "Option::is_none")]
                system_id: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                interval: self.interval,
                pretty: self.pretty,
                source: self.source,
                system_api_version: self.system_api_version,
                system_id: self.system_id,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
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
    pub fn bulk(&self, parts: MonitoringBulkUrlParts) -> MonitoringBulk<()> {
        MonitoringBulk::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Monitoring APIs"]
    pub fn monitoring(&self) -> Monitoring {
        Monitoring::new(self.clone())
    }
}