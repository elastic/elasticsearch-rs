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

//! Inference APIs
//!
//! The inference APIs enable you to create inference endpoints and use machine learning models of different providers - such as Amazon Bedrock, Anthropic, Azure AI Studio, Cohere, Google AI, Mistral, OpenAI, or HuggingFace - as a service.

#![allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        self,
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        transport::Transport,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::{borrow::Cow, time::Duration};
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Chat Completion Unified API"]
pub enum InferenceChatCompletionUnifiedParts<'b> {
    #[doc = "InferenceId"]
    InferenceId(&'b str),
}
impl<'b> InferenceChatCompletionUnifiedParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Chat Completion Unified API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferenceChatCompletionUnifiedParts::InferenceId(inference_id) => {
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(36usize + encoded_inference_id.len());
                p.push_str("/_inference/chat_completion/");
                p.push_str(encoded_inference_id.as_ref());
                p.push_str("/_stream");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Chat Completion Unified API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/chat-completion-inference.html)\n\nPerform chat completion inference"]
#[derive(Clone, Debug)]
pub struct InferenceChatCompletionUnified<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferenceChatCompletionUnifiedParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferenceChatCompletionUnified<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferenceChatCompletionUnified] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferenceChatCompletionUnifiedParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferenceChatCompletionUnified {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferenceChatCompletionUnified<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferenceChatCompletionUnified {
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
    #[doc = "Creates an asynchronous call to the Inference Chat Completion Unified API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Completion API"]
pub enum InferenceCompletionParts<'b> {
    #[doc = "InferenceId"]
    InferenceId(&'b str),
}
impl<'b> InferenceCompletionParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Completion API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferenceCompletionParts::InferenceId(inference_id) => {
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_inference_id.len());
                p.push_str("/_inference/completion/");
                p.push_str(encoded_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Completion API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/post-inference-api.html)\n\nPerform completion inference"]
#[derive(Clone, Debug)]
pub struct InferenceCompletion<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferenceCompletionParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferenceCompletion<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferenceCompletion] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferenceCompletionParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferenceCompletion {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferenceCompletion<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferenceCompletion {
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
    #[doc = "Creates an asynchronous call to the Inference Completion API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Delete API"]
pub enum InferenceDeleteParts<'b> {
    #[doc = "InferenceId"]
    InferenceId(&'b str),
    #[doc = "TaskType and InferenceId"]
    TaskTypeInferenceId(&'b str, &'b str),
}
impl<'b> InferenceDeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferenceDeleteParts::InferenceId(inference_id) => {
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_inference_id.len());
                p.push_str("/_inference/");
                p.push_str(encoded_inference_id.as_ref());
                p.into()
            }
            InferenceDeleteParts::TaskTypeInferenceId(task_type, inference_id) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/delete-inference-api.html)\n\nDelete an inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferenceDelete<'a, 'b> {
    transport: &'a Transport,
    parts: InferenceDeleteParts<'b>,
    dry_run: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    force: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> InferenceDelete<'a, 'b> {
    #[doc = "Creates a new instance of [InferenceDelete] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferenceDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferenceDelete {
            transport,
            parts,
            headers,
            dry_run: None,
            error_trace: None,
            filter_path: None,
            force: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "If true the endpoint will not be deleted and a list of ingest processors which reference this endpoint will be returned."]
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = Some(dry_run);
        self
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
    #[doc = "If true the endpoint will be forcefully stopped (regardless of whether or not it is referenced by any ingest processors or semantic text fields)."]
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
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
    #[doc = "Creates an asynchronous call to the Inference Delete API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Delete;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                dry_run: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                force: Option<bool>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                dry_run: self.dry_run,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                force: self.force,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Get API"]
pub enum InferenceGetParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "InferenceId"]
    InferenceId(&'b str),
    #[doc = "TaskType and InferenceId"]
    TaskTypeInferenceId(&'b str, &'b str),
}
impl<'b> InferenceGetParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferenceGetParts::None => "/_inference".into(),
            InferenceGetParts::InferenceId(inference_id) => {
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_inference_id.len());
                p.push_str("/_inference/");
                p.push_str(encoded_inference_id.as_ref());
                p.into()
            }
            InferenceGetParts::TaskTypeInferenceId(task_type, inference_id) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Get API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/get-inference-api.html)\n\nGet an inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferenceGet<'a, 'b> {
    transport: &'a Transport,
    parts: InferenceGetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> InferenceGet<'a, 'b> {
    #[doc = "Creates a new instance of [InferenceGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferenceGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferenceGet {
            transport,
            parts,
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
    #[doc = "Creates an asynchronous call to the Inference Get API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Inference API"]
pub enum InferenceInferenceParts<'b> {
    #[doc = "InferenceId"]
    InferenceId(&'b str),
    #[doc = "TaskType and InferenceId"]
    TaskTypeInferenceId(&'b str, &'b str),
}
impl<'b> InferenceInferenceParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Inference API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferenceInferenceParts::InferenceId(inference_id) => {
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_inference_id.len());
                p.push_str("/_inference/");
                p.push_str(encoded_inference_id.as_ref());
                p.into()
            }
            InferenceInferenceParts::TaskTypeInferenceId(task_type, inference_id) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Inference API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/post-inference-api.html)\n\nPerform inference"]
