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
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue},
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
    params::*,
};
use serde::Serialize;
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the License Delete API"]
pub enum LicenseDeleteParts {
    #[doc = "No parts"]
    None,
}
impl LicenseDeleteParts {
    #[doc = "Builds a relative URL path to the License Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicenseDeleteParts::None => "/_license".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/master/delete-license.html)."]
pub struct LicenseDelete<'a> {
    client: Elasticsearch,
    parts: LicenseDeleteParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> LicenseDelete<'a> {
    #[doc = "Creates a new instance of [LicenseDelete]"]
    pub fn new(client: Elasticsearch) -> Self {
        LicenseDelete {
            client,
            parts: LicenseDeleteParts::None,
            headers: HeaderMap::new(),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
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
    #[doc = "Creates an asynchronous call to the License Delete API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
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
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the License Get API"]
pub enum LicenseGetParts {
    #[doc = "No parts"]
    None,
}
impl LicenseGetParts {
    #[doc = "Builds a relative URL path to the License Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicenseGetParts::None => "/_license".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Get API](https://www.elastic.co/guide/en/elasticsearch/reference/master/get-license.html)."]
pub struct LicenseGet<'a> {
    client: Elasticsearch,
    parts: LicenseGetParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> LicenseGet<'a> {
    #[doc = "Creates a new instance of [LicenseGet]"]
    pub fn new(client: Elasticsearch) -> Self {
        LicenseGet {
            client,
            parts: LicenseGetParts::None,
            headers: HeaderMap::new(),
            error_trace: None,
            filter_path: None,
            human: None,
            local: None,
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
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the License Get API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
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
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                local: self.local,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the License Get Basic Status API"]
pub enum LicenseGetBasicStatusParts {
    #[doc = "No parts"]
    None,
}
impl LicenseGetBasicStatusParts {
    #[doc = "Builds a relative URL path to the License Get Basic Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicenseGetBasicStatusParts::None => "/_license/basic_status".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Get Basic Status API](https://www.elastic.co/guide/en/elasticsearch/reference/master/get-basic-status.html)."]
pub struct LicenseGetBasicStatus<'a> {
    client: Elasticsearch,
    parts: LicenseGetBasicStatusParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> LicenseGetBasicStatus<'a> {
    #[doc = "Creates a new instance of [LicenseGetBasicStatus]"]
    pub fn new(client: Elasticsearch) -> Self {
        LicenseGetBasicStatus {
            client,
            parts: LicenseGetBasicStatusParts::None,
            headers: HeaderMap::new(),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
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
    #[doc = "Creates an asynchronous call to the License Get Basic Status API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
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
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the License Get Trial Status API"]
pub enum LicenseGetTrialStatusParts {
    #[doc = "No parts"]
    None,
}
impl LicenseGetTrialStatusParts {
    #[doc = "Builds a relative URL path to the License Get Trial Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicenseGetTrialStatusParts::None => "/_license/trial_status".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Get Trial Status API](https://www.elastic.co/guide/en/elasticsearch/reference/master/get-trial-status.html)."]
pub struct LicenseGetTrialStatus<'a> {
    client: Elasticsearch,
    parts: LicenseGetTrialStatusParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> LicenseGetTrialStatus<'a> {
    #[doc = "Creates a new instance of [LicenseGetTrialStatus]"]
    pub fn new(client: Elasticsearch) -> Self {
        LicenseGetTrialStatus {
            client,
            parts: LicenseGetTrialStatusParts::None,
            headers: HeaderMap::new(),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
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
    #[doc = "Creates an asynchronous call to the License Get Trial Status API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
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
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the License Post API"]
pub enum LicensePostParts {
    #[doc = "No parts"]
    None,
}
impl LicensePostParts {
    #[doc = "Builds a relative URL path to the License Post API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicensePostParts::None => "/_license".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Post API](https://www.elastic.co/guide/en/elasticsearch/reference/master/update-license.html)."]
pub struct LicensePost<'a, B> {
    client: Elasticsearch,
    parts: LicensePostParts,
    acknowledge: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> LicensePost<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LicensePost]"]
    pub fn new(client: Elasticsearch) -> Self {
        LicensePost {
            client,
            parts: LicensePostParts::None,
            headers: HeaderMap::new(),
            acknowledge: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "whether the user has acknowledged acknowledge messages (default: false)"]
    pub fn acknowledge(mut self, acknowledge: bool) -> Self {
        self.acknowledge = Some(acknowledge);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LicensePost<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        LicensePost {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            acknowledge: self.acknowledge,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
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
    #[doc = "Creates an asynchronous call to the License Post API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "acknowledge")]
                acknowledge: Option<bool>,
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
                acknowledge: self.acknowledge,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the License Post Start Basic API"]
pub enum LicensePostStartBasicParts {
    #[doc = "No parts"]
    None,
}
impl LicensePostStartBasicParts {
    #[doc = "Builds a relative URL path to the License Post Start Basic API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicensePostStartBasicParts::None => "/_license/start_basic".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Post Start Basic API](https://www.elastic.co/guide/en/elasticsearch/reference/master/start-basic.html)."]
pub struct LicensePostStartBasic<'a, B> {
    client: Elasticsearch,
    parts: LicensePostStartBasicParts,
    acknowledge: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> LicensePostStartBasic<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LicensePostStartBasic]"]
    pub fn new(client: Elasticsearch) -> Self {
        LicensePostStartBasic {
            client,
            parts: LicensePostStartBasicParts::None,
            headers: HeaderMap::new(),
            acknowledge: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "whether the user has acknowledged acknowledge messages (default: false)"]
    pub fn acknowledge(mut self, acknowledge: bool) -> Self {
        self.acknowledge = Some(acknowledge);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LicensePostStartBasic<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        LicensePostStartBasic {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            acknowledge: self.acknowledge,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
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
    #[doc = "Creates an asynchronous call to the License Post Start Basic API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "acknowledge")]
                acknowledge: Option<bool>,
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
                acknowledge: self.acknowledge,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the License Post Start Trial API"]
pub enum LicensePostStartTrialParts {
    #[doc = "No parts"]
    None,
}
impl LicensePostStartTrialParts {
    #[doc = "Builds a relative URL path to the License Post Start Trial API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicensePostStartTrialParts::None => "/_license/start_trial".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Post Start Trial API](https://www.elastic.co/guide/en/elasticsearch/reference/master/start-trial.html)."]
pub struct LicensePostStartTrial<'a, B> {
    client: Elasticsearch,
    parts: LicensePostStartTrialParts,
    acknowledge: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    ty: Option<&'a str>,
}
impl<'a, B> LicensePostStartTrial<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LicensePostStartTrial]"]
    pub fn new(client: Elasticsearch) -> Self {
        LicensePostStartTrial {
            client,
            parts: LicensePostStartTrialParts::None,
            headers: HeaderMap::new(),
            acknowledge: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            ty: None,
        }
    }
    #[doc = "whether the user has acknowledged acknowledge messages (default: false)"]
    pub fn acknowledge(mut self, acknowledge: bool) -> Self {
        self.acknowledge = Some(acknowledge);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LicensePostStartTrial<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        LicensePostStartTrial {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            acknowledge: self.acknowledge,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            ty: self.ty,
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
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
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
    #[doc = "The type of trial license to generate (default: \"trial\")"]
    pub fn ty(mut self, ty: &'a str) -> Self {
        self.ty = Some(ty);
        self
    }
    #[doc = "Creates an asynchronous call to the License Post Start Trial API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "acknowledge")]
                acknowledge: Option<bool>,
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
                #[serde(rename = "type")]
                ty: Option<&'a str>,
            }
            let query_params = QueryParams {
                acknowledge: self.acknowledge,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                ty: self.ty,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for License APIs"]
pub struct License {
    client: Elasticsearch,
}
impl License {
    #[doc = "Creates a new instance of [License]"]
    pub fn new(client: Elasticsearch) -> Self {
        Self { client }
    }
    pub fn delete<'a>(&self) -> LicenseDelete<'a> {
        LicenseDelete::new(self.client.clone())
    }
    pub fn get<'a>(&self) -> LicenseGet<'a> {
        LicenseGet::new(self.client.clone())
    }
    pub fn get_basic_status<'a>(&self) -> LicenseGetBasicStatus<'a> {
        LicenseGetBasicStatus::new(self.client.clone())
    }
    pub fn get_trial_status<'a>(&self) -> LicenseGetTrialStatus<'a> {
        LicenseGetTrialStatus::new(self.client.clone())
    }
    pub fn post<'a>(&self) -> LicensePost<'a, ()> {
        LicensePost::new(self.client.clone())
    }
    pub fn post_start_basic<'a>(&self) -> LicensePostStartBasic<'a, ()> {
        LicensePostStartBasic::new(self.client.clone())
    }
    pub fn post_start_trial<'a>(&self) -> LicensePostStartTrial<'a, ()> {
        LicensePostStartTrial::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for License APIs"]
    pub fn license(&self) -> License {
        License::new(self.clone())
    }
}