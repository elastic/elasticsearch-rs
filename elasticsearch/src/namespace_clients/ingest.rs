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
#[doc = "API parts for the Ingest Delete Pipeline API"]
pub enum IngestDeletePipelineParts<'a> {
    #[doc = "Id"]
    Id(&'a str),
}
impl<'a> IngestDeletePipelineParts<'a> {
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
#[doc = "Builder for the [Ingest Delete Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/master/delete-pipeline-api.html). Deletes a pipeline."]
pub struct IngestDeletePipeline<'a> {
    client: Elasticsearch,
    parts: IngestDeletePipelineParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> IngestDeletePipeline<'a> {
    #[doc = "Creates a new instance of [IngestDeletePipeline] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: IngestDeletePipelineParts<'a>) -> Self {
        IngestDeletePipeline {
            client,
            parts,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Delete Pipeline API that can be awaited"]
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ingest Get Pipeline API"]
pub enum IngestGetPipelineParts<'a> {
    #[doc = "No parts"]
    None,
    #[doc = "Id"]
    Id(&'a str),
}
impl<'a> IngestGetPipelineParts<'a> {
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
#[doc = "Builder for the [Ingest Get Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/master/get-pipeline-api.html). Returns a pipeline."]
pub struct IngestGetPipeline<'a> {
    client: Elasticsearch,
    parts: IngestGetPipelineParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IngestGetPipeline<'a> {
    #[doc = "Creates a new instance of [IngestGetPipeline] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: IngestGetPipelineParts<'a>) -> Self {
        IngestGetPipeline {
            client,
            parts,
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
    #[doc = "Explicit operation timeout for connection to master node"]
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
    #[doc = "Creates an asynchronous call to the Ingest Get Pipeline API that can be awaited"]
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
#[doc = "Builder for the [Ingest Processor Grok API](https://www.elastic.co/guide/en/elasticsearch/reference/master/grok-processor.html#grok-processor-rest-get). Returns a list of the built-in patterns."]
pub struct IngestProcessorGrok<'a> {
    client: Elasticsearch,
    parts: IngestProcessorGrokParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IngestProcessorGrok<'a> {
    #[doc = "Creates a new instance of [IngestProcessorGrok]"]
    pub fn new(client: Elasticsearch) -> Self {
        IngestProcessorGrok {
            client,
            parts: IngestProcessorGrokParts::None,
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
    #[doc = "Creates an asynchronous call to the Ingest Processor Grok API that can be awaited"]
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
#[doc = "API parts for the Ingest Put Pipeline API"]
pub enum IngestPutPipelineParts<'a> {
    #[doc = "Id"]
    Id(&'a str),
}
impl<'a> IngestPutPipelineParts<'a> {
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
#[doc = "Builder for the [Ingest Put Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/master/put-pipeline-api.html). Creates or updates a pipeline."]
pub struct IngestPutPipeline<'a, B> {
    client: Elasticsearch,
    parts: IngestPutPipelineParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> IngestPutPipeline<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IngestPutPipeline] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: IngestPutPipelineParts<'a>) -> Self {
        IngestPutPipeline {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> IngestPutPipeline<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IngestPutPipeline {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Put Pipeline API that can be awaited"]
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ingest Simulate API"]
pub enum IngestSimulateParts<'a> {
    #[doc = "No parts"]
    None,
    #[doc = "Id"]
    Id(&'a str),
}
impl<'a> IngestSimulateParts<'a> {
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
#[doc = "Builder for the [Ingest Simulate API](https://www.elastic.co/guide/en/elasticsearch/reference/master/simulate-pipeline-api.html). Allows to simulate a pipeline with example documents."]
pub struct IngestSimulate<'a, B> {
    client: Elasticsearch,
    parts: IngestSimulateParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    verbose: Option<bool>,
}
impl<'a, B> IngestSimulate<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IngestSimulate] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: IngestSimulateParts<'a>) -> Self {
        IngestSimulate {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> IngestSimulate<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IngestSimulate {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Ingest APIs"]
pub struct Ingest {
    client: Elasticsearch,
}
impl Ingest {
    #[doc = "Creates a new instance of [Ingest]"]
    pub fn new(client: Elasticsearch) -> Self {
        Self { client }
    }
    #[doc = "Deletes a pipeline."]
    pub fn delete_pipeline<'a>(
        &self,
        parts: IngestDeletePipelineParts<'a>,
    ) -> IngestDeletePipeline<'a> {
        IngestDeletePipeline::new(self.client.clone(), parts)
    }
    #[doc = "Returns a pipeline."]
    pub fn get_pipeline<'a>(&self, parts: IngestGetPipelineParts<'a>) -> IngestGetPipeline<'a> {
        IngestGetPipeline::new(self.client.clone(), parts)
    }
    #[doc = "Returns a list of the built-in patterns."]
    pub fn processor_grok<'a>(&self) -> IngestProcessorGrok<'a> {
        IngestProcessorGrok::new(self.client.clone())
    }
    #[doc = "Creates or updates a pipeline."]
    pub fn put_pipeline<'a>(&self, parts: IngestPutPipelineParts<'a>) -> IngestPutPipeline<'a, ()> {
        IngestPutPipeline::new(self.client.clone(), parts)
    }
    #[doc = "Allows to simulate a pipeline with example documents."]
    pub fn simulate<'a>(&self, parts: IngestSimulateParts<'a>) -> IngestSimulate<'a, ()> {
        IngestSimulate::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Ingest APIs"]
    pub fn ingest(&self) -> Ingest {
        Ingest::new(self.clone())
    }
}