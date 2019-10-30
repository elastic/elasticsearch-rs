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
pub struct XpackInfo {
    client: Elasticsearch,
    categories: Option<Vec<String>>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl XpackInfo {
    pub fn new(client: Elasticsearch) -> Self {
        XpackInfo {
            client,
            ..Default::default()
        }
    }
    #[doc = "Comma-separated list of info categories. Can be any of: build, license, features"]
    pub fn categories(mut self, categories: Option<Vec<String>>) -> Self {
        self.categories = categories;
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
}
impl Sender for XpackInfo {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_xpack";
        let method = HttpMethod::Get;
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "categories")]
                categories: Option<Vec<String>>,
            }
            let query_params = QueryParamsStruct {
                categories: self.categories,
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
#[derive(Default)]
pub struct XpackUsage {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl XpackUsage {
    pub fn new(client: Elasticsearch) -> Self {
        XpackUsage {
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
    #[doc = "Specify timeout for watch write operation"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
}
impl Sender for XpackUsage {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_xpack/usage";
        let method = HttpMethod::Get;
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                master_timeout: self.master_timeout,
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
#[doc = "Xpack APIs"]
pub struct Xpack {
    client: Elasticsearch,
}
impl Xpack {
    pub fn new(client: Elasticsearch) -> Self {
        Xpack { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/info-api.html"]
    pub fn info(&self) -> XpackInfo {
        XpackInfo::new(self.client.clone())
    }
    #[doc = "Retrieve information about xpack features usage"]
    pub fn usage(&self) -> XpackUsage {
        XpackUsage::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Xpack APIs"]
    pub fn xpack(&self) -> Xpack {
        Xpack::new(self.clone())
    }
}