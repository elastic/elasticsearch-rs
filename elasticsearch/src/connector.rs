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

//! Connector APIs
//!
//! The connector and sync jobs APIs provide a convenient way to create and manage [Elastic connectors](https://www.elastic.co/guide/en/enterprise-search/master/connectors.html) and sync jobs in an internal index. To get started with Connector APIs, check out the tutorial.
//!
//! Connectors are Elasticsearch integrations that bring content from third-party data sources, which can be deployed on Elastic Cloud or hosted on your own infrastructure:
//!
//! * Native connectors are a managed service on Elastic Cloud
//! * Connector clients are self-managed on your infrastructure
//!
//! This API provides an alternative to relying solely on Kibana UI for connector and sync job management. The API comes with a set of validations and assertions to ensure that the state representation in the internal index remains valid.

#![cfg(feature = "experimental-apis")]
#![doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
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
#[doc = "API parts for the Connector Check In API"]
pub enum ConnectorCheckInParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorCheckInParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Check In API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorCheckInParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_check_in");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Check In API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/check-in-connector-api.html)\n\nUpdates the last_seen timestamp in the connector document."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorCheckIn<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorCheckInParts<'b>,
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
impl<'a, 'b, B> ConnectorCheckIn<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorCheckIn] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorCheckInParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorCheckIn {
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
    pub fn body<T>(self, body: T) -> ConnectorCheckIn<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorCheckIn {
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
    #[doc = "Creates an asynchronous call to the Connector Check In API that can be awaited"]
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
#[doc = "API parts for the Connector Delete API"]
pub enum ConnectorDeleteParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorDeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorDeleteParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-connector-api.html)\n\nDeletes a connector."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorDelete<'a, 'b> {
    transport: &'a Transport,
    parts: ConnectorDeleteParts<'b>,
    delete_sync_jobs: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> ConnectorDelete<'a, 'b> {
    #[doc = "Creates a new instance of [ConnectorDelete] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorDelete {
            transport,
            parts,
            headers,
            delete_sync_jobs: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Determines whether associated sync jobs are also deleted."]
    pub fn delete_sync_jobs(mut self, delete_sync_jobs: bool) -> Self {
        self.delete_sync_jobs = Some(delete_sync_jobs);
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
    #[doc = "Creates an asynchronous call to the Connector Delete API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Delete;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                delete_sync_jobs: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                delete_sync_jobs: self.delete_sync_jobs,
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
#[doc = "API parts for the Connector Get API"]
pub enum ConnectorGetParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorGetParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorGetParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Get API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-connector-api.html)\n\nReturns the details about a connector."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorGet<'a, 'b> {
    transport: &'a Transport,
    parts: ConnectorGetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> ConnectorGet<'a, 'b> {
    #[doc = "Creates a new instance of [ConnectorGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorGet {
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
    #[doc = "Creates an asynchronous call to the Connector Get API that can be awaited"]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Connector Last Sync API"]
pub enum ConnectorLastSyncParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorLastSyncParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Last Sync API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorLastSyncParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_last_sync");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Last Sync API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-last-sync-api.html)\n\nUpdates the stats of last sync in the connector document."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorLastSync<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorLastSyncParts<'b>,
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
impl<'a, 'b, B> ConnectorLastSync<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorLastSync] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorLastSyncParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorLastSync {
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
    pub fn body<T>(self, body: T) -> ConnectorLastSync<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorLastSync {
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
    #[doc = "Creates an asynchronous call to the Connector Last Sync API that can be awaited"]
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
#[doc = "API parts for the Connector List API"]
pub enum ConnectorListParts {
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl ConnectorListParts {
    #[doc = "Builds a relative URL path to the Connector List API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorListParts::None => "/_connector".into(),
        }
    }
}
#[doc = "Builder for the [Connector List API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/list-connector-api.html)\n\nLists all connectors."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorList<'a, 'b> {
    transport: &'a Transport,
    parts: ConnectorListParts,
    connector_name: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    index_name: Option<&'b [&'b str]>,
    pretty: Option<bool>,
    query: Option<&'b str>,
    request_timeout: Option<Duration>,
    service_type: Option<&'b [&'b str]>,
    size: Option<i32>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> ConnectorList<'a, 'b> {
    #[doc = "Creates a new instance of [ConnectorList]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ConnectorList {
            transport,
            parts: ConnectorListParts::None,
            headers,
            connector_name: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            index_name: None,
            pretty: None,
            query: None,
            request_timeout: None,
            service_type: None,
            size: None,
            source: None,
        }
    }
    #[doc = "A comma-separated list of connector names to fetch connector documents for"]
    pub fn connector_name(mut self, connector_name: &'b [&'b str]) -> Self {
        self.connector_name = Some(connector_name);
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
    #[doc = "Starting offset (default: 0)"]
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
    #[doc = "A comma-separated list of connector index names to fetch connector documents for"]
    pub fn index_name(mut self, index_name: &'b [&'b str]) -> Self {
        self.index_name = Some(index_name);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "A search string for querying connectors, filtering results by matching against connector names, descriptions, and index names"]
    pub fn query(mut self, query: &'b str) -> Self {
        self.query = Some(query);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of connector service types to fetch connector documents for"]
    pub fn service_type(mut self, service_type: &'b [&'b str]) -> Self {
        self.service_type = Some(service_type);
        self
    }
    #[doc = "Specifies a max number of results to get (default: 100)"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Connector List API that can be awaited"]
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
                connector_name: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                from: Option<i32>,
                human: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                index_name: Option<&'b [&'b str]>,
                pretty: Option<bool>,
                query: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                service_type: Option<&'b [&'b str]>,
                size: Option<i32>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                connector_name: self.connector_name,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                index_name: self.index_name,
                pretty: self.pretty,
                query: self.query,
                service_type: self.service_type,
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Connector Post API"]
pub enum ConnectorPostParts {
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl ConnectorPostParts {
    #[doc = "Builds a relative URL path to the Connector Post API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorPostParts::None => "/_connector".into(),
        }
    }
}
#[doc = "Builder for the [Connector Post API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/create-connector-api.html)\n\nCreates a connector."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorPost<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorPostParts,
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
impl<'a, 'b, B> ConnectorPost<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorPost]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ConnectorPost {
            transport,
            parts: ConnectorPostParts::None,
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
    pub fn body<T>(self, body: T) -> ConnectorPost<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorPost {
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
    #[doc = "Creates an asynchronous call to the Connector Post API that can be awaited"]
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
#[doc = "API parts for the Connector Put API"]
pub enum ConnectorPutParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorPutParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Put API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorPutParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.into()
            }
            ConnectorPutParts::None => "/_connector".into(),
        }
    }
}
#[doc = "Builder for the [Connector Put API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/create-connector-api.html)\n\nCreates or updates a connector."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorPut<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorPutParts<'b>,
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
impl<'a, 'b, B> ConnectorPut<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorPut] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorPutParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorPut {
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
    pub fn body<T>(self, body: T) -> ConnectorPut<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorPut {
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
    #[doc = "Creates an asynchronous call to the Connector Put API that can be awaited"]
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
#[doc = "API parts for the Connector Secret Delete API"]
pub enum ConnectorSecretDeleteParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorSecretDeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Secret Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorSecretDeleteParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_id.len());
                p.push_str("/_connector/_secret/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Connector Secret Delete API\n\nDeletes a connector secret."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorSecretDelete<'a, 'b> {
    transport: &'a Transport,
    parts: ConnectorSecretDeleteParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> ConnectorSecretDelete<'a, 'b> {
    #[doc = "Creates a new instance of [ConnectorSecretDelete] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorSecretDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorSecretDelete {
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
    #[doc = "Creates an asynchronous call to the Connector Secret Delete API that can be awaited"]
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
#[doc = "API parts for the Connector Secret Get API"]
pub enum ConnectorSecretGetParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorSecretGetParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Secret Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorSecretGetParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_id.len());
                p.push_str("/_connector/_secret/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Connector Secret Get API\n\nRetrieves a secret stored by Connectors."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorSecretGet<'a, 'b> {
    transport: &'a Transport,
    parts: ConnectorSecretGetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> ConnectorSecretGet<'a, 'b> {
    #[doc = "Creates a new instance of [ConnectorSecretGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorSecretGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorSecretGet {
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
    #[doc = "Creates an asynchronous call to the Connector Secret Get API that can be awaited"]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Connector Secret Post API"]
pub enum ConnectorSecretPostParts {
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl ConnectorSecretPostParts {
    #[doc = "Builds a relative URL path to the Connector Secret Post API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorSecretPostParts::None => "/_connector/_secret".into(),
        }
    }
}
#[doc = "Builder for the Connector Secret Post API\n\nCreates a secret for a Connector."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorSecretPost<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorSecretPostParts,
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
impl<'a, 'b, B> ConnectorSecretPost<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorSecretPost]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ConnectorSecretPost {
            transport,
            parts: ConnectorSecretPostParts::None,
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
    pub fn body<T>(self, body: T) -> ConnectorSecretPost<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorSecretPost {
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
    #[doc = "Creates an asynchronous call to the Connector Secret Post API that can be awaited"]
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
#[doc = "API parts for the Connector Secret Put API"]
pub enum ConnectorSecretPutParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorSecretPutParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Secret Put API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorSecretPutParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_id.len());
                p.push_str("/_connector/_secret/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Connector Secret Put API\n\nCreates or updates a secret for a Connector."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorSecretPut<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorSecretPutParts<'b>,
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
impl<'a, 'b, B> ConnectorSecretPut<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorSecretPut] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorSecretPutParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorSecretPut {
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
    pub fn body<T>(self, body: T) -> ConnectorSecretPut<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorSecretPut {
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
    #[doc = "Creates an asynchronous call to the Connector Secret Put API that can be awaited"]
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
#[doc = "API parts for the Connector Sync Job Cancel API"]
pub enum ConnectorSyncJobCancelParts<'b> {
    #[doc = "ConnectorSyncJobId"]
    ConnectorSyncJobId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorSyncJobCancelParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Sync Job Cancel API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorSyncJobCancelParts::ConnectorSyncJobId(connector_sync_job_id) => {
                let encoded_connector_sync_job_id: Cow<str> =
                    percent_encode(connector_sync_job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_connector_sync_job_id.len());
                p.push_str("/_connector/_sync_job/");
                p.push_str(encoded_connector_sync_job_id.as_ref());
                p.push_str("/_cancel");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Sync Job Cancel API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/cancel-connector-sync-job-api.html)\n\nCancels a connector sync job."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorSyncJobCancel<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorSyncJobCancelParts<'b>,
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
impl<'a, 'b, B> ConnectorSyncJobCancel<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorSyncJobCancel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorSyncJobCancelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorSyncJobCancel {
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
    pub fn body<T>(self, body: T) -> ConnectorSyncJobCancel<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorSyncJobCancel {
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
    #[doc = "Creates an asynchronous call to the Connector Sync Job Cancel API that can be awaited"]
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
#[doc = "API parts for the Connector Sync Job Check In API"]
pub enum ConnectorSyncJobCheckInParts<'b> {
    #[doc = "ConnectorSyncJobId"]
    ConnectorSyncJobId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorSyncJobCheckInParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Sync Job Check In API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorSyncJobCheckInParts::ConnectorSyncJobId(connector_sync_job_id) => {
                let encoded_connector_sync_job_id: Cow<str> =
                    percent_encode(connector_sync_job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_connector_sync_job_id.len());
                p.push_str("/_connector/_sync_job/");
                p.push_str(encoded_connector_sync_job_id.as_ref());
                p.push_str("/_check_in");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Sync Job Check In API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/check-in-connector-sync-job-api.html)\n\nChecks in a connector sync job (refreshes 'last_seen')."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorSyncJobCheckIn<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorSyncJobCheckInParts<'b>,
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
impl<'a, 'b, B> ConnectorSyncJobCheckIn<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorSyncJobCheckIn] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorSyncJobCheckInParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorSyncJobCheckIn {
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
    pub fn body<T>(self, body: T) -> ConnectorSyncJobCheckIn<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorSyncJobCheckIn {
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
    #[doc = "Creates an asynchronous call to the Connector Sync Job Check In API that can be awaited"]
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
#[doc = "API parts for the Connector Sync Job Claim API"]
pub enum ConnectorSyncJobClaimParts<'b> {
    #[doc = "ConnectorSyncJobId"]
    ConnectorSyncJobId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorSyncJobClaimParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Sync Job Claim API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorSyncJobClaimParts::ConnectorSyncJobId(connector_sync_job_id) => {
                let encoded_connector_sync_job_id: Cow<str> =
                    percent_encode(connector_sync_job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_connector_sync_job_id.len());
                p.push_str("/_connector/_sync_job/");
                p.push_str(encoded_connector_sync_job_id.as_ref());
                p.push_str("/_claim");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Sync Job Claim API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/claim-connector-sync-job-api.html)\n\nClaims a connector sync job."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorSyncJobClaim<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorSyncJobClaimParts<'b>,
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
impl<'a, 'b, B> ConnectorSyncJobClaim<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorSyncJobClaim] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorSyncJobClaimParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorSyncJobClaim {
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
    pub fn body<T>(self, body: T) -> ConnectorSyncJobClaim<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorSyncJobClaim {
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
    #[doc = "Creates an asynchronous call to the Connector Sync Job Claim API that can be awaited"]
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
#[doc = "API parts for the Connector Sync Job Delete API"]
pub enum ConnectorSyncJobDeleteParts<'b> {
    #[doc = "ConnectorSyncJobId"]
    ConnectorSyncJobId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorSyncJobDeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Sync Job Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorSyncJobDeleteParts::ConnectorSyncJobId(connector_sync_job_id) => {
                let encoded_connector_sync_job_id: Cow<str> =
                    percent_encode(connector_sync_job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_connector_sync_job_id.len());
                p.push_str("/_connector/_sync_job/");
                p.push_str(encoded_connector_sync_job_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Sync Job Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-connector-sync-job-api.html)\n\nDeletes a connector sync job."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorSyncJobDelete<'a, 'b> {
    transport: &'a Transport,
    parts: ConnectorSyncJobDeleteParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> ConnectorSyncJobDelete<'a, 'b> {
    #[doc = "Creates a new instance of [ConnectorSyncJobDelete] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorSyncJobDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorSyncJobDelete {
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
    #[doc = "Creates an asynchronous call to the Connector Sync Job Delete API that can be awaited"]
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
#[doc = "API parts for the Connector Sync Job Error API"]
pub enum ConnectorSyncJobErrorParts<'b> {
    #[doc = "ConnectorSyncJobId"]
    ConnectorSyncJobId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorSyncJobErrorParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Sync Job Error API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorSyncJobErrorParts::ConnectorSyncJobId(connector_sync_job_id) => {
                let encoded_connector_sync_job_id: Cow<str> =
                    percent_encode(connector_sync_job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_connector_sync_job_id.len());
                p.push_str("/_connector/_sync_job/");
                p.push_str(encoded_connector_sync_job_id.as_ref());
                p.push_str("/_error");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Sync Job Error API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/set-connector-sync-job-error-api.html)\n\nSets an error for a connector sync job."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorSyncJobError<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorSyncJobErrorParts<'b>,
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
impl<'a, 'b, B> ConnectorSyncJobError<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorSyncJobError] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorSyncJobErrorParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorSyncJobError {
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
    pub fn body<T>(self, body: T) -> ConnectorSyncJobError<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorSyncJobError {
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
    #[doc = "Creates an asynchronous call to the Connector Sync Job Error API that can be awaited"]
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
#[doc = "API parts for the Connector Sync Job Get API"]
pub enum ConnectorSyncJobGetParts<'b> {
    #[doc = "ConnectorSyncJobId"]
    ConnectorSyncJobId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorSyncJobGetParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Sync Job Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorSyncJobGetParts::ConnectorSyncJobId(connector_sync_job_id) => {
                let encoded_connector_sync_job_id: Cow<str> =
                    percent_encode(connector_sync_job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_connector_sync_job_id.len());
                p.push_str("/_connector/_sync_job/");
                p.push_str(encoded_connector_sync_job_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Sync Job Get API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-connector-sync-job-api.html)\n\nReturns the details about a connector sync job."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorSyncJobGet<'a, 'b> {
    transport: &'a Transport,
    parts: ConnectorSyncJobGetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> ConnectorSyncJobGet<'a, 'b> {
    #[doc = "Creates a new instance of [ConnectorSyncJobGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorSyncJobGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorSyncJobGet {
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
    #[doc = "Creates an asynchronous call to the Connector Sync Job Get API that can be awaited"]
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Connector Sync Job List API"]
pub enum ConnectorSyncJobListParts {
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl ConnectorSyncJobListParts {
    #[doc = "Builds a relative URL path to the Connector Sync Job List API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorSyncJobListParts::None => "/_connector/_sync_job".into(),
        }
    }
}
#[doc = "Builder for the [Connector Sync Job List API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/list-connector-sync-jobs-api.html)\n\nLists all connector sync jobs."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorSyncJobList<'a, 'b> {
    transport: &'a Transport,
    parts: ConnectorSyncJobListParts,
    connector_id: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    job_type: Option<&'b [&'b str]>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    size: Option<i32>,
    source: Option<&'b str>,
    status: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> ConnectorSyncJobList<'a, 'b> {
    #[doc = "Creates a new instance of [ConnectorSyncJobList]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ConnectorSyncJobList {
            transport,
            parts: ConnectorSyncJobListParts::None,
            headers,
            connector_id: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            job_type: None,
            pretty: None,
            request_timeout: None,
            size: None,
            source: None,
            status: None,
        }
    }
    #[doc = "Id of the connector to fetch the sync jobs for"]
    pub fn connector_id(mut self, connector_id: &'b str) -> Self {
        self.connector_id = Some(connector_id);
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
    #[doc = "Starting offset (default: 0)"]
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
    #[doc = "A comma-separated list of job types"]
    pub fn job_type(mut self, job_type: &'b [&'b str]) -> Self {
        self.job_type = Some(job_type);
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
    #[doc = "specifies a max number of results to get (default: 100)"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Sync job status, which sync jobs are fetched for"]
    pub fn status(mut self, status: &'b str) -> Self {
        self.status = Some(status);
        self
    }
    #[doc = "Creates an asynchronous call to the Connector Sync Job List API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                connector_id: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                from: Option<i32>,
                human: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                job_type: Option<&'b [&'b str]>,
                pretty: Option<bool>,
                size: Option<i32>,
                source: Option<&'b str>,
                status: Option<&'b str>,
            }
            let query_params = QueryParams {
                connector_id: self.connector_id,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                job_type: self.job_type,
                pretty: self.pretty,
                size: self.size,
                source: self.source,
                status: self.status,
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
#[doc = "API parts for the Connector Sync Job Post API"]
pub enum ConnectorSyncJobPostParts {
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl ConnectorSyncJobPostParts {
    #[doc = "Builds a relative URL path to the Connector Sync Job Post API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorSyncJobPostParts::None => "/_connector/_sync_job".into(),
        }
    }
}
#[doc = "Builder for the [Connector Sync Job Post API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/create-connector-sync-job-api.html)\n\nCreates a connector sync job."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorSyncJobPost<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorSyncJobPostParts,
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
impl<'a, 'b, B> ConnectorSyncJobPost<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorSyncJobPost]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ConnectorSyncJobPost {
            transport,
            parts: ConnectorSyncJobPostParts::None,
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
    pub fn body<T>(self, body: T) -> ConnectorSyncJobPost<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorSyncJobPost {
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
    #[doc = "Creates an asynchronous call to the Connector Sync Job Post API that can be awaited"]
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
#[doc = "API parts for the Connector Sync Job Update Stats API"]
pub enum ConnectorSyncJobUpdateStatsParts<'b> {
    #[doc = "ConnectorSyncJobId"]
    ConnectorSyncJobId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorSyncJobUpdateStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Sync Job Update Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorSyncJobUpdateStatsParts::ConnectorSyncJobId(connector_sync_job_id) => {
                let encoded_connector_sync_job_id: Cow<str> =
                    percent_encode(connector_sync_job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_connector_sync_job_id.len());
                p.push_str("/_connector/_sync_job/");
                p.push_str(encoded_connector_sync_job_id.as_ref());
                p.push_str("/_stats");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Sync Job Update Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/set-connector-sync-job-stats-api.html)\n\nUpdates the stats fields in the connector sync job document."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorSyncJobUpdateStats<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorSyncJobUpdateStatsParts<'b>,
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
impl<'a, 'b, B> ConnectorSyncJobUpdateStats<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorSyncJobUpdateStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorSyncJobUpdateStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorSyncJobUpdateStats {
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
    pub fn body<T>(self, body: T) -> ConnectorSyncJobUpdateStats<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorSyncJobUpdateStats {
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
    #[doc = "Creates an asynchronous call to the Connector Sync Job Update Stats API that can be awaited"]
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
#[doc = "API parts for the Connector Update Active Filtering API"]
pub enum ConnectorUpdateActiveFilteringParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdateActiveFilteringParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Active Filtering API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdateActiveFilteringParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_filtering/_activate");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Active Filtering API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-filtering-api.html)\n\nActivates the draft filtering rules if they are in a validated state."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdateActiveFiltering<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdateActiveFilteringParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdateActiveFiltering<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdateActiveFiltering] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorUpdateActiveFilteringParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdateActiveFiltering {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdateActiveFiltering<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdateActiveFiltering {
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
    #[doc = "Creates an asynchronous call to the Connector Update Active Filtering API that can be awaited"]
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
#[doc = "API parts for the Connector Update Api Key Id API"]
pub enum ConnectorUpdateApiKeyIdParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdateApiKeyIdParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Api Key Id API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdateApiKeyIdParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_api_key_id");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Api Key Id API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-api-key-id-api.html)\n\nUpdates the API key id and/or API key secret id fields in the connector document."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdateApiKeyId<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdateApiKeyIdParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdateApiKeyId<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdateApiKeyId] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorUpdateApiKeyIdParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdateApiKeyId {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdateApiKeyId<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdateApiKeyId {
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
    #[doc = "Creates an asynchronous call to the Connector Update Api Key Id API that can be awaited"]
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
#[doc = "API parts for the Connector Update Configuration API"]
pub enum ConnectorUpdateConfigurationParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdateConfigurationParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Configuration API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdateConfigurationParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_configuration");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Configuration API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-configuration-api.html)\n\nUpdates the connector configuration."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdateConfiguration<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdateConfigurationParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdateConfiguration<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdateConfiguration] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorUpdateConfigurationParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdateConfiguration {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdateConfiguration<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdateConfiguration {
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
    #[doc = "Creates an asynchronous call to the Connector Update Configuration API that can be awaited"]
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
#[doc = "API parts for the Connector Update Error API"]
pub enum ConnectorUpdateErrorParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdateErrorParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Error API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdateErrorParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_error");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Error API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-error-api.html)\n\nUpdates the error field in the connector document."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdateError<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdateErrorParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdateError<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdateError] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorUpdateErrorParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdateError {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdateError<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdateError {
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
    #[doc = "Creates an asynchronous call to the Connector Update Error API that can be awaited"]
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
#[doc = "API parts for the Connector Update Features API"]
pub enum ConnectorUpdateFeaturesParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdateFeaturesParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Features API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdateFeaturesParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_features");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Features API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-features-api.html)\n\nUpdates the connector features in the connector document."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdateFeatures<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdateFeaturesParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdateFeatures<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdateFeatures] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorUpdateFeaturesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdateFeatures {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdateFeatures<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdateFeatures {
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
    #[doc = "Creates an asynchronous call to the Connector Update Features API that can be awaited"]
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
#[doc = "API parts for the Connector Update Filtering API"]
pub enum ConnectorUpdateFilteringParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdateFilteringParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Filtering API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdateFilteringParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_filtering");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Filtering API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-filtering-api.html)\n\nUpdates the filtering field in the connector document."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdateFiltering<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdateFilteringParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdateFiltering<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdateFiltering] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorUpdateFilteringParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdateFiltering {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdateFiltering<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdateFiltering {
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
    #[doc = "Creates an asynchronous call to the Connector Update Filtering API that can be awaited"]
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
#[doc = "API parts for the Connector Update Filtering Validation API"]
pub enum ConnectorUpdateFilteringValidationParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdateFilteringValidationParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Filtering Validation API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdateFilteringValidationParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(35usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_filtering/_validation");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Filtering Validation API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-filtering-api.html)\n\nUpdates the validation info of the draft filtering rules."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdateFilteringValidation<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdateFilteringValidationParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdateFilteringValidation<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdateFilteringValidation] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: ConnectorUpdateFilteringValidationParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdateFilteringValidation {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdateFilteringValidation<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdateFilteringValidation {
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
    #[doc = "Creates an asynchronous call to the Connector Update Filtering Validation API that can be awaited"]
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
#[doc = "API parts for the Connector Update Index Name API"]
pub enum ConnectorUpdateIndexNameParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdateIndexNameParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Index Name API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdateIndexNameParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_index_name");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Index Name API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-index-name-api.html)\n\nUpdates the index name of the connector."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdateIndexName<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdateIndexNameParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdateIndexName<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdateIndexName] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorUpdateIndexNameParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdateIndexName {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdateIndexName<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdateIndexName {
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
    #[doc = "Creates an asynchronous call to the Connector Update Index Name API that can be awaited"]
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
#[doc = "API parts for the Connector Update Name API"]
pub enum ConnectorUpdateNameParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdateNameParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Name API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdateNameParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_name");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Name API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-name-description-api.html)\n\nUpdates the name and/or description fields in the connector document."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdateName<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdateNameParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdateName<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdateName] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorUpdateNameParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdateName {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdateName<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdateName {
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
    #[doc = "Creates an asynchronous call to the Connector Update Name API that can be awaited"]
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
#[doc = "API parts for the Connector Update Native API"]
pub enum ConnectorUpdateNativeParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdateNativeParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Native API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdateNativeParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_native");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Native API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/connector-apis.html)\n\nUpdates the is_native flag of the connector."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdateNative<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdateNativeParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdateNative<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdateNative] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorUpdateNativeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdateNative {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdateNative<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdateNative {
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
    #[doc = "Creates an asynchronous call to the Connector Update Native API that can be awaited"]
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
#[doc = "API parts for the Connector Update Pipeline API"]
pub enum ConnectorUpdatePipelineParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdatePipelineParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Pipeline API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdatePipelineParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_pipeline");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-pipeline-api.html)\n\nUpdates the pipeline field in the connector document."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdatePipeline<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdatePipelineParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdatePipeline<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdatePipeline] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorUpdatePipelineParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdatePipeline {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdatePipeline<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdatePipeline {
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
    #[doc = "Creates an asynchronous call to the Connector Update Pipeline API that can be awaited"]
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
#[doc = "API parts for the Connector Update Scheduling API"]
pub enum ConnectorUpdateSchedulingParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdateSchedulingParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Scheduling API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdateSchedulingParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_scheduling");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Scheduling API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-scheduling-api.html)\n\nUpdates the scheduling field in the connector document."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdateScheduling<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdateSchedulingParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdateScheduling<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdateScheduling] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorUpdateSchedulingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdateScheduling {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdateScheduling<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdateScheduling {
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
    #[doc = "Creates an asynchronous call to the Connector Update Scheduling API that can be awaited"]
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
#[doc = "API parts for the Connector Update Service Type API"]
pub enum ConnectorUpdateServiceTypeParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdateServiceTypeParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Service Type API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdateServiceTypeParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(26usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_service_type");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Service Type API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-service-type-api.html)\n\nUpdates the service type of the connector."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdateServiceType<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdateServiceTypeParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdateServiceType<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdateServiceType] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorUpdateServiceTypeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdateServiceType {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdateServiceType<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdateServiceType {
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
    #[doc = "Creates an asynchronous call to the Connector Update Service Type API that can be awaited"]
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
#[doc = "API parts for the Connector Update Status API"]
pub enum ConnectorUpdateStatusParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> ConnectorUpdateStatusParts<'b> {
    #[doc = "Builds a relative URL path to the Connector Update Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ConnectorUpdateStatusParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_connector_id.len());
                p.push_str("/_connector/");
                p.push_str(encoded_connector_id.as_ref());
                p.push_str("/_status");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Connector Update Status API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-status-api.html)\n\nUpdates the status of the connector."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct ConnectorUpdateStatus<'a, 'b, B> {
    transport: &'a Transport,
    parts: ConnectorUpdateStatusParts<'b>,
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
impl<'a, 'b, B> ConnectorUpdateStatus<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ConnectorUpdateStatus] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ConnectorUpdateStatusParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ConnectorUpdateStatus {
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
    pub fn body<T>(self, body: T) -> ConnectorUpdateStatus<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ConnectorUpdateStatus {
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
    #[doc = "Creates an asynchronous call to the Connector Update Status API that can be awaited"]
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
#[doc = "Namespace client for Connector APIs"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
pub struct Connector<'a> {
    transport: &'a Transport,
}
#[cfg(feature = "experimental-apis")]
impl<'a> Connector<'a> {
    #[doc = "Creates a new instance of [Connector]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Connector Check In API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/check-in-connector-api.html)\n\nUpdates the last_seen timestamp in the connector document."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn check_in<'b>(
        &'a self,
        parts: ConnectorCheckInParts<'b>,
    ) -> ConnectorCheckIn<'a, 'b, ()> {
        ConnectorCheckIn::new(self.transport(), parts)
    }
    #[doc = "[Connector Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-connector-api.html)\n\nDeletes a connector."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn delete<'b>(&'a self, parts: ConnectorDeleteParts<'b>) -> ConnectorDelete<'a, 'b> {
        ConnectorDelete::new(self.transport(), parts)
    }
    #[doc = "[Connector Get API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-connector-api.html)\n\nReturns the details about a connector."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn get<'b>(&'a self, parts: ConnectorGetParts<'b>) -> ConnectorGet<'a, 'b> {
        ConnectorGet::new(self.transport(), parts)
    }
    #[doc = "[Connector Last Sync API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-last-sync-api.html)\n\nUpdates the stats of last sync in the connector document."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn last_sync<'b>(
        &'a self,
        parts: ConnectorLastSyncParts<'b>,
    ) -> ConnectorLastSync<'a, 'b, ()> {
        ConnectorLastSync::new(self.transport(), parts)
    }
    #[doc = "[Connector List API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/list-connector-api.html)\n\nLists all connectors."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn list<'b>(&'a self) -> ConnectorList<'a, 'b> {
        ConnectorList::new(self.transport())
    }
    #[doc = "[Connector Post API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/create-connector-api.html)\n\nCreates a connector."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn post<'b>(&'a self) -> ConnectorPost<'a, 'b, ()> {
        ConnectorPost::new(self.transport())
    }
    #[doc = "[Connector Put API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/create-connector-api.html)\n\nCreates or updates a connector."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn put<'b>(&'a self, parts: ConnectorPutParts<'b>) -> ConnectorPut<'a, 'b, ()> {
        ConnectorPut::new(self.transport(), parts)
    }
    #[doc = "Connector Secret Delete API\n\nDeletes a connector secret."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn secret_delete<'b>(
        &'a self,
        parts: ConnectorSecretDeleteParts<'b>,
    ) -> ConnectorSecretDelete<'a, 'b> {
        ConnectorSecretDelete::new(self.transport(), parts)
    }
    #[doc = "Connector Secret Get API\n\nRetrieves a secret stored by Connectors."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn secret_get<'b>(
        &'a self,
        parts: ConnectorSecretGetParts<'b>,
    ) -> ConnectorSecretGet<'a, 'b> {
        ConnectorSecretGet::new(self.transport(), parts)
    }
    #[doc = "Connector Secret Post API\n\nCreates a secret for a Connector."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn secret_post<'b>(&'a self) -> ConnectorSecretPost<'a, 'b, ()> {
        ConnectorSecretPost::new(self.transport())
    }
    #[doc = "Connector Secret Put API\n\nCreates or updates a secret for a Connector."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn secret_put<'b>(
        &'a self,
        parts: ConnectorSecretPutParts<'b>,
    ) -> ConnectorSecretPut<'a, 'b, ()> {
        ConnectorSecretPut::new(self.transport(), parts)
    }
    #[doc = "[Connector Sync Job Cancel API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/cancel-connector-sync-job-api.html)\n\nCancels a connector sync job."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn sync_job_cancel<'b>(
        &'a self,
        parts: ConnectorSyncJobCancelParts<'b>,
    ) -> ConnectorSyncJobCancel<'a, 'b, ()> {
        ConnectorSyncJobCancel::new(self.transport(), parts)
    }
    #[doc = "[Connector Sync Job Check In API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/check-in-connector-sync-job-api.html)\n\nChecks in a connector sync job (refreshes 'last_seen')."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn sync_job_check_in<'b>(
        &'a self,
        parts: ConnectorSyncJobCheckInParts<'b>,
    ) -> ConnectorSyncJobCheckIn<'a, 'b, ()> {
        ConnectorSyncJobCheckIn::new(self.transport(), parts)
    }
    #[doc = "[Connector Sync Job Claim API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/claim-connector-sync-job-api.html)\n\nClaims a connector sync job."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn sync_job_claim<'b>(
        &'a self,
        parts: ConnectorSyncJobClaimParts<'b>,
    ) -> ConnectorSyncJobClaim<'a, 'b, ()> {
        ConnectorSyncJobClaim::new(self.transport(), parts)
    }
    #[doc = "[Connector Sync Job Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-connector-sync-job-api.html)\n\nDeletes a connector sync job."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn sync_job_delete<'b>(
        &'a self,
        parts: ConnectorSyncJobDeleteParts<'b>,
    ) -> ConnectorSyncJobDelete<'a, 'b> {
        ConnectorSyncJobDelete::new(self.transport(), parts)
    }
    #[doc = "[Connector Sync Job Error API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/set-connector-sync-job-error-api.html)\n\nSets an error for a connector sync job."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn sync_job_error<'b>(
        &'a self,
        parts: ConnectorSyncJobErrorParts<'b>,
    ) -> ConnectorSyncJobError<'a, 'b, ()> {
        ConnectorSyncJobError::new(self.transport(), parts)
    }
    #[doc = "[Connector Sync Job Get API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-connector-sync-job-api.html)\n\nReturns the details about a connector sync job."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn sync_job_get<'b>(
        &'a self,
        parts: ConnectorSyncJobGetParts<'b>,
    ) -> ConnectorSyncJobGet<'a, 'b> {
        ConnectorSyncJobGet::new(self.transport(), parts)
    }
    #[doc = "[Connector Sync Job List API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/list-connector-sync-jobs-api.html)\n\nLists all connector sync jobs."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn sync_job_list<'b>(&'a self) -> ConnectorSyncJobList<'a, 'b> {
        ConnectorSyncJobList::new(self.transport())
    }
    #[doc = "[Connector Sync Job Post API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/create-connector-sync-job-api.html)\n\nCreates a connector sync job."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn sync_job_post<'b>(&'a self) -> ConnectorSyncJobPost<'a, 'b, ()> {
        ConnectorSyncJobPost::new(self.transport())
    }
    #[doc = "[Connector Sync Job Update Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/set-connector-sync-job-stats-api.html)\n\nUpdates the stats fields in the connector sync job document."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn sync_job_update_stats<'b>(
        &'a self,
        parts: ConnectorSyncJobUpdateStatsParts<'b>,
    ) -> ConnectorSyncJobUpdateStats<'a, 'b, ()> {
        ConnectorSyncJobUpdateStats::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Active Filtering API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-filtering-api.html)\n\nActivates the draft filtering rules if they are in a validated state."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_active_filtering<'b>(
        &'a self,
        parts: ConnectorUpdateActiveFilteringParts<'b>,
    ) -> ConnectorUpdateActiveFiltering<'a, 'b, ()> {
        ConnectorUpdateActiveFiltering::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Api Key Id API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-api-key-id-api.html)\n\nUpdates the API key id and/or API key secret id fields in the connector document."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_api_key_id<'b>(
        &'a self,
        parts: ConnectorUpdateApiKeyIdParts<'b>,
    ) -> ConnectorUpdateApiKeyId<'a, 'b, ()> {
        ConnectorUpdateApiKeyId::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Configuration API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-configuration-api.html)\n\nUpdates the connector configuration."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_configuration<'b>(
        &'a self,
        parts: ConnectorUpdateConfigurationParts<'b>,
    ) -> ConnectorUpdateConfiguration<'a, 'b, ()> {
        ConnectorUpdateConfiguration::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Error API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-error-api.html)\n\nUpdates the error field in the connector document."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_error<'b>(
        &'a self,
        parts: ConnectorUpdateErrorParts<'b>,
    ) -> ConnectorUpdateError<'a, 'b, ()> {
        ConnectorUpdateError::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Features API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-features-api.html)\n\nUpdates the connector features in the connector document."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_features<'b>(
        &'a self,
        parts: ConnectorUpdateFeaturesParts<'b>,
    ) -> ConnectorUpdateFeatures<'a, 'b, ()> {
        ConnectorUpdateFeatures::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Filtering API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-filtering-api.html)\n\nUpdates the filtering field in the connector document."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_filtering<'b>(
        &'a self,
        parts: ConnectorUpdateFilteringParts<'b>,
    ) -> ConnectorUpdateFiltering<'a, 'b, ()> {
        ConnectorUpdateFiltering::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Filtering Validation API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-filtering-api.html)\n\nUpdates the validation info of the draft filtering rules."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_filtering_validation<'b>(
        &'a self,
        parts: ConnectorUpdateFilteringValidationParts<'b>,
    ) -> ConnectorUpdateFilteringValidation<'a, 'b, ()> {
        ConnectorUpdateFilteringValidation::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Index Name API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-index-name-api.html)\n\nUpdates the index name of the connector."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_index_name<'b>(
        &'a self,
        parts: ConnectorUpdateIndexNameParts<'b>,
    ) -> ConnectorUpdateIndexName<'a, 'b, ()> {
        ConnectorUpdateIndexName::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Name API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-name-description-api.html)\n\nUpdates the name and/or description fields in the connector document."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_name<'b>(
        &'a self,
        parts: ConnectorUpdateNameParts<'b>,
    ) -> ConnectorUpdateName<'a, 'b, ()> {
        ConnectorUpdateName::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Native API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/connector-apis.html)\n\nUpdates the is_native flag of the connector."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_native<'b>(
        &'a self,
        parts: ConnectorUpdateNativeParts<'b>,
    ) -> ConnectorUpdateNative<'a, 'b, ()> {
        ConnectorUpdateNative::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Pipeline API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-pipeline-api.html)\n\nUpdates the pipeline field in the connector document."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_pipeline<'b>(
        &'a self,
        parts: ConnectorUpdatePipelineParts<'b>,
    ) -> ConnectorUpdatePipeline<'a, 'b, ()> {
        ConnectorUpdatePipeline::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Scheduling API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-scheduling-api.html)\n\nUpdates the scheduling field in the connector document."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_scheduling<'b>(
        &'a self,
        parts: ConnectorUpdateSchedulingParts<'b>,
    ) -> ConnectorUpdateScheduling<'a, 'b, ()> {
        ConnectorUpdateScheduling::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Service Type API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-service-type-api.html)\n\nUpdates the service type of the connector."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_service_type<'b>(
        &'a self,
        parts: ConnectorUpdateServiceTypeParts<'b>,
    ) -> ConnectorUpdateServiceType<'a, 'b, ()> {
        ConnectorUpdateServiceType::new(self.transport(), parts)
    }
    #[doc = "[Connector Update Status API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/update-connector-status-api.html)\n\nUpdates the status of the connector."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn update_status<'b>(
        &'a self,
        parts: ConnectorUpdateStatusParts<'b>,
    ) -> ConnectorUpdateStatus<'a, 'b, ()> {
        ConnectorUpdateStatus::new(self.transport(), parts)
    }
}
#[cfg(feature = "experimental-apis")]
impl Elasticsearch {
    #[doc = "Creates a namespace client for Connector APIs"]
    pub fn connector(&self) -> Connector {
        Connector::new(self.transport())
    }
}
