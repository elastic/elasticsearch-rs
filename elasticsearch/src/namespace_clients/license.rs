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
pub struct LicenseDelete {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl LicenseDelete {
    pub fn new(client: Elasticsearch) -> Self {
        LicenseDelete {
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
}
impl Sender for LicenseDelete {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_license";
        let method = HttpMethod::Delete;
        let query_params = None::<()>;
        let body: Option<()> = None;
        let response = self
            .client
            .send(method, path, query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct LicenseGet {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl LicenseGet {
    pub fn new(client: Elasticsearch) -> Self {
        LicenseGet {
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
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
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
impl Sender for LicenseGet {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_license";
        let method = HttpMethod::Get;
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "local")]
                local: Option<bool>,
            }
            let query_params = QueryParamsStruct { local: self.local };
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
pub struct LicenseGetBasicStatus {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl LicenseGetBasicStatus {
    pub fn new(client: Elasticsearch) -> Self {
        LicenseGetBasicStatus {
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
}
impl Sender for LicenseGetBasicStatus {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_license/basic_status";
        let method = HttpMethod::Get;
        let query_params = None::<()>;
        let body: Option<()> = None;
        let response = self
            .client
            .send(method, path, query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct LicenseGetTrialStatus {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl LicenseGetTrialStatus {
    pub fn new(client: Elasticsearch) -> Self {
        LicenseGetTrialStatus {
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
}
impl Sender for LicenseGetTrialStatus {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_license/trial_status";
        let method = HttpMethod::Get;
        let query_params = None::<()>;
        let body: Option<()> = None;
        let response = self
            .client
            .send(method, path, query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct LicensePost {
    client: Elasticsearch,
    acknowledge: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl LicensePost {
    pub fn new(client: Elasticsearch) -> Self {
        LicensePost {
            client,
            ..Default::default()
        }
    }
    #[doc = "whether the user has acknowledged acknowledge messages (default: false)"]
    pub fn acknowledge(mut self, acknowledge: Option<bool>) -> Self {
        self.acknowledge = acknowledge;
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
impl Sender for LicensePost {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_license";
        let method = HttpMethod::Post;
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "acknowledge")]
                acknowledge: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                acknowledge: self.acknowledge,
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
pub struct LicensePostStartBasic {
    client: Elasticsearch,
    acknowledge: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl LicensePostStartBasic {
    pub fn new(client: Elasticsearch) -> Self {
        LicensePostStartBasic {
            client,
            ..Default::default()
        }
    }
    #[doc = "whether the user has acknowledged acknowledge messages (default: false)"]
    pub fn acknowledge(mut self, acknowledge: Option<bool>) -> Self {
        self.acknowledge = acknowledge;
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
impl Sender for LicensePostStartBasic {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_license/start_basic";
        let method = HttpMethod::Post;
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "acknowledge")]
                acknowledge: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                acknowledge: self.acknowledge,
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
pub struct LicensePostStartTrial {
    client: Elasticsearch,
    acknowledge: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    ty: Option<String>,
}
impl LicensePostStartTrial {
    pub fn new(client: Elasticsearch) -> Self {
        LicensePostStartTrial {
            client,
            ..Default::default()
        }
    }
    #[doc = "whether the user has acknowledged acknowledge messages (default: false)"]
    pub fn acknowledge(mut self, acknowledge: Option<bool>) -> Self {
        self.acknowledge = acknowledge;
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
    #[doc = "The type of trial license to generate (default: \"trial\")"]
    pub fn ty(mut self, ty: Option<String>) -> Self {
        self.ty = ty;
        self
    }
}
impl Sender for LicensePostStartTrial {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_license/start_trial";
        let method = HttpMethod::Post;
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "acknowledge")]
                acknowledge: Option<bool>,
                #[serde(rename = "type")]
                ty: Option<String>,
            }
            let query_params = QueryParamsStruct {
                acknowledge: self.acknowledge,
                ty: self.ty,
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
#[doc = "License APIs"]
pub struct License {
    client: Elasticsearch,
}
impl License {
    pub fn new(client: Elasticsearch) -> Self {
        License { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/delete-license.html"]
    pub fn delete(&self) -> LicenseDelete {
        LicenseDelete::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-license.html"]
    pub fn get(&self) -> LicenseGet {
        LicenseGet::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-basic-status.html"]
    pub fn get_basic_status(&self) -> LicenseGetBasicStatus {
        LicenseGetBasicStatus::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-trial-status.html"]
    pub fn get_trial_status(&self) -> LicenseGetTrialStatus {
        LicenseGetTrialStatus::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/update-license.html"]
    pub fn post(&self) -> LicensePost {
        LicensePost::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/start-basic.html"]
    pub fn post_start_basic(&self) -> LicensePostStartBasic {
        LicensePostStartBasic::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/start-trial.html"]
    pub fn post_start_trial(&self) -> LicensePostStartTrial {
        LicensePostStartTrial::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "License APIs"]
    pub fn license(&self) -> License {
        License::new(self.clone())
    }
}