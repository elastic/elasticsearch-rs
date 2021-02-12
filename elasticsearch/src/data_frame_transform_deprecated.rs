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

#![cfg(feature = "beta-apis")]
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
#[cfg(feature = "beta-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Data Frame Transform Deprecated Delete Transform API"]
pub enum DataFrameTransformDeprecatedDeleteTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
#[cfg(feature = "beta-apis")]
impl<'b> DataFrameTransformDeprecatedDeleteTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Data Frame Transform Deprecated Delete Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DataFrameTransformDeprecatedDeleteTransformParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(encoded_transform_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Data Frame Transform Deprecated Delete Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/delete-transform.html)\n\nDeletes an existing transform."]
#[cfg(feature = "beta-apis")]
pub struct DataFrameTransformDeprecatedDeleteTransform<'a, 'b> {
    transport: &'a Transport,
    parts: DataFrameTransformDeprecatedDeleteTransformParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    force: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "beta-apis")]
impl<'a, 'b> DataFrameTransformDeprecatedDeleteTransform<'a, 'b> {
    #[doc = "Creates a new instance of [DataFrameTransformDeprecatedDeleteTransform] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: DataFrameTransformDeprecatedDeleteTransformParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        DataFrameTransformDeprecatedDeleteTransform {
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
    #[doc = "Creates an asynchronous call to the Data Frame Transform Deprecated Delete Transform API that can be awaited"]
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
#[cfg(feature = "beta-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Data Frame Transform Deprecated Get Transform API"]
pub enum DataFrameTransformDeprecatedGetTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "beta-apis")]
impl<'b> DataFrameTransformDeprecatedGetTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Data Frame Transform Deprecated Get Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DataFrameTransformDeprecatedGetTransformParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(encoded_transform_id.as_ref());
                p.into()
            }
            DataFrameTransformDeprecatedGetTransformParts::None => "/_data_frame/transforms".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Data Frame Transform Deprecated Get Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/get-transform.html)\n\nRetrieves configuration information for transforms."]
