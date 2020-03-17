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
        transport::Transport,
        Method,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Dangling Indices Delete Dangling Index API"]
pub enum DanglingIndicesDeleteDanglingIndexParts<'b> {
    #[doc = "IndexUuid"]
    IndexUuid(&'b str),
}
impl<'b> DanglingIndicesDeleteDanglingIndexParts<'b> {
    #[doc = "Builds a relative URL path to the Dangling Indices Delete Dangling Index API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DanglingIndicesDeleteDanglingIndexParts::IndexUuid(ref index_uuid) => {
                let encoded_index_uuid: Cow<str> =
                    percent_encode(index_uuid.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_index_uuid.len());
                p.push_str("/_dangling/");
                p.push_str(encoded_index_uuid.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for DanglingIndicesDeleteDanglingIndexParts<'b> {
    #[doc = "Builds a [DanglingIndicesDeleteDanglingIndexParts::IndexUuid] for the Dangling Indices Delete Dangling Index API"]
    fn from(t: &'b str) -> DanglingIndicesDeleteDanglingIndexParts<'b> {
        DanglingIndicesDeleteDanglingIndexParts::IndexUuid(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Dangling Indices Delete Dangling Index API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/modules-gateway-dangling-indices.html)\n\nDeletes the specified dangling index"]
pub struct DanglingIndicesDeleteDanglingIndex<'a, 'b> {
    transport: &'a Transport,
    parts: DanglingIndicesDeleteDanglingIndexParts<'b>,
    accept_data_loss: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> DanglingIndicesDeleteDanglingIndex<'a, 'b> {
    #[doc = "Creates a new instance of [DanglingIndicesDeleteDanglingIndex] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<DanglingIndicesDeleteDanglingIndexParts<'b>>,
    {
        let headers = HeaderMap::new();
        DanglingIndicesDeleteDanglingIndex {
            transport,
            parts: parts.into(),
            headers,
            accept_data_loss: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Must be set to true in order to delete the dangling index"]
    pub fn accept_data_loss(mut self, accept_data_loss: bool) -> Self {
        self.accept_data_loss = Some(accept_data_loss);
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Dangling Indices Delete Dangling Index API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "accept_data_loss")]
                accept_data_loss: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                accept_data_loss: self.accept_data_loss,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Dangling Indices Import Dangling Index API"]
pub enum DanglingIndicesImportDanglingIndexParts<'b> {
    #[doc = "IndexUuid"]
    IndexUuid(&'b str),
}
impl<'b> DanglingIndicesImportDanglingIndexParts<'b> {
    #[doc = "Builds a relative URL path to the Dangling Indices Import Dangling Index API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DanglingIndicesImportDanglingIndexParts::IndexUuid(ref index_uuid) => {
                let encoded_index_uuid: Cow<str> =
                    percent_encode(index_uuid.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_index_uuid.len());
                p.push_str("/_dangling/");
                p.push_str(encoded_index_uuid.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for DanglingIndicesImportDanglingIndexParts<'b> {
    #[doc = "Builds a [DanglingIndicesImportDanglingIndexParts::IndexUuid] for the Dangling Indices Import Dangling Index API"]
    fn from(t: &'b str) -> DanglingIndicesImportDanglingIndexParts<'b> {
        DanglingIndicesImportDanglingIndexParts::IndexUuid(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Dangling Indices Import Dangling Index API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/modules-gateway-dangling-indices.html)\n\nImports the specified dangling index"]
pub struct DanglingIndicesImportDanglingIndex<'a, 'b, B> {
    transport: &'a Transport,
    parts: DanglingIndicesImportDanglingIndexParts<'b>,
    accept_data_loss: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> DanglingIndicesImportDanglingIndex<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [DanglingIndicesImportDanglingIndex] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<DanglingIndicesImportDanglingIndexParts<'b>>,
    {
        let headers = HeaderMap::new();
        DanglingIndicesImportDanglingIndex {
            transport,
            parts: parts.into(),
            headers,
            accept_data_loss: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Must be set to true in order to import the dangling index"]
    pub fn accept_data_loss(mut self, accept_data_loss: bool) -> Self {
        self.accept_data_loss = Some(accept_data_loss);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> DanglingIndicesImportDanglingIndex<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        DanglingIndicesImportDanglingIndex {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            accept_data_loss: self.accept_data_loss,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Dangling Indices Import Dangling Index API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "accept_data_loss")]
                accept_data_loss: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                accept_data_loss: self.accept_data_loss,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Dangling Indices List Dangling Indices API"]
pub enum DanglingIndicesListDanglingIndicesParts {
    #[doc = "No parts"]
    None,
}
impl DanglingIndicesListDanglingIndicesParts {
    #[doc = "Builds a relative URL path to the Dangling Indices List Dangling Indices API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DanglingIndicesListDanglingIndicesParts::None => "/_dangling".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Dangling Indices List Dangling Indices API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/modules-gateway-dangling-indices.html)\n\nReturns all dangling indices."]
pub struct DanglingIndicesListDanglingIndices<'a, 'b> {
    transport: &'a Transport,
    parts: DanglingIndicesListDanglingIndicesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> DanglingIndicesListDanglingIndices<'a, 'b> {
    #[doc = "Creates a new instance of [DanglingIndicesListDanglingIndices]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        DanglingIndicesListDanglingIndices {
            transport,
            parts: DanglingIndicesListDanglingIndicesParts::None,
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
    #[doc = "Creates an asynchronous call to the Dangling Indices List Dangling Indices API that can be awaited"]
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for DanglingIndices APIs"]
pub struct DanglingIndices<'a> {
    transport: &'a Transport,
}
impl<'a> DanglingIndices<'a> {
    #[doc = "Creates a new instance of [DanglingIndices]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Dangling Indices Delete Dangling Index API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/modules-gateway-dangling-indices.html)\n\nDeletes the specified dangling index"]
    pub fn delete_dangling_index<'b, P>(
        &'a self,
        parts: P,
    ) -> DanglingIndicesDeleteDanglingIndex<'a, 'b>
    where
        P: Into<DanglingIndicesDeleteDanglingIndexParts<'b>>,
    {
        DanglingIndicesDeleteDanglingIndex::new(self.transport(), parts)
    }
    #[doc = "[Dangling Indices Import Dangling Index API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/modules-gateway-dangling-indices.html)\n\nImports the specified dangling index"]
    pub fn import_dangling_index<'b, P>(
        &'a self,
        parts: P,
    ) -> DanglingIndicesImportDanglingIndex<'a, 'b, ()>
    where
        P: Into<DanglingIndicesImportDanglingIndexParts<'b>>,
    {
        DanglingIndicesImportDanglingIndex::new(self.transport(), parts)
    }
    #[doc = "[Dangling Indices List Dangling Indices API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/modules-gateway-dangling-indices.html)\n\nReturns all dangling indices."]
    pub fn list_dangling_indices<'b>(&'a self) -> DanglingIndicesListDanglingIndices<'a, 'b> {
        DanglingIndicesListDanglingIndices::new(self.transport())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for DanglingIndices APIs"]
    pub fn dangling_indices(&self) -> DanglingIndices {
        DanglingIndices::new(self.transport())
    }
}
