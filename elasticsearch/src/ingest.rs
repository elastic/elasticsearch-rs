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

//! Ingest APIs
//!
//! [Manage ingest pipelines](https://www.elastic.co/guide/en/elasticsearch/reference/master/ingest-apis.html).
//! Ingest pipelines can be used on a node with the `ingest` role to
//! pre-process documents before indexing, to apply transformations and enrich data. Transformations are performed
//! by [processors](https://www.elastic.co/guide/en/elasticsearch/reference/master/ingest-processors.html)
//! in the pipeline, and can include such operations as
//!
//! - add, remove and append fields within the document
//! - point documents to the right time-based index based on a timestamp within the document
//! - extract details from fields with known formats and add new fields with extracted data
//!
//! and many more.
//!
//! All nodes enable ingest by default, so any node can handle ingest tasks. Ingest pipelines can
//! be conditionally executed, and failures within pipelines can be explicitly handled by defining
//! processors to execute in the event of failure.

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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ingest Delete Geoip Database API"]
pub enum IngestDeleteGeoipDatabaseParts<'b> {
    #[doc = "Id"]
    Id(&'b [&'b str]),
}
impl<'b> IngestDeleteGeoipDatabaseParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Delete Geoip Database API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestDeleteGeoipDatabaseParts::Id(id) => {
                let id_str = id.join(",");
                let encoded_id: Cow<str> = percent_encode(id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_id.len());
                p.push_str("/_ingest/geoip/database/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Delete Geoip Database API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/delete-geoip-database-api.html)\n\nDeletes a geoip database configuration"]
#[derive(Clone, Debug)]
pub struct IngestDeleteGeoipDatabase<'a, 'b> {
    transport: &'a Transport,
    parts: IngestDeleteGeoipDatabaseParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> IngestDeleteGeoipDatabase<'a, 'b> {
    #[doc = "Creates a new instance of [IngestDeleteGeoipDatabase] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestDeleteGeoipDatabaseParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestDeleteGeoipDatabase {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Delete Geoip Database API that can be awaited"]
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
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Ingest Delete Ip Location Database API"]
pub enum IngestDeleteIpLocationDatabaseParts<'b> {
    #[doc = "Id"]
    Id(&'b [&'b str]),
}
impl<'b> IngestDeleteIpLocationDatabaseParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Delete Ip Location Database API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestDeleteIpLocationDatabaseParts::Id(id) => {
                let id_str = id.join(",");
                let encoded_id: Cow<str> = percent_encode(id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_id.len());
                p.push_str("/_ingest/ip_location/database/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Delete Ip Location Database API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/delete-ip-location-database-api.html)\n\nDeletes an ip location database configuration"]
