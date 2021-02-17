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

//! Logstash APIs
//!
//! The [Logstash APIs](https://www.elastic.co/guide/en/elasticsearch/reference/master/logstash-apis.html) are used to
//! manage pipelines used by Logstash Central Management.

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
#[doc = "API parts for the Logstash Delete Pipeline API"]
pub enum LogstashDeletePipelineParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> LogstashDeletePipelineParts<'b> {
    #[doc = "Builds a relative URL path to the Logstash Delete Pipeline API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LogstashDeletePipelineParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_id.len());
                p.push_str("/_logstash/pipeline/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Logstash Delete Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/logstash-api-delete-pipeline.html)\n\nDeletes Logstash Pipelines used by Central Management"]
#[derive(Clone, Debug)]
pub struct LogstashDeletePipeline<'a, 'b> {
    transport: &'a Transport,
    parts: LogstashDeletePipelineParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> LogstashDeletePipeline<'a, 'b> {
    #[doc = "Creates a new instance of [LogstashDeletePipeline] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LogstashDeletePipelineParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LogstashDeletePipeline {
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
    #[doc = "Creates an asynchronous call to the Logstash Delete Pipeline API that can be awaited"]
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
#[doc = "API parts for the Logstash Get Pipeline API"]
pub enum LogstashGetPipelineParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> LogstashGetPipelineParts<'b> {
    #[doc = "Builds a relative URL path to the Logstash Get Pipeline API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LogstashGetPipelineParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_id.len());
                p.push_str("/_logstash/pipeline/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Logstash Get Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/logstash-api-get-pipeline.html)\n\nRetrieves Logstash Pipelines used by Central Management"]
#[derive(Clone, Debug)]
pub struct LogstashGetPipeline<'a, 'b> {
    transport: &'a Transport,
    parts: LogstashGetPipelineParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> LogstashGetPipeline<'a, 'b> {
    #[doc = "Creates a new instance of [LogstashGetPipeline] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LogstashGetPipelineParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LogstashGetPipeline {
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
    #[doc = "Creates an asynchronous call to the Logstash Get Pipeline API that can be awaited"]
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
#[doc = "API parts for the Logstash Put Pipeline API"]
pub enum LogstashPutPipelineParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> LogstashPutPipelineParts<'b> {
    #[doc = "Builds a relative URL path to the Logstash Put Pipeline API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LogstashPutPipelineParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_id.len());
                p.push_str("/_logstash/pipeline/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Logstash Put Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/logstash-api-put-pipeline.html)\n\nAdds and updates Logstash Pipelines used for Central Management"]
#[derive(Clone, Debug)]
pub struct LogstashPutPipeline<'a, 'b, B> {
    transport: &'a Transport,
    parts: LogstashPutPipelineParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> LogstashPutPipeline<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LogstashPutPipeline] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LogstashPutPipelineParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LogstashPutPipeline {
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
    pub fn body<T>(self, body: T) -> LogstashPutPipeline<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LogstashPutPipeline {
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
    #[doc = "Creates an asynchronous call to the Logstash Put Pipeline API that can be awaited"]
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
#[doc = "Namespace client for Logstash APIs"]
pub struct Logstash<'a> {
    transport: &'a Transport,
}
impl<'a> Logstash<'a> {
    #[doc = "Creates a new instance of [Logstash]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Logstash Delete Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/logstash-api-delete-pipeline.html)\n\nDeletes Logstash Pipelines used by Central Management"]
    pub fn delete_pipeline<'b>(
        &'a self,
        parts: LogstashDeletePipelineParts<'b>,
    ) -> LogstashDeletePipeline<'a, 'b> {
        LogstashDeletePipeline::new(self.transport(), parts)
    }
    #[doc = "[Logstash Get Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/logstash-api-get-pipeline.html)\n\nRetrieves Logstash Pipelines used by Central Management"]
    pub fn get_pipeline<'b>(
        &'a self,
        parts: LogstashGetPipelineParts<'b>,
    ) -> LogstashGetPipeline<'a, 'b> {
        LogstashGetPipeline::new(self.transport(), parts)
    }
    #[doc = "[Logstash Put Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/logstash-api-put-pipeline.html)\n\nAdds and updates Logstash Pipelines used for Central Management"]
    pub fn put_pipeline<'b>(
        &'a self,
        parts: LogstashPutPipelineParts<'b>,
    ) -> LogstashPutPipeline<'a, 'b, ()> {
        LogstashPutPipeline::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Logstash APIs"]
    pub fn logstash(&self) -> Logstash {
        Logstash::new(self.transport())
    }
}
