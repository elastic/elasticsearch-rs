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

//! Rollup APIs
//!
//! The Elastic Stack [data rollup features](https://www.elastic.co/guide/en/elasticsearch/reference/master/xpack-rollup.html)
//! provide a means to summarize and store historical data so that it can still be used for analysis, but at a fraction of
//! the storage cost of raw data.

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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Rollup Delete Job API"]
pub enum RollupDeleteJobParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> RollupDeleteJobParts<'b> {
    #[doc = "Builds a relative URL path to the Rollup Delete Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RollupDeleteJobParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_id.len());
                p.push_str("/_rollup/job/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Rollup Delete Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-delete-job.html)\n\nDeletes an existing rollup job."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct RollupDeleteJob<'a, 'b> {
    transport: &'a Transport,
    parts: RollupDeleteJobParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> RollupDeleteJob<'a, 'b> {
    #[doc = "Creates a new instance of [RollupDeleteJob] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: RollupDeleteJobParts<'b>) -> Self {
        let headers = HeaderMap::new();
        RollupDeleteJob {
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
    #[doc = "Creates an asynchronous call to the Rollup Delete Job API that can be awaited"]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Rollup Get Jobs API"]
pub enum RollupGetJobsParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl<'b> RollupGetJobsParts<'b> {
    #[doc = "Builds a relative URL path to the Rollup Get Jobs API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RollupGetJobsParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_id.len());
                p.push_str("/_rollup/job/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            RollupGetJobsParts::None => "/_rollup/job/".into(),
        }
    }
}
#[doc = "Builder for the [Rollup Get Jobs API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-get-job.html)\n\nRetrieves the configuration, stats, and status of rollup jobs."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct RollupGetJobs<'a, 'b> {
    transport: &'a Transport,
    parts: RollupGetJobsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> RollupGetJobs<'a, 'b> {
    #[doc = "Creates a new instance of [RollupGetJobs] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: RollupGetJobsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        RollupGetJobs {
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
    #[doc = "Creates an asynchronous call to the Rollup Get Jobs API that can be awaited"]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Rollup Get Rollup Caps API"]
pub enum RollupGetRollupCapsParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl<'b> RollupGetRollupCapsParts<'b> {
    #[doc = "Builds a relative URL path to the Rollup Get Rollup Caps API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RollupGetRollupCapsParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_id.len());
                p.push_str("/_rollup/data/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            RollupGetRollupCapsParts::None => "/_rollup/data/".into(),
        }
    }
}
#[doc = "Builder for the [Rollup Get Rollup Caps API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-get-rollup-caps.html)\n\nReturns the capabilities of any rollup jobs that have been configured for a specific index or index pattern."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct RollupGetRollupCaps<'a, 'b> {
    transport: &'a Transport,
    parts: RollupGetRollupCapsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> RollupGetRollupCaps<'a, 'b> {
    #[doc = "Creates a new instance of [RollupGetRollupCaps] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: RollupGetRollupCapsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        RollupGetRollupCaps {
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
    #[doc = "Creates an asynchronous call to the Rollup Get Rollup Caps API that can be awaited"]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Rollup Get Rollup Index Caps API"]
pub enum RollupGetRollupIndexCapsParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> RollupGetRollupIndexCapsParts<'b> {
    #[doc = "Builds a relative URL path to the Rollup Get Rollup Index Caps API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RollupGetRollupIndexCapsParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_rollup/data");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Rollup Get Rollup Index Caps API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-get-rollup-index-caps.html)\n\nReturns the rollup capabilities of all jobs inside of a rollup index (e.g. the index where rollup data is stored)."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct RollupGetRollupIndexCaps<'a, 'b> {
    transport: &'a Transport,
    parts: RollupGetRollupIndexCapsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> RollupGetRollupIndexCaps<'a, 'b> {
    #[doc = "Creates a new instance of [RollupGetRollupIndexCaps] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: RollupGetRollupIndexCapsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        RollupGetRollupIndexCaps {
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
    #[doc = "Creates an asynchronous call to the Rollup Get Rollup Index Caps API that can be awaited"]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Rollup Put Job API"]
pub enum RollupPutJobParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> RollupPutJobParts<'b> {
    #[doc = "Builds a relative URL path to the Rollup Put Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RollupPutJobParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_id.len());
                p.push_str("/_rollup/job/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Rollup Put Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-put-job.html)\n\nCreates a rollup job."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct RollupPutJob<'a, 'b, B> {
    transport: &'a Transport,
    parts: RollupPutJobParts<'b>,
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
impl<'a, 'b, B> RollupPutJob<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [RollupPutJob] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: RollupPutJobParts<'b>) -> Self {
        let headers = HeaderMap::new();
        RollupPutJob {
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
    pub fn body<T>(self, body: T) -> RollupPutJob<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        RollupPutJob {
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
    #[doc = "Creates an asynchronous call to the Rollup Put Job API that can be awaited"]
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
#[doc = "API parts for the Rollup Rollup API"]
pub enum RollupRollupParts<'b> {
    #[doc = "Index and RollupIndex"]
    IndexRollupIndex(&'b str, &'b str),
}
impl<'b> RollupRollupParts<'b> {
    #[doc = "Builds a relative URL path to the Rollup Rollup API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RollupRollupParts::IndexRollupIndex(ref index, ref rollup_index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_rollup_index: Cow<str> =
                    percent_encode(rollup_index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    10usize + encoded_index.len() + encoded_rollup_index.len(),
                );
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_rollup/");
                p.push_str(encoded_rollup_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Rollup Rollup API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-api.html)\n\nRollup an index"]
#[derive(Clone, Debug)]
pub struct RollupRollup<'a, 'b, B> {
    transport: &'a Transport,
    parts: RollupRollupParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> RollupRollup<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [RollupRollup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: RollupRollupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        RollupRollup {
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
    pub fn body<T>(self, body: T) -> RollupRollup<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        RollupRollup {
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
    #[doc = "Creates an asynchronous call to the Rollup Rollup API that can be awaited"]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Rollup Rollup Search API"]
pub enum RollupRollupSearchParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> RollupRollupSearchParts<'b> {
    #[doc = "Builds a relative URL path to the Rollup Rollup Search API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RollupRollupSearchParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_rollup_search");
                p.into()
            }
            RollupRollupSearchParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_ty: Cow<str> = percent_encode(ty.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_index.len() + encoded_ty.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/");
                p.push_str(encoded_ty.as_ref());
                p.push_str("/_rollup_search");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Rollup Rollup Search API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-search.html)\n\nEnables searching rolled-up data using the standard query DSL."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct RollupRollupSearch<'a, 'b, B> {
    transport: &'a Transport,
    parts: RollupRollupSearchParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    rest_total_hits_as_int: Option<bool>,
    source: Option<&'b str>,
    typed_keys: Option<bool>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b, B> RollupRollupSearch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [RollupRollupSearch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: RollupRollupSearchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        RollupRollupSearch {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            rest_total_hits_as_int: None,
            source: None,
            typed_keys: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> RollupRollupSearch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        RollupRollupSearch {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            source: self.source,
            typed_keys: self.typed_keys,
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
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "Creates an asynchronous call to the Rollup Rollup Search API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
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
                rest_total_hits_as_int: Option<bool>,
                source: Option<&'b str>,
                typed_keys: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                source: self.source,
                typed_keys: self.typed_keys,
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Rollup Start Job API"]
pub enum RollupStartJobParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> RollupStartJobParts<'b> {
    #[doc = "Builds a relative URL path to the Rollup Start Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RollupStartJobParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_id.len());
                p.push_str("/_rollup/job/");
                p.push_str(encoded_id.as_ref());
                p.push_str("/_start");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Rollup Start Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-start-job.html)\n\nStarts an existing, stopped rollup job."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct RollupStartJob<'a, 'b, B> {
    transport: &'a Transport,
    parts: RollupStartJobParts<'b>,
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
impl<'a, 'b, B> RollupStartJob<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [RollupStartJob] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: RollupStartJobParts<'b>) -> Self {
        let headers = HeaderMap::new();
        RollupStartJob {
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
    pub fn body<T>(self, body: T) -> RollupStartJob<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        RollupStartJob {
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
    #[doc = "Creates an asynchronous call to the Rollup Start Job API that can be awaited"]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Rollup Stop Job API"]
pub enum RollupStopJobParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> RollupStopJobParts<'b> {
    #[doc = "Builds a relative URL path to the Rollup Stop Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RollupStopJobParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_id.len());
                p.push_str("/_rollup/job/");
                p.push_str(encoded_id.as_ref());
                p.push_str("/_stop");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Rollup Stop Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-stop-job.html)\n\nStops an existing, started rollup job."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct RollupStopJob<'a, 'b, B> {
    transport: &'a Transport,
    parts: RollupStopJobParts<'b>,
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
#[cfg(feature = "experimental-apis")]
impl<'a, 'b, B> RollupStopJob<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [RollupStopJob] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: RollupStopJobParts<'b>) -> Self {
        let headers = HeaderMap::new();
        RollupStopJob {
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
            wait_for_completion: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> RollupStopJob<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        RollupStopJob {
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
    #[doc = "Block for (at maximum) the specified duration while waiting for the job to stop.  Defaults to 30s."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "True if the API should block until the job has fully stopped, false if should be executed async. Defaults to false."]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Rollup Stop Job API that can be awaited"]
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
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
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
#[doc = "Namespace client for Rollup APIs"]
pub struct Rollup<'a> {
    transport: &'a Transport,
}
impl<'a> Rollup<'a> {
    #[doc = "Creates a new instance of [Rollup]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Rollup Delete Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-delete-job.html)\n\nDeletes an existing rollup job."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn delete_job<'b>(&'a self, parts: RollupDeleteJobParts<'b>) -> RollupDeleteJob<'a, 'b> {
        RollupDeleteJob::new(self.transport(), parts)
    }
    #[doc = "[Rollup Get Jobs API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-get-job.html)\n\nRetrieves the configuration, stats, and status of rollup jobs."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn get_jobs<'b>(&'a self, parts: RollupGetJobsParts<'b>) -> RollupGetJobs<'a, 'b> {
        RollupGetJobs::new(self.transport(), parts)
    }
    #[doc = "[Rollup Get Rollup Caps API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-get-rollup-caps.html)\n\nReturns the capabilities of any rollup jobs that have been configured for a specific index or index pattern."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn get_rollup_caps<'b>(
        &'a self,
        parts: RollupGetRollupCapsParts<'b>,
    ) -> RollupGetRollupCaps<'a, 'b> {
        RollupGetRollupCaps::new(self.transport(), parts)
    }
    #[doc = "[Rollup Get Rollup Index Caps API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-get-rollup-index-caps.html)\n\nReturns the rollup capabilities of all jobs inside of a rollup index (e.g. the index where rollup data is stored)."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn get_rollup_index_caps<'b>(
        &'a self,
        parts: RollupGetRollupIndexCapsParts<'b>,
    ) -> RollupGetRollupIndexCaps<'a, 'b> {
        RollupGetRollupIndexCaps::new(self.transport(), parts)
    }
    #[doc = "[Rollup Put Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-put-job.html)\n\nCreates a rollup job."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn put_job<'b>(&'a self, parts: RollupPutJobParts<'b>) -> RollupPutJob<'a, 'b, ()> {
        RollupPutJob::new(self.transport(), parts)
    }
    #[doc = "[Rollup Rollup API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-api.html)\n\nRollup an index"]
    pub fn rollup<'b>(&'a self, parts: RollupRollupParts<'b>) -> RollupRollup<'a, 'b, ()> {
        RollupRollup::new(self.transport(), parts)
    }
    #[doc = "[Rollup Rollup Search API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-search.html)\n\nEnables searching rolled-up data using the standard query DSL."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn rollup_search<'b>(
        &'a self,
        parts: RollupRollupSearchParts<'b>,
    ) -> RollupRollupSearch<'a, 'b, ()> {
        RollupRollupSearch::new(self.transport(), parts)
    }
    #[doc = "[Rollup Start Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-start-job.html)\n\nStarts an existing, stopped rollup job."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn start_job<'b>(&'a self, parts: RollupStartJobParts<'b>) -> RollupStartJob<'a, 'b, ()> {
        RollupStartJob::new(self.transport(), parts)
    }
    #[doc = "[Rollup Stop Job API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/rollup-stop-job.html)\n\nStops an existing, started rollup job."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn stop_job<'b>(&'a self, parts: RollupStopJobParts<'b>) -> RollupStopJob<'a, 'b, ()> {
        RollupStopJob::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Rollup APIs"]
    pub fn rollup(&self) -> Rollup {
        Rollup::new(self.transport())
    }
}
