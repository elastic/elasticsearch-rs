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
// cargo run -p api_generator
// -----------------------------------------------
#![allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        Method,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the License Delete API"]
pub enum LicenseDeleteParts {
    #[doc = "No parts"]
    None,
}
impl LicenseDeleteParts {
    #[doc = "Builds a relative URL path to the License Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicenseDeleteParts::None => "/_license".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/delete-license.html)\n\nDeletes licensing information for the cluster"]
pub struct LicenseDelete<'a, 'b> {
    client: &'a Elasticsearch,
    parts: LicenseDeleteParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> LicenseDelete<'a, 'b> {
    #[doc = "Creates a new instance of [LicenseDelete]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        LicenseDelete {
            client,
            parts: LicenseDeleteParts::None,
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
    #[doc = "Creates an asynchronous call to the License Delete API that can be awaited"]
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
#[doc = "API parts for the License Get API"]
pub enum LicenseGetParts {
    #[doc = "No parts"]
    None,
}
impl LicenseGetParts {
    #[doc = "Builds a relative URL path to the License Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicenseGetParts::None => "/_license".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/get-license.html)\n\nRetrieves licensing information for the cluster"]
pub struct LicenseGet<'a, 'b> {
    client: &'a Elasticsearch,
    parts: LicenseGetParts,
    accept_enterprise: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> LicenseGet<'a, 'b> {
    #[doc = "Creates a new instance of [LicenseGet]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        LicenseGet {
            client,
            parts: LicenseGetParts::None,
            headers,
            accept_enterprise: None,
            error_trace: None,
            filter_path: None,
            human: None,
            local: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "If the active license is an enterprise license, return type as 'enterprise' (default: false)"]
    pub fn accept_enterprise(mut self, accept_enterprise: bool) -> Self {
        self.accept_enterprise = Some(accept_enterprise);
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
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the License Get API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "accept_enterprise")]
                accept_enterprise: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                accept_enterprise: self.accept_enterprise,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                local: self.local,
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
#[doc = "API parts for the License Get Basic Status API"]
pub enum LicenseGetBasicStatusParts {
    #[doc = "No parts"]
    None,
}
impl LicenseGetBasicStatusParts {
    #[doc = "Builds a relative URL path to the License Get Basic Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicenseGetBasicStatusParts::None => "/_license/basic_status".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Get Basic Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/get-basic-status.html)\n\nRetrieves information about the status of the basic license."]
pub struct LicenseGetBasicStatus<'a, 'b> {
    client: &'a Elasticsearch,
    parts: LicenseGetBasicStatusParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> LicenseGetBasicStatus<'a, 'b> {
    #[doc = "Creates a new instance of [LicenseGetBasicStatus]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        LicenseGetBasicStatus {
            client,
            parts: LicenseGetBasicStatusParts::None,
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
    #[doc = "Creates an asynchronous call to the License Get Basic Status API that can be awaited"]
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
#[doc = "API parts for the License Get Trial Status API"]
pub enum LicenseGetTrialStatusParts {
    #[doc = "No parts"]
    None,
}
impl LicenseGetTrialStatusParts {
    #[doc = "Builds a relative URL path to the License Get Trial Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicenseGetTrialStatusParts::None => "/_license/trial_status".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Get Trial Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/get-trial-status.html)\n\nRetrieves information about the status of the trial license."]
pub struct LicenseGetTrialStatus<'a, 'b> {
    client: &'a Elasticsearch,
    parts: LicenseGetTrialStatusParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> LicenseGetTrialStatus<'a, 'b> {
    #[doc = "Creates a new instance of [LicenseGetTrialStatus]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        LicenseGetTrialStatus {
            client,
            parts: LicenseGetTrialStatusParts::None,
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
    #[doc = "Creates an asynchronous call to the License Get Trial Status API that can be awaited"]
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
#[doc = "API parts for the License Post API"]
pub enum LicensePostParts {
    #[doc = "No parts"]
    None,
}
impl LicensePostParts {
    #[doc = "Builds a relative URL path to the License Post API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicensePostParts::None => "/_license".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Post API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/update-license.html)\n\nUpdates the license for the cluster."]
pub struct LicensePost<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: LicensePostParts,
    acknowledge: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> LicensePost<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LicensePost]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        LicensePost {
            client,
            parts: LicensePostParts::None,
            headers,
            acknowledge: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "whether the user has acknowledged acknowledge messages (default: false)"]
    pub fn acknowledge(mut self, acknowledge: bool) -> Self {
        self.acknowledge = Some(acknowledge);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LicensePost<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LicensePost {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            acknowledge: self.acknowledge,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    #[doc = "Creates an asynchronous call to the License Post API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "acknowledge")]
                acknowledge: Option<bool>,
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
                acknowledge: self.acknowledge,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the License Post Start Basic API"]
pub enum LicensePostStartBasicParts {
    #[doc = "No parts"]
    None,
}
impl LicensePostStartBasicParts {
    #[doc = "Builds a relative URL path to the License Post Start Basic API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicensePostStartBasicParts::None => "/_license/start_basic".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Post Start Basic API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/start-basic.html)\n\nStarts an indefinite basic license."]
pub struct LicensePostStartBasic<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: LicensePostStartBasicParts,
    acknowledge: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> LicensePostStartBasic<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LicensePostStartBasic]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        LicensePostStartBasic {
            client,
            parts: LicensePostStartBasicParts::None,
            headers,
            acknowledge: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "whether the user has acknowledged acknowledge messages (default: false)"]
    pub fn acknowledge(mut self, acknowledge: bool) -> Self {
        self.acknowledge = Some(acknowledge);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LicensePostStartBasic<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LicensePostStartBasic {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            acknowledge: self.acknowledge,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    #[doc = "Creates an asynchronous call to the License Post Start Basic API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "acknowledge")]
                acknowledge: Option<bool>,
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
                acknowledge: self.acknowledge,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the License Post Start Trial API"]
pub enum LicensePostStartTrialParts {
    #[doc = "No parts"]
    None,
}
impl LicensePostStartTrialParts {
    #[doc = "Builds a relative URL path to the License Post Start Trial API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LicensePostStartTrialParts::None => "/_license/start_trial".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [License Post Start Trial API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/start-trial.html)\n\nstarts a limited time trial license."]
pub struct LicensePostStartTrial<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: LicensePostStartTrialParts,
    acknowledge: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    ty: Option<&'b str>,
}
impl<'a, 'b, B> LicensePostStartTrial<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LicensePostStartTrial]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        LicensePostStartTrial {
            client,
            parts: LicensePostStartTrialParts::None,
            headers,
            acknowledge: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            ty: None,
        }
    }
    #[doc = "whether the user has acknowledged acknowledge messages (default: false)"]
    pub fn acknowledge(mut self, acknowledge: bool) -> Self {
        self.acknowledge = Some(acknowledge);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LicensePostStartTrial<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LicensePostStartTrial {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            acknowledge: self.acknowledge,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            ty: self.ty,
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
    #[doc = "The type of trial license to generate (default: \"trial\")"]
    pub fn ty(mut self, ty: &'b str) -> Self {
        self.ty = Some(ty);
        self
    }
    #[doc = "Creates an asynchronous call to the License Post Start Trial API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "acknowledge")]
                acknowledge: Option<bool>,
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
                #[serde(rename = "type")]
                ty: Option<&'b str>,
            }
            let query_params = QueryParams {
                acknowledge: self.acknowledge,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                ty: self.ty,
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
#[doc = "Namespace client for License APIs"]
pub struct License<'a> {
    client: &'a Elasticsearch,
}
impl<'a> License<'a> {
    #[doc = "Creates a new instance of [License]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        Self { client }
    }
    #[doc = "[License Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/delete-license.html)\n\nDeletes licensing information for the cluster"]
    pub fn delete<'b>(&'a self) -> LicenseDelete<'a, 'b> {
        LicenseDelete::new(&self.client)
    }
    #[doc = "[License Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/get-license.html)\n\nRetrieves licensing information for the cluster"]
    pub fn get<'b>(&'a self) -> LicenseGet<'a, 'b> {
        LicenseGet::new(&self.client)
    }
    #[doc = "[License Get Basic Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/get-basic-status.html)\n\nRetrieves information about the status of the basic license."]
    pub fn get_basic_status<'b>(&'a self) -> LicenseGetBasicStatus<'a, 'b> {
        LicenseGetBasicStatus::new(&self.client)
    }
    #[doc = "[License Get Trial Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/get-trial-status.html)\n\nRetrieves information about the status of the trial license."]
    pub fn get_trial_status<'b>(&'a self) -> LicenseGetTrialStatus<'a, 'b> {
        LicenseGetTrialStatus::new(&self.client)
    }
    #[doc = "[License Post API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/update-license.html)\n\nUpdates the license for the cluster."]
    pub fn post<'b>(&'a self) -> LicensePost<'a, 'b, ()> {
        LicensePost::new(&self.client)
    }
    #[doc = "[License Post Start Basic API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/start-basic.html)\n\nStarts an indefinite basic license."]
    pub fn post_start_basic<'b>(&'a self) -> LicensePostStartBasic<'a, 'b, ()> {
        LicensePostStartBasic::new(&self.client)
    }
    #[doc = "[License Post Start Trial API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/start-trial.html)\n\nstarts a limited time trial license."]
    pub fn post_start_trial<'b>(&'a self) -> LicensePostStartTrial<'a, 'b, ()> {
        LicensePostStartTrial::new(&self.client)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for License APIs"]
    pub fn license(&self) -> License {
        License::new(&self)
    }
}
