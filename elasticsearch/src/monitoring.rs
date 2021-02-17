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

//! Monitoring APIs
//!
//! The Elastic Stack [monitoring features](https://www.elastic.co/guide/en/elasticsearch/reference/master/monitor-elasticsearch-cluster.html)
//! provide a way to keep a pulse on thehealth and performance of your Elasticsearch cluster.

#![cfg(feature = "experimental-apis")]
#![doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
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
#[doc = "API parts for the Monitoring Bulk API"]
pub enum MonitoringBulkParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Type"]
    Type(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> MonitoringBulkParts<'b> {
    #[doc = "Builds a relative URL path to the Monitoring Bulk API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MonitoringBulkParts::None => "/_monitoring/bulk".into(),
            MonitoringBulkParts::Type(ref ty) => {
                let encoded_ty: Cow<str> = percent_encode(ty.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_ty.len());
                p.push_str("/_monitoring/");
                p.push_str(encoded_ty.as_ref());
                p.push_str("/bulk");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Monitoring Bulk API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/monitor-elasticsearch-cluster.html)\n\nUsed by the monitoring features to send monitoring data."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct MonitoringBulk<'a, 'b, B> {
    transport: &'a Transport,
    parts: MonitoringBulkParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    interval: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    system_api_version: Option<&'b str>,
    system_id: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b, B> MonitoringBulk<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MonitoringBulk] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MonitoringBulkParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MonitoringBulk {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            interval: None,
            pretty: None,
            request_timeout: None,
            source: None,
            system_api_version: None,
            system_id: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: Vec<T>) -> MonitoringBulk<'a, 'b, NdBody<T>>
    where
        T: Body,
    {
        MonitoringBulk {
            transport: self.transport,
            parts: self.parts,
            body: Some(NdBody(body)),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            interval: self.interval,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            system_api_version: self.system_api_version,
            system_id: self.system_id,
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
    #[doc = "Collection interval (e.g., '10s' or '10000ms') of the payload"]
    pub fn interval(mut self, interval: &'b str) -> Self {
        self.interval = Some(interval);
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
    #[doc = "API Version of the monitored system"]
    pub fn system_api_version(mut self, system_api_version: &'b str) -> Self {
        self.system_api_version = Some(system_api_version);
        self
    }
    #[doc = "Identifier of the monitored system"]
    pub fn system_id(mut self, system_id: &'b str) -> Self {
        self.system_id = Some(system_id);
        self
    }
    #[doc = "Creates an asynchronous call to the Monitoring Bulk API that can be awaited"]
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
                interval: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                system_api_version: Option<&'b str>,
                system_id: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                interval: self.interval,
                pretty: self.pretty,
                source: self.source,
                system_api_version: self.system_api_version,
                system_id: self.system_id,
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
#[doc = "Namespace client for Monitoring APIs"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
pub struct Monitoring<'a> {
    transport: &'a Transport,
}
#[cfg(feature = "experimental-apis")]
impl<'a> Monitoring<'a> {
    #[doc = "Creates a new instance of [Monitoring]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Monitoring Bulk API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/monitor-elasticsearch-cluster.html)\n\nUsed by the monitoring features to send monitoring data."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn bulk<'b>(&'a self, parts: MonitoringBulkParts<'b>) -> MonitoringBulk<'a, 'b, ()> {
        MonitoringBulk::new(self.transport(), parts)
    }
}
#[cfg(feature = "experimental-apis")]
impl Elasticsearch {
    #[doc = "Creates a namespace client for Monitoring APIs"]
    pub fn monitoring(&self) -> Monitoring {
        Monitoring::new(self.transport())
    }
}
