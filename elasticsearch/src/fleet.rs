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
#[doc = "API parts for the Fleet Global Checkpoints API"]
pub enum FleetGlobalCheckpointsParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> FleetGlobalCheckpointsParts<'b> {
    #[doc = "Builds a relative URL path to the Fleet Global Checkpoints API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FleetGlobalCheckpointsParts::Index(ref index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_index.len());
                p.push_str("/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_fleet/global_checkpoints");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Fleet Global Checkpoints API\n\nReturns the current global checkpoints for an index. This API is design for internal use by the fleet server project."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
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
#[cfg(feature = "experimental-apis")]
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
        let method = Method::Get;
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
#[doc = "Namespace client for Fleet APIs"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
pub struct Fleet<'a> {
    transport: &'a Transport,
}
#[cfg(feature = "experimental-apis")]
impl<'a> Fleet<'a> {
    #[doc = "Creates a new instance of [Fleet]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "Fleet Global Checkpoints API\n\nReturns the current global checkpoints for an index. This API is design for internal use by the fleet server project."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn global_checkpoints<'b>(
        &'a self,
        parts: FleetGlobalCheckpointsParts<'b>,
    ) -> FleetGlobalCheckpoints<'a, 'b> {
        FleetGlobalCheckpoints::new(self.transport(), parts)
    }
}
#[cfg(feature = "experimental-apis")]
impl Elasticsearch {
    #[doc = "Creates a namespace client for Fleet APIs"]
    pub fn fleet(&self) -> Fleet {
        Fleet::new(self.transport())
    }
}
