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

//! X-Pack APIs
//!
//! Provide general information about the installed X-Pack features and their usage.

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
#[doc = "API parts for the Xpack Info API"]
pub enum XpackInfoParts {
    #[doc = "No parts"]
    None,
}
impl XpackInfoParts {
    #[doc = "Builds a relative URL path to the Xpack Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            XpackInfoParts::None => "/_xpack".into(),
        }
    }
}
#[doc = "Builder for the [Xpack Info API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/info-api.html)\n\nRetrieves information about the installed X-Pack features."]
#[derive(Clone, Debug)]
pub struct XpackInfo<'a, 'b> {
    transport: &'a Transport,
    parts: XpackInfoParts,
    accept_enterprise: Option<bool>,
    categories: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> XpackInfo<'a, 'b> {
    #[doc = "Creates a new instance of [XpackInfo]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        XpackInfo {
            transport,
            parts: XpackInfoParts::None,
            headers,
            accept_enterprise: None,
            categories: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "If an enterprise license is installed, return the type and mode as 'enterprise' (default: false)"]
    pub fn accept_enterprise(mut self, accept_enterprise: bool) -> Self {
        self.accept_enterprise = Some(accept_enterprise);
        self
    }
    #[doc = "Comma-separated list of info categories. Can be any of: build, license, features"]
    pub fn categories(mut self, categories: &'b [&'b str]) -> Self {
        self.categories = Some(categories);
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
    #[doc = "Creates an asynchronous call to the Xpack Info API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                accept_enterprise: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                categories: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                accept_enterprise: self.accept_enterprise,
                categories: self.categories,
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
#[doc = "API parts for the Xpack Usage API"]
pub enum XpackUsageParts {
    #[doc = "No parts"]
    None,
}
impl XpackUsageParts {
    #[doc = "Builds a relative URL path to the Xpack Usage API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            XpackUsageParts::None => "/_xpack/usage".into(),
        }
    }
}
#[doc = "Builder for the [Xpack Usage API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/usage-api.html)\n\nRetrieves usage information about the installed X-Pack features."]
#[derive(Clone, Debug)]
pub struct XpackUsage<'a, 'b> {
    transport: &'a Transport,
    parts: XpackUsageParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> XpackUsage<'a, 'b> {
    #[doc = "Creates a new instance of [XpackUsage]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        XpackUsage {
            transport,
            parts: XpackUsageParts::None,
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
    #[doc = "Specify timeout for watch write operation"]
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
    #[doc = "Creates an asynchronous call to the Xpack Usage API that can be awaited"]
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
#[doc = "Namespace client for X-Pack APIs"]
pub struct Xpack<'a> {
    transport: &'a Transport,
}
impl<'a> Xpack<'a> {
    #[doc = "Creates a new instance of [Xpack]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Xpack Info API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/info-api.html)\n\nRetrieves information about the installed X-Pack features."]
    pub fn info<'b>(&'a self) -> XpackInfo<'a, 'b> {
        XpackInfo::new(self.transport())
    }
    #[doc = "[Xpack Usage API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/usage-api.html)\n\nRetrieves usage information about the installed X-Pack features."]
    pub fn usage<'b>(&'a self) -> XpackUsage<'a, 'b> {
        XpackUsage::new(self.transport())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for X-Pack APIs"]
    pub fn xpack(&self) -> Xpack {
        Xpack::new(self.transport())
    }
}
