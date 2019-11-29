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
#[doc = "Url parts for the Slm Delete Lifecycle API"]
pub enum SlmDeleteLifecycleUrlParts {
    PolicyId(String),
}
impl SlmDeleteLifecycleUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SlmDeleteLifecycleUrlParts::PolicyId(ref policy_id) => {
                let mut p = String::with_capacity(13usize + policy_id.len());
                p.push_str("/_slm/policy/");
                p.push_str(policy_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Slm Delete Lifecycle API"]
pub struct SlmDeleteLifecycle {
    client: Elasticsearch,
    parts: SlmDeleteLifecycleUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SlmDeleteLifecycle {
    pub fn new(client: Elasticsearch, parts: SlmDeleteLifecycleUrlParts) -> Self {
        SlmDeleteLifecycle {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
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
    #[doc = "Creates an asynchronous request to the Slm Delete Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
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
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Slm Execute Lifecycle API"]
pub enum SlmExecuteLifecycleUrlParts {
    PolicyId(String),
}
impl SlmExecuteLifecycleUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SlmExecuteLifecycleUrlParts::PolicyId(ref policy_id) => {
                let mut p = String::with_capacity(22usize + policy_id.len());
                p.push_str("/_slm/policy/");
                p.push_str(policy_id.as_ref());
                p.push_str("/_execute");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Slm Execute Lifecycle API"]
pub struct SlmExecuteLifecycle<B> {
    client: Elasticsearch,
    parts: SlmExecuteLifecycleUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> SlmExecuteLifecycle<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SlmExecuteLifecycleUrlParts) -> Self {
        SlmExecuteLifecycle {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SlmExecuteLifecycle<T>
    where
        T: Serialize,
    {
        SlmExecuteLifecycle {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
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
    #[doc = "Creates an asynchronous request to the Slm Execute Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
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
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Slm Get Lifecycle API"]
pub enum SlmGetLifecycleUrlParts {
    PolicyId(String),
    None,
}
impl SlmGetLifecycleUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SlmGetLifecycleUrlParts::PolicyId(ref policy_id) => {
                let mut p = String::with_capacity(13usize + policy_id.len());
                p.push_str("/_slm/policy/");
                p.push_str(policy_id.as_ref());
                p.into()
            }
            SlmGetLifecycleUrlParts::None => "/_slm/policy".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Slm Get Lifecycle API"]
pub struct SlmGetLifecycle {
    client: Elasticsearch,
    parts: SlmGetLifecycleUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SlmGetLifecycle {
    pub fn new(client: Elasticsearch, parts: SlmGetLifecycleUrlParts) -> Self {
        SlmGetLifecycle {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
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
    #[doc = "Creates an asynchronous request to the Slm Get Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
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
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Slm Put Lifecycle API"]
pub enum SlmPutLifecycleUrlParts {
    PolicyId(String),
}
impl SlmPutLifecycleUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SlmPutLifecycleUrlParts::PolicyId(ref policy_id) => {
                let mut p = String::with_capacity(13usize + policy_id.len());
                p.push_str("/_slm/policy/");
                p.push_str(policy_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Slm Put Lifecycle API"]
pub struct SlmPutLifecycle<B> {
    client: Elasticsearch,
    parts: SlmPutLifecycleUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> SlmPutLifecycle<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SlmPutLifecycleUrlParts) -> Self {
        SlmPutLifecycle {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SlmPutLifecycle<T>
    where
        T: Serialize,
    {
        SlmPutLifecycle {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
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
    #[doc = "Creates an asynchronous request to the Slm Put Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
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
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
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
#[doc = "Slm APIs"]
pub struct Slm {
    client: Elasticsearch,
}
impl Slm {
    pub fn new(client: Elasticsearch) -> Self {
        Slm { client }
    }
    pub fn delete_lifecycle(&self, parts: SlmDeleteLifecycleUrlParts) -> SlmDeleteLifecycle {
        SlmDeleteLifecycle::new(self.client.clone(), parts)
    }
    pub fn execute_lifecycle(&self, parts: SlmExecuteLifecycleUrlParts) -> SlmExecuteLifecycle<()> {
        SlmExecuteLifecycle::new(self.client.clone(), parts)
    }
    pub fn get_lifecycle(&self, parts: SlmGetLifecycleUrlParts) -> SlmGetLifecycle {
        SlmGetLifecycle::new(self.client.clone(), parts)
    }
    pub fn put_lifecycle(&self, parts: SlmPutLifecycleUrlParts) -> SlmPutLifecycle<()> {
        SlmPutLifecycle::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Slm APIs"]
    pub fn slm(&self) -> Slm {
        Slm::new(self.clone())
    }
}