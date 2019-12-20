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
#[doc = "API parts for the Slm Delete Lifecycle API"]
pub enum SlmDeleteLifecycleParts<'a> {
    #[doc = "PolicyId"]
    PolicyId(&'a str),
}
impl<'a> SlmDeleteLifecycleParts<'a> {
    #[doc = "Builds a relative URL path to the Slm Delete Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmDeleteLifecycleParts::PolicyId(ref policy_id) => {
                let mut p = String::with_capacity(13usize + policy_id.len());
                p.push_str("/_slm/policy/");
                p.push_str(policy_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Delete Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/current/slm-api-delete.html)."]
pub struct SlmDeleteLifecycle<'a> {
    client: Elasticsearch,
    parts: SlmDeleteLifecycleParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SlmDeleteLifecycle<'a> {
    #[doc = "Creates a new instance of [SlmDeleteLifecycle] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SlmDeleteLifecycleParts<'a>) -> Self {
        SlmDeleteLifecycle {
            client,
            parts,
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
    #[doc = "Creates an asynchronous call to the Slm Delete Lifecycle API that can be awaited"]
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
#[doc = "API parts for the Slm Execute Lifecycle API"]
pub enum SlmExecuteLifecycleParts<'a> {
    #[doc = "PolicyId"]
    PolicyId(&'a str),
}
impl<'a> SlmExecuteLifecycleParts<'a> {
    #[doc = "Builds a relative URL path to the Slm Execute Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmExecuteLifecycleParts::PolicyId(ref policy_id) => {
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
#[doc = "Builder for the [Slm Execute Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/current/slm-api-execute.html)."]
pub struct SlmExecuteLifecycle<'a, B> {
    client: Elasticsearch,
    parts: SlmExecuteLifecycleParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> SlmExecuteLifecycle<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SlmExecuteLifecycle] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SlmExecuteLifecycleParts<'a>) -> Self {
        SlmExecuteLifecycle {
            client,
            parts,
            headers: HeaderMap::new(),
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SlmExecuteLifecycle<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SlmExecuteLifecycle {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous call to the Slm Execute Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Slm Execute Retention API"]
pub enum SlmExecuteRetentionParts {
    #[doc = "No parts"]
    None,
}
impl SlmExecuteRetentionParts {
    #[doc = "Builds a relative URL path to the Slm Execute Retention API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmExecuteRetentionParts::None => "/_slm/_execute_retention".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Execute Retention API](https://www.elastic.co/guide/en/elasticsearch/reference/current/slm-api-execute-retention.html)."]
pub struct SlmExecuteRetention<'a, B> {
    client: Elasticsearch,
    parts: SlmExecuteRetentionParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> SlmExecuteRetention<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SlmExecuteRetention]"]
    pub fn new(client: Elasticsearch) -> Self {
        SlmExecuteRetention {
            client,
            parts: SlmExecuteRetentionParts::None,
            headers: HeaderMap::new(),
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SlmExecuteRetention<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SlmExecuteRetention {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous call to the Slm Execute Retention API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Slm Get Lifecycle API"]
pub enum SlmGetLifecycleParts<'a> {
    #[doc = "PolicyId"]
    PolicyId(&'a [&'a str]),
    #[doc = "No parts"]
    None,
}
impl<'a> SlmGetLifecycleParts<'a> {
    #[doc = "Builds a relative URL path to the Slm Get Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmGetLifecycleParts::PolicyId(ref policy_id) => {
                let policy_id_str = policy_id.join(",");
                let mut p = String::with_capacity(13usize + policy_id_str.len());
                p.push_str("/_slm/policy/");
                p.push_str(policy_id_str.as_ref());
                p.into()
            }
            SlmGetLifecycleParts::None => "/_slm/policy".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Get Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/current/slm-api-get.html)."]
pub struct SlmGetLifecycle<'a> {
    client: Elasticsearch,
    parts: SlmGetLifecycleParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SlmGetLifecycle<'a> {
    #[doc = "Creates a new instance of [SlmGetLifecycle] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SlmGetLifecycleParts<'a>) -> Self {
        SlmGetLifecycle {
            client,
            parts,
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
    #[doc = "Creates an asynchronous call to the Slm Get Lifecycle API that can be awaited"]
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
#[doc = "API parts for the Slm Get Stats API"]
pub enum SlmGetStatsParts {
    #[doc = "No parts"]
    None,
}
impl SlmGetStatsParts {
    #[doc = "Builds a relative URL path to the Slm Get Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmGetStatsParts::None => "/_slm/stats".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Get Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/master/slm-get-stats.html)."]
pub struct SlmGetStats<'a> {
    client: Elasticsearch,
    parts: SlmGetStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SlmGetStats<'a> {
    #[doc = "Creates a new instance of [SlmGetStats]"]
    pub fn new(client: Elasticsearch) -> Self {
        SlmGetStats {
            client,
            parts: SlmGetStatsParts::None,
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
    #[doc = "Creates an asynchronous call to the Slm Get Stats API that can be awaited"]
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
#[doc = "API parts for the Slm Put Lifecycle API"]
pub enum SlmPutLifecycleParts<'a> {
    #[doc = "PolicyId"]
    PolicyId(&'a str),
}
impl<'a> SlmPutLifecycleParts<'a> {
    #[doc = "Builds a relative URL path to the Slm Put Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmPutLifecycleParts::PolicyId(ref policy_id) => {
                let mut p = String::with_capacity(13usize + policy_id.len());
                p.push_str("/_slm/policy/");
                p.push_str(policy_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Put Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/current/slm-api-put.html)."]
pub struct SlmPutLifecycle<'a, B> {
    client: Elasticsearch,
    parts: SlmPutLifecycleParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> SlmPutLifecycle<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SlmPutLifecycle] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SlmPutLifecycleParts<'a>) -> Self {
        SlmPutLifecycle {
            client,
            parts,
            headers: HeaderMap::new(),
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SlmPutLifecycle<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SlmPutLifecycle {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous call to the Slm Put Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Snapshot Lifecycle Management APIs"]
pub struct Slm {
    client: Elasticsearch,
}
impl Slm {
    #[doc = "Creates a new instance of [Slm]"]
    pub fn new(client: Elasticsearch) -> Self {
        Self { client }
    }
    pub fn delete_lifecycle<'a>(
        &self,
        parts: SlmDeleteLifecycleParts<'a>,
    ) -> SlmDeleteLifecycle<'a> {
        SlmDeleteLifecycle::new(self.client.clone(), parts)
    }
    pub fn execute_lifecycle<'a>(
        &self,
        parts: SlmExecuteLifecycleParts<'a>,
    ) -> SlmExecuteLifecycle<'a, ()> {
        SlmExecuteLifecycle::new(self.client.clone(), parts)
    }
    pub fn execute_retention<'a>(&self) -> SlmExecuteRetention<'a, ()> {
        SlmExecuteRetention::new(self.client.clone())
    }
    pub fn get_lifecycle<'a>(&self, parts: SlmGetLifecycleParts<'a>) -> SlmGetLifecycle<'a> {
        SlmGetLifecycle::new(self.client.clone(), parts)
    }
    pub fn get_stats<'a>(&self) -> SlmGetStats<'a> {
        SlmGetStats::new(self.client.clone())
    }
    pub fn put_lifecycle<'a>(&self, parts: SlmPutLifecycleParts<'a>) -> SlmPutLifecycle<'a, ()> {
        SlmPutLifecycle::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Snapshot Lifecycle Management APIs"]
    pub fn slm(&self) -> Slm {
        Slm::new(self.clone())
    }
}