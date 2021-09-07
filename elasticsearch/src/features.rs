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

//! Features APIs
//!
//! Allows [introspecting and managing features](https://www.elastic.co/guide/en/elasticsearch/reference/current/features-apis.html) provided by Elasticsearch and Elasticsearch plugins.

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
#[doc = "API parts for the Features Get Features API"]
pub enum FeaturesGetFeaturesParts {
    #[doc = "No parts"]
    None,
}
impl FeaturesGetFeaturesParts {
    #[doc = "Builds a relative URL path to the Features Get Features API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FeaturesGetFeaturesParts::None => "/_features".into(),
        }
    }
}
#[doc = "Builder for the [Features Get Features API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/get-features-api.html)\n\nGets a list of features which can be included in snapshots using the feature_states field when creating a snapshot"]
#[derive(Clone, Debug)]
pub struct FeaturesGetFeatures<'a, 'b> {
    transport: &'a Transport,
    parts: FeaturesGetFeaturesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> FeaturesGetFeatures<'a, 'b> {
    #[doc = "Creates a new instance of [FeaturesGetFeatures]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        FeaturesGetFeatures {
            transport,
            parts: FeaturesGetFeaturesParts::None,
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
    #[doc = "Creates an asynchronous call to the Features Get Features API that can be awaited"]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Features Reset Features API"]
pub enum FeaturesResetFeaturesParts {
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl FeaturesResetFeaturesParts {
    #[doc = "Builds a relative URL path to the Features Reset Features API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FeaturesResetFeaturesParts::None => "/_features/_reset".into(),
        }
    }
}
#[doc = "Builder for the [Features Reset Features API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/modules-snapshots.html)\n\nResets the internal state of features, usually by deleting system indices"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct FeaturesResetFeatures<'a, 'b, B> {
    transport: &'a Transport,
    parts: FeaturesResetFeaturesParts,
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
impl<'a, 'b, B> FeaturesResetFeatures<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [FeaturesResetFeatures]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        FeaturesResetFeatures {
            transport,
            parts: FeaturesResetFeaturesParts::None,
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
    pub fn body<T>(self, body: T) -> FeaturesResetFeatures<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        FeaturesResetFeatures {
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
    #[doc = "Creates an asynchronous call to the Features Reset Features API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "Namespace client for Features APIs"]
pub struct Features<'a> {
    transport: &'a Transport,
}
impl<'a> Features<'a> {
    #[doc = "Creates a new instance of [Features]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Features Get Features API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/get-features-api.html)\n\nGets a list of features which can be included in snapshots using the feature_states field when creating a snapshot"]
    pub fn get_features<'b>(&'a self) -> FeaturesGetFeatures<'a, 'b> {
        FeaturesGetFeatures::new(self.transport())
    }
    #[doc = "[Features Reset Features API](https://www.elastic.co/guide/en/elasticsearch/reference/7.13/modules-snapshots.html)\n\nResets the internal state of features, usually by deleting system indices"]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn reset_features<'b>(&'a self) -> FeaturesResetFeatures<'a, 'b, ()> {
        FeaturesResetFeatures::new(self.transport())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Features APIs"]
    pub fn features(&self) -> Features {
        Features::new(self.transport())
    }
}
