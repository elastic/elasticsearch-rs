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

//! [The Fleet APIs](https://www.elastic.co/guide/en/elasticsearch/reference/master/fleet-apis.html) support the use of Elasticsearch as a data store for internal agent and action data. These APIs are experimental and for internal use by Fleet only.

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
#[doc = "API parts for the Fleet Delete Secret API"]
pub enum FleetDeleteSecretParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> FleetDeleteSecretParts<'b> {
    #[doc = "Builds a relative URL path to the Fleet Delete Secret API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FleetDeleteSecretParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_id.len());
                p.push_str("/_fleet/secret/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Fleet Delete Secret API\n\nDeletes a secret stored by Fleet."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct FleetDeleteSecret<'a, 'b> {
    transport: &'a Transport,
    parts: FleetDeleteSecretParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> FleetDeleteSecret<'a, 'b> {
    #[doc = "Creates a new instance of [FleetDeleteSecret] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: FleetDeleteSecretParts<'b>) -> Self {
        let headers = HeaderMap::new();
        FleetDeleteSecret {
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
    #[doc = "Creates an asynchronous call to the Fleet Delete Secret API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Delete;
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
#[doc = "API parts for the Fleet Get Secret API"]
pub enum FleetGetSecretParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> FleetGetSecretParts<'b> {
    #[doc = "Builds a relative URL path to the Fleet Get Secret API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FleetGetSecretParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_id.len());
                p.push_str("/_fleet/secret/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Fleet Get Secret API\n\nRetrieves a secret stored by Fleet."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct FleetGetSecret<'a, 'b> {
    transport: &'a Transport,
    parts: FleetGetSecretParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> FleetGetSecret<'a, 'b> {
    #[doc = "Creates a new instance of [FleetGetSecret] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: FleetGetSecretParts<'b>) -> Self {
        let headers = HeaderMap::new();
        FleetGetSecret {
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
    #[doc = "Creates an asynchronous call to the Fleet Get Secret API that can be awaited"]
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
#[doc = "API parts for the Fleet Global Checkpoints API"]
pub enum FleetGlobalCheckpointsParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> FleetGlobalCheckpointsParts<'b> {
    #[doc = "Builds a relative URL path to the Fleet Global Checkpoints API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FleetGlobalCheckpointsParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_fleet/global_checkpoints");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Fleet Global Checkpoints API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-global-checkpoints.html)\n\nReturns the current global checkpoints for an index. This API is design for internal use by the fleet server project."]
#[derive(Clone, Debug)]
pub struct FleetGlobalCheckpoints<'a, 'b> {
    transport: &'a Transport,
    parts: FleetGlobalCheckpointsParts<'b>,
    checkpoints: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_advance: Option<bool>,
    wait_for_index: Option<bool>,
}
impl<'a, 'b> FleetGlobalCheckpoints<'a, 'b> {
    #[doc = "Creates a new instance of [FleetGlobalCheckpoints] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: FleetGlobalCheckpointsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        FleetGlobalCheckpoints {
            transport,
            parts,
            headers,
            checkpoints: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_advance: None,
            wait_for_index: None,
        }
    }
    #[doc = "Comma separated list of checkpoints"]
    pub fn checkpoints(mut self, checkpoints: &'b [&'b str]) -> Self {
        self.checkpoints = Some(checkpoints);
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
    #[doc = "Timeout to wait for global checkpoint to advance"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Whether to wait for the global checkpoint to advance past the specified current checkpoints"]
    pub fn wait_for_advance(mut self, wait_for_advance: bool) -> Self {
        self.wait_for_advance = Some(wait_for_advance);
        self
    }
    #[doc = "Whether to wait for the target index to exist and all primary shards be active"]
    pub fn wait_for_index(mut self, wait_for_index: bool) -> Self {
        self.wait_for_index = Some(wait_for_index);
        self
    }
    #[doc = "Creates an asynchronous call to the Fleet Global Checkpoints API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                checkpoints: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_advance: Option<bool>,
                wait_for_index: Option<bool>,
            }
            let query_params = QueryParams {
                checkpoints: self.checkpoints,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                wait_for_advance: self.wait_for_advance,
                wait_for_index: self.wait_for_index,
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
#[doc = "API parts for the Fleet Msearch API"]
pub enum FleetMsearchParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> FleetMsearchParts<'b> {
    #[doc = "Builds a relative URL path to the Fleet Msearch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FleetMsearchParts::None => "/_fleet/_fleet_msearch".into(),
            FleetMsearchParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_fleet/_fleet_msearch");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Fleet Msearch API\n\nMulti Search API where the search will only be executed after specified checkpoints are available due to a refresh. This API is designed for internal use by the fleet server project."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct FleetMsearch<'a, 'b, B> {
    transport: &'a Transport,
    parts: FleetMsearchParts<'b>,
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
impl<'a, 'b, B> FleetMsearch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [FleetMsearch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: FleetMsearchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        FleetMsearch {
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
    pub fn body<T>(self, body: Vec<T>) -> FleetMsearch<'a, 'b, NdBody<T>>
    where
        T: Body,
    {
        FleetMsearch {
            transport: self.transport,
            parts: self.parts,
            body: Some(NdBody::new(body)),
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
    #[doc = "Creates an asynchronous call to the Fleet Msearch API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => http::Method::Post,
            None => http::Method::Get,
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
#[doc = "API parts for the Fleet Post Secret API"]
pub enum FleetPostSecretParts {
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl FleetPostSecretParts {
    #[doc = "Builds a relative URL path to the Fleet Post Secret API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FleetPostSecretParts::None => "/_fleet/secret".into(),
        }
    }
}
#[doc = "Builder for the Fleet Post Secret API\n\nCreates a secret stored by Fleet."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct FleetPostSecret<'a, 'b, B> {
    transport: &'a Transport,
    parts: FleetPostSecretParts,
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
impl<'a, 'b, B> FleetPostSecret<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [FleetPostSecret]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        FleetPostSecret {
            transport,
            parts: FleetPostSecretParts::None,
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
    pub fn body<T>(self, body: T) -> FleetPostSecret<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        FleetPostSecret {
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
    #[doc = "Creates an asynchronous call to the Fleet Post Secret API that can be awaited"]
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
#[doc = "API parts for the Fleet Search API"]
pub enum FleetSearchParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> FleetSearchParts<'b> {
    #[doc = "Builds a relative URL path to the Fleet Search API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FleetSearchParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_fleet/_fleet_search");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Fleet Search API\n\nSearch API where the search will only be executed after specified checkpoints are available due to a refresh. This API is designed for internal use by the fleet server project."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct FleetSearch<'a, 'b, B> {
    transport: &'a Transport,
    parts: FleetSearchParts<'b>,
    allow_partial_search_results: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_checkpoints: Option<&'b [&'b str]>,
    wait_for_checkpoints_timeout: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b, B> FleetSearch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [FleetSearch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: FleetSearchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        FleetSearch {
            transport,
            parts,
            headers,
            allow_partial_search_results: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_for_checkpoints: None,
            wait_for_checkpoints_timeout: None,
        }
    }
    #[doc = "Indicate if an error should be returned if there is a partial search failure or timeout"]
    pub fn allow_partial_search_results(mut self, allow_partial_search_results: bool) -> Self {
        self.allow_partial_search_results = Some(allow_partial_search_results);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> FleetSearch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        FleetSearch {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_partial_search_results: self.allow_partial_search_results,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            wait_for_checkpoints: self.wait_for_checkpoints,
            wait_for_checkpoints_timeout: self.wait_for_checkpoints_timeout,
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
    #[doc = "Comma separated list of checkpoints, one per shard"]
    pub fn wait_for_checkpoints(mut self, wait_for_checkpoints: &'b [&'b str]) -> Self {
        self.wait_for_checkpoints = Some(wait_for_checkpoints);
        self
    }
    #[doc = "Explicit wait_for_checkpoints timeout"]
    pub fn wait_for_checkpoints_timeout(mut self, wait_for_checkpoints_timeout: &'b str) -> Self {
        self.wait_for_checkpoints_timeout = Some(wait_for_checkpoints_timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Fleet Search API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => http::Method::Post,
            None => http::Method::Get,
        };
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_partial_search_results: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                wait_for_checkpoints: Option<&'b [&'b str]>,
                wait_for_checkpoints_timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_partial_search_results: self.allow_partial_search_results,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                wait_for_checkpoints: self.wait_for_checkpoints,
                wait_for_checkpoints_timeout: self.wait_for_checkpoints_timeout,
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
#[doc = "Namespace client for Fleet APIs"]
pub struct Fleet<'a> {
    transport: &'a Transport,
}
impl<'a> Fleet<'a> {
    #[doc = "Creates a new instance of [Fleet]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "Fleet Delete Secret API\n\nDeletes a secret stored by Fleet."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn delete_secret<'b>(
        &'a self,
        parts: FleetDeleteSecretParts<'b>,
    ) -> FleetDeleteSecret<'a, 'b> {
        FleetDeleteSecret::new(self.transport(), parts)
    }
    #[doc = "Fleet Get Secret API\n\nRetrieves a secret stored by Fleet."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn get_secret<'b>(&'a self, parts: FleetGetSecretParts<'b>) -> FleetGetSecret<'a, 'b> {
        FleetGetSecret::new(self.transport(), parts)
    }
    #[doc = "[Fleet Global Checkpoints API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-global-checkpoints.html)\n\nReturns the current global checkpoints for an index. This API is design for internal use by the fleet server project."]
    pub fn global_checkpoints<'b>(
        &'a self,
        parts: FleetGlobalCheckpointsParts<'b>,
    ) -> FleetGlobalCheckpoints<'a, 'b> {
        FleetGlobalCheckpoints::new(self.transport(), parts)
    }
    #[doc = "Fleet Msearch API\n\nMulti Search API where the search will only be executed after specified checkpoints are available due to a refresh. This API is designed for internal use by the fleet server project."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn msearch<'b>(&'a self, parts: FleetMsearchParts<'b>) -> FleetMsearch<'a, 'b, ()> {
        FleetMsearch::new(self.transport(), parts)
    }
    #[doc = "Fleet Post Secret API\n\nCreates a secret stored by Fleet."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn post_secret<'b>(&'a self) -> FleetPostSecret<'a, 'b, ()> {
        FleetPostSecret::new(self.transport())
    }
    #[doc = "Fleet Search API\n\nSearch API where the search will only be executed after specified checkpoints are available due to a refresh. This API is designed for internal use by the fleet server project."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn search<'b>(&'a self, parts: FleetSearchParts<'b>) -> FleetSearch<'a, 'b, ()> {
        FleetSearch::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Fleet APIs"]
    pub fn fleet(&self) -> Fleet {
        Fleet::new(self.transport())
    }
}