#[cfg(feature = "beta-apis")]
pub struct DataFrameTransformDeprecatedGetTransform<'a, 'b> {
    transport: &'a Transport,
    parts: DataFrameTransformDeprecatedGetTransformParts<'b>,
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
#[cfg(feature = "beta-apis")]
impl<'a, 'b> DataFrameTransformDeprecatedGetTransform<'a, 'b> {
    #[doc = "Creates a new instance of [DataFrameTransformDeprecatedGetTransform] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: DataFrameTransformDeprecatedGetTransformParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        DataFrameTransformDeprecatedGetTransform {
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
    #[doc = "Omits generated fields. Allows transform configurations to be easily copied between clusters and within the same cluster"]
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
    #[doc = "Creates an asynchronous call to the Data Frame Transform Deprecated Get Transform API that can be awaited"]
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
#[cfg(feature = "beta-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Data Frame Transform Deprecated Get Transform Stats API"]
pub enum DataFrameTransformDeprecatedGetTransformStatsParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
#[cfg(feature = "beta-apis")]
impl<'b> DataFrameTransformDeprecatedGetTransformStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Data Frame Transform Deprecated Get Transform Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DataFrameTransformDeprecatedGetTransformStatsParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(31usize + encoded_transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(encoded_transform_id.as_ref());
                p.push_str("/_stats");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Data Frame Transform Deprecated Get Transform Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/get-transform-stats.html)\n\nRetrieves usage information for transforms."]
#[cfg(feature = "beta-apis")]
pub struct DataFrameTransformDeprecatedGetTransformStats<'a, 'b> {
    transport: &'a Transport,
    parts: DataFrameTransformDeprecatedGetTransformStatsParts<'b>,
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
#[cfg(feature = "beta-apis")]
impl<'a, 'b> DataFrameTransformDeprecatedGetTransformStats<'a, 'b> {
    #[doc = "Creates a new instance of [DataFrameTransformDeprecatedGetTransformStats] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: DataFrameTransformDeprecatedGetTransformStatsParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        DataFrameTransformDeprecatedGetTransformStats {
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
    #[doc = "Creates an asynchronous call to the Data Frame Transform Deprecated Get Transform Stats API that can be awaited"]
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
#[cfg(feature = "beta-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Data Frame Transform Deprecated Preview Transform API"]
pub enum DataFrameTransformDeprecatedPreviewTransformParts {
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "beta-apis")]
impl DataFrameTransformDeprecatedPreviewTransformParts {
    #[doc = "Builds a relative URL path to the Data Frame Transform Deprecated Preview Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DataFrameTransformDeprecatedPreviewTransformParts::None => {
                "/_data_frame/transforms/_preview".into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Data Frame Transform Deprecated Preview Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/preview-transform.html)\n\nPreviews a transform."]
#[cfg(feature = "beta-apis")]
pub struct DataFrameTransformDeprecatedPreviewTransform<'a, 'b, B> {
    transport: &'a Transport,
    parts: DataFrameTransformDeprecatedPreviewTransformParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "beta-apis")]
impl<'a, 'b, B> DataFrameTransformDeprecatedPreviewTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [DataFrameTransformDeprecatedPreviewTransform]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        DataFrameTransformDeprecatedPreviewTransform {
            transport,
            parts: DataFrameTransformDeprecatedPreviewTransformParts::None,
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
    pub fn body<T>(
        self,
        body: T,
    ) -> DataFrameTransformDeprecatedPreviewTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        DataFrameTransformDeprecatedPreviewTransform {
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
    #[doc = "Creates an asynchronous call to the Data Frame Transform Deprecated Preview Transform API that can be awaited"]
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
#[cfg(feature = "beta-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Data Frame Transform Deprecated Put Transform API"]
pub enum DataFrameTransformDeprecatedPutTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
#[cfg(feature = "beta-apis")]
impl<'b> DataFrameTransformDeprecatedPutTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Data Frame Transform Deprecated Put Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DataFrameTransformDeprecatedPutTransformParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(encoded_transform_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Data Frame Transform Deprecated Put Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/put-transform.html)\n\nInstantiates a transform."]
#[cfg(feature = "beta-apis")]
pub struct DataFrameTransformDeprecatedPutTransform<'a, 'b, B> {
    transport: &'a Transport,
    parts: DataFrameTransformDeprecatedPutTransformParts<'b>,
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
#[cfg(feature = "beta-apis")]
impl<'a, 'b, B> DataFrameTransformDeprecatedPutTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [DataFrameTransformDeprecatedPutTransform] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: DataFrameTransformDeprecatedPutTransformParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        DataFrameTransformDeprecatedPutTransform {
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
    pub fn body<T>(self, body: T) -> DataFrameTransformDeprecatedPutTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        DataFrameTransformDeprecatedPutTransform {
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
    #[doc = "Creates an asynchronous call to the Data Frame Transform Deprecated Put Transform API that can be awaited"]
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
#[cfg(feature = "beta-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Data Frame Transform Deprecated Start Transform API"]
pub enum DataFrameTransformDeprecatedStartTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
#[cfg(feature = "beta-apis")]
impl<'b> DataFrameTransformDeprecatedStartTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Data Frame Transform Deprecated Start Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DataFrameTransformDeprecatedStartTransformParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(31usize + encoded_transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(encoded_transform_id.as_ref());
                p.push_str("/_start");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Data Frame Transform Deprecated Start Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/start-transform.html)\n\nStarts one or more transforms."]
#[cfg(feature = "beta-apis")]
pub struct DataFrameTransformDeprecatedStartTransform<'a, 'b, B> {
    transport: &'a Transport,
    parts: DataFrameTransformDeprecatedStartTransformParts<'b>,
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
#[cfg(feature = "beta-apis")]
impl<'a, 'b, B> DataFrameTransformDeprecatedStartTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [DataFrameTransformDeprecatedStartTransform] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: DataFrameTransformDeprecatedStartTransformParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        DataFrameTransformDeprecatedStartTransform {
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
    pub fn body<T>(self, body: T) -> DataFrameTransformDeprecatedStartTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        DataFrameTransformDeprecatedStartTransform {
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
    #[doc = "Creates an asynchronous call to the Data Frame Transform Deprecated Start Transform API that can be awaited"]
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
#[cfg(feature = "beta-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Data Frame Transform Deprecated Stop Transform API"]
pub enum DataFrameTransformDeprecatedStopTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
#[cfg(feature = "beta-apis")]
impl<'b> DataFrameTransformDeprecatedStopTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Data Frame Transform Deprecated Stop Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DataFrameTransformDeprecatedStopTransformParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(encoded_transform_id.as_ref());
                p.push_str("/_stop");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Data Frame Transform Deprecated Stop Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/stop-transform.html)\n\nStops one or more transforms."]
#[cfg(feature = "beta-apis")]
pub struct DataFrameTransformDeprecatedStopTransform<'a, 'b, B> {
    transport: &'a Transport,
    parts: DataFrameTransformDeprecatedStopTransformParts<'b>,
    allow_no_match: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
#[cfg(feature = "beta-apis")]
impl<'a, 'b, B> DataFrameTransformDeprecatedStopTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [DataFrameTransformDeprecatedStopTransform] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: DataFrameTransformDeprecatedStopTransformParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        DataFrameTransformDeprecatedStopTransform {
            transport,
            parts,
            headers,
            allow_no_match: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_completion: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no transforms. (This includes `_all` string or when no transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: bool) -> Self {
        self.allow_no_match = Some(allow_no_match);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> DataFrameTransformDeprecatedStopTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        DataFrameTransformDeprecatedStopTransform {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_match: self.allow_no_match,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
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
    #[doc = "Controls the time to wait until the transform has stopped. Default to 30 seconds"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Whether to wait for the transform to fully stop before returning or not. Default to false"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Data Frame Transform Deprecated Stop Transform API that can be awaited"]
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
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_match: self.allow_no_match,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[cfg(feature = "beta-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Data Frame Transform Deprecated Update Transform API"]
pub enum DataFrameTransformDeprecatedUpdateTransformParts<'b> {
    #[doc = "TransformId"]
    TransformId(&'b str),
}
#[cfg(feature = "beta-apis")]
impl<'b> DataFrameTransformDeprecatedUpdateTransformParts<'b> {
    #[doc = "Builds a relative URL path to the Data Frame Transform Deprecated Update Transform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DataFrameTransformDeprecatedUpdateTransformParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(encoded_transform_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Data Frame Transform Deprecated Update Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/update-transform.html)\n\nUpdates certain properties of a transform."]
#[cfg(feature = "beta-apis")]
pub struct DataFrameTransformDeprecatedUpdateTransform<'a, 'b, B> {
    transport: &'a Transport,
    parts: DataFrameTransformDeprecatedUpdateTransformParts<'b>,
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
#[cfg(feature = "beta-apis")]
impl<'a, 'b, B> DataFrameTransformDeprecatedUpdateTransform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [DataFrameTransformDeprecatedUpdateTransform] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: DataFrameTransformDeprecatedUpdateTransformParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        DataFrameTransformDeprecatedUpdateTransform {
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
    pub fn body<T>(
        self,
        body: T,
    ) -> DataFrameTransformDeprecatedUpdateTransform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        DataFrameTransformDeprecatedUpdateTransform {
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
    #[doc = "Creates an asynchronous call to the Data Frame Transform Deprecated Update Transform API that can be awaited"]
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
#[doc = "Namespace client for DataFrameTransformDeprecated APIs"]
#[cfg(feature = "beta-apis")]
pub struct DataFrameTransformDeprecated<'a> {
    transport: &'a Transport,
}
#[cfg(feature = "beta-apis")]
impl<'a> DataFrameTransformDeprecated<'a> {
    #[doc = "Creates a new instance of [DataFrameTransformDeprecated]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Data Frame Transform Deprecated Delete Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/delete-transform.html)\n\nDeletes an existing transform."]
    #[cfg(feature = "beta-apis")]
    pub fn delete_transform<'b>(
        &'a self,
        parts: DataFrameTransformDeprecatedDeleteTransformParts<'b>,
    ) -> DataFrameTransformDeprecatedDeleteTransform<'a, 'b> {
        DataFrameTransformDeprecatedDeleteTransform::new(self.transport(), parts)
    }
    #[doc = "[Data Frame Transform Deprecated Get Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/get-transform.html)\n\nRetrieves configuration information for transforms."]
    #[cfg(feature = "beta-apis")]
    pub fn get_transform<'b>(
        &'a self,
        parts: DataFrameTransformDeprecatedGetTransformParts<'b>,
    ) -> DataFrameTransformDeprecatedGetTransform<'a, 'b> {
        DataFrameTransformDeprecatedGetTransform::new(self.transport(), parts)
    }
    #[doc = "[Data Frame Transform Deprecated Get Transform Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/get-transform-stats.html)\n\nRetrieves usage information for transforms."]
    #[cfg(feature = "beta-apis")]
    pub fn get_transform_stats<'b>(
        &'a self,
        parts: DataFrameTransformDeprecatedGetTransformStatsParts<'b>,
    ) -> DataFrameTransformDeprecatedGetTransformStats<'a, 'b> {
        DataFrameTransformDeprecatedGetTransformStats::new(self.transport(), parts)
    }
    #[doc = "[Data Frame Transform Deprecated Preview Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/preview-transform.html)\n\nPreviews a transform."]
    #[cfg(feature = "beta-apis")]
    pub fn preview_transform<'b>(
        &'a self,
    ) -> DataFrameTransformDeprecatedPreviewTransform<'a, 'b, ()> {
        DataFrameTransformDeprecatedPreviewTransform::new(self.transport())
    }
    #[doc = "[Data Frame Transform Deprecated Put Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/put-transform.html)\n\nInstantiates a transform."]
    #[cfg(feature = "beta-apis")]
    pub fn put_transform<'b>(
        &'a self,
        parts: DataFrameTransformDeprecatedPutTransformParts<'b>,
    ) -> DataFrameTransformDeprecatedPutTransform<'a, 'b, ()> {
        DataFrameTransformDeprecatedPutTransform::new(self.transport(), parts)
    }
    #[doc = "[Data Frame Transform Deprecated Start Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/start-transform.html)\n\nStarts one or more transforms."]
    #[cfg(feature = "beta-apis")]
    pub fn start_transform<'b>(
        &'a self,
        parts: DataFrameTransformDeprecatedStartTransformParts<'b>,
    ) -> DataFrameTransformDeprecatedStartTransform<'a, 'b, ()> {
        DataFrameTransformDeprecatedStartTransform::new(self.transport(), parts)
    }
    #[doc = "[Data Frame Transform Deprecated Stop Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/stop-transform.html)\n\nStops one or more transforms."]
    #[cfg(feature = "beta-apis")]
    pub fn stop_transform<'b>(
        &'a self,
        parts: DataFrameTransformDeprecatedStopTransformParts<'b>,
    ) -> DataFrameTransformDeprecatedStopTransform<'a, 'b, ()> {
        DataFrameTransformDeprecatedStopTransform::new(self.transport(), parts)
    }
    #[doc = "[Data Frame Transform Deprecated Update Transform API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/update-transform.html)\n\nUpdates certain properties of a transform."]
    #[cfg(feature = "beta-apis")]
    pub fn update_transform<'b>(
        &'a self,
        parts: DataFrameTransformDeprecatedUpdateTransformParts<'b>,
    ) -> DataFrameTransformDeprecatedUpdateTransform<'a, 'b, ()> {
        DataFrameTransformDeprecatedUpdateTransform::new(self.transport(), parts)
    }
}
#[cfg(feature = "beta-apis")]
impl Elasticsearch {
    #[doc = "Creates a namespace client for DataFrameTransformDeprecated APIs"]
    pub fn data_frame_transform_deprecated(&self) -> DataFrameTransformDeprecated {
        DataFrameTransformDeprecated::new(self.transport())
    }
}
