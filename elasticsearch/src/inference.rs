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

#![cfg(feature = "experimental-apis")]
#![doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Delete API"]
pub enum InferenceDeleteParts<'b> {
    #[doc = "InferenceId"]
    InferenceId(&'b str),
    #[doc = "TaskType and InferenceId"]
    TaskTypeInferenceId(&'b str, &'b str),
}
#[cfg(feature = "experimental-apis")]
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
#[doc = "Builder for the [Inference Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-inference-api.html)\n\nDelete an inference endpoint"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
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
#[cfg(feature = "experimental-apis")]
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
#[cfg(feature = "experimental-apis")]
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
#[cfg(feature = "experimental-apis")]
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
#[doc = "Builder for the [Inference Get API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-inference-api.html)\n\nGet an inference endpoint"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
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
#[cfg(feature = "experimental-apis")]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Inference API"]
pub enum InferenceInferenceParts<'b> {
    #[doc = "InferenceId"]
    InferenceId(&'b str),
    #[doc = "TaskType and InferenceId"]
    TaskTypeInferenceId(&'b str, &'b str),
}
#[cfg(feature = "experimental-apis")]
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
#[doc = "Builder for the [Inference Inference API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/post-inference-api.html)\n\nPerform inference"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
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
#[cfg(feature = "experimental-apis")]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Inference Put API"]
pub enum InferencePutParts<'b> {
    #[doc = "InferenceId"]
    InferenceId(&'b str),
    #[doc = "TaskType and InferenceId"]
    TaskTypeInferenceId(&'b str, &'b str),
}
#[cfg(feature = "experimental-apis")]
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
#[doc = "Builder for the [Inference Put API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/put-inference-api.html)\n\nConfigure an inference endpoint for use in the Inference API"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
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
#[cfg(feature = "experimental-apis")]
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
#[doc = "Namespace client for Inference APIs"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
pub struct Inference<'a> {
    transport: &'a Transport,
}
#[cfg(feature = "experimental-apis")]
impl<'a> Inference<'a> {
    #[doc = "Creates a new instance of [Inference]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Inference Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-inference-api.html)\n\nDelete an inference endpoint"]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn delete<'b>(&'a self, parts: InferenceDeleteParts<'b>) -> InferenceDelete<'a, 'b> {
        InferenceDelete::new(self.transport(), parts)
    }
    #[doc = "[Inference Get API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-inference-api.html)\n\nGet an inference endpoint"]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn get<'b>(&'a self, parts: InferenceGetParts<'b>) -> InferenceGet<'a, 'b> {
        InferenceGet::new(self.transport(), parts)
    }
    #[doc = "[Inference Inference API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/post-inference-api.html)\n\nPerform inference"]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn inference<'b>(
        &'a self,
        parts: InferenceInferenceParts<'b>,
    ) -> InferenceInference<'a, 'b, ()> {
        InferenceInference::new(self.transport(), parts)
    }
    #[doc = "[Inference Put API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/put-inference-api.html)\n\nConfigure an inference endpoint for use in the Inference API"]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn put<'b>(&'a self, parts: InferencePutParts<'b>) -> InferencePut<'a, 'b, ()> {
        InferencePut::new(self.transport(), parts)
    }
}
#[cfg(feature = "experimental-apis")]
impl Elasticsearch {
    #[doc = "Creates a namespace client for Inference APIs"]
    pub fn inference(&self) -> Inference {
        Inference::new(self.transport())
    }
}