#[derive(Clone, Debug)]
pub struct InferenceInference<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferenceInferenceParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferenceInference<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferenceInference] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferenceInferenceParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferenceInference {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferenceInference<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferenceInference {
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
    #[doc = "Creates an asynchronous call to the Inference Inference API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put API"]
pub enum InferencePutParts<'b> {
    #[doc = "InferenceId"]
    InferenceId(&'b str),
    #[doc = "TaskType and InferenceId"]
    TaskTypeInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutParts::InferenceId(inference_id) => {
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_inference_id.len());
                p.push_str("/_inference/");
                p.push_str(encoded_inference_id.as_ref());
                p.into()
            }
            InferencePutParts::TaskTypeInferenceId(task_type, inference_id) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/put-inference-api.html)\n\nConfigure an inference endpoint for use in the Inference API"]
#[derive(Clone, Debug)]
pub struct InferencePut<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePut<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePut] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePut {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePut<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePut {
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
    #[doc = "Creates an asynchronous call to the Inference Put API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Alibabacloud API"]
pub enum InferencePutAlibabacloudParts<'b> {
    #[doc = "TaskType and AlibabacloudInferenceId"]
    TaskTypeAlibabacloudInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutAlibabacloudParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Alibabacloud API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutAlibabacloudParts::TaskTypeAlibabacloudInferenceId(
                task_type,
                alibabacloud_inference_id,
            ) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_alibabacloud_inference_id: Cow<str> =
                    percent_encode(alibabacloud_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_alibabacloud_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_alibabacloud_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Alibabacloud API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-alibabacloud-ai-search.html)\n\nConfigure an AlibabaCloud AI Search inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutAlibabacloud<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutAlibabacloudParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutAlibabacloud<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutAlibabacloud] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutAlibabacloudParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutAlibabacloud {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutAlibabacloud<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutAlibabacloud {
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
    #[doc = "Creates an asynchronous call to the Inference Put Alibabacloud API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Amazonbedrock API"]
pub enum InferencePutAmazonbedrockParts<'b> {
    #[doc = "TaskType and AmazonbedrockInferenceId"]
    TaskTypeAmazonbedrockInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutAmazonbedrockParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Amazonbedrock API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutAmazonbedrockParts::TaskTypeAmazonbedrockInferenceId(
                task_type,
                amazonbedrock_inference_id,
            ) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_amazonbedrock_inference_id: Cow<str> =
                    percent_encode(amazonbedrock_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_amazonbedrock_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_amazonbedrock_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Amazonbedrock API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-amazon-bedrock.html)\n\nConfigure an Amazon Bedrock inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutAmazonbedrock<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutAmazonbedrockParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutAmazonbedrock<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutAmazonbedrock] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutAmazonbedrockParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutAmazonbedrock {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutAmazonbedrock<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutAmazonbedrock {
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
    #[doc = "Creates an asynchronous call to the Inference Put Amazonbedrock API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Anthropic API"]
pub enum InferencePutAnthropicParts<'b> {
    #[doc = "TaskType and AnthropicInferenceId"]
    TaskTypeAnthropicInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutAnthropicParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Anthropic API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutAnthropicParts::TaskTypeAnthropicInferenceId(
                task_type,
                anthropic_inference_id,
            ) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_anthropic_inference_id: Cow<str> =
                    percent_encode(anthropic_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_anthropic_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_anthropic_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Anthropic API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-anthropic.html)\n\nConfigure an Anthropic inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutAnthropic<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutAnthropicParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutAnthropic<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutAnthropic] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutAnthropicParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutAnthropic {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutAnthropic<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutAnthropic {
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
    #[doc = "Creates an asynchronous call to the Inference Put Anthropic API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Azureaistudio API"]
pub enum InferencePutAzureaistudioParts<'b> {
    #[doc = "TaskType and AzureaistudioInferenceId"]
    TaskTypeAzureaistudioInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutAzureaistudioParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Azureaistudio API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutAzureaistudioParts::TaskTypeAzureaistudioInferenceId(
                task_type,
                azureaistudio_inference_id,
            ) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_azureaistudio_inference_id: Cow<str> =
                    percent_encode(azureaistudio_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_azureaistudio_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_azureaistudio_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Azureaistudio API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-azure-ai-studio.html)\n\nConfigure an Azure AI Studio inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutAzureaistudio<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutAzureaistudioParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutAzureaistudio<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutAzureaistudio] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutAzureaistudioParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutAzureaistudio {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutAzureaistudio<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutAzureaistudio {
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
    #[doc = "Creates an asynchronous call to the Inference Put Azureaistudio API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Azureopenai API"]
pub enum InferencePutAzureopenaiParts<'b> {
    #[doc = "TaskType and AzureopenaiInferenceId"]
    TaskTypeAzureopenaiInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutAzureopenaiParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Azureopenai API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutAzureopenaiParts::TaskTypeAzureopenaiInferenceId(
                task_type,
                azureopenai_inference_id,
            ) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_azureopenai_inference_id: Cow<str> =
                    percent_encode(azureopenai_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_azureopenai_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_azureopenai_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Azureopenai API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-azure-openai.html)\n\nConfigure an Azure OpenAI inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutAzureopenai<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutAzureopenaiParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutAzureopenai<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutAzureopenai] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutAzureopenaiParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutAzureopenai {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutAzureopenai<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutAzureopenai {
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
    #[doc = "Creates an asynchronous call to the Inference Put Azureopenai API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Cohere API"]
pub enum InferencePutCohereParts<'b> {
    #[doc = "TaskType and CohereInferenceId"]
    TaskTypeCohereInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutCohereParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Cohere API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutCohereParts::TaskTypeCohereInferenceId(task_type, cohere_inference_id) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_cohere_inference_id: Cow<str> =
                    percent_encode(cohere_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_cohere_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_cohere_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Cohere API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-cohere.html)\n\nConfigure a Cohere inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutCohere<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutCohereParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutCohere<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutCohere] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutCohereParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutCohere {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutCohere<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutCohere {
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
    #[doc = "Creates an asynchronous call to the Inference Put Cohere API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Elasticsearch API"]
pub enum InferencePutElasticsearchParts<'b> {
    #[doc = "TaskType and ElasticsearchInferenceId"]
    TaskTypeElasticsearchInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutElasticsearchParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Elasticsearch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutElasticsearchParts::TaskTypeElasticsearchInferenceId(
                task_type,
                elasticsearch_inference_id,
            ) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_elasticsearch_inference_id: Cow<str> =
                    percent_encode(elasticsearch_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_elasticsearch_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_elasticsearch_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Elasticsearch API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-elasticsearch.html)\n\nConfigure an Elasticsearch inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutElasticsearch<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutElasticsearchParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutElasticsearch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutElasticsearch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutElasticsearchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutElasticsearch {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutElasticsearch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutElasticsearch {
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
    #[doc = "Creates an asynchronous call to the Inference Put Elasticsearch API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Elser API"]
pub enum InferencePutElserParts<'b> {
    #[doc = "TaskType and ElserInferenceId"]
    TaskTypeElserInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutElserParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Elser API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutElserParts::TaskTypeElserInferenceId(task_type, elser_inference_id) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_elser_inference_id: Cow<str> =
                    percent_encode(elser_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_elser_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_elser_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Elser API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-elser.html)\n\nConfigure an ELSER inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutElser<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutElserParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutElser<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutElser] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutElserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutElser {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutElser<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutElser {
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
    #[doc = "Creates an asynchronous call to the Inference Put Elser API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Googleaistudio API"]
pub enum InferencePutGoogleaistudioParts<'b> {
    #[doc = "TaskType and GoogleaistudioInferenceId"]
    TaskTypeGoogleaistudioInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutGoogleaistudioParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Googleaistudio API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutGoogleaistudioParts::TaskTypeGoogleaistudioInferenceId(
                task_type,
                googleaistudio_inference_id,
            ) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_googleaistudio_inference_id: Cow<str> =
                    percent_encode(googleaistudio_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_googleaistudio_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_googleaistudio_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Googleaistudio API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-google-ai-studio.html)\n\nConfigure a Google AI Studio inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutGoogleaistudio<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutGoogleaistudioParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutGoogleaistudio<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutGoogleaistudio] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutGoogleaistudioParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutGoogleaistudio {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutGoogleaistudio<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutGoogleaistudio {
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
    #[doc = "Creates an asynchronous call to the Inference Put Googleaistudio API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Googlevertexai API"]
pub enum InferencePutGooglevertexaiParts<'b> {
    #[doc = "TaskType and GooglevertexaiInferenceId"]
    TaskTypeGooglevertexaiInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutGooglevertexaiParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Googlevertexai API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutGooglevertexaiParts::TaskTypeGooglevertexaiInferenceId(
                task_type,
                googlevertexai_inference_id,
            ) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_googlevertexai_inference_id: Cow<str> =
                    percent_encode(googlevertexai_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_googlevertexai_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_googlevertexai_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Googlevertexai API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-google-vertex-ai.html)\n\nConfigure a Google Vertex AI inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutGooglevertexai<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutGooglevertexaiParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutGooglevertexai<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutGooglevertexai] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutGooglevertexaiParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutGooglevertexai {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutGooglevertexai<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutGooglevertexai {
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
    #[doc = "Creates an asynchronous call to the Inference Put Googlevertexai API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Hugging Face API"]
pub enum InferencePutHuggingFaceParts<'b> {
    #[doc = "TaskType and HuggingfaceInferenceId"]
    TaskTypeHuggingfaceInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutHuggingFaceParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Hugging Face API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutHuggingFaceParts::TaskTypeHuggingfaceInferenceId(
                task_type,
                huggingface_inference_id,
            ) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_huggingface_inference_id: Cow<str> =
                    percent_encode(huggingface_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_huggingface_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_huggingface_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Hugging Face API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-hugging-face.html)\n\nConfigure a HuggingFace inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutHuggingFace<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutHuggingFaceParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutHuggingFace<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutHuggingFace] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutHuggingFaceParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutHuggingFace {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutHuggingFace<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutHuggingFace {
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
    #[doc = "Creates an asynchronous call to the Inference Put Hugging Face API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Jinaai API"]
pub enum InferencePutJinaaiParts<'b> {
    #[doc = "TaskType and JinaaiInferenceId"]
    TaskTypeJinaaiInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutJinaaiParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Jinaai API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutJinaaiParts::TaskTypeJinaaiInferenceId(task_type, jinaai_inference_id) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_jinaai_inference_id: Cow<str> =
                    percent_encode(jinaai_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_jinaai_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_jinaai_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Jinaai API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-jinaai.html)\n\nConfigure a JinaAI inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutJinaai<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutJinaaiParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutJinaai<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutJinaai] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutJinaaiParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutJinaai {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutJinaai<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutJinaai {
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
    #[doc = "Creates an asynchronous call to the Inference Put Jinaai API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Mistral API"]
pub enum InferencePutMistralParts<'b> {
    #[doc = "TaskType and MistralInferenceId"]
    TaskTypeMistralInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutMistralParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Mistral API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutMistralParts::TaskTypeMistralInferenceId(
                task_type,
                mistral_inference_id,
            ) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_mistral_inference_id: Cow<str> =
                    percent_encode(mistral_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_mistral_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_mistral_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Mistral API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-mistral.html)\n\nConfigure a Mistral inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutMistral<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutMistralParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutMistral<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutMistral] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutMistralParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutMistral {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutMistral<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutMistral {
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
    #[doc = "Creates an asynchronous call to the Inference Put Mistral API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Openai API"]
pub enum InferencePutOpenaiParts<'b> {
    #[doc = "TaskType and OpenaiInferenceId"]
    TaskTypeOpenaiInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutOpenaiParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Openai API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutOpenaiParts::TaskTypeOpenaiInferenceId(task_type, openai_inference_id) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_openai_inference_id: Cow<str> =
                    percent_encode(openai_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_openai_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_openai_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Openai API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-openai.html)\n\nConfigure an OpenAI inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutOpenai<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutOpenaiParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutOpenai<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutOpenai] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutOpenaiParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutOpenai {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutOpenai<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutOpenai {
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
    #[doc = "Creates an asynchronous call to the Inference Put Openai API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Voyageai API"]
pub enum InferencePutVoyageaiParts<'b> {
    #[doc = "TaskType and VoyageaiInferenceId"]
    TaskTypeVoyageaiInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutVoyageaiParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Voyageai API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutVoyageaiParts::TaskTypeVoyageaiInferenceId(
                task_type,
                voyageai_inference_id,
            ) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_voyageai_inference_id: Cow<str> =
                    percent_encode(voyageai_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_voyageai_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_voyageai_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Voyageai API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/inference-apis.html)\n\nConfigure a VoyageAI inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutVoyageai<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutVoyageaiParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutVoyageai<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutVoyageai] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutVoyageaiParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutVoyageai {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutVoyageai<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutVoyageai {
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
    #[doc = "Creates an asynchronous call to the Inference Put Voyageai API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put Watsonx API"]
pub enum InferencePutWatsonxParts<'b> {
    #[doc = "TaskType and WatsonxInferenceId"]
    TaskTypeWatsonxInferenceId(&'b str, &'b str),
}
impl<'b> InferencePutWatsonxParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Put Watsonx API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferencePutWatsonxParts::TaskTypeWatsonxInferenceId(
                task_type,
                watsonx_inference_id,
            ) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_watsonx_inference_id: Cow<str> =
                    percent_encode(watsonx_inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_task_type.len() + encoded_watsonx_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_watsonx_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Put Watsonx API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-watsonx-ai.html)\n\nConfigure a Watsonx inference endpoint"]
#[derive(Clone, Debug)]
pub struct InferencePutWatsonx<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferencePutWatsonxParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferencePutWatsonx<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferencePutWatsonx] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferencePutWatsonxParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferencePutWatsonx {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferencePutWatsonx<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferencePutWatsonx {
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
    #[doc = "Creates an asynchronous call to the Inference Put Watsonx API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Rerank API"]
pub enum InferenceRerankParts<'b> {
    #[doc = "InferenceId"]
    InferenceId(&'b str),
}
impl<'b> InferenceRerankParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Rerank API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferenceRerankParts::InferenceId(inference_id) => {
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_inference_id.len());
                p.push_str("/_inference/rerank/");
                p.push_str(encoded_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Rerank API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/post-inference-api.html)\n\nPerform reranking inference"]
#[derive(Clone, Debug)]
pub struct InferenceRerank<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferenceRerankParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferenceRerank<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferenceRerank] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferenceRerankParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferenceRerank {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferenceRerank<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferenceRerank {
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
    #[doc = "Creates an asynchronous call to the Inference Rerank API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Sparse Embedding API"]
pub enum InferenceSparseEmbeddingParts<'b> {
    #[doc = "InferenceId"]
    InferenceId(&'b str),
}
impl<'b> InferenceSparseEmbeddingParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Sparse Embedding API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferenceSparseEmbeddingParts::InferenceId(inference_id) => {
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_inference_id.len());
                p.push_str("/_inference/sparse_embedding/");
                p.push_str(encoded_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Sparse Embedding API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/post-inference-api.html)\n\nPerform sparse embedding inference"]
#[derive(Clone, Debug)]
pub struct InferenceSparseEmbedding<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferenceSparseEmbeddingParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferenceSparseEmbedding<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferenceSparseEmbedding] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferenceSparseEmbeddingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferenceSparseEmbedding {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferenceSparseEmbedding<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferenceSparseEmbedding {
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
    #[doc = "Creates an asynchronous call to the Inference Sparse Embedding API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Stream Completion API"]
pub enum InferenceStreamCompletionParts<'b> {
    #[doc = "InferenceId"]
    InferenceId(&'b str),
}
impl<'b> InferenceStreamCompletionParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Stream Completion API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferenceStreamCompletionParts::InferenceId(inference_id) => {
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(31usize + encoded_inference_id.len());
                p.push_str("/_inference/completion/");
                p.push_str(encoded_inference_id.as_ref());
                p.push_str("/_stream");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Stream Completion API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/post-stream-inference-api.html)\n\nPerform streaming completion inference"]
#[derive(Clone, Debug)]
pub struct InferenceStreamCompletion<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferenceStreamCompletionParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferenceStreamCompletion<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferenceStreamCompletion] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferenceStreamCompletionParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferenceStreamCompletion {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferenceStreamCompletion<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferenceStreamCompletion {
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
    #[doc = "Creates an asynchronous call to the Inference Stream Completion API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Text Embedding API"]
pub enum InferenceTextEmbeddingParts<'b> {
    #[doc = "InferenceId"]
    InferenceId(&'b str),
}
impl<'b> InferenceTextEmbeddingParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Text Embedding API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferenceTextEmbeddingParts::InferenceId(inference_id) => {
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_inference_id.len());
                p.push_str("/_inference/text_embedding/");
                p.push_str(encoded_inference_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Text Embedding API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/post-inference-api.html)\n\nPerform text embedding inference"]
#[derive(Clone, Debug)]
pub struct InferenceTextEmbedding<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferenceTextEmbeddingParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferenceTextEmbedding<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferenceTextEmbedding] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferenceTextEmbeddingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferenceTextEmbedding {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferenceTextEmbedding<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferenceTextEmbedding {
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
    #[doc = "Creates an asynchronous call to the Inference Text Embedding API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Update API"]
pub enum InferenceUpdateParts<'b> {
    #[doc = "InferenceId"]
    InferenceId(&'b str),
    #[doc = "TaskType and InferenceId"]
    TaskTypeInferenceId(&'b str, &'b str),
}
impl<'b> InferenceUpdateParts<'b> {
    #[doc = "Builds a relative URL path to the Inference Update API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InferenceUpdateParts::InferenceId(inference_id) => {
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_inference_id.len());
                p.push_str("/_inference/");
                p.push_str(encoded_inference_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
            InferenceUpdateParts::TaskTypeInferenceId(task_type, inference_id) => {
                let encoded_task_type: Cow<str> =
                    percent_encode(task_type.as_bytes(), PARTS_ENCODED).into();
                let encoded_inference_id: Cow<str> =
                    percent_encode(inference_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    21usize + encoded_task_type.len() + encoded_inference_id.len(),
                );
                p.push_str("/_inference/");
                p.push_str(encoded_task_type.as_ref());
                p.push('/');
                p.push_str(encoded_inference_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Inference Update API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/update-inference-api.html)\n\nUpdate inference"]
#[derive(Clone, Debug)]
pub struct InferenceUpdate<'a, 'b, B> {
    transport: &'a Transport,
    parts: InferenceUpdateParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> InferenceUpdate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [InferenceUpdate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: InferenceUpdateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        InferenceUpdate {
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
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> InferenceUpdate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        InferenceUpdate {
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
    #[doc = "Creates an asynchronous call to the Inference Update API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Inference APIs"]
pub struct Inference<'a> {
    transport: &'a Transport,
}
impl<'a> Inference<'a> {
    #[doc = "Creates a new instance of [Inference]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Inference Chat Completion Unified API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/chat-completion-inference.html)\n\nPerform chat completion inference"]
    pub fn chat_completion_unified<'b>(
        &'a self,
        parts: InferenceChatCompletionUnifiedParts<'b>,
    ) -> InferenceChatCompletionUnified<'a, 'b, ()> {
        InferenceChatCompletionUnified::new(self.transport(), parts)
    }
    #[doc = "[Inference Completion API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/post-inference-api.html)\n\nPerform completion inference"]
    pub fn completion<'b>(
        &'a self,
        parts: InferenceCompletionParts<'b>,
    ) -> InferenceCompletion<'a, 'b, ()> {
        InferenceCompletion::new(self.transport(), parts)
    }
    #[doc = "[Inference Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/delete-inference-api.html)\n\nDelete an inference endpoint"]
    pub fn delete<'b>(&'a self, parts: InferenceDeleteParts<'b>) -> InferenceDelete<'a, 'b> {
        InferenceDelete::new(self.transport(), parts)
    }
    #[doc = "[Inference Get API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/get-inference-api.html)\n\nGet an inference endpoint"]
    pub fn get<'b>(&'a self, parts: InferenceGetParts<'b>) -> InferenceGet<'a, 'b> {
        InferenceGet::new(self.transport(), parts)
    }
    #[doc = "[Inference Inference API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/post-inference-api.html)\n\nPerform inference"]
    pub fn inference<'b>(
        &'a self,
        parts: InferenceInferenceParts<'b>,
    ) -> InferenceInference<'a, 'b, ()> {
        InferenceInference::new(self.transport(), parts)
    }
    #[doc = "[Inference Put API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/put-inference-api.html)\n\nConfigure an inference endpoint for use in the Inference API"]
    pub fn put<'b>(&'a self, parts: InferencePutParts<'b>) -> InferencePut<'a, 'b, ()> {
        InferencePut::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Alibabacloud API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-alibabacloud-ai-search.html)\n\nConfigure an AlibabaCloud AI Search inference endpoint"]
    pub fn put_alibabacloud<'b>(
        &'a self,
        parts: InferencePutAlibabacloudParts<'b>,
    ) -> InferencePutAlibabacloud<'a, 'b, ()> {
        InferencePutAlibabacloud::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Amazonbedrock API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-amazon-bedrock.html)\n\nConfigure an Amazon Bedrock inference endpoint"]
    pub fn put_amazonbedrock<'b>(
        &'a self,
        parts: InferencePutAmazonbedrockParts<'b>,
    ) -> InferencePutAmazonbedrock<'a, 'b, ()> {
        InferencePutAmazonbedrock::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Anthropic API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-anthropic.html)\n\nConfigure an Anthropic inference endpoint"]
    pub fn put_anthropic<'b>(
        &'a self,
        parts: InferencePutAnthropicParts<'b>,
    ) -> InferencePutAnthropic<'a, 'b, ()> {
        InferencePutAnthropic::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Azureaistudio API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-azure-ai-studio.html)\n\nConfigure an Azure AI Studio inference endpoint"]
    pub fn put_azureaistudio<'b>(
        &'a self,
        parts: InferencePutAzureaistudioParts<'b>,
    ) -> InferencePutAzureaistudio<'a, 'b, ()> {
        InferencePutAzureaistudio::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Azureopenai API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-azure-openai.html)\n\nConfigure an Azure OpenAI inference endpoint"]
    pub fn put_azureopenai<'b>(
        &'a self,
        parts: InferencePutAzureopenaiParts<'b>,
    ) -> InferencePutAzureopenai<'a, 'b, ()> {
        InferencePutAzureopenai::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Cohere API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-cohere.html)\n\nConfigure a Cohere inference endpoint"]
    pub fn put_cohere<'b>(
        &'a self,
        parts: InferencePutCohereParts<'b>,
    ) -> InferencePutCohere<'a, 'b, ()> {
        InferencePutCohere::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Elasticsearch API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-elasticsearch.html)\n\nConfigure an Elasticsearch inference endpoint"]
    pub fn put_elasticsearch<'b>(
        &'a self,
        parts: InferencePutElasticsearchParts<'b>,
    ) -> InferencePutElasticsearch<'a, 'b, ()> {
        InferencePutElasticsearch::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Elser API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-elser.html)\n\nConfigure an ELSER inference endpoint"]
    pub fn put_elser<'b>(
        &'a self,
        parts: InferencePutElserParts<'b>,
    ) -> InferencePutElser<'a, 'b, ()> {
        InferencePutElser::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Googleaistudio API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-google-ai-studio.html)\n\nConfigure a Google AI Studio inference endpoint"]
    pub fn put_googleaistudio<'b>(
        &'a self,
        parts: InferencePutGoogleaistudioParts<'b>,
    ) -> InferencePutGoogleaistudio<'a, 'b, ()> {
        InferencePutGoogleaistudio::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Googlevertexai API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-google-vertex-ai.html)\n\nConfigure a Google Vertex AI inference endpoint"]
    pub fn put_googlevertexai<'b>(
        &'a self,
        parts: InferencePutGooglevertexaiParts<'b>,
    ) -> InferencePutGooglevertexai<'a, 'b, ()> {
        InferencePutGooglevertexai::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Hugging Face API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-hugging-face.html)\n\nConfigure a HuggingFace inference endpoint"]
    pub fn put_hugging_face<'b>(
        &'a self,
        parts: InferencePutHuggingFaceParts<'b>,
    ) -> InferencePutHuggingFace<'a, 'b, ()> {
        InferencePutHuggingFace::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Jinaai API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-jinaai.html)\n\nConfigure a JinaAI inference endpoint"]
    pub fn put_jinaai<'b>(
        &'a self,
        parts: InferencePutJinaaiParts<'b>,
    ) -> InferencePutJinaai<'a, 'b, ()> {
        InferencePutJinaai::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Mistral API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-mistral.html)\n\nConfigure a Mistral inference endpoint"]
    pub fn put_mistral<'b>(
        &'a self,
        parts: InferencePutMistralParts<'b>,
    ) -> InferencePutMistral<'a, 'b, ()> {
        InferencePutMistral::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Openai API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-openai.html)\n\nConfigure an OpenAI inference endpoint"]
    pub fn put_openai<'b>(
        &'a self,
        parts: InferencePutOpenaiParts<'b>,
    ) -> InferencePutOpenai<'a, 'b, ()> {
        InferencePutOpenai::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Voyageai API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/inference-apis.html)\n\nConfigure a VoyageAI inference endpoint"]
    pub fn put_voyageai<'b>(
        &'a self,
        parts: InferencePutVoyageaiParts<'b>,
    ) -> InferencePutVoyageai<'a, 'b, ()> {
        InferencePutVoyageai::new(self.transport(), parts)
    }
    #[doc = "[Inference Put Watsonx API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/infer-service-watsonx-ai.html)\n\nConfigure a Watsonx inference endpoint"]
    pub fn put_watsonx<'b>(
        &'a self,
        parts: InferencePutWatsonxParts<'b>,
    ) -> InferencePutWatsonx<'a, 'b, ()> {
        InferencePutWatsonx::new(self.transport(), parts)
    }
    #[doc = "[Inference Rerank API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/post-inference-api.html)\n\nPerform reranking inference"]
    pub fn rerank<'b>(&'a self, parts: InferenceRerankParts<'b>) -> InferenceRerank<'a, 'b, ()> {
        InferenceRerank::new(self.transport(), parts)
    }
    #[doc = "[Inference Sparse Embedding API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/post-inference-api.html)\n\nPerform sparse embedding inference"]
    pub fn sparse_embedding<'b>(
        &'a self,
        parts: InferenceSparseEmbeddingParts<'b>,
    ) -> InferenceSparseEmbedding<'a, 'b, ()> {
        InferenceSparseEmbedding::new(self.transport(), parts)
    }
    #[doc = "[Inference Stream Completion API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/post-stream-inference-api.html)\n\nPerform streaming completion inference"]
    pub fn stream_completion<'b>(
        &'a self,
        parts: InferenceStreamCompletionParts<'b>,
    ) -> InferenceStreamCompletion<'a, 'b, ()> {
        InferenceStreamCompletion::new(self.transport(), parts)
    }
    #[doc = "[Inference Text Embedding API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/post-inference-api.html)\n\nPerform text embedding inference"]
    pub fn text_embedding<'b>(
        &'a self,
        parts: InferenceTextEmbeddingParts<'b>,
    ) -> InferenceTextEmbedding<'a, 'b, ()> {
        InferenceTextEmbedding::new(self.transport(), parts)
    }
    #[doc = "[Inference Update API](https://www.elastic.co/guide/en/elasticsearch/reference/8.18/update-inference-api.html)\n\nUpdate inference"]
    pub fn update<'b>(&'a self, parts: InferenceUpdateParts<'b>) -> InferenceUpdate<'a, 'b, ()> {
        InferenceUpdate::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Inference APIs"]
    pub fn inference(&self) -> Inference {
        Inference::new(self.transport())
    }
}
