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

//! Synonyms APIs
//!
//! The synonyms management API provides a convenient way to define and manage synonyms in an internal system index. Related synonyms can be grouped in a "synonyms set". Create as many synonym sets as you need.
//!
//! This provides an alternative to:
//!
//! * Defining inline synonyms in an analyzer definition, which impacts mapping size and can lead to performance issues.
//! * Using synonyms files, which implies uploading and managing file consistency on all cluster nodes.
//!
//! Synonyms sets can be used to configure synonym graph token filters and synonym token filters. These filters are applied as part of the analysis process by the search analyzer.
//!

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
#[doc = "API parts for the Synonyms Delete Synonym API"]
pub enum SynonymsDeleteSynonymParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> SynonymsDeleteSynonymParts<'b> {
    #[doc = "Builds a relative URL path to the Synonyms Delete Synonym API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SynonymsDeleteSynonymParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_id.len());
                p.push_str("/_synonyms/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Synonyms Delete Synonym API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-synonyms-set.html)\n\nDeletes a synonym set"]
#[derive(Clone, Debug)]
pub struct SynonymsDeleteSynonym<'a, 'b> {
    transport: &'a Transport,
    parts: SynonymsDeleteSynonymParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SynonymsDeleteSynonym<'a, 'b> {
    #[doc = "Creates a new instance of [SynonymsDeleteSynonym] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SynonymsDeleteSynonymParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SynonymsDeleteSynonym {
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
    #[doc = "Creates an asynchronous call to the Synonyms Delete Synonym API that can be awaited"]
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Synonyms Delete Synonym Rule API"]
pub enum SynonymsDeleteSynonymRuleParts<'b> {
    #[doc = "SetId and RuleId"]
    SetIdRuleId(&'b str, &'b str),
}
impl<'b> SynonymsDeleteSynonymRuleParts<'b> {
    #[doc = "Builds a relative URL path to the Synonyms Delete Synonym Rule API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SynonymsDeleteSynonymRuleParts::SetIdRuleId(set_id, rule_id) => {
                let encoded_set_id: Cow<str> =
                    percent_encode(set_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_rule_id: Cow<str> =
                    percent_encode(rule_id.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(12usize + encoded_set_id.len() + encoded_rule_id.len());
                p.push_str("/_synonyms/");
                p.push_str(encoded_set_id.as_ref());
                p.push('/');
                p.push_str(encoded_rule_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Synonyms Delete Synonym Rule API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-synonym-rule.html)\n\nDeletes a synonym rule in a synonym set"]
#[derive(Clone, Debug)]
pub struct SynonymsDeleteSynonymRule<'a, 'b> {
    transport: &'a Transport,
    parts: SynonymsDeleteSynonymRuleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SynonymsDeleteSynonymRule<'a, 'b> {
    #[doc = "Creates a new instance of [SynonymsDeleteSynonymRule] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SynonymsDeleteSynonymRuleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SynonymsDeleteSynonymRule {
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
    #[doc = "Creates an asynchronous call to the Synonyms Delete Synonym Rule API that can be awaited"]
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Synonyms Get Synonym API"]
pub enum SynonymsGetSynonymParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> SynonymsGetSynonymParts<'b> {
    #[doc = "Builds a relative URL path to the Synonyms Get Synonym API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SynonymsGetSynonymParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_id.len());
                p.push_str("/_synonyms/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Synonyms Get Synonym API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-synonyms-set.html)\n\nRetrieves a synonym set"]
#[derive(Clone, Debug)]
pub struct SynonymsGetSynonym<'a, 'b> {
    transport: &'a Transport,
    parts: SynonymsGetSynonymParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    size: Option<i32>,
    source: Option<&'b str>,
}
impl<'a, 'b> SynonymsGetSynonym<'a, 'b> {
    #[doc = "Creates a new instance of [SynonymsGetSynonym] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SynonymsGetSynonymParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SynonymsGetSynonym {
            transport,
            parts,
            headers,
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
    #[doc = "Starting offset"]
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
    #[doc = "Creates an asynchronous call to the Synonyms Get Synonym API that can be awaited"]
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
                size: Option<i32>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Synonyms Get Synonym Rule API"]
pub enum SynonymsGetSynonymRuleParts<'b> {
    #[doc = "SetId and RuleId"]
    SetIdRuleId(&'b str, &'b str),
}
impl<'b> SynonymsGetSynonymRuleParts<'b> {
    #[doc = "Builds a relative URL path to the Synonyms Get Synonym Rule API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SynonymsGetSynonymRuleParts::SetIdRuleId(set_id, rule_id) => {
                let encoded_set_id: Cow<str> =
                    percent_encode(set_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_rule_id: Cow<str> =
                    percent_encode(rule_id.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(12usize + encoded_set_id.len() + encoded_rule_id.len());
                p.push_str("/_synonyms/");
                p.push_str(encoded_set_id.as_ref());
                p.push('/');
                p.push_str(encoded_rule_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Synonyms Get Synonym Rule API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-synonym-rule.html)\n\nRetrieves a synonym rule from a synonym set"]
#[derive(Clone, Debug)]
pub struct SynonymsGetSynonymRule<'a, 'b> {
    transport: &'a Transport,
    parts: SynonymsGetSynonymRuleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SynonymsGetSynonymRule<'a, 'b> {
    #[doc = "Creates a new instance of [SynonymsGetSynonymRule] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SynonymsGetSynonymRuleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SynonymsGetSynonymRule {
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
    #[doc = "Creates an asynchronous call to the Synonyms Get Synonym Rule API that can be awaited"]
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
#[doc = "API parts for the Synonyms Get Synonyms Sets API"]
pub enum SynonymsGetSynonymsSetsParts {
    #[doc = "No parts"]
    None,
}
impl SynonymsGetSynonymsSetsParts {
    #[doc = "Builds a relative URL path to the Synonyms Get Synonyms Sets API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SynonymsGetSynonymsSetsParts::None => "/_synonyms".into(),
        }
    }
}
#[doc = "Builder for the [Synonyms Get Synonyms Sets API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/list-synonyms-sets.html)\n\nRetrieves a summary of all defined synonym sets"]
#[derive(Clone, Debug)]
pub struct SynonymsGetSynonymsSets<'a, 'b> {
    transport: &'a Transport,
    parts: SynonymsGetSynonymsSetsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i32>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    size: Option<i32>,
    source: Option<&'b str>,
}
impl<'a, 'b> SynonymsGetSynonymsSets<'a, 'b> {
    #[doc = "Creates a new instance of [SynonymsGetSynonymsSets]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SynonymsGetSynonymsSets {
            transport,
            parts: SynonymsGetSynonymsSetsParts::None,
            headers,
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
    #[doc = "Starting offset"]
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
    #[doc = "Creates an asynchronous call to the Synonyms Get Synonyms Sets API that can be awaited"]
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
                size: Option<i32>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Synonyms Put Synonym API"]
pub enum SynonymsPutSynonymParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> SynonymsPutSynonymParts<'b> {
    #[doc = "Builds a relative URL path to the Synonyms Put Synonym API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SynonymsPutSynonymParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_id.len());
                p.push_str("/_synonyms/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Synonyms Put Synonym API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/put-synonyms-set.html)\n\nCreates or updates a synonyms set"]
#[derive(Clone, Debug)]
pub struct SynonymsPutSynonym<'a, 'b, B> {
    transport: &'a Transport,
    parts: SynonymsPutSynonymParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SynonymsPutSynonym<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SynonymsPutSynonym] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SynonymsPutSynonymParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SynonymsPutSynonym {
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
    pub fn body<T>(self, body: T) -> SynonymsPutSynonym<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SynonymsPutSynonym {
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
    #[doc = "Creates an asynchronous call to the Synonyms Put Synonym API that can be awaited"]
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Synonyms Put Synonym Rule API"]
pub enum SynonymsPutSynonymRuleParts<'b> {
    #[doc = "SetId and RuleId"]
    SetIdRuleId(&'b str, &'b str),
}
impl<'b> SynonymsPutSynonymRuleParts<'b> {
    #[doc = "Builds a relative URL path to the Synonyms Put Synonym Rule API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SynonymsPutSynonymRuleParts::SetIdRuleId(set_id, rule_id) => {
                let encoded_set_id: Cow<str> =
                    percent_encode(set_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_rule_id: Cow<str> =
                    percent_encode(rule_id.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(12usize + encoded_set_id.len() + encoded_rule_id.len());
                p.push_str("/_synonyms/");
                p.push_str(encoded_set_id.as_ref());
                p.push('/');
                p.push_str(encoded_rule_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Synonyms Put Synonym Rule API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/put-synonym-rule.html)\n\nCreates or updates a synonym rule in a synonym set"]
#[derive(Clone, Debug)]
pub struct SynonymsPutSynonymRule<'a, 'b, B> {
    transport: &'a Transport,
    parts: SynonymsPutSynonymRuleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SynonymsPutSynonymRule<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SynonymsPutSynonymRule] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SynonymsPutSynonymRuleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SynonymsPutSynonymRule {
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
    pub fn body<T>(self, body: T) -> SynonymsPutSynonymRule<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SynonymsPutSynonymRule {
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
    #[doc = "Creates an asynchronous call to the Synonyms Put Synonym Rule API that can be awaited"]
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
#[doc = "Namespace client for Synonyms APIs"]
pub struct Synonyms<'a> {
    transport: &'a Transport,
}
impl<'a> Synonyms<'a> {
    #[doc = "Creates a new instance of [Synonyms]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Synonyms Delete Synonym API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-synonyms-set.html)\n\nDeletes a synonym set"]
    pub fn delete_synonym<'b>(
        &'a self,
        parts: SynonymsDeleteSynonymParts<'b>,
    ) -> SynonymsDeleteSynonym<'a, 'b> {
        SynonymsDeleteSynonym::new(self.transport(), parts)
    }
    #[doc = "[Synonyms Delete Synonym Rule API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-synonym-rule.html)\n\nDeletes a synonym rule in a synonym set"]
    pub fn delete_synonym_rule<'b>(
        &'a self,
        parts: SynonymsDeleteSynonymRuleParts<'b>,
    ) -> SynonymsDeleteSynonymRule<'a, 'b> {
        SynonymsDeleteSynonymRule::new(self.transport(), parts)
    }
    #[doc = "[Synonyms Get Synonym API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-synonyms-set.html)\n\nRetrieves a synonym set"]
    pub fn get_synonym<'b>(
        &'a self,
        parts: SynonymsGetSynonymParts<'b>,
    ) -> SynonymsGetSynonym<'a, 'b> {
        SynonymsGetSynonym::new(self.transport(), parts)
    }
    #[doc = "[Synonyms Get Synonym Rule API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-synonym-rule.html)\n\nRetrieves a synonym rule from a synonym set"]
    pub fn get_synonym_rule<'b>(
        &'a self,
        parts: SynonymsGetSynonymRuleParts<'b>,
    ) -> SynonymsGetSynonymRule<'a, 'b> {
        SynonymsGetSynonymRule::new(self.transport(), parts)
    }
    #[doc = "[Synonyms Get Synonyms Sets API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/list-synonyms-sets.html)\n\nRetrieves a summary of all defined synonym sets"]
    pub fn get_synonyms_sets<'b>(&'a self) -> SynonymsGetSynonymsSets<'a, 'b> {
        SynonymsGetSynonymsSets::new(self.transport())
    }
    #[doc = "[Synonyms Put Synonym API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/put-synonyms-set.html)\n\nCreates or updates a synonyms set"]
    pub fn put_synonym<'b>(
        &'a self,
        parts: SynonymsPutSynonymParts<'b>,
    ) -> SynonymsPutSynonym<'a, 'b, ()> {
        SynonymsPutSynonym::new(self.transport(), parts)
    }
    #[doc = "[Synonyms Put Synonym Rule API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/put-synonym-rule.html)\n\nCreates or updates a synonym rule in a synonym set"]
    pub fn put_synonym_rule<'b>(
        &'a self,
        parts: SynonymsPutSynonymRuleParts<'b>,
    ) -> SynonymsPutSynonymRule<'a, 'b, ()> {
        SynonymsPutSynonymRule::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Synonyms APIs"]
    pub fn synonyms(&self) -> Synonyms {
        Synonyms::new(self.transport())
    }
}
