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

//! Enrich APIs
//!
//! Manage [enrich policies](https://www.elastic.co/guide/en/elasticsearch/reference/master/ingest-enriching-data.html#enrich-policy)
//! that can be used by the [enrich processor](https://www.elastic.co/guide/en/elasticsearch/reference/master/enrich-processor.html)
//! as part of an [ingest pipeline](../ingest/index.html), to add data from your existing indices
//! to incoming documents during ingest.

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
#[doc = "API parts for the Enrich Delete Policy API"]
pub enum EnrichDeletePolicyParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> EnrichDeletePolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Enrich Delete Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            EnrichDeletePolicyParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_name.len());
                p.push_str("/_enrich/policy/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Enrich Delete Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/delete-enrich-policy-api.html)\n\nDeletes an existing enrich policy and its enrich index."]
#[derive(Clone, Debug)]
pub struct EnrichDeletePolicy<'a, 'b> {
    transport: &'a Transport,
    parts: EnrichDeletePolicyParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> EnrichDeletePolicy<'a, 'b> {
    #[doc = "Creates a new instance of [EnrichDeletePolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: EnrichDeletePolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        EnrichDeletePolicy {
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
    #[doc = "Creates an asynchronous call to the Enrich Delete Policy API that can be awaited"]
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
#[doc = "API parts for the Enrich Execute Policy API"]
pub enum EnrichExecutePolicyParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> EnrichExecutePolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Enrich Execute Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            EnrichExecutePolicyParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(25usize + encoded_name.len());
                p.push_str("/_enrich/policy/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/_execute");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Enrich Execute Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/execute-enrich-policy-api.html)\n\nCreates the enrich index for an existing enrich policy."]
#[derive(Clone, Debug)]
pub struct EnrichExecutePolicy<'a, 'b, B> {
    transport: &'a Transport,
    parts: EnrichExecutePolicyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> EnrichExecutePolicy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [EnrichExecutePolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: EnrichExecutePolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        EnrichExecutePolicy {
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
            wait_for_completion: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> EnrichExecutePolicy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        EnrichExecutePolicy {
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
            wait_for_completion: self.wait_for_completion,
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
    #[doc = "Should the request should block until the execution is complete."]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Enrich Execute Policy API that can be awaited"]
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
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                wait_for_completion: self.wait_for_completion,
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
#[doc = "API parts for the Enrich Get Policy API"]
pub enum EnrichGetPolicyParts<'b> {
    #[doc = "Name"]
    Name(&'b [&'b str]),
    #[doc = "No parts"]
    None,
}
impl<'b> EnrichGetPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Enrich Get Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            EnrichGetPolicyParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_name.len());
                p.push_str("/_enrich/policy/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
            EnrichGetPolicyParts::None => "/_enrich/policy/".into(),
        }
    }
}
#[doc = "Builder for the [Enrich Get Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/get-enrich-policy-api.html)\n\nGets information about an enrich policy."]
#[derive(Clone, Debug)]
pub struct EnrichGetPolicy<'a, 'b> {
    transport: &'a Transport,
    parts: EnrichGetPolicyParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> EnrichGetPolicy<'a, 'b> {
    #[doc = "Creates a new instance of [EnrichGetPolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: EnrichGetPolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        EnrichGetPolicy {
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
    #[doc = "Creates an asynchronous call to the Enrich Get Policy API that can be awaited"]
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
#[doc = "API parts for the Enrich Put Policy API"]
pub enum EnrichPutPolicyParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> EnrichPutPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Enrich Put Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            EnrichPutPolicyParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_name.len());
                p.push_str("/_enrich/policy/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Enrich Put Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/put-enrich-policy-api.html)\n\nCreates a new enrich policy."]
#[derive(Clone, Debug)]
pub struct EnrichPutPolicy<'a, 'b, B> {
    transport: &'a Transport,
    parts: EnrichPutPolicyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> EnrichPutPolicy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [EnrichPutPolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: EnrichPutPolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        EnrichPutPolicy {
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
    pub fn body<T>(self, body: T) -> EnrichPutPolicy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        EnrichPutPolicy {
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
    #[doc = "Creates an asynchronous call to the Enrich Put Policy API that can be awaited"]
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Enrich Stats API"]
pub enum EnrichStatsParts {
    #[doc = "No parts"]
    None,
}
impl EnrichStatsParts {
    #[doc = "Builds a relative URL path to the Enrich Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            EnrichStatsParts::None => "/_enrich/_stats".into(),
        }
    }
}
#[doc = "Builder for the [Enrich Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/enrich-stats-api.html)\n\nGets enrich coordinator statistics and information about enrich policies that are currently executing."]
#[derive(Clone, Debug)]
pub struct EnrichStats<'a, 'b> {
    transport: &'a Transport,
    parts: EnrichStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> EnrichStats<'a, 'b> {
    #[doc = "Creates a new instance of [EnrichStats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        EnrichStats {
            transport,
            parts: EnrichStatsParts::None,
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
    #[doc = "Creates an asynchronous call to the Enrich Stats API that can be awaited"]
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
#[doc = "Namespace client for Enrich APIs"]
pub struct Enrich<'a> {
    transport: &'a Transport,
}
impl<'a> Enrich<'a> {
    #[doc = "Creates a new instance of [Enrich]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Enrich Delete Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/delete-enrich-policy-api.html)\n\nDeletes an existing enrich policy and its enrich index."]
    pub fn delete_policy<'b>(
        &'a self,
        parts: EnrichDeletePolicyParts<'b>,
    ) -> EnrichDeletePolicy<'a, 'b> {
        EnrichDeletePolicy::new(self.transport(), parts)
    }
    #[doc = "[Enrich Execute Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/execute-enrich-policy-api.html)\n\nCreates the enrich index for an existing enrich policy."]
    pub fn execute_policy<'b>(
        &'a self,
        parts: EnrichExecutePolicyParts<'b>,
    ) -> EnrichExecutePolicy<'a, 'b, ()> {
        EnrichExecutePolicy::new(self.transport(), parts)
    }
    #[doc = "[Enrich Get Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/get-enrich-policy-api.html)\n\nGets information about an enrich policy."]
    pub fn get_policy<'b>(&'a self, parts: EnrichGetPolicyParts<'b>) -> EnrichGetPolicy<'a, 'b> {
        EnrichGetPolicy::new(self.transport(), parts)
    }
    #[doc = "[Enrich Put Policy API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/put-enrich-policy-api.html)\n\nCreates a new enrich policy."]
    pub fn put_policy<'b>(
        &'a self,
        parts: EnrichPutPolicyParts<'b>,
    ) -> EnrichPutPolicy<'a, 'b, ()> {
        EnrichPutPolicy::new(self.transport(), parts)
    }
    #[doc = "[Enrich Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/enrich-stats-api.html)\n\nGets enrich coordinator statistics and information about enrich policies that are currently executing."]
    pub fn stats<'b>(&'a self) -> EnrichStats<'a, 'b> {
        EnrichStats::new(self.transport())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Enrich APIs"]
    pub fn enrich(&self) -> Enrich {
        Enrich::new(self.transport())
    }
}
