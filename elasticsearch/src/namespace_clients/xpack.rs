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
#[allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    enums::*,
    error::Error,
    http::{
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
};
use serde::Serialize;
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Xpack Info API"]
pub enum XpackInfoParts {
    #[doc = "No parts"]
    None,
}
impl XpackInfoParts {
    #[doc = "Builds a relative URL path to the Xpack Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            XpackInfoParts::None => "/_xpack".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Xpack Info API](https://www.elastic.co/guide/en/elasticsearch/reference/current/info-api.html)."]
pub struct XpackInfo<'a> {
    client: Elasticsearch,
    parts: XpackInfoParts,
    categories: Option<&'a [&'a str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> XpackInfo<'a> {
    #[doc = "Creates a new instance of [XpackInfo]"]
    pub fn new(client: Elasticsearch) -> Self {
        XpackInfo {
            client,
            parts: XpackInfoParts::None,
            categories: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Comma-separated list of info categories. Can be any of: build, license, features"]
    pub fn categories(mut self, categories: &'a [&'a str]) -> Self {
        self.categories = Some(categories);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Xpack Info API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(
                    rename = "categories",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                categories: Option<&'a [&'a str]>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                categories: self.categories,
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
#[doc = "API parts for the Xpack Usage API"]
pub enum XpackUsageParts {
    #[doc = "No parts"]
    None,
}
impl XpackUsageParts {
    #[doc = "Builds a relative URL path to the Xpack Usage API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            XpackUsageParts::None => "/_xpack/usage".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Xpack Usage API](Retrieve information about xpack features usage)."]
pub struct XpackUsage<'a> {
    client: Elasticsearch,
    parts: XpackUsageParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> XpackUsage<'a> {
    #[doc = "Creates a new instance of [XpackUsage]"]
    pub fn new(client: Elasticsearch) -> Self {
        XpackUsage {
            client,
            parts: XpackUsageParts::None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Specify timeout for watch write operation"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Xpack Usage API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
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
#[doc = "Namespace client for X-Pack APIs"]
pub struct Xpack {
    client: Elasticsearch,
}
impl Xpack {
    #[doc = "Creates a new instance of [Xpack]"]
    pub fn new(client: Elasticsearch) -> Self {
        Self { client }
    }
    pub fn info<'a>(&self) -> XpackInfo<'a> {
        XpackInfo::new(self.client.clone())
    }
    pub fn usage<'a>(&self) -> XpackUsage<'a> {
        XpackUsage::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for X-Pack APIs"]
    pub fn xpack(&self) -> Xpack {
        Xpack::new(self.clone())
    }
}