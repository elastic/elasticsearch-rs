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

//! Transform APIs
//!
//! [Transforms ](https://www.elastic.co/guide/en/elasticsearch/reference/master/transform-apis.html)
//! can be used to copy data from source indices, transforms it, and persists it into an
//! entity-centric destination index.

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
#[doc = "API parts for the Transform Delete Transform API"]
pub enum TransformDeleteTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
impl<'b> TransformDeleteTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Delete Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformDeleteTransformParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_transform_id.len());
                p.push_str("/_transform/");
                p.push_str(encoded_transform_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Transform Delete Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/delete-transform.html)\n\nDeletes an existing transform."]
#[derive(Clone, Debug)]
pub struct TransformDeleteTransform<'a, 'b> {
    transport: &'a Transport,
    parts: TransformDeleteTransformParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    force: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> TransformDeleteTransform<'a, 'b> {
    #[doc = "Creates a new instance of [TransformDeleteTransform] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TransformDeleteTransformParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformDeleteTransform {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            force: None,
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
    #[doc = "When `true`, the transform is deleted regardless of its current state. The default value is `false`, meaning that the transform must be `stopped` before it can be deleted."]
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
    #[doc = "Creates an asynchronous call to the Transform Delete Transform API that can be awaited"]
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
                force: Option<bool>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Transform Get Transform API"]
pub enum TransformGetTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
    #[doc = "No parts"]
    None,
}
impl<'b> TransformGetTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Get Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformGetTransformParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_transform_id.len());
                p.push_str("/_transform/");
                p.push_str(encoded_transform_id.as_ref());
                p.into()
            }
            TransformGetTransformParts::None => "/_transform".into(),
        }
    }
}
#[doc = "Builder for the [Transform Get Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/get-transform.html)\n\nRetrieves configuration information for transforms."]
#[derive(Clone, Debug)]
pub struct TransformGetTransform<'a, 'b> {
    transport: &'a Transport,
    parts: TransformGetTransformParts<'b>,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    exclude_generated: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    size: Option<i32>,
    source: Option<&'b str>,
}
impl<'a, 'b> TransformGetTransform<'a, 'b> {
    #[doc = "Creates a new instance of [TransformGetTransform] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TransformGetTransformParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformGetTransform {
            transport,
            parts,
            headers,
            allow_no_match: None,
            error_trace: None,
            exclude_generated: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            request_timeout: None,
            size: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no transforms. (This includes `_all` string or when no transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: bool) -> Self {
        self.allow_no_match = Some(allow_no_match);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Omits fields that are illegal to set on transform PUT"]
    pub fn exclude_generated(mut self, exclude_generated: bool) -> Self {
        self.exclude_generated = Some(exclude_generated);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of transform configs, defaults to 0"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
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
    #[doc = "specifies a max number of transforms to get, defaults to 100"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Transform Get Transform API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_match: Option<bool>,
                error_trace: Option<bool>,
                exclude_generated: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                from: Option<i32>,
                human: Option<bool>,
                pretty: Option<bool>,
                size: Option<i32>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_match: self.allow_no_match,
                error_trace: self.error_trace,
                exclude_generated: self.exclude_generated,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                size: self.size,
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
#[doc = "API parts for the Transform Get Transform Stats API"]
pub enum TransformGetTransformStatsParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
impl<'b> TransformGetTransformStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Get Transform Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformGetTransformStatsParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_transform_id.len());
                p.push_str("/_transform/");
                p.push_str(encoded_transform_id.as_ref());
                p.push_str("/_stats");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Transform Get Transform Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/get-transform-stats.html)\n\nRetrieves usage information for transforms."]
#[derive(Clone, Debug)]
pub struct TransformGetTransformStats<'a, 'b> {
    transport: &'a Transport,
    parts: TransformGetTransformStatsParts<'b>,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    size: Option<i64>,
    source: Option<&'b str>,
}
impl<'a, 'b> TransformGetTransformStats<'a, 'b> {
    #[doc = "Creates a new instance of [TransformGetTransformStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TransformGetTransformStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformGetTransformStats {
            transport,
            parts,
            headers,
            allow_no_match: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            request_timeout: None,
            size: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no transforms. (This includes `_all` string or when no transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: bool) -> Self {
        self.allow_no_match = Some(allow_no_match);
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
    #[doc = "skips a number of transform stats, defaults to 0"]
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from);
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
    #[doc = "specifies a max number of transform stats to get, defaults to 100"]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Transform Get Transform Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_match: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                from: Option<i64>,
                human: Option<bool>,
                pretty: Option<bool>,
                size: Option<i64>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_match: self.allow_no_match,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                size: self.size,
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
#[doc = "API parts for the Transform Preview Transform API"]
pub enum TransformPreviewTransformParts {
    #[doc = "No parts"]
    None,
}
impl TransformPreviewTransformParts {
    #[doc = "Builds a relative URL path to the Transform Preview Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformPreviewTransformParts::None => "/_transform/_preview".into(),
        }
    }
}
#[doc = "Builder for the [Transform Preview Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/preview-transform.html)\n\nPreviews a transform."]
#[derive(Clone, Debug)]
pub struct TransformPreviewTransform<'a, 'b, B> {
    transport: &'a Transport,
    parts: TransformPreviewTransformParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> TransformPreviewTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformPreviewTransform]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        TransformPreviewTransform {
            transport,
            parts: TransformPreviewTransformParts::None,
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
    pub fn body<T>(self, body: T) -> TransformPreviewTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformPreviewTransform {
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
    #[doc = "Creates an asynchronous call to the Transform Preview Transform API that can be awaited"]
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Transform Put Transform API"]
pub enum TransformPutTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
impl<'b> TransformPutTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Put Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformPutTransformParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_transform_id.len());
                p.push_str("/_transform/");
                p.push_str(encoded_transform_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Transform Put Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/put-transform.html)\n\nInstantiates a transform."]
#[derive(Clone, Debug)]
pub struct TransformPutTransform<'a, 'b, B> {
    transport: &'a Transport,
    parts: TransformPutTransformParts<'b>,
    body: Option<B>,
    defer_validation: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> TransformPutTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformPutTransform] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TransformPutTransformParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformPutTransform {
            transport,
            parts,
            headers,
            body: None,
            defer_validation: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TransformPutTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformPutTransform {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            defer_validation: self.defer_validation,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "If validations should be deferred until transform starts, defaults to false."]
    pub fn defer_validation(mut self, defer_validation: bool) -> Self {
        self.defer_validation = Some(defer_validation);
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
    #[doc = "Creates an asynchronous call to the Transform Put Transform API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                defer_validation: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                defer_validation: self.defer_validation,
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
#[doc = "API parts for the Transform Start Transform API"]
pub enum TransformStartTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
impl<'b> TransformStartTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Start Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformStartTransformParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_transform_id.len());
                p.push_str("/_transform/");
                p.push_str(encoded_transform_id.as_ref());
                p.push_str("/_start");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Transform Start Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/start-transform.html)\n\nStarts one or more transforms."]
#[derive(Clone, Debug)]
pub struct TransformStartTransform<'a, 'b, B> {
    transport: &'a Transport,
    parts: TransformStartTransformParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> TransformStartTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformStartTransform] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TransformStartTransformParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformStartTransform {
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
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TransformStartTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformStartTransform {
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
    #[doc = "Controls the time to wait for the transform to start"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Transform Start Transform API that can be awaited"]
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
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
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
#[doc = "API parts for the Transform Stop Transform API"]
pub enum TransformStopTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
impl<'b> TransformStopTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Stop Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformStopTransformParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_transform_id.len());
                p.push_str("/_transform/");
                p.push_str(encoded_transform_id.as_ref());
                p.push_str("/_stop");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Transform Stop Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/stop-transform.html)\n\nStops one or more transforms."]
#[derive(Clone, Debug)]
pub struct TransformStopTransform<'a, 'b, B> {
    transport: &'a Transport,
    parts: TransformStopTransformParts<'b>,
    allow_no_match: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    force: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_checkpoint: Option<bool>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> TransformStopTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformStopTransform] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TransformStopTransformParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformStopTransform {
            transport,
            parts,
            headers,
            allow_no_match: None,
            body: None,
            error_trace: None,
            filter_path: None,
            force: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_checkpoint: None,
            wait_for_completion: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no transforms. (This includes `_all` string or when no transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: bool) -> Self {
        self.allow_no_match = Some(allow_no_match);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TransformStopTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformStopTransform {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_match: self.allow_no_match,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            force: self.force,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
            wait_for_checkpoint: self.wait_for_checkpoint,
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
    #[doc = "Whether to force stop a failed transform or not. Default to false"]
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
    #[doc = "Controls the time to wait until the transform has stopped. Default to 30 seconds"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Whether to wait for the transform to reach a checkpoint before stopping. Default to false"]
    pub fn wait_for_checkpoint(mut self, wait_for_checkpoint: bool) -> Self {
        self.wait_for_checkpoint = Some(wait_for_checkpoint);
        self
    }
    #[doc = "Whether to wait for the transform to fully stop before returning or not. Default to false"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Transform Stop Transform API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_match: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                force: Option<bool>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_checkpoint: Option<bool>,
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_match: self.allow_no_match,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                force: self.force,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                wait_for_checkpoint: self.wait_for_checkpoint,
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
#[doc = "API parts for the Transform Update Transform API"]
pub enum TransformUpdateTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
impl<'b> TransformUpdateTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Transform Update Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformUpdateTransformParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_transform_id.len());
                p.push_str("/_transform/");
                p.push_str(encoded_transform_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Transform Update Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/update-transform.html)\n\nUpdates certain properties of a transform."]
#[derive(Clone, Debug)]
pub struct TransformUpdateTransform<'a, 'b, B> {
    transport: &'a Transport,
    parts: TransformUpdateTransformParts<'b>,
    body: Option<B>,
    defer_validation: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> TransformUpdateTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformUpdateTransform] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TransformUpdateTransformParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformUpdateTransform {
            transport,
            parts,
            headers,
            body: None,
            defer_validation: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TransformUpdateTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformUpdateTransform {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            defer_validation: self.defer_validation,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "If validations should be deferred until transform starts, defaults to false."]
    pub fn defer_validation(mut self, defer_validation: bool) -> Self {
        self.defer_validation = Some(defer_validation);
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
    #[doc = "Creates an asynchronous call to the Transform Update Transform API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                defer_validation: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                defer_validation: self.defer_validation,
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
#[doc = "Namespace client for Transform APIs"]
pub struct Transform<'a> {
    transport: &'a Transport,
}
impl<'a> Transform<'a> {
    #[doc = "Creates a new instance of [Transform]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Transform Delete Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/delete-transform.html)\n\nDeletes an existing transform."]
    pub fn delete_transform<'b>(
        &'a self,
        parts: TransformDeleteTransformParts<'b>,
    ) -> TransformDeleteTransform<'a, 'b> {
        TransformDeleteTransform::new(self.transport(), parts)
    }
    #[doc = "[Transform Get Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/get-transform.html)\n\nRetrieves configuration information for transforms."]
    pub fn get_transform<'b>(
        &'a self,
        parts: TransformGetTransformParts<'b>,
    ) -> TransformGetTransform<'a, 'b> {
        TransformGetTransform::new(self.transport(), parts)
    }
    #[doc = "[Transform Get Transform Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/get-transform-stats.html)\n\nRetrieves usage information for transforms."]
    pub fn get_transform_stats<'b>(
        &'a self,
        parts: TransformGetTransformStatsParts<'b>,
    ) -> TransformGetTransformStats<'a, 'b> {
        TransformGetTransformStats::new(self.transport(), parts)
    }
    #[doc = "[Transform Preview Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/preview-transform.html)\n\nPreviews a transform."]
    pub fn preview_transform<'b>(&'a self) -> TransformPreviewTransform<'a, 'b, ()> {
        TransformPreviewTransform::new(self.transport())
    }
    #[doc = "[Transform Put Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/put-transform.html)\n\nInstantiates a transform."]
    pub fn put_transform<'b>(
        &'a self,
        parts: TransformPutTransformParts<'b>,
    ) -> TransformPutTransform<'a, 'b, ()> {
        TransformPutTransform::new(self.transport(), parts)
    }
    #[doc = "[Transform Start Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/start-transform.html)\n\nStarts one or more transforms."]
    pub fn start_transform<'b>(
        &'a self,
        parts: TransformStartTransformParts<'b>,
    ) -> TransformStartTransform<'a, 'b, ()> {
        TransformStartTransform::new(self.transport(), parts)
    }
    #[doc = "[Transform Stop Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/stop-transform.html)\n\nStops one or more transforms."]
    pub fn stop_transform<'b>(
        &'a self,
        parts: TransformStopTransformParts<'b>,
    ) -> TransformStopTransform<'a, 'b, ()> {
        TransformStopTransform::new(self.transport(), parts)
    }
    #[doc = "[Transform Update Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/update-transform.html)\n\nUpdates certain properties of a transform."]
    pub fn update_transform<'b>(
        &'a self,
        parts: TransformUpdateTransformParts<'b>,
    ) -> TransformUpdateTransform<'a, 'b, ()> {
        TransformUpdateTransform::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Transform APIs"]
    pub fn transform(&self) -> Transform {
        Transform::new(self.transport())
    }
}
