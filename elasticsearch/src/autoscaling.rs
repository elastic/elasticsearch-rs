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

//! Autoscaling APIs
//!
//! The [autoscaling feature](https://www.elastic.co/guide/en/elasticsearch/reference/master/xpack-autoscaling.html)
//! enables an operator to configure tiers of nodes that self-monitor whether or not they need to scale based on an
//! operator-defined policy.

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
#[doc = "API parts for the Autoscaling Delete Autoscaling Policy API"]
pub enum AutoscalingDeleteAutoscalingPolicyParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> AutoscalingDeleteAutoscalingPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Autoscaling Delete Autoscaling Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            AutoscalingDeleteAutoscalingPolicyParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_autoscaling/policy/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Autoscaling Delete Autoscaling Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/autoscaling-delete-autoscaling-policy.html)\n\nDeletes an autoscaling policy. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
#[derive(Clone, Debug)]
pub struct AutoscalingDeleteAutoscalingPolicy<'a, 'b> {
    transport: &'a Transport,
    parts: AutoscalingDeleteAutoscalingPolicyParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> AutoscalingDeleteAutoscalingPolicy<'a, 'b> {
    #[doc = "Creates a new instance of [AutoscalingDeleteAutoscalingPolicy] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: AutoscalingDeleteAutoscalingPolicyParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        AutoscalingDeleteAutoscalingPolicy {
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
    #[doc = "Creates an asynchronous call to the Autoscaling Delete Autoscaling Policy API that can be awaited"]
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
#[doc = "API parts for the Autoscaling Get Autoscaling Capacity API"]
pub enum AutoscalingGetAutoscalingCapacityParts {
    #[doc = "No parts"]
    None,
}
impl AutoscalingGetAutoscalingCapacityParts {
    #[doc = "Builds a relative URL path to the Autoscaling Get Autoscaling Capacity API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            AutoscalingGetAutoscalingCapacityParts::None => "/_autoscaling/capacity".into(),
        }
    }
}
#[doc = "Builder for the [Autoscaling Get Autoscaling Capacity API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/autoscaling-get-autoscaling-capacity.html)\n\nGets the current autoscaling capacity based on the configured autoscaling policy. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
#[derive(Clone, Debug)]
pub struct AutoscalingGetAutoscalingCapacity<'a, 'b> {
    transport: &'a Transport,
    parts: AutoscalingGetAutoscalingCapacityParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> AutoscalingGetAutoscalingCapacity<'a, 'b> {
    #[doc = "Creates a new instance of [AutoscalingGetAutoscalingCapacity]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        AutoscalingGetAutoscalingCapacity {
            transport,
            parts: AutoscalingGetAutoscalingCapacityParts::None,
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
    #[doc = "Creates an asynchronous call to the Autoscaling Get Autoscaling Capacity API that can be awaited"]
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
#[doc = "API parts for the Autoscaling Get Autoscaling Policy API"]
pub enum AutoscalingGetAutoscalingPolicyParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> AutoscalingGetAutoscalingPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Autoscaling Get Autoscaling Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            AutoscalingGetAutoscalingPolicyParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_autoscaling/policy/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Autoscaling Get Autoscaling Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/autoscaling-get-autoscaling-policy.html)\n\nRetrieves an autoscaling policy. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
#[derive(Clone, Debug)]
pub struct AutoscalingGetAutoscalingPolicy<'a, 'b> {
    transport: &'a Transport,
    parts: AutoscalingGetAutoscalingPolicyParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> AutoscalingGetAutoscalingPolicy<'a, 'b> {
    #[doc = "Creates a new instance of [AutoscalingGetAutoscalingPolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: AutoscalingGetAutoscalingPolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        AutoscalingGetAutoscalingPolicy {
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
    #[doc = "Creates an asynchronous call to the Autoscaling Get Autoscaling Policy API that can be awaited"]
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
#[doc = "API parts for the Autoscaling Put Autoscaling Policy API"]
pub enum AutoscalingPutAutoscalingPolicyParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> AutoscalingPutAutoscalingPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Autoscaling Put Autoscaling Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            AutoscalingPutAutoscalingPolicyParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_autoscaling/policy/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Autoscaling Put Autoscaling Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/autoscaling-put-autoscaling-policy.html)\n\nCreates a new autoscaling policy. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
#[derive(Clone, Debug)]
pub struct AutoscalingPutAutoscalingPolicy<'a, 'b, B> {
    transport: &'a Transport,
    parts: AutoscalingPutAutoscalingPolicyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> AutoscalingPutAutoscalingPolicy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [AutoscalingPutAutoscalingPolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: AutoscalingPutAutoscalingPolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        AutoscalingPutAutoscalingPolicy {
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
    pub fn body<T>(self, body: T) -> AutoscalingPutAutoscalingPolicy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        AutoscalingPutAutoscalingPolicy {
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
    #[doc = "Creates an asynchronous call to the Autoscaling Put Autoscaling Policy API that can be awaited"]
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
#[doc = "Namespace client for Autoscaling APIs"]
pub struct Autoscaling<'a> {
    transport: &'a Transport,
}
impl<'a> Autoscaling<'a> {
    #[doc = "Creates a new instance of [Autoscaling]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Autoscaling Delete Autoscaling Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/autoscaling-delete-autoscaling-policy.html)\n\nDeletes an autoscaling policy. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
    pub fn delete_autoscaling_policy<'b>(
        &'a self,
        parts: AutoscalingDeleteAutoscalingPolicyParts<'b>,
    ) -> AutoscalingDeleteAutoscalingPolicy<'a, 'b> {
        AutoscalingDeleteAutoscalingPolicy::new(self.transport(), parts)
    }
    #[doc = "[Autoscaling Get Autoscaling Capacity API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/autoscaling-get-autoscaling-capacity.html)\n\nGets the current autoscaling capacity based on the configured autoscaling policy. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
    pub fn get_autoscaling_capacity<'b>(&'a self) -> AutoscalingGetAutoscalingCapacity<'a, 'b> {
        AutoscalingGetAutoscalingCapacity::new(self.transport())
    }
    #[doc = "[Autoscaling Get Autoscaling Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/autoscaling-get-autoscaling-policy.html)\n\nRetrieves an autoscaling policy. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
    pub fn get_autoscaling_policy<'b>(
        &'a self,
        parts: AutoscalingGetAutoscalingPolicyParts<'b>,
    ) -> AutoscalingGetAutoscalingPolicy<'a, 'b> {
        AutoscalingGetAutoscalingPolicy::new(self.transport(), parts)
    }
    #[doc = "[Autoscaling Put Autoscaling Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/autoscaling-put-autoscaling-policy.html)\n\nCreates a new autoscaling policy. Designed for indirect use by ECE/ESS and ECK. Direct use is not supported."]
    pub fn put_autoscaling_policy<'b>(
        &'a self,
        parts: AutoscalingPutAutoscalingPolicyParts<'b>,
    ) -> AutoscalingPutAutoscalingPolicy<'a, 'b, ()> {
        AutoscalingPutAutoscalingPolicy::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Autoscaling APIs"]
    pub fn autoscaling(&self) -> Autoscaling {
        Autoscaling::new(self.transport())
    }
}
