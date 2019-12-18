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
#[doc = "API parts for the Ilm Delete Lifecycle API"]
pub enum IlmDeleteLifecycleParts<'a> {
    #[doc = "Policy"]
    Policy(&'a str),
}
impl<'a> IlmDeleteLifecycleParts<'a> {
    #[doc = "Builds a relative URL path to the Ilm Delete Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmDeleteLifecycleParts::Policy(ref policy) => {
                let mut p = String::with_capacity(13usize + policy.len());
                p.push_str("/_ilm/policy/");
                p.push_str(policy.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Delete Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-delete-lifecycle.html)."]
pub struct IlmDeleteLifecycle<'a> {
    client: Elasticsearch,
    parts: IlmDeleteLifecycleParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IlmDeleteLifecycle<'a> {
    #[doc = "Creates a new instance of [IlmDeleteLifecycle] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: IlmDeleteLifecycleParts<'a>) -> Self {
        IlmDeleteLifecycle {
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
    #[doc = "Creates an asynchronous call to the Ilm Delete Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Explain Lifecycle API"]
pub enum IlmExplainLifecycleParts<'a> {
    #[doc = "Index"]
    Index(&'a str),
}
impl<'a> IlmExplainLifecycleParts<'a> {
    #[doc = "Builds a relative URL path to the Ilm Explain Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmExplainLifecycleParts::Index(ref index) => {
                let mut p = String::with_capacity(14usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_ilm/explain");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Explain Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-explain-lifecycle.html)."]
pub struct IlmExplainLifecycle<'a> {
    client: Elasticsearch,
    parts: IlmExplainLifecycleParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    only_errors: Option<bool>,
    only_managed: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IlmExplainLifecycle<'a> {
    #[doc = "Creates a new instance of [IlmExplainLifecycle] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: IlmExplainLifecycleParts<'a>) -> Self {
        IlmExplainLifecycle {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            only_errors: None,
            only_managed: None,
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
    #[doc = "filters the indices included in the response to ones in an ILM error state, implies only_managed"]
    pub fn only_errors(mut self, only_errors: bool) -> Self {
        self.only_errors = Some(only_errors);
        self
    }
    #[doc = "filters the indices included in the response to ones managed by ILM"]
    pub fn only_managed(mut self, only_managed: bool) -> Self {
        self.only_managed = Some(only_managed);
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
    #[doc = "Creates an asynchronous call to the Ilm Explain Lifecycle API that can be awaited"]
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
                #[serde(rename = "only_errors")]
                only_errors: Option<bool>,
                #[serde(rename = "only_managed")]
                only_managed: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                only_errors: self.only_errors,
                only_managed: self.only_managed,
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
#[doc = "API parts for the Ilm Get Lifecycle API"]
pub enum IlmGetLifecycleParts<'a> {
    #[doc = "Policy"]
    Policy(&'a str),
    #[doc = "No parts"]
    None,
}
impl<'a> IlmGetLifecycleParts<'a> {
    #[doc = "Builds a relative URL path to the Ilm Get Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmGetLifecycleParts::Policy(ref policy) => {
                let mut p = String::with_capacity(13usize + policy.len());
                p.push_str("/_ilm/policy/");
                p.push_str(policy.as_ref());
                p.into()
            }
            IlmGetLifecycleParts::None => "/_ilm/policy".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Get Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-lifecycle.html)."]
pub struct IlmGetLifecycle<'a> {
    client: Elasticsearch,
    parts: IlmGetLifecycleParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IlmGetLifecycle<'a> {
    #[doc = "Creates a new instance of [IlmGetLifecycle] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: IlmGetLifecycleParts<'a>) -> Self {
        IlmGetLifecycle {
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
    #[doc = "Creates an asynchronous call to the Ilm Get Lifecycle API that can be awaited"]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Get Status API"]
pub enum IlmGetStatusParts {
    #[doc = "No parts"]
    None,
}
impl IlmGetStatusParts {
    #[doc = "Builds a relative URL path to the Ilm Get Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmGetStatusParts::None => "/_ilm/status".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Get Status API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-status.html)."]
pub struct IlmGetStatus<'a> {
    client: Elasticsearch,
    parts: IlmGetStatusParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IlmGetStatus<'a> {
    #[doc = "Creates a new instance of [IlmGetStatus]"]
    pub fn new(client: Elasticsearch) -> Self {
        IlmGetStatus {
            client,
            parts: IlmGetStatusParts::None,
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
    #[doc = "Creates an asynchronous call to the Ilm Get Status API that can be awaited"]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Move To Step API"]
pub enum IlmMoveToStepParts<'a> {
    #[doc = "Index"]
    Index(&'a str),
}
impl<'a> IlmMoveToStepParts<'a> {
    #[doc = "Builds a relative URL path to the Ilm Move To Step API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmMoveToStepParts::Index(ref index) => {
                let mut p = String::with_capacity(11usize + index.len());
                p.push_str("/_ilm/move/");
                p.push_str(index.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Move To Step API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-move-to-step.html)."]
pub struct IlmMoveToStep<'a, B> {
    client: Elasticsearch,
    parts: IlmMoveToStepParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> IlmMoveToStep<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IlmMoveToStep] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: IlmMoveToStepParts<'a>) -> Self {
        IlmMoveToStep {
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
    pub fn body<T>(self, body: T) -> IlmMoveToStep<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IlmMoveToStep {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous call to the Ilm Move To Step API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Put Lifecycle API"]
pub enum IlmPutLifecycleParts<'a> {
    #[doc = "Policy"]
    Policy(&'a str),
}
impl<'a> IlmPutLifecycleParts<'a> {
    #[doc = "Builds a relative URL path to the Ilm Put Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmPutLifecycleParts::Policy(ref policy) => {
                let mut p = String::with_capacity(13usize + policy.len());
                p.push_str("/_ilm/policy/");
                p.push_str(policy.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Put Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-put-lifecycle.html)."]
pub struct IlmPutLifecycle<'a, B> {
    client: Elasticsearch,
    parts: IlmPutLifecycleParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> IlmPutLifecycle<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IlmPutLifecycle] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: IlmPutLifecycleParts<'a>) -> Self {
        IlmPutLifecycle {
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
    pub fn body<T>(self, body: T) -> IlmPutLifecycle<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IlmPutLifecycle {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous call to the Ilm Put Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Remove Policy API"]
pub enum IlmRemovePolicyParts<'a> {
    #[doc = "Index"]
    Index(&'a str),
}
impl<'a> IlmRemovePolicyParts<'a> {
    #[doc = "Builds a relative URL path to the Ilm Remove Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmRemovePolicyParts::Index(ref index) => {
                let mut p = String::with_capacity(13usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_ilm/remove");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Remove Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-remove-policy.html)."]
pub struct IlmRemovePolicy<'a, B> {
    client: Elasticsearch,
    parts: IlmRemovePolicyParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> IlmRemovePolicy<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IlmRemovePolicy] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: IlmRemovePolicyParts<'a>) -> Self {
        IlmRemovePolicy {
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
    pub fn body<T>(self, body: T) -> IlmRemovePolicy<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IlmRemovePolicy {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous call to the Ilm Remove Policy API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Retry API"]
pub enum IlmRetryParts<'a> {
    #[doc = "Index"]
    Index(&'a str),
}
impl<'a> IlmRetryParts<'a> {
    #[doc = "Builds a relative URL path to the Ilm Retry API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmRetryParts::Index(ref index) => {
                let mut p = String::with_capacity(12usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_ilm/retry");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Retry API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-retry-policy.html)."]
pub struct IlmRetry<'a, B> {
    client: Elasticsearch,
    parts: IlmRetryParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> IlmRetry<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IlmRetry] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: IlmRetryParts<'a>) -> Self {
        IlmRetry {
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
    pub fn body<T>(self, body: T) -> IlmRetry<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IlmRetry {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous call to the Ilm Retry API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Start API"]
pub enum IlmStartParts {
    #[doc = "No parts"]
    None,
}
impl IlmStartParts {
    #[doc = "Builds a relative URL path to the Ilm Start API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmStartParts::None => "/_ilm/start".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Start API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-start.html)."]
pub struct IlmStart<'a, B> {
    client: Elasticsearch,
    parts: IlmStartParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> IlmStart<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IlmStart]"]
    pub fn new(client: Elasticsearch) -> Self {
        IlmStart {
            client,
            parts: IlmStartParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IlmStart<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IlmStart {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous call to the Ilm Start API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ilm Stop API"]
pub enum IlmStopParts {
    #[doc = "No parts"]
    None,
}
impl IlmStopParts {
    #[doc = "Builds a relative URL path to the Ilm Stop API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IlmStopParts::None => "/_ilm/stop".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ilm Stop API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-stop.html)."]
pub struct IlmStop<'a, B> {
    client: Elasticsearch,
    parts: IlmStopParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> IlmStop<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IlmStop]"]
    pub fn new(client: Elasticsearch) -> Self {
        IlmStop {
            client,
            parts: IlmStopParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IlmStop<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IlmStop {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous call to the Ilm Stop API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Index Lifecycle Management APIs"]
pub struct Ilm {
    client: Elasticsearch,
}
impl Ilm {
    #[doc = "Creates a new instance of [Ilm]"]
    pub fn new(client: Elasticsearch) -> Self {
        Self { client }
    }
    pub fn delete_lifecycle<'a>(
        &self,
        parts: IlmDeleteLifecycleParts<'a>,
    ) -> IlmDeleteLifecycle<'a> {
        IlmDeleteLifecycle::new(self.client.clone(), parts)
    }
    pub fn explain_lifecycle<'a>(
        &self,
        parts: IlmExplainLifecycleParts<'a>,
    ) -> IlmExplainLifecycle<'a> {
        IlmExplainLifecycle::new(self.client.clone(), parts)
    }
    pub fn get_lifecycle<'a>(&self, parts: IlmGetLifecycleParts<'a>) -> IlmGetLifecycle<'a> {
        IlmGetLifecycle::new(self.client.clone(), parts)
    }
    pub fn get_status<'a>(&self) -> IlmGetStatus<'a> {
        IlmGetStatus::new(self.client.clone())
    }
    pub fn move_to_step<'a>(&self, parts: IlmMoveToStepParts<'a>) -> IlmMoveToStep<'a, ()> {
        IlmMoveToStep::new(self.client.clone(), parts)
    }
    pub fn put_lifecycle<'a>(&self, parts: IlmPutLifecycleParts<'a>) -> IlmPutLifecycle<'a, ()> {
        IlmPutLifecycle::new(self.client.clone(), parts)
    }
    pub fn remove_policy<'a>(&self, parts: IlmRemovePolicyParts<'a>) -> IlmRemovePolicy<'a, ()> {
        IlmRemovePolicy::new(self.client.clone(), parts)
    }
    pub fn retry<'a>(&self, parts: IlmRetryParts<'a>) -> IlmRetry<'a, ()> {
        IlmRetry::new(self.client.clone(), parts)
    }
    pub fn start<'a>(&self) -> IlmStart<'a, ()> {
        IlmStart::new(self.client.clone())
    }
    pub fn stop<'a>(&self) -> IlmStop<'a, ()> {
        IlmStop::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Index Lifecycle Management APIs"]
    pub fn ilm(&self) -> Ilm {
        Ilm::new(self.clone())
    }
}