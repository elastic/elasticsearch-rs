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
    client::{Elasticsearch, Sender},
    enums::*,
    error::ElasticsearchError,
    http_method::HttpMethod,
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
#[derive(Clone, Debug)]
pub struct MonitoringBulk<B> {
    client: Elasticsearch,
    body: Option<B>,
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
impl<B> MonitoringBulk<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        MonitoringBulk {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            interval: None,
            pretty: None,
            source: None,
            system_api_version: None,
            system_id: None,
            ty: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
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
    #[doc = "Default document type for items which don't provide one"]
    pub fn ty(mut self, ty: Option<String>) -> Self {
        self.ty = ty;
        self
    }
}
impl<B> Sender for MonitoringBulk<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.ty {
            Some(ty) => {
                let ty = ty;
                let mut p = String::with_capacity(18usize + ty.len());
                p.push_str("/_monitoring/");
                p.push_str(ty.as_ref());
                p.push_str("/bulk");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_monitoring/bulk"),
        };
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
            .send(method, &path, query_string.as_ref(), body)?;
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
    pub fn bulk<B>(&self) -> MonitoringBulk<B>
    where
        B: Serialize,
    {
        MonitoringBulk::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Monitoring APIs"]
    pub fn monitoring(&self) -> Monitoring {
        Monitoring::new(self.clone())
    }
}