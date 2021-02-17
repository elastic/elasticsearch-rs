/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

// -----------------------------------------------
// This file is generated, Please do not edit it manually.
// Run the following in the root of the repo to regenerate:
//
// cargo make generate-api
// -----------------------------------------------

//! Ingest APIs
//!
//! [Manage ingest pipelines](https://www.elastic.co/guide/en/elasticsearch/reference/master/ingest-apis.html).
//! Ingest pipelines can be used on a node with the `ingest` role to
//! pre-process documents before indexing, to apply transformations and enrich data. Transformations are performed
//! by [processors](https://www.elastic.co/guide/en/elasticsearch/reference/master/ingest-processors.html)
//! in the pipeline, and can include such operations as
//!
//! - add, remove and append fields within the document
//! - point documents to the right time-based index based on a timestamp within the document
//! - extract details from fields with known formats and add new fields with extracted data
//!
//! and many more.
//!
//! All nodes enable ingest by default, so any node can handle ingest tasks. Ingest pipelines can
//! be conditionally executed, and failures within pipelines can be explicitly handled by defining
//! processors to execute in the event of failure.

#![allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        transport::Transport,
        Method,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::{borrow::Cow, time::Duration};
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
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_id.len());
                p.push_str("/_ingest/pipeline/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Delete Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/delete-pipeline-api.html)\n\nDeletes a pipeline."]
#[derive(Clone, Debug)]
pub struct IngestDeletePipeline<'a, 'b> {
    transport: &'a Transport,
    parts: IngestDeletePipelineParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> IngestDeletePipeline<'a, 'b> {
    #[doc = "Creates a new instance of [IngestDeletePipeline] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestDeletePipelineParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestDeletePipeline {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_id.len());
                p.push_str("/_ingest/pipeline/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Get Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/get-pipeline-api.html)\n\nReturns a pipeline."]
#[derive(Clone, Debug)]
pub struct IngestGetPipeline<'a, 'b> {
    transport: &'a Transport,
    parts: IngestGetPipelineParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IngestGetPipeline<'a, 'b> {
    #[doc = "Creates a new instance of [IngestGetPipeline] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestGetPipelineParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestGetPipeline {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
#[doc = "Builder for the [Ingest Processor Grok API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/grok-processor.html#grok-processor-rest-get)\n\nReturns a list of the built-in patterns."]
#[derive(Clone, Debug)]
pub struct IngestProcessorGrok<'a, 'b> {
    transport: &'a Transport,
    parts: IngestProcessorGrokParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IngestProcessorGrok<'a, 'b> {
    #[doc = "Creates a new instance of [IngestProcessorGrok]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        IngestProcessorGrok {
            transport,
            parts: IngestProcessorGrokParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_id.len());
                p.push_str("/_ingest/pipeline/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Put Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/put-pipeline-api.html)\n\nCreates or updates a pipeline."]
#[derive(Clone, Debug)]
pub struct IngestPutPipeline<'a, 'b, B> {
    transport: &'a Transport,
    parts: IngestPutPipelineParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> IngestPutPipeline<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IngestPutPipeline] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestPutPipelineParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestPutPipeline {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
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
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(28usize + encoded_id.len());
                p.push_str("/_ingest/pipeline/");
                p.push_str(encoded_id.as_ref());
                p.push_str("/_simulate");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Simulate API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/simulate-pipeline-api.html)\n\nAllows to simulate a pipeline with example documents."]
#[derive(Clone, Debug)]
pub struct IngestSimulate<'a, 'b, B> {
    transport: &'a Transport,
    parts: IngestSimulateParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    verbose: Option<bool>,
}
impl<'a, 'b, B> IngestSimulate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IngestSimulate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestSimulateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestSimulate {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
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
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Ingest APIs"]
pub struct Ingest<'a> {
    transport: &'a Transport,
}
impl<'a> Ingest<'a> {
    #[doc = "Creates a new instance of [Ingest]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Ingest Delete Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/delete-pipeline-api.html)\n\nDeletes a pipeline."]
    pub fn delete_pipeline<'b>(
        &'a self,
        parts: IngestDeletePipelineParts<'b>,
    ) -> IngestDeletePipeline<'a, 'b> {
        IngestDeletePipeline::new(self.transport(), parts)
    }
    #[doc = "[Ingest Get Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/get-pipeline-api.html)\n\nReturns a pipeline."]
    pub fn get_pipeline<'b>(
        &'a self,
        parts: IngestGetPipelineParts<'b>,
    ) -> IngestGetPipeline<'a, 'b> {
        IngestGetPipeline::new(self.transport(), parts)
    }
    #[doc = "[Ingest Processor Grok API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/grok-processor.html#grok-processor-rest-get)\n\nReturns a list of the built-in patterns."]
    pub fn processor_grok<'b>(&'a self) -> IngestProcessorGrok<'a, 'b> {
        IngestProcessorGrok::new(self.transport())
    }
    #[doc = "[Ingest Put Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/put-pipeline-api.html)\n\nCreates or updates a pipeline."]
    pub fn put_pipeline<'b>(
        &'a self,
        parts: IngestPutPipelineParts<'b>,
    ) -> IngestPutPipeline<'a, 'b, ()> {
        IngestPutPipeline::new(self.transport(), parts)
    }
    #[doc = "[Ingest Simulate API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/simulate-pipeline-api.html)\n\nAllows to simulate a pipeline with example documents."]
    pub fn simulate<'b>(&'a self, parts: IngestSimulateParts<'b>) -> IngestSimulate<'a, 'b, ()> {
        IngestSimulate::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Ingest APIs"]
    pub fn ingest(&self) -> Ingest {
        Ingest::new(self.transport())
    }
}