#[derive(Clone, Debug)]
pub struct IngestDeleteIpLocationDatabase<'a, 'b> {
    transport: &'a Transport,
    parts: IngestDeleteIpLocationDatabaseParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> IngestDeleteIpLocationDatabase<'a, 'b> {
    #[doc = "Creates a new instance of [IngestDeleteIpLocationDatabase] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestDeleteIpLocationDatabaseParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestDeleteIpLocationDatabase {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Delete Ip Location Database API that can be awaited"]
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
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Ingest Delete Pipeline API"]
pub enum IngestDeletePipelineParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> IngestDeletePipelineParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Delete Pipeline API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestDeletePipelineParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_id.len());
                p.push_str("/_ingest/pipeline/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Delete Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/delete-pipeline-api.html)\n\nDeletes a pipeline."]
#[derive(Clone, Debug)]
pub struct IngestDeletePipeline<'a, 'b> {
    transport: &'a Transport,
    parts: IngestDeletePipelineParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> IngestDeletePipeline<'a, 'b> {
    #[doc = "Creates a new instance of [IngestDeletePipeline] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestDeletePipelineParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestDeletePipeline {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Delete Pipeline API that can be awaited"]
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
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Ingest Geo Ip Stats API"]
pub enum IngestGeoIpStatsParts {
    #[doc = "No parts"]
    None,
}
impl IngestGeoIpStatsParts {
    #[doc = "Builds a relative URL path to the Ingest Geo Ip Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestGeoIpStatsParts::None => "/_ingest/geoip/stats".into(),
        }
    }
}
#[doc = "Builder for the [Ingest Geo Ip Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/geoip-stats-api.html)\n\nReturns statistical information about geoip databases"]
#[derive(Clone, Debug)]
pub struct IngestGeoIpStats<'a, 'b> {
    transport: &'a Transport,
    parts: IngestGeoIpStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IngestGeoIpStats<'a, 'b> {
    #[doc = "Creates a new instance of [IngestGeoIpStats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        IngestGeoIpStats {
            transport,
            parts: IngestGeoIpStatsParts::None,
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
    #[doc = "Creates an asynchronous call to the Ingest Geo Ip Stats API that can be awaited"]
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
#[doc = "API parts for the Ingest Get Geoip Database API"]
pub enum IngestGetGeoipDatabaseParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Id"]
    Id(&'b [&'b str]),
}
impl<'b> IngestGetGeoipDatabaseParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Get Geoip Database API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestGetGeoipDatabaseParts::None => "/_ingest/geoip/database".into(),
            IngestGetGeoipDatabaseParts::Id(id) => {
                let id_str = id.join(",");
                let encoded_id: Cow<str> = percent_encode(id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_id.len());
                p.push_str("/_ingest/geoip/database/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Get Geoip Database API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/get-geoip-database-api.html)\n\nReturns geoip database configuration."]
#[derive(Clone, Debug)]
pub struct IngestGetGeoipDatabase<'a, 'b> {
    transport: &'a Transport,
    parts: IngestGetGeoipDatabaseParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IngestGetGeoipDatabase<'a, 'b> {
    #[doc = "Creates a new instance of [IngestGetGeoipDatabase] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestGetGeoipDatabaseParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestGetGeoipDatabase {
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
    #[doc = "Creates an asynchronous call to the Ingest Get Geoip Database API that can be awaited"]
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
#[doc = "API parts for the Ingest Get Ip Location Database API"]
pub enum IngestGetIpLocationDatabaseParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Id"]
    Id(&'b [&'b str]),
}
impl<'b> IngestGetIpLocationDatabaseParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Get Ip Location Database API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestGetIpLocationDatabaseParts::None => "/_ingest/ip_location/database".into(),
            IngestGetIpLocationDatabaseParts::Id(id) => {
                let id_str = id.join(",");
                let encoded_id: Cow<str> = percent_encode(id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_id.len());
                p.push_str("/_ingest/ip_location/database/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Get Ip Location Database API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/get-ip-location-database-api.html)\n\nReturns the specified ip location database configuration"]
#[derive(Clone, Debug)]
pub struct IngestGetIpLocationDatabase<'a, 'b> {
    transport: &'a Transport,
    parts: IngestGetIpLocationDatabaseParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IngestGetIpLocationDatabase<'a, 'b> {
    #[doc = "Creates a new instance of [IngestGetIpLocationDatabase] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestGetIpLocationDatabaseParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestGetIpLocationDatabase {
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
    #[doc = "Creates an asynchronous call to the Ingest Get Ip Location Database API that can be awaited"]
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
#[doc = "API parts for the Ingest Get Pipeline API"]
pub enum IngestGetPipelineParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> IngestGetPipelineParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Get Pipeline API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestGetPipelineParts::None => "/_ingest/pipeline".into(),
            IngestGetPipelineParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_id.len());
                p.push_str("/_ingest/pipeline/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Get Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/get-pipeline-api.html)\n\nReturns a pipeline."]
#[derive(Clone, Debug)]
pub struct IngestGetPipeline<'a, 'b> {
    transport: &'a Transport,
    parts: IngestGetPipelineParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    summary: Option<bool>,
}
impl<'a, 'b> IngestGetPipeline<'a, 'b> {
    #[doc = "Creates a new instance of [IngestGetPipeline] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestGetPipelineParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestGetPipeline {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            summary: None,
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
    #[doc = "Return pipelines without their definitions (default: false)"]
    pub fn summary(mut self, summary: bool) -> Self {
        self.summary = Some(summary);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Get Pipeline API that can be awaited"]
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
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                summary: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                summary: self.summary,
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
#[doc = "API parts for the Ingest Processor Grok API"]
pub enum IngestProcessorGrokParts {
    #[doc = "No parts"]
    None,
}
impl IngestProcessorGrokParts {
    #[doc = "Builds a relative URL path to the Ingest Processor Grok API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestProcessorGrokParts::None => "/_ingest/processor/grok".into(),
        }
    }
}
#[doc = "Builder for the [Ingest Processor Grok API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/grok-processor.html#grok-processor-rest-get)\n\nReturns a list of the built-in patterns."]
#[derive(Clone, Debug)]
pub struct IngestProcessorGrok<'a, 'b> {
    transport: &'a Transport,
    parts: IngestProcessorGrokParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IngestProcessorGrok<'a, 'b> {
    #[doc = "Creates a new instance of [IngestProcessorGrok]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        IngestProcessorGrok {
            transport,
            parts: IngestProcessorGrokParts::None,
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
    #[doc = "Creates an asynchronous call to the Ingest Processor Grok API that can be awaited"]
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
#[doc = "API parts for the Ingest Put Geoip Database API"]
pub enum IngestPutGeoipDatabaseParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> IngestPutGeoipDatabaseParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Put Geoip Database API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestPutGeoipDatabaseParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_id.len());
                p.push_str("/_ingest/geoip/database/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Put Geoip Database API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/put-geoip-database-api.html)\n\nPuts the configuration for a geoip database to be downloaded"]
#[derive(Clone, Debug)]
pub struct IngestPutGeoipDatabase<'a, 'b, B> {
    transport: &'a Transport,
    parts: IngestPutGeoipDatabaseParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> IngestPutGeoipDatabase<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IngestPutGeoipDatabase] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestPutGeoipDatabaseParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestPutGeoipDatabase {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IngestPutGeoipDatabase<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IngestPutGeoipDatabase {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Put Geoip Database API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ingest Put Ip Location Database API"]
pub enum IngestPutIpLocationDatabaseParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> IngestPutIpLocationDatabaseParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Put Ip Location Database API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestPutIpLocationDatabaseParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_id.len());
                p.push_str("/_ingest/ip_location/database/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Put Ip Location Database API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/put-ip-location-database-api.html)\n\nPuts the configuration for a ip location database to be downloaded"]
#[derive(Clone, Debug)]
pub struct IngestPutIpLocationDatabase<'a, 'b, B> {
    transport: &'a Transport,
    parts: IngestPutIpLocationDatabaseParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> IngestPutIpLocationDatabase<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IngestPutIpLocationDatabase] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestPutIpLocationDatabaseParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestPutIpLocationDatabase {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IngestPutIpLocationDatabase<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IngestPutIpLocationDatabase {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Put Ip Location Database API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ingest Put Pipeline API"]
pub enum IngestPutPipelineParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> IngestPutPipelineParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Put Pipeline API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestPutPipelineParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_id.len());
                p.push_str("/_ingest/pipeline/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Put Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/put-pipeline-api.html)\n\nCreates or updates a pipeline."]
#[derive(Clone, Debug)]
pub struct IngestPutPipeline<'a, 'b, B> {
    transport: &'a Transport,
    parts: IngestPutPipelineParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    if_version: Option<i32>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> IngestPutPipeline<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IngestPutPipeline] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestPutPipelineParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestPutPipeline {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_version: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IngestPutPipeline<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IngestPutPipeline {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            if_version: self.if_version,
            master_timeout: self.master_timeout,
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
    #[doc = "Required version for optimistic concurrency control for pipeline updates"]
    pub fn if_version(mut self, if_version: i32) -> Self {
        self.if_version = Some(if_version);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Put Pipeline API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
                if_version: Option<i32>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_version: self.if_version,
                master_timeout: self.master_timeout,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ingest Simulate API"]
pub enum IngestSimulateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> IngestSimulateParts<'b> {
    #[doc = "Builds a relative URL path to the Ingest Simulate API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestSimulateParts::None => "/_ingest/pipeline/_simulate".into(),
            IngestSimulateParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(28usize + encoded_id.len());
                p.push_str("/_ingest/pipeline/");
                p.push_str(encoded_id.as_ref());
                p.push_str("/_simulate");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingest Simulate API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/simulate-pipeline-api.html)\n\nAllows to simulate a pipeline with example documents."]
#[derive(Clone, Debug)]
pub struct IngestSimulate<'a, 'b, B> {
    transport: &'a Transport,
    parts: IngestSimulateParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    verbose: Option<bool>,
}
impl<'a, 'b, B> IngestSimulate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IngestSimulate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestSimulateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestSimulate {
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
            verbose: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IngestSimulate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IngestSimulate {
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
            verbose: self.verbose,
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
    #[doc = "Verbose mode. Display data output for each processor in executed pipeline"]
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = Some(verbose);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingest Simulate API that can be awaited"]
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
                verbose: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                verbose: self.verbose,
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
#[doc = "Namespace client for Ingest APIs"]
pub struct Ingest<'a> {
    transport: &'a Transport,
}
impl<'a> Ingest<'a> {
    #[doc = "Creates a new instance of [Ingest]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Ingest Delete Geoip Database API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/delete-geoip-database-api.html)\n\nDeletes a geoip database configuration"]
    pub fn delete_geoip_database<'b>(
        &'a self,
        parts: IngestDeleteGeoipDatabaseParts<'b>,
    ) -> IngestDeleteGeoipDatabase<'a, 'b> {
        IngestDeleteGeoipDatabase::new(self.transport(), parts)
    }
    #[doc = "[Ingest Delete Ip Location Database API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/delete-ip-location-database-api.html)\n\nDeletes an ip location database configuration"]
    pub fn delete_ip_location_database<'b>(
        &'a self,
        parts: IngestDeleteIpLocationDatabaseParts<'b>,
    ) -> IngestDeleteIpLocationDatabase<'a, 'b> {
        IngestDeleteIpLocationDatabase::new(self.transport(), parts)
    }
    #[doc = "[Ingest Delete Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/delete-pipeline-api.html)\n\nDeletes a pipeline."]
    pub fn delete_pipeline<'b>(
        &'a self,
        parts: IngestDeletePipelineParts<'b>,
    ) -> IngestDeletePipeline<'a, 'b> {
        IngestDeletePipeline::new(self.transport(), parts)
    }
    #[doc = "[Ingest Geo Ip Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/geoip-stats-api.html)\n\nReturns statistical information about geoip databases"]
    pub fn geo_ip_stats<'b>(&'a self) -> IngestGeoIpStats<'a, 'b> {
        IngestGeoIpStats::new(self.transport())
    }
    #[doc = "[Ingest Get Geoip Database API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/get-geoip-database-api.html)\n\nReturns geoip database configuration."]
    pub fn get_geoip_database<'b>(
        &'a self,
        parts: IngestGetGeoipDatabaseParts<'b>,
    ) -> IngestGetGeoipDatabase<'a, 'b> {
        IngestGetGeoipDatabase::new(self.transport(), parts)
    }
    #[doc = "[Ingest Get Ip Location Database API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/get-ip-location-database-api.html)\n\nReturns the specified ip location database configuration"]
    pub fn get_ip_location_database<'b>(
        &'a self,
        parts: IngestGetIpLocationDatabaseParts<'b>,
    ) -> IngestGetIpLocationDatabase<'a, 'b> {
        IngestGetIpLocationDatabase::new(self.transport(), parts)
    }
    #[doc = "[Ingest Get Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/get-pipeline-api.html)\n\nReturns a pipeline."]
    pub fn get_pipeline<'b>(
        &'a self,
        parts: IngestGetPipelineParts<'b>,
    ) -> IngestGetPipeline<'a, 'b> {
        IngestGetPipeline::new(self.transport(), parts)
    }
    #[doc = "[Ingest Processor Grok API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/grok-processor.html#grok-processor-rest-get)\n\nReturns a list of the built-in patterns."]
    pub fn processor_grok<'b>(&'a self) -> IngestProcessorGrok<'a, 'b> {
        IngestProcessorGrok::new(self.transport())
    }
    #[doc = "[Ingest Put Geoip Database API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/put-geoip-database-api.html)\n\nPuts the configuration for a geoip database to be downloaded"]
    pub fn put_geoip_database<'b>(
        &'a self,
        parts: IngestPutGeoipDatabaseParts<'b>,
    ) -> IngestPutGeoipDatabase<'a, 'b, ()> {
        IngestPutGeoipDatabase::new(self.transport(), parts)
    }
    #[doc = "[Ingest Put Ip Location Database API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/put-ip-location-database-api.html)\n\nPuts the configuration for a ip location database to be downloaded"]
    pub fn put_ip_location_database<'b>(
        &'a self,
        parts: IngestPutIpLocationDatabaseParts<'b>,
    ) -> IngestPutIpLocationDatabase<'a, 'b, ()> {
        IngestPutIpLocationDatabase::new(self.transport(), parts)
    }
    #[doc = "[Ingest Put Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/put-pipeline-api.html)\n\nCreates or updates a pipeline."]
    pub fn put_pipeline<'b>(
        &'a self,
        parts: IngestPutPipelineParts<'b>,
    ) -> IngestPutPipeline<'a, 'b, ()> {
        IngestPutPipeline::new(self.transport(), parts)
    }
    #[doc = "[Ingest Simulate API](https://www.elastic.co/guide/en/elasticsearch/reference/9.0/simulate-pipeline-api.html)\n\nAllows to simulate a pipeline with example documents."]
    pub fn simulate<'b>(&'a self, parts: IngestSimulateParts<'b>) -> IngestSimulate<'a, 'b, ()> {
        IngestSimulate::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Ingest APIs"]
    pub fn ingest(&self) -> Ingest {
        Ingest::new(self.transport())
    }
}
