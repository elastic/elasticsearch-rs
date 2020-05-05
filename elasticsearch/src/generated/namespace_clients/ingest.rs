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
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
    params::*,
};
use serde::Serialize;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ingest Delete Pipeline API"]
pub enum IngestDeletePipelineParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> IngestDeletePipelineParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Delete Pipeline API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestDeletePipelineParts::Id(ref id) => {
                let mut p = String::with_capacity(18usize + id.len());
                p.push_str("/_ingest/pipeline/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ingest Delete Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/delete-pipeline-api.html)\n\nDeletes a pipeline."]
pub struct IngestDeletePipeline<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IngestDeletePipelineParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> IngestDeletePipeline<'a, 'b> {
    #[doc = "Creates a new instance of [IngestDeletePipeline] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IngestDeletePipelineParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestDeletePipeline {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
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
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Delete Pipeline API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Ingest Get Pipeline API"]
pub enum IngestGetPipelineParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> IngestGetPipelineParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Get Pipeline API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestGetPipelineParts::None => "/_ingest/pipeline".into(),
            IngestGetPipelineParts::Id(ref id) => {
                let mut p = String::with_capacity(18usize + id.len());
                p.push_str("/_ingest/pipeline/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ingest Get Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/get-pipeline-api.html)\n\nReturns a pipeline."]
pub struct IngestGetPipeline<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IngestGetPipelineParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IngestGetPipeline<'a, 'b> {
    #[doc = "Creates a new instance of [IngestGetPipeline] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IngestGetPipelineParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestGetPipeline {
            client,
            parts,
            headers,
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
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
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
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Get Pipeline API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ingest Processor Grok API"]
pub enum IngestProcessorGrokParts {
    #[doc = "No parts"]
    None,
}
impl IngestProcessorGrokParts {
    #[doc = "Builds a relative URL path to the Ingest Processor Grok API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestProcessorGrokParts::None => "/_ingest/processor/grok".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ingest Processor Grok API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/grok-processor.html#grok-processor-rest-get)\n\nReturns a list of the built-in patterns."]
pub struct IngestProcessorGrok<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IngestProcessorGrokParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IngestProcessorGrok<'a, 'b> {
    #[doc = "Creates a new instance of [IngestProcessorGrok]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        IngestProcessorGrok {
            client,
            parts: IngestProcessorGrokParts::None,
            headers,
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
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Processor Grok API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
#[doc = "API parts for the Ingest Put Pipeline API"]
pub enum IngestPutPipelineParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> IngestPutPipelineParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Put Pipeline API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestPutPipelineParts::Id(ref id) => {
                let mut p = String::with_capacity(18usize + id.len());
                p.push_str("/_ingest/pipeline/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ingest Put Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/put-pipeline-api.html)\n\nCreates or updates a pipeline."]
pub struct IngestPutPipeline<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IngestPutPipelineParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> IngestPutPipeline<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IngestPutPipeline] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IngestPutPipelineParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestPutPipeline {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IngestPutPipeline<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IngestPutPipeline {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
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
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Put Pipeline API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Ingest Simulate API"]
pub enum IngestSimulateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> IngestSimulateParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Simulate API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestSimulateParts::None => "/_ingest/pipeline/_simulate".into(),
            IngestSimulateParts::Id(ref id) => {
                let mut p = String::with_capacity(28usize + id.len());
                p.push_str("/_ingest/pipeline/");
                p.push_str(id.as_ref());
                p.push_str("/_simulate");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ingest Simulate API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/simulate-pipeline-api.html)\n\nAllows to simulate a pipeline with example documents."]
pub struct IngestSimulate<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IngestSimulateParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    verbose: Option<bool>,
}
impl<'a, 'b, B> IngestSimulate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IngestSimulate] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IngestSimulateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestSimulate {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            verbose: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IngestSimulate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IngestSimulate {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            verbose: self.verbose,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display data output for each processor in executed pipeline"]
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = Some(verbose);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Simulate API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "verbose")]
                verbose: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                verbose: self.verbose,
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
#[doc = "Namespace client for Ingest APIs"]
pub struct Ingest<'a> {
    client: &'a Elasticsearch,
}
impl<'a> Ingest<'a> {
    #[doc = "Creates a new instance of [Ingest]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        Self { client }
    }
    #[doc = "[Ingest Delete Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/delete-pipeline-api.html)\n\nDeletes a pipeline."]
    pub fn delete_pipeline<'b>(
        &'a self,
        parts: IngestDeletePipelineParts<'b>,
    ) -> IngestDeletePipeline<'a, 'b> {
        IngestDeletePipeline::new(&self.client, parts)
    }
    #[doc = "[Ingest Get Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/get-pipeline-api.html)\n\nReturns a pipeline."]
    pub fn get_pipeline<'b>(
        &'a self,
        parts: IngestGetPipelineParts<'b>,
    ) -> IngestGetPipeline<'a, 'b> {
        IngestGetPipeline::new(&self.client, parts)
    }
    #[doc = "[Ingest Processor Grok API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/grok-processor.html#grok-processor-rest-get)\n\nReturns a list of the built-in patterns."]
    pub fn processor_grok<'b>(&'a self) -> IngestProcessorGrok<'a, 'b> {
        IngestProcessorGrok::new(&self.client)
    }
    #[doc = "[Ingest Put Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/put-pipeline-api.html)\n\nCreates or updates a pipeline."]
    pub fn put_pipeline<'b>(
        &'a self,
        parts: IngestPutPipelineParts<'b>,
    ) -> IngestPutPipeline<'a, 'b, ()> {
        IngestPutPipeline::new(&self.client, parts)
    }
    #[doc = "[Ingest Simulate API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/simulate-pipeline-api.html)\n\nAllows to simulate a pipeline with example documents."]
    pub fn simulate<'b>(&'a self, parts: IngestSimulateParts<'b>) -> IngestSimulate<'a, 'b, ()> {
        IngestSimulate::new(&self.client, parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Ingest APIs"]
    pub fn ingest(&self) -> Ingest {
        Ingest::new(&self)
    }
}
