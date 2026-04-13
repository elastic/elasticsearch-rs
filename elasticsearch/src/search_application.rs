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

//! Search Application APIs
//!
//! Use Search Application APIs to manage tasks and resources related to Search Applications.

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
#[doc = "API parts for the Search Application Delete API"]
pub enum SearchApplicationDeleteParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> SearchApplicationDeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Search Application Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchApplicationDeleteParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_name.len());
                p.push_str("/_application/search_application/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Search Application Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/delete-search-application.html)\n\nDeletes a search application."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct SearchApplicationDelete<'a, 'b> {
    transport: &'a Transport,
    parts: SearchApplicationDeleteParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> SearchApplicationDelete<'a, 'b> {
    #[doc = "Creates a new instance of [SearchApplicationDelete] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchApplicationDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchApplicationDelete {
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
    #[doc = "Creates an asynchronous call to the Search Application Delete API that can be awaited"]
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
#[doc = "API parts for the Search Application Get API"]
pub enum SearchApplicationGetParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> SearchApplicationGetParts<'b> {
    #[doc = "Builds a relative URL path to the Search Application Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchApplicationGetParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_name.len());
                p.push_str("/_application/search_application/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Search Application Get API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/get-search-application.html)\n\nReturns the details about a search application."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct SearchApplicationGet<'a, 'b> {
    transport: &'a Transport,
    parts: SearchApplicationGetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> SearchApplicationGet<'a, 'b> {
    #[doc = "Creates a new instance of [SearchApplicationGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchApplicationGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchApplicationGet {
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
    #[doc = "Creates an asynchronous call to the Search Application Get API that can be awaited"]
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
#[doc = "API parts for the Search Application List API"]
pub enum SearchApplicationListParts {
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl SearchApplicationListParts {
    #[doc = "Builds a relative URL path to the Search Application List API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchApplicationListParts::None => "/_application/search_application".into(),
        }
    }
}
#[doc = "Builder for the [Search Application List API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/list-search-applications.html)\n\nReturns the existing search applications."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct SearchApplicationList<'a, 'b> {
    transport: &'a Transport,
    parts: SearchApplicationListParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    request_timeout: Option<Duration>,
    size: Option<i32>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> SearchApplicationList<'a, 'b> {
    #[doc = "Creates a new instance of [SearchApplicationList]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SearchApplicationList {
            transport,
            parts: SearchApplicationListParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            q: None,
            request_timeout: None,
            size: None,
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
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: &'b str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "specifies a max number of results to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Search Application List API that can be awaited"]
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
                from: Option<i32>,
                human: Option<bool>,
                pretty: Option<bool>,
                q: Option<&'b str>,
                size: Option<i32>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                q: self.q,
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
#[doc = "API parts for the Search Application Put API"]
pub enum SearchApplicationPutParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> SearchApplicationPutParts<'b> {
    #[doc = "Builds a relative URL path to the Search Application Put API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchApplicationPutParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_name.len());
                p.push_str("/_application/search_application/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Search Application Put API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/put-search-application.html)\n\nCreates or updates a search application."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct SearchApplicationPut<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchApplicationPutParts<'b>,
    body: Option<B>,
    create: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b, B> SearchApplicationPut<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchApplicationPut] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchApplicationPutParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchApplicationPut {
            transport,
            parts,
            headers,
            body: None,
            create: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SearchApplicationPut<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchApplicationPut {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            create: self.create,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "If true, requires that a search application with the specified resource_id does not already exist. (default: false)"]
    pub fn create(mut self, create: bool) -> Self {
        self.create = Some(create);
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
    #[doc = "Creates an asynchronous call to the Search Application Put API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                create: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                create: self.create,
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
#[doc = "API parts for the Search Application Render Query API"]
pub enum SearchApplicationRenderQueryParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> SearchApplicationRenderQueryParts<'b> {
    #[doc = "Builds a relative URL path to the Search Application Render Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchApplicationRenderQueryParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(47usize + encoded_name.len());
                p.push_str("/_application/search_application/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/_render_query");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Search Application Render Query API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/search-application-render-query.html)\n\nRenders a query for given search application search parameters"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct SearchApplicationRenderQuery<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchApplicationRenderQueryParts<'b>,
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
impl<'a, 'b, B> SearchApplicationRenderQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchApplicationRenderQuery] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchApplicationRenderQueryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchApplicationRenderQuery {
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
    pub fn body<T>(self, body: T) -> SearchApplicationRenderQuery<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchApplicationRenderQuery {
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
    #[doc = "Creates an asynchronous call to the Search Application Render Query API that can be awaited"]
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
#[doc = "API parts for the Search Application Search API"]
pub enum SearchApplicationSearchParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> SearchApplicationSearchParts<'b> {
    #[doc = "Builds a relative URL path to the Search Application Search API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchApplicationSearchParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(41usize + encoded_name.len());
                p.push_str("/_application/search_application/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/_search");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Search Application Search API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/search-application-search.html)\n\nPerform a search against a search application"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct SearchApplicationSearch<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchApplicationSearchParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    typed_keys: Option<bool>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b, B> SearchApplicationSearch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchApplicationSearch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchApplicationSearchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchApplicationSearch {
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
            typed_keys: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SearchApplicationSearch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchApplicationSearch {
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
    #[doc = "Creates an asynchronous call to the Search Application Search API that can be awaited"]
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
                typed_keys: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
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
#[doc = "Namespace client for SearchApplication APIs"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
pub struct SearchApplication<'a> {
    transport: &'a Transport,
}
#[cfg(feature = "experimental-apis")]
impl<'a> SearchApplication<'a> {
    #[doc = "Creates a new instance of [SearchApplication]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Search Application Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/delete-search-application.html)\n\nDeletes a search application."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn delete<'b>(
        &'a self,
        parts: SearchApplicationDeleteParts<'b>,
    ) -> SearchApplicationDelete<'a, 'b> {
        SearchApplicationDelete::new(self.transport(), parts)
    }
    #[doc = "[Search Application Get API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/get-search-application.html)\n\nReturns the details about a search application."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn get<'b>(&'a self, parts: SearchApplicationGetParts<'b>) -> SearchApplicationGet<'a, 'b> {
        SearchApplicationGet::new(self.transport(), parts)
    }
    #[doc = "[Search Application List API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/list-search-applications.html)\n\nReturns the existing search applications."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn list<'b>(&'a self) -> SearchApplicationList<'a, 'b> {
        SearchApplicationList::new(self.transport())
    }
    #[doc = "[Search Application Put API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/put-search-application.html)\n\nCreates or updates a search application."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn put<'b>(
        &'a self,
        parts: SearchApplicationPutParts<'b>,
    ) -> SearchApplicationPut<'a, 'b, ()> {
        SearchApplicationPut::new(self.transport(), parts)
    }
    #[doc = "[Search Application Render Query API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/search-application-render-query.html)\n\nRenders a query for given search application search parameters"]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn render_query<'b>(
        &'a self,
        parts: SearchApplicationRenderQueryParts<'b>,
    ) -> SearchApplicationRenderQuery<'a, 'b, ()> {
        SearchApplicationRenderQuery::new(self.transport(), parts)
    }
    #[doc = "[Search Application Search API](https://www.elastic.co/guide/en/elasticsearch/reference/9.1/search-application-search.html)\n\nPerform a search against a search application"]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn search<'b>(
        &'a self,
        parts: SearchApplicationSearchParts<'b>,
    ) -> SearchApplicationSearch<'a, 'b, ()> {
        SearchApplicationSearch::new(self.transport(), parts)
    }
}
#[cfg(feature = "experimental-apis")]
impl Elasticsearch {
    #[doc = "Creates a namespace client for SearchApplication APIs"]
    pub fn search_application(&self) -> SearchApplication<'_> {
        SearchApplication::new(self.transport())
    }
}
