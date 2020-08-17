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
#[doc = "API parts for the Cat Aliases API"]
pub enum CatAliasesParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> CatAliasesParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Aliases API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatAliasesParts::None => "/_cat/aliases".into(),
            CatAliasesParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_name.len());
                p.push_str("/_cat/aliases/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b [&'b str]> for CatAliasesParts<'b> {
    #[doc = "Builds a [CatAliasesParts::Name] for the Cat Aliases API"]
    fn from(t: &'b [&'b str]) -> CatAliasesParts<'b> {
        CatAliasesParts::Name(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Aliases API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-alias.html)\n\nShows information about currently configured aliases to indices including filter and routing infos."]
pub struct CatAliases<'a, 'b> {
    transport: &'a Transport,
    parts: CatAliasesParts<'b>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatAliases<'a, 'b> {
    #[doc = "Creates a new instance of [CatAliases] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatAliasesParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatAliases {
            transport,
            parts: parts.into(),
            headers,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Aliases API that can be awaited"]
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
                    rename = "expand_wildcards",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
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
#[doc = "API parts for the Cat Allocation API"]
pub enum CatAllocationParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "NodeId"]
    NodeId(&'b [&'b str]),
}
impl<'b> CatAllocationParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Allocation API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatAllocationParts::None => "/_cat/allocation".into(),
            CatAllocationParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_node_id.len());
                p.push_str("/_cat/allocation/");
                p.push_str(encoded_node_id.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b [&'b str]> for CatAllocationParts<'b> {
    #[doc = "Builds a [CatAllocationParts::NodeId] for the Cat Allocation API"]
    fn from(t: &'b [&'b str]) -> CatAllocationParts<'b> {
        CatAllocationParts::NodeId(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Allocation API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-allocation.html)\n\nProvides a snapshot of how many shards are allocated to each data node and how much disk space they are using."]
pub struct CatAllocation<'a, 'b> {
    transport: &'a Transport,
    parts: CatAllocationParts<'b>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatAllocation<'a, 'b> {
    #[doc = "Creates a new instance of [CatAllocation] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatAllocationParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatAllocation {
            transport,
            parts: parts.into(),
            headers,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Allocation API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
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
#[doc = "API parts for the Cat Count API"]
pub enum CatCountParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CatCountParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Count API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatCountParts::None => "/_cat/count".into(),
            CatCountParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_index.len());
                p.push_str("/_cat/count/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b [&'b str]> for CatCountParts<'b> {
    #[doc = "Builds a [CatCountParts::Index] for the Cat Count API"]
    fn from(t: &'b [&'b str]) -> CatCountParts<'b> {
        CatCountParts::Index(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Count API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-count.html)\n\nProvides quick access to the document count of the entire cluster, or individual indices."]
pub struct CatCount<'a, 'b> {
    transport: &'a Transport,
    parts: CatCountParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatCount<'a, 'b> {
    #[doc = "Creates a new instance of [CatCount] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatCountParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatCount {
            transport,
            parts: parts.into(),
            headers,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Count API that can be awaited"]
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
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
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
#[doc = "API parts for the Cat Fielddata API"]
pub enum CatFielddataParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Fields"]
    Fields(&'b [&'b str]),
}
impl<'b> CatFielddataParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Fielddata API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatFielddataParts::None => "/_cat/fielddata".into(),
            CatFielddataParts::Fields(ref fields) => {
                let fields_str = fields.join(",");
                let encoded_fields: Cow<str> =
                    percent_encode(fields_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_fields.len());
                p.push_str("/_cat/fielddata/");
                p.push_str(encoded_fields.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b [&'b str]> for CatFielddataParts<'b> {
    #[doc = "Builds a [CatFielddataParts::Fields] for the Cat Fielddata API"]
    fn from(t: &'b [&'b str]) -> CatFielddataParts<'b> {
        CatFielddataParts::Fields(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Fielddata API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-fielddata.html)\n\nShows how much heap memory is currently being used by fielddata on every data node in the cluster."]
pub struct CatFielddata<'a, 'b> {
    transport: &'a Transport,
    parts: CatFielddataParts<'b>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    fields: Option<&'b [&'b str]>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatFielddata<'a, 'b> {
    #[doc = "Creates a new instance of [CatFielddata] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatFielddataParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatFielddata {
            transport,
            parts: parts.into(),
            headers,
            bytes: None,
            error_trace: None,
            fields: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of fields to return in the output"]
    pub fn fields(mut self, fields: &'b [&'b str]) -> Self {
        self.fields = Some(fields);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Fielddata API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "fields", serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'b [&'b str]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                fields: self.fields,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
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
#[doc = "API parts for the Cat Health API"]
pub enum CatHealthParts {
    #[doc = "No parts"]
    None,
}
impl CatHealthParts {
    #[doc = "Builds a relative URL path to the Cat Health API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatHealthParts::None => "/_cat/health".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Health API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-health.html)\n\nReturns a concise representation of the cluster health."]
pub struct CatHealth<'a, 'b> {
    transport: &'a Transport,
    parts: CatHealthParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    ts: Option<bool>,
    v: Option<bool>,
}
impl<'a, 'b> CatHealth<'a, 'b> {
    #[doc = "Creates a new instance of [CatHealth]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatHealth {
            transport,
            parts: CatHealthParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            ts: None,
            v: None,
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Set to false to disable timestamping"]
    pub fn ts(mut self, ts: bool) -> Self {
        self.ts = Some(ts);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Health API that can be awaited"]
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
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "ts")]
                ts: Option<bool>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                ts: self.ts,
                v: self.v,
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
#[doc = "API parts for the Cat Help API"]
pub enum CatHelpParts {
    #[doc = "No parts"]
    None,
}
impl CatHelpParts {
    #[doc = "Builds a relative URL path to the Cat Help API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatHelpParts::None => "/_cat".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Help API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat.html)\n\nReturns help for the Cat APIs."]
pub struct CatHelp<'a, 'b> {
    transport: &'a Transport,
    parts: CatHelpParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
}
impl<'a, 'b> CatHelp<'a, 'b> {
    #[doc = "Creates a new instance of [CatHelp]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatHelp {
            transport,
            parts: CatHelpParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
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
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Help API that can be awaited"]
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
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
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
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Cat Indices API"]
pub enum CatIndicesParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CatIndicesParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Indices API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatIndicesParts::None => "/_cat/indices".into(),
            CatIndicesParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_index.len());
                p.push_str("/_cat/indices/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b [&'b str]> for CatIndicesParts<'b> {
    #[doc = "Builds a [CatIndicesParts::Index] for the Cat Indices API"]
    fn from(t: &'b [&'b str]) -> CatIndicesParts<'b> {
        CatIndicesParts::Index(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Indices API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-indices.html)\n\nReturns information about indices: number of primaries and replicas, document counts, disk size, ..."]
pub struct CatIndices<'a, 'b> {
    transport: &'a Transport,
    parts: CatIndicesParts<'b>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    health: Option<Health>,
    help: Option<bool>,
    human: Option<bool>,
    include_unloaded_segments: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    pri: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatIndices<'a, 'b> {
    #[doc = "Creates a new instance of [CatIndices] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatIndicesParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatIndices {
            transport,
            parts: parts.into(),
            headers,
            bytes: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            format: None,
            h: None,
            health: None,
            help: None,
            human: None,
            include_unloaded_segments: None,
            local: None,
            master_timeout: None,
            pretty: None,
            pri: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "A health status (\"green\", \"yellow\", or \"red\" to filter only indices matching the specified health status"]
    pub fn health(mut self, health: Health) -> Self {
        self.health = Some(health);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "If set to true segment stats will include stats for segments that are not currently loaded into memory"]
    pub fn include_unloaded_segments(mut self, include_unloaded_segments: bool) -> Self {
        self.include_unloaded_segments = Some(include_unloaded_segments);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Set to true to return stats only for primary shards"]
    pub fn pri(mut self, pri: bool) -> Self {
        self.pri = Some(pri);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Indices API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "expand_wildcards",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "health")]
                health: Option<Health>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "include_unloaded_segments")]
                include_unloaded_segments: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "pri")]
                pri: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                health: self.health,
                help: self.help,
                human: self.human,
                include_unloaded_segments: self.include_unloaded_segments,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                pri: self.pri,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
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
#[doc = "API parts for the Cat Master API"]
pub enum CatMasterParts {
    #[doc = "No parts"]
    None,
}
impl CatMasterParts {
    #[doc = "Builds a relative URL path to the Cat Master API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatMasterParts::None => "/_cat/master".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Master API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-master.html)\n\nReturns information about the master node."]
pub struct CatMaster<'a, 'b> {
    transport: &'a Transport,
    parts: CatMasterParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatMaster<'a, 'b> {
    #[doc = "Creates a new instance of [CatMaster]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatMaster {
            transport,
            parts: CatMasterParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Master API that can be awaited"]
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
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
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
#[doc = "API parts for the Cat Ml Data Frame Analytics API"]
pub enum CatMlDataFrameAnalyticsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> CatMlDataFrameAnalyticsParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Ml Data Frame Analytics API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatMlDataFrameAnalyticsParts::None => "/_cat/ml/data_frame/analytics".into(),
            CatMlDataFrameAnalyticsParts::Id(ref id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_id.len());
                p.push_str("/_cat/ml/data_frame/analytics/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CatMlDataFrameAnalyticsParts<'b> {
    #[doc = "Builds a [CatMlDataFrameAnalyticsParts::Id] for the Cat Ml Data Frame Analytics API"]
    fn from(t: &'b str) -> CatMlDataFrameAnalyticsParts<'b> {
        CatMlDataFrameAnalyticsParts::Id(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Ml Data Frame Analytics API](http://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-dfanalytics.html)\n\nGets configuration and usage information about data frame analytics jobs."]
pub struct CatMlDataFrameAnalytics<'a, 'b> {
    transport: &'a Transport,
    parts: CatMlDataFrameAnalyticsParts<'b>,
    allow_no_match: Option<bool>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatMlDataFrameAnalytics<'a, 'b> {
    #[doc = "Creates a new instance of [CatMlDataFrameAnalytics] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatMlDataFrameAnalyticsParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatMlDataFrameAnalytics {
            transport,
            parts: parts.into(),
            headers,
            allow_no_match: None,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no configs. (This includes `_all` string or when no configs have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: bool) -> Self {
        self.allow_no_match = Some(allow_no_match);
        self
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Ml Data Frame Analytics API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_match: self.allow_no_match,
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
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
#[doc = "API parts for the Cat Ml Datafeeds API"]
pub enum CatMlDatafeedsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "DatafeedId"]
    DatafeedId(&'b str),
}
impl<'b> CatMlDatafeedsParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Ml Datafeeds API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatMlDatafeedsParts::None => "/_cat/ml/datafeeds".into(),
            CatMlDatafeedsParts::DatafeedId(ref datafeed_id) => {
                let encoded_datafeed_id: Cow<str> =
                    percent_encode(datafeed_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_datafeed_id.len());
                p.push_str("/_cat/ml/datafeeds/");
                p.push_str(encoded_datafeed_id.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CatMlDatafeedsParts<'b> {
    #[doc = "Builds a [CatMlDatafeedsParts::DatafeedId] for the Cat Ml Datafeeds API"]
    fn from(t: &'b str) -> CatMlDatafeedsParts<'b> {
        CatMlDatafeedsParts::DatafeedId(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Ml Datafeeds API](http://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-datafeeds.html)\n\nGets configuration and usage information about datafeeds."]
pub struct CatMlDatafeeds<'a, 'b> {
    transport: &'a Transport,
    parts: CatMlDatafeedsParts<'b>,
    allow_no_datafeeds: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatMlDatafeeds<'a, 'b> {
    #[doc = "Creates a new instance of [CatMlDatafeeds] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatMlDatafeedsParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatMlDatafeeds {
            transport,
            parts: parts.into(),
            headers,
            allow_no_datafeeds: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no datafeeds. (This includes `_all` string or when no datafeeds have been specified)"]
    pub fn allow_no_datafeeds(mut self, allow_no_datafeeds: bool) -> Self {
        self.allow_no_datafeeds = Some(allow_no_datafeeds);
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Ml Datafeeds API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_datafeeds")]
                allow_no_datafeeds: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_datafeeds: self.allow_no_datafeeds,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
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
#[doc = "API parts for the Cat Ml Jobs API"]
pub enum CatMlJobsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "JobId"]
    JobId(&'b str),
}
impl<'b> CatMlJobsParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Ml Jobs API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatMlJobsParts::None => "/_cat/ml/anomaly_detectors".into(),
            CatMlJobsParts::JobId(ref job_id) => {
                let encoded_job_id: Cow<str> =
                    percent_encode(job_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_job_id.len());
                p.push_str("/_cat/ml/anomaly_detectors/");
                p.push_str(encoded_job_id.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CatMlJobsParts<'b> {
    #[doc = "Builds a [CatMlJobsParts::JobId] for the Cat Ml Jobs API"]
    fn from(t: &'b str) -> CatMlJobsParts<'b> {
        CatMlJobsParts::JobId(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Ml Jobs API](http://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-anomaly-detectors.html)\n\nGets configuration and usage information about anomaly detection jobs."]
pub struct CatMlJobs<'a, 'b> {
    transport: &'a Transport,
    parts: CatMlJobsParts<'b>,
    allow_no_jobs: Option<bool>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatMlJobs<'a, 'b> {
    #[doc = "Creates a new instance of [CatMlJobs] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatMlJobsParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatMlJobs {
            transport,
            parts: parts.into(),
            headers,
            allow_no_jobs: None,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: bool) -> Self {
        self.allow_no_jobs = Some(allow_no_jobs);
        self
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Ml Jobs API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_jobs")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_jobs: self.allow_no_jobs,
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
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
#[doc = "API parts for the Cat Ml Trained Models API"]
pub enum CatMlTrainedModelsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> CatMlTrainedModelsParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Ml Trained Models API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatMlTrainedModelsParts::None => "/_cat/ml/trained_models".into(),
            CatMlTrainedModelsParts::ModelId(ref model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_model_id.len());
                p.push_str("/_cat/ml/trained_models/");
                p.push_str(encoded_model_id.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CatMlTrainedModelsParts<'b> {
    #[doc = "Builds a [CatMlTrainedModelsParts::ModelId] for the Cat Ml Trained Models API"]
    fn from(t: &'b str) -> CatMlTrainedModelsParts<'b> {
        CatMlTrainedModelsParts::ModelId(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Ml Trained Models API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-trained-model.html)\n\nGets configuration and usage information about inference trained models."]
pub struct CatMlTrainedModels<'a, 'b> {
    transport: &'a Transport,
    parts: CatMlTrainedModelsParts<'b>,
    allow_no_match: Option<bool>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    from: Option<i32>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    size: Option<i32>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatMlTrainedModels<'a, 'b> {
    #[doc = "Creates a new instance of [CatMlTrainedModels] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatMlTrainedModelsParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatMlTrainedModels {
            transport,
            parts: parts.into(),
            headers,
            allow_no_match: None,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            from: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            size: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no trained models. (This includes `_all` string or when no trained models have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: bool) -> Self {
        self.allow_no_match = Some(allow_no_match);
        self
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "skips a number of trained models"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "specifies a max number of trained models to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Ml Trained Models API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_match: self.allow_no_match,
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                from: self.from,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                size: self.size,
                source: self.source,
                time: self.time,
                v: self.v,
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
#[doc = "API parts for the Cat Nodeattrs API"]
pub enum CatNodeattrsParts {
    #[doc = "No parts"]
    None,
}
impl CatNodeattrsParts {
    #[doc = "Builds a relative URL path to the Cat Nodeattrs API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatNodeattrsParts::None => "/_cat/nodeattrs".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Nodeattrs API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-nodeattrs.html)\n\nReturns information about custom node attributes."]
pub struct CatNodeattrs<'a, 'b> {
    transport: &'a Transport,
    parts: CatNodeattrsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatNodeattrs<'a, 'b> {
    #[doc = "Creates a new instance of [CatNodeattrs]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatNodeattrs {
            transport,
            parts: CatNodeattrsParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Nodeattrs API that can be awaited"]
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
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
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
#[doc = "API parts for the Cat Nodes API"]
pub enum CatNodesParts {
    #[doc = "No parts"]
    None,
}
impl CatNodesParts {
    #[doc = "Builds a relative URL path to the Cat Nodes API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatNodesParts::None => "/_cat/nodes".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Nodes API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-nodes.html)\n\nReturns basic statistics about performance of cluster nodes."]
pub struct CatNodes<'a, 'b> {
    transport: &'a Transport,
    parts: CatNodesParts,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    full_id: Option<bool>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatNodes<'a, 'b> {
    #[doc = "Creates a new instance of [CatNodes]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatNodes {
            transport,
            parts: CatNodesParts::None,
            headers,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            full_id: None,
            h: None,
            help: None,
            human: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Return the full node ID instead of the shortened version (default: false)"]
    pub fn full_id(mut self, full_id: bool) -> Self {
        self.full_id = Some(full_id);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Nodes API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "full_id")]
                full_id: Option<bool>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                full_id: self.full_id,
                h: self.h,
                help: self.help,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
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
#[doc = "API parts for the Cat Pending Tasks API"]
pub enum CatPendingTasksParts {
    #[doc = "No parts"]
    None,
}
impl CatPendingTasksParts {
    #[doc = "Builds a relative URL path to the Cat Pending Tasks API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatPendingTasksParts::None => "/_cat/pending_tasks".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Pending Tasks API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-pending-tasks.html)\n\nReturns a concise representation of the cluster pending tasks."]
pub struct CatPendingTasks<'a, 'b> {
    transport: &'a Transport,
    parts: CatPendingTasksParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatPendingTasks<'a, 'b> {
    #[doc = "Creates a new instance of [CatPendingTasks]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatPendingTasks {
            transport,
            parts: CatPendingTasksParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Pending Tasks API that can be awaited"]
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
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
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
#[doc = "API parts for the Cat Plugins API"]
pub enum CatPluginsParts {
    #[doc = "No parts"]
    None,
}
impl CatPluginsParts {
    #[doc = "Builds a relative URL path to the Cat Plugins API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatPluginsParts::None => "/_cat/plugins".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Plugins API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-plugins.html)\n\nReturns information about installed plugins across nodes node."]
pub struct CatPlugins<'a, 'b> {
    transport: &'a Transport,
    parts: CatPluginsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatPlugins<'a, 'b> {
    #[doc = "Creates a new instance of [CatPlugins]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatPlugins {
            transport,
            parts: CatPluginsParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Plugins API that can be awaited"]
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
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
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
#[doc = "API parts for the Cat Recovery API"]
pub enum CatRecoveryParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CatRecoveryParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Recovery API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatRecoveryParts::None => "/_cat/recovery".into(),
            CatRecoveryParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_index.len());
                p.push_str("/_cat/recovery/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b [&'b str]> for CatRecoveryParts<'b> {
    #[doc = "Builds a [CatRecoveryParts::Index] for the Cat Recovery API"]
    fn from(t: &'b [&'b str]) -> CatRecoveryParts<'b> {
        CatRecoveryParts::Index(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Recovery API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-recovery.html)\n\nReturns information about index shard recoveries, both on-going completed."]
pub struct CatRecovery<'a, 'b> {
    transport: &'a Transport,
    parts: CatRecoveryParts<'b>,
    active_only: Option<bool>,
    bytes: Option<Bytes>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    index: Option<&'b [&'b str]>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatRecovery<'a, 'b> {
    #[doc = "Creates a new instance of [CatRecovery] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatRecoveryParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatRecovery {
            transport,
            parts: parts.into(),
            headers,
            active_only: None,
            bytes: None,
            detailed: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            index: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "If `true`, the response only includes ongoing shard recoveries"]
    pub fn active_only(mut self, active_only: bool) -> Self {
        self.active_only = Some(active_only);
        self
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "If `true`, the response includes detailed information about shard recoveries"]
    pub fn detailed(mut self, detailed: bool) -> Self {
        self.detailed = Some(detailed);
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Comma-separated list or wildcard expression of index names to limit the returned information"]
    pub fn index(mut self, index: &'b [&'b str]) -> Self {
        self.index = Some(index);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Recovery API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "active_only")]
                active_only: Option<bool>,
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "detailed")]
                detailed: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "index", serialize_with = "crate::client::serialize_coll_qs")]
                index: Option<&'b [&'b str]>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                active_only: self.active_only,
                bytes: self.bytes,
                detailed: self.detailed,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                index: self.index,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
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
#[doc = "API parts for the Cat Repositories API"]
pub enum CatRepositoriesParts {
    #[doc = "No parts"]
    None,
}
impl CatRepositoriesParts {
    #[doc = "Builds a relative URL path to the Cat Repositories API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatRepositoriesParts::None => "/_cat/repositories".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Repositories API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-repositories.html)\n\nReturns information about snapshot repositories registered in the cluster."]
pub struct CatRepositories<'a, 'b> {
    transport: &'a Transport,
    parts: CatRepositoriesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatRepositories<'a, 'b> {
    #[doc = "Creates a new instance of [CatRepositories]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatRepositories {
            transport,
            parts: CatRepositoriesParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Repositories API that can be awaited"]
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
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
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
#[doc = "API parts for the Cat Segments API"]
pub enum CatSegmentsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CatSegmentsParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Segments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatSegmentsParts::None => "/_cat/segments".into(),
            CatSegmentsParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_index.len());
                p.push_str("/_cat/segments/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b [&'b str]> for CatSegmentsParts<'b> {
    #[doc = "Builds a [CatSegmentsParts::Index] for the Cat Segments API"]
    fn from(t: &'b [&'b str]) -> CatSegmentsParts<'b> {
        CatSegmentsParts::Index(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Segments API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-segments.html)\n\nProvides low-level information about the segments in the shards of an index."]
pub struct CatSegments<'a, 'b> {
    transport: &'a Transport,
    parts: CatSegmentsParts<'b>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatSegments<'a, 'b> {
    #[doc = "Creates a new instance of [CatSegments] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatSegmentsParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatSegments {
            transport,
            parts: parts.into(),
            headers,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Segments API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
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
#[doc = "API parts for the Cat Shards API"]
pub enum CatShardsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CatShardsParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Shards API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatShardsParts::None => "/_cat/shards".into(),
            CatShardsParts::Index(ref index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_index.len());
                p.push_str("/_cat/shards/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b [&'b str]> for CatShardsParts<'b> {
    #[doc = "Builds a [CatShardsParts::Index] for the Cat Shards API"]
    fn from(t: &'b [&'b str]) -> CatShardsParts<'b> {
        CatShardsParts::Index(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Shards API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-shards.html)\n\nProvides a detailed view of shard allocation on nodes."]
pub struct CatShards<'a, 'b> {
    transport: &'a Transport,
    parts: CatShardsParts<'b>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatShards<'a, 'b> {
    #[doc = "Creates a new instance of [CatShards] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatShardsParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatShards {
            transport,
            parts: parts.into(),
            headers,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Shards API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
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
#[doc = "API parts for the Cat Snapshots API"]
pub enum CatSnapshotsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Repository"]
    Repository(&'b [&'b str]),
}
impl<'b> CatSnapshotsParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Snapshots API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatSnapshotsParts::None => "/_cat/snapshots".into(),
            CatSnapshotsParts::Repository(ref repository) => {
                let repository_str = repository.join(",");
                let encoded_repository: Cow<str> =
                    percent_encode(repository_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_repository.len());
                p.push_str("/_cat/snapshots/");
                p.push_str(encoded_repository.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b [&'b str]> for CatSnapshotsParts<'b> {
    #[doc = "Builds a [CatSnapshotsParts::Repository] for the Cat Snapshots API"]
    fn from(t: &'b [&'b str]) -> CatSnapshotsParts<'b> {
        CatSnapshotsParts::Repository(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Snapshots API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-snapshots.html)\n\nReturns all snapshots in a specific repository."]
pub struct CatSnapshots<'a, 'b> {
    transport: &'a Transport,
    parts: CatSnapshotsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatSnapshots<'a, 'b> {
    #[doc = "Creates a new instance of [CatSnapshots] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatSnapshotsParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatSnapshots {
            transport,
            parts: parts.into(),
            headers,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Set to true to ignore unavailable snapshots"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Snapshots API that can be awaited"]
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
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
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
#[doc = "API parts for the Cat Tasks API"]
pub enum CatTasksParts {
    #[doc = "No parts"]
    None,
}
impl CatTasksParts {
    #[doc = "Builds a relative URL path to the Cat Tasks API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatTasksParts::None => "/_cat/tasks".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Tasks API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/tasks.html)\n\nReturns information about the tasks currently executing on one or more nodes in the cluster."]
pub struct CatTasks<'a, 'b> {
    transport: &'a Transport,
    parts: CatTasksParts,
    actions: Option<&'b [&'b str]>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    node_id: Option<&'b [&'b str]>,
    parent_task: Option<i64>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatTasks<'a, 'b> {
    #[doc = "Creates a new instance of [CatTasks]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatTasks {
            transport,
            parts: CatTasksParts::None,
            headers,
            actions: None,
            detailed: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            node_id: None,
            parent_task: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "A comma-separated list of actions that should be returned. Leave empty to return all."]
    pub fn actions(mut self, actions: &'b [&'b str]) -> Self {
        self.actions = Some(actions);
        self
    }
    #[doc = "Return detailed task information (default: false)"]
    pub fn detailed(mut self, detailed: bool) -> Self {
        self.detailed = Some(detailed);
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "A comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes"]
    pub fn node_id(mut self, node_id: &'b [&'b str]) -> Self {
        self.node_id = Some(node_id);
        self
    }
    #[doc = "Return tasks with specified parent task id. Set to -1 to return all."]
    pub fn parent_task(mut self, parent_task: i64) -> Self {
        self.parent_task = Some(parent_task);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Tasks API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(
                    rename = "actions",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                actions: Option<&'b [&'b str]>,
                #[serde(rename = "detailed")]
                detailed: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(
                    rename = "node_id",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                node_id: Option<&'b [&'b str]>,
                #[serde(rename = "parent_task")]
                parent_task: Option<i64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                actions: self.actions,
                detailed: self.detailed,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                node_id: self.node_id,
                parent_task: self.parent_task,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
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
#[doc = "API parts for the Cat Templates API"]
pub enum CatTemplatesParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> CatTemplatesParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Templates API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatTemplatesParts::None => "/_cat/templates".into(),
            CatTemplatesParts::Name(ref name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_name.len());
                p.push_str("/_cat/templates/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CatTemplatesParts<'b> {
    #[doc = "Builds a [CatTemplatesParts::Name] for the Cat Templates API"]
    fn from(t: &'b str) -> CatTemplatesParts<'b> {
        CatTemplatesParts::Name(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Templates API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-templates.html)\n\nReturns information about existing templates."]
pub struct CatTemplates<'a, 'b> {
    transport: &'a Transport,
    parts: CatTemplatesParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatTemplates<'a, 'b> {
    #[doc = "Creates a new instance of [CatTemplates] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatTemplatesParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatTemplates {
            transport,
            parts: parts.into(),
            headers,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Templates API that can be awaited"]
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
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
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
#[doc = "API parts for the Cat Thread Pool API"]
pub enum CatThreadPoolParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "ThreadPoolPatterns"]
    ThreadPoolPatterns(&'b [&'b str]),
}
impl<'b> CatThreadPoolParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Thread Pool API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatThreadPoolParts::None => "/_cat/thread_pool".into(),
            CatThreadPoolParts::ThreadPoolPatterns(ref thread_pool_patterns) => {
                let thread_pool_patterns_str = thread_pool_patterns.join(",");
                let encoded_thread_pool_patterns: Cow<str> =
                    percent_encode(thread_pool_patterns_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_thread_pool_patterns.len());
                p.push_str("/_cat/thread_pool/");
                p.push_str(encoded_thread_pool_patterns.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b [&'b str]> for CatThreadPoolParts<'b> {
    #[doc = "Builds a [CatThreadPoolParts::ThreadPoolPatterns] for the Cat Thread Pool API"]
    fn from(t: &'b [&'b str]) -> CatThreadPoolParts<'b> {
        CatThreadPoolParts::ThreadPoolPatterns(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Thread Pool API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-thread-pool.html)\n\nReturns cluster-wide thread pool statistics per node.\nBy default the active, queue and rejected statistics are returned for all thread pools."]
pub struct CatThreadPool<'a, 'b> {
    transport: &'a Transport,
    parts: CatThreadPoolParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatThreadPool<'a, 'b> {
    #[doc = "Creates a new instance of [CatThreadPool] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatThreadPoolParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatThreadPool {
            transport,
            parts: parts.into(),
            headers,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Thread Pool API that can be awaited"]
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
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
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
#[doc = "API parts for the Cat Transforms API"]
pub enum CatTransformsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "TransformId"]
    TransformId(&'b str),
}
impl<'b> CatTransformsParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Transforms API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatTransformsParts::None => "/_cat/transforms".into(),
            CatTransformsParts::TransformId(ref transform_id) => {
                let encoded_transform_id: Cow<str> =
                    percent_encode(transform_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_transform_id.len());
                p.push_str("/_cat/transforms/");
                p.push_str(encoded_transform_id.as_ref());
                p.into()
            }
        }
    }
}
impl<'b> From<&'b str> for CatTransformsParts<'b> {
    #[doc = "Builds a [CatTransformsParts::TransformId] for the Cat Transforms API"]
    fn from(t: &'b str) -> CatTransformsParts<'b> {
        CatTransformsParts::TransformId(t)
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Cat Transforms API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-transforms.html)\n\nGets configuration and usage information about transforms."]
pub struct CatTransforms<'a, 'b> {
    transport: &'a Transport,
    parts: CatTransformsParts<'b>,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    from: Option<i32>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'b [&'b str]>,
    size: Option<i32>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatTransforms<'a, 'b> {
    #[doc = "Creates a new instance of [CatTransforms] with the specified API parts"]
    pub fn new<P>(transport: &'a Transport, parts: P) -> Self
    where
        P: Into<CatTransformsParts<'b>>,
    {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatTransforms {
            transport,
            parts: parts.into(),
            headers,
            allow_no_match: None,
            error_trace: None,
            filter_path: None,
            format: None,
            from: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            size: None,
            source: None,
            time: None,
            v: None,
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
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "skips a number of transform configs, defaults to 0"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
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
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Transforms API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "format")]
                format: Option<&'b str>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_match: self.allow_no_match,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                from: self.from,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                size: self.size,
                source: self.source,
                time: self.time,
                v: self.v,
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
#[doc = "Namespace client for Cat APIs"]
pub struct Cat<'a> {
    transport: &'a Transport,
}
impl<'a> Cat<'a> {
    #[doc = "Creates a new instance of [Cat]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Cat Aliases API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-alias.html)\n\nShows information about currently configured aliases to indices including filter and routing infos."]
    pub fn aliases<'b, P>(&'a self, parts: P) -> CatAliases<'a, 'b>
    where
        P: Into<CatAliasesParts<'b>>,
    {
        CatAliases::new(self.transport(), parts)
    }
    #[doc = "[Cat Allocation API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-allocation.html)\n\nProvides a snapshot of how many shards are allocated to each data node and how much disk space they are using."]
    pub fn allocation<'b, P>(&'a self, parts: P) -> CatAllocation<'a, 'b>
    where
        P: Into<CatAllocationParts<'b>>,
    {
        CatAllocation::new(self.transport(), parts)
    }
    #[doc = "[Cat Count API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-count.html)\n\nProvides quick access to the document count of the entire cluster, or individual indices."]
    pub fn count<'b, P>(&'a self, parts: P) -> CatCount<'a, 'b>
    where
        P: Into<CatCountParts<'b>>,
    {
        CatCount::new(self.transport(), parts)
    }
    #[doc = "[Cat Fielddata API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-fielddata.html)\n\nShows how much heap memory is currently being used by fielddata on every data node in the cluster."]
    pub fn fielddata<'b, P>(&'a self, parts: P) -> CatFielddata<'a, 'b>
    where
        P: Into<CatFielddataParts<'b>>,
    {
        CatFielddata::new(self.transport(), parts)
    }
    #[doc = "[Cat Health API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-health.html)\n\nReturns a concise representation of the cluster health."]
    pub fn health<'b>(&'a self) -> CatHealth<'a, 'b> {
        CatHealth::new(self.transport())
    }
    #[doc = "[Cat Help API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat.html)\n\nReturns help for the Cat APIs."]
    pub fn help<'b>(&'a self) -> CatHelp<'a, 'b> {
        CatHelp::new(self.transport())
    }
    #[doc = "[Cat Indices API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-indices.html)\n\nReturns information about indices: number of primaries and replicas, document counts, disk size, ..."]
    pub fn indices<'b, P>(&'a self, parts: P) -> CatIndices<'a, 'b>
    where
        P: Into<CatIndicesParts<'b>>,
    {
        CatIndices::new(self.transport(), parts)
    }
    #[doc = "[Cat Master API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-master.html)\n\nReturns information about the master node."]
    pub fn master<'b>(&'a self) -> CatMaster<'a, 'b> {
        CatMaster::new(self.transport())
    }
    #[doc = "[Cat Ml Data Frame Analytics API](http://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-dfanalytics.html)\n\nGets configuration and usage information about data frame analytics jobs."]
    pub fn ml_data_frame_analytics<'b, P>(&'a self, parts: P) -> CatMlDataFrameAnalytics<'a, 'b>
    where
        P: Into<CatMlDataFrameAnalyticsParts<'b>>,
    {
        CatMlDataFrameAnalytics::new(self.transport(), parts)
    }
    #[doc = "[Cat Ml Datafeeds API](http://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-datafeeds.html)\n\nGets configuration and usage information about datafeeds."]
    pub fn ml_datafeeds<'b, P>(&'a self, parts: P) -> CatMlDatafeeds<'a, 'b>
    where
        P: Into<CatMlDatafeedsParts<'b>>,
    {
        CatMlDatafeeds::new(self.transport(), parts)
    }
    #[doc = "[Cat Ml Jobs API](http://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-anomaly-detectors.html)\n\nGets configuration and usage information about anomaly detection jobs."]
    pub fn ml_jobs<'b, P>(&'a self, parts: P) -> CatMlJobs<'a, 'b>
    where
        P: Into<CatMlJobsParts<'b>>,
    {
        CatMlJobs::new(self.transport(), parts)
    }
    #[doc = "[Cat Ml Trained Models API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-trained-model.html)\n\nGets configuration and usage information about inference trained models."]
    pub fn ml_trained_models<'b, P>(&'a self, parts: P) -> CatMlTrainedModels<'a, 'b>
    where
        P: Into<CatMlTrainedModelsParts<'b>>,
    {
        CatMlTrainedModels::new(self.transport(), parts)
    }
    #[doc = "[Cat Nodeattrs API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-nodeattrs.html)\n\nReturns information about custom node attributes."]
    pub fn nodeattrs<'b>(&'a self) -> CatNodeattrs<'a, 'b> {
        CatNodeattrs::new(self.transport())
    }
    #[doc = "[Cat Nodes API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-nodes.html)\n\nReturns basic statistics about performance of cluster nodes."]
    pub fn nodes<'b>(&'a self) -> CatNodes<'a, 'b> {
        CatNodes::new(self.transport())
    }
    #[doc = "[Cat Pending Tasks API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-pending-tasks.html)\n\nReturns a concise representation of the cluster pending tasks."]
    pub fn pending_tasks<'b>(&'a self) -> CatPendingTasks<'a, 'b> {
        CatPendingTasks::new(self.transport())
    }
    #[doc = "[Cat Plugins API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-plugins.html)\n\nReturns information about installed plugins across nodes node."]
    pub fn plugins<'b>(&'a self) -> CatPlugins<'a, 'b> {
        CatPlugins::new(self.transport())
    }
    #[doc = "[Cat Recovery API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-recovery.html)\n\nReturns information about index shard recoveries, both on-going completed."]
    pub fn recovery<'b, P>(&'a self, parts: P) -> CatRecovery<'a, 'b>
    where
        P: Into<CatRecoveryParts<'b>>,
    {
        CatRecovery::new(self.transport(), parts)
    }
    #[doc = "[Cat Repositories API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-repositories.html)\n\nReturns information about snapshot repositories registered in the cluster."]
    pub fn repositories<'b>(&'a self) -> CatRepositories<'a, 'b> {
        CatRepositories::new(self.transport())
    }
    #[doc = "[Cat Segments API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-segments.html)\n\nProvides low-level information about the segments in the shards of an index."]
    pub fn segments<'b, P>(&'a self, parts: P) -> CatSegments<'a, 'b>
    where
        P: Into<CatSegmentsParts<'b>>,
    {
        CatSegments::new(self.transport(), parts)
    }
    #[doc = "[Cat Shards API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-shards.html)\n\nProvides a detailed view of shard allocation on nodes."]
    pub fn shards<'b, P>(&'a self, parts: P) -> CatShards<'a, 'b>
    where
        P: Into<CatShardsParts<'b>>,
    {
        CatShards::new(self.transport(), parts)
    }
    #[doc = "[Cat Snapshots API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-snapshots.html)\n\nReturns all snapshots in a specific repository."]
    pub fn snapshots<'b, P>(&'a self, parts: P) -> CatSnapshots<'a, 'b>
    where
        P: Into<CatSnapshotsParts<'b>>,
    {
        CatSnapshots::new(self.transport(), parts)
    }
    #[doc = "[Cat Tasks API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/tasks.html)\n\nReturns information about the tasks currently executing on one or more nodes in the cluster."]
    pub fn tasks<'b>(&'a self) -> CatTasks<'a, 'b> {
        CatTasks::new(self.transport())
    }
    #[doc = "[Cat Templates API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-templates.html)\n\nReturns information about existing templates."]
    pub fn templates<'b, P>(&'a self, parts: P) -> CatTemplates<'a, 'b>
    where
        P: Into<CatTemplatesParts<'b>>,
    {
        CatTemplates::new(self.transport(), parts)
    }
    #[doc = "[Cat Thread Pool API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-thread-pool.html)\n\nReturns cluster-wide thread pool statistics per node.\nBy default the active, queue and rejected statistics are returned for all thread pools."]
    pub fn thread_pool<'b, P>(&'a self, parts: P) -> CatThreadPool<'a, 'b>
    where
        P: Into<CatThreadPoolParts<'b>>,
    {
        CatThreadPool::new(self.transport(), parts)
    }
    #[doc = "[Cat Transforms API](https://www.elastic.co/guide/en/elasticsearch/reference/8.0/cat-transforms.html)\n\nGets configuration and usage information about transforms."]
    pub fn transforms<'b, P>(&'a self, parts: P) -> CatTransforms<'a, 'b>
    where
        P: Into<CatTransformsParts<'b>>,
    {
        CatTransforms::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Cat APIs"]
    pub fn cat(&self) -> Cat {
        Cat::new(self.transport())
    }
}
