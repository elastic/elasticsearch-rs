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

//! Query rules APIs
//!
//! Query rules allow you to configure per-query rules that are applied at query time to queries that match the specific rule. Query rules are organized into rulesets, collections of query rules that are matched against incoming queries. Query rules are applied using the rule query.
//!
//! If a query matches one or more rules in the ruleset, the query is re-written to apply the rules before searching. This allows pinning documents for only queries that match a specific term.

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
#[doc = "API parts for the Query Rules Delete Rule API"]
pub enum QueryRulesDeleteRuleParts<'b> {
    #[doc = "RulesetId and RuleId"]
    RulesetIdRuleId(&'b str, &'b str),
}
impl<'b> QueryRulesDeleteRuleParts<'b> {
    #[doc = "Builds a relative URL path to the Query Rules Delete Rule API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            QueryRulesDeleteRuleParts::RulesetIdRuleId(ruleset_id, rule_id) => {
                let encoded_ruleset_id: Cow<str> =
                    percent_encode(ruleset_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_rule_id: Cow<str> =
                    percent_encode(rule_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    21usize + encoded_ruleset_id.len() + encoded_rule_id.len(),
                );
                p.push_str("/_query_rules/");
                p.push_str(encoded_ruleset_id.as_ref());
                p.push_str("/_rule/");
                p.push_str(encoded_rule_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Query Rules Delete Rule API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-query-rule.html)\n\nDeletes an individual query rule within a ruleset."]
#[derive(Clone, Debug)]
pub struct QueryRulesDeleteRule<'a, 'b> {
    transport: &'a Transport,
    parts: QueryRulesDeleteRuleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> QueryRulesDeleteRule<'a, 'b> {
    #[doc = "Creates a new instance of [QueryRulesDeleteRule] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: QueryRulesDeleteRuleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        QueryRulesDeleteRule {
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
    #[doc = "Creates an asynchronous call to the Query Rules Delete Rule API that can be awaited"]
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
#[doc = "API parts for the Query Rules Delete Ruleset API"]
pub enum QueryRulesDeleteRulesetParts<'b> {
    #[doc = "RulesetId"]
    RulesetId(&'b str),
}
impl<'b> QueryRulesDeleteRulesetParts<'b> {
    #[doc = "Builds a relative URL path to the Query Rules Delete Ruleset API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            QueryRulesDeleteRulesetParts::RulesetId(ruleset_id) => {
                let encoded_ruleset_id: Cow<str> =
                    percent_encode(ruleset_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_ruleset_id.len());
                p.push_str("/_query_rules/");
                p.push_str(encoded_ruleset_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Query Rules Delete Ruleset API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-query-ruleset.html)\n\nDeletes a query ruleset."]
#[derive(Clone, Debug)]
pub struct QueryRulesDeleteRuleset<'a, 'b> {
    transport: &'a Transport,
    parts: QueryRulesDeleteRulesetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> QueryRulesDeleteRuleset<'a, 'b> {
    #[doc = "Creates a new instance of [QueryRulesDeleteRuleset] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: QueryRulesDeleteRulesetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        QueryRulesDeleteRuleset {
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
    #[doc = "Creates an asynchronous call to the Query Rules Delete Ruleset API that can be awaited"]
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
#[doc = "API parts for the Query Rules Get Rule API"]
pub enum QueryRulesGetRuleParts<'b> {
    #[doc = "RulesetId and RuleId"]
    RulesetIdRuleId(&'b str, &'b str),
}
impl<'b> QueryRulesGetRuleParts<'b> {
    #[doc = "Builds a relative URL path to the Query Rules Get Rule API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            QueryRulesGetRuleParts::RulesetIdRuleId(ruleset_id, rule_id) => {
                let encoded_ruleset_id: Cow<str> =
                    percent_encode(ruleset_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_rule_id: Cow<str> =
                    percent_encode(rule_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    21usize + encoded_ruleset_id.len() + encoded_rule_id.len(),
                );
                p.push_str("/_query_rules/");
                p.push_str(encoded_ruleset_id.as_ref());
                p.push_str("/_rule/");
                p.push_str(encoded_rule_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Query Rules Get Rule API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-query-rule.html)\n\nReturns the details about an individual query rule within a ruleset."]
#[derive(Clone, Debug)]
pub struct QueryRulesGetRule<'a, 'b> {
    transport: &'a Transport,
    parts: QueryRulesGetRuleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> QueryRulesGetRule<'a, 'b> {
    #[doc = "Creates a new instance of [QueryRulesGetRule] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: QueryRulesGetRuleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        QueryRulesGetRule {
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
    #[doc = "Creates an asynchronous call to the Query Rules Get Rule API that can be awaited"]
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
#[doc = "API parts for the Query Rules Get Ruleset API"]
pub enum QueryRulesGetRulesetParts<'b> {
    #[doc = "RulesetId"]
    RulesetId(&'b str),
}
impl<'b> QueryRulesGetRulesetParts<'b> {
    #[doc = "Builds a relative URL path to the Query Rules Get Ruleset API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            QueryRulesGetRulesetParts::RulesetId(ruleset_id) => {
                let encoded_ruleset_id: Cow<str> =
                    percent_encode(ruleset_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_ruleset_id.len());
                p.push_str("/_query_rules/");
                p.push_str(encoded_ruleset_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Query Rules Get Ruleset API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-query-ruleset.html)\n\nReturns the details about a query ruleset."]
#[derive(Clone, Debug)]
pub struct QueryRulesGetRuleset<'a, 'b> {
    transport: &'a Transport,
    parts: QueryRulesGetRulesetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> QueryRulesGetRuleset<'a, 'b> {
    #[doc = "Creates a new instance of [QueryRulesGetRuleset] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: QueryRulesGetRulesetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        QueryRulesGetRuleset {
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
    #[doc = "Creates an asynchronous call to the Query Rules Get Ruleset API that can be awaited"]
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
#[doc = "API parts for the Query Rules List Rulesets API"]
pub enum QueryRulesListRulesetsParts {
    #[doc = "No parts"]
    None,
}
impl QueryRulesListRulesetsParts {
    #[doc = "Builds a relative URL path to the Query Rules List Rulesets API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            QueryRulesListRulesetsParts::None => "/_query_rules".into(),
        }
    }
}
#[doc = "Builder for the [Query Rules List Rulesets API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/list-query-rulesets.html)\n\nLists query rulesets."]
#[derive(Clone, Debug)]
pub struct QueryRulesListRulesets<'a, 'b> {
    transport: &'a Transport,
    parts: QueryRulesListRulesetsParts,
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
impl<'a, 'b> QueryRulesListRulesets<'a, 'b> {
    #[doc = "Creates a new instance of [QueryRulesListRulesets]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        QueryRulesListRulesets {
            transport,
            parts: QueryRulesListRulesetsParts::None,
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
    #[doc = "Creates an asynchronous call to the Query Rules List Rulesets API that can be awaited"]
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
#[doc = "API parts for the Query Rules Put Rule API"]
pub enum QueryRulesPutRuleParts<'b> {
    #[doc = "RulesetId and RuleId"]
    RulesetIdRuleId(&'b str, &'b str),
}
impl<'b> QueryRulesPutRuleParts<'b> {
    #[doc = "Builds a relative URL path to the Query Rules Put Rule API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            QueryRulesPutRuleParts::RulesetIdRuleId(ruleset_id, rule_id) => {
                let encoded_ruleset_id: Cow<str> =
                    percent_encode(ruleset_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_rule_id: Cow<str> =
                    percent_encode(rule_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    21usize + encoded_ruleset_id.len() + encoded_rule_id.len(),
                );
                p.push_str("/_query_rules/");
                p.push_str(encoded_ruleset_id.as_ref());
                p.push_str("/_rule/");
                p.push_str(encoded_rule_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Query Rules Put Rule API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/put-query-rule.html)\n\nCreates or updates a query rule within a ruleset."]
#[derive(Clone, Debug)]
pub struct QueryRulesPutRule<'a, 'b, B> {
    transport: &'a Transport,
    parts: QueryRulesPutRuleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> QueryRulesPutRule<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [QueryRulesPutRule] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: QueryRulesPutRuleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        QueryRulesPutRule {
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
    pub fn body<T>(self, body: T) -> QueryRulesPutRule<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        QueryRulesPutRule {
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
    #[doc = "Creates an asynchronous call to the Query Rules Put Rule API that can be awaited"]
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
#[doc = "API parts for the Query Rules Put Ruleset API"]
pub enum QueryRulesPutRulesetParts<'b> {
    #[doc = "RulesetId"]
    RulesetId(&'b str),
}
impl<'b> QueryRulesPutRulesetParts<'b> {
    #[doc = "Builds a relative URL path to the Query Rules Put Ruleset API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            QueryRulesPutRulesetParts::RulesetId(ruleset_id) => {
                let encoded_ruleset_id: Cow<str> =
                    percent_encode(ruleset_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_ruleset_id.len());
                p.push_str("/_query_rules/");
                p.push_str(encoded_ruleset_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Query Rules Put Ruleset API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/put-query-ruleset.html)\n\nCreates or updates a query ruleset."]
#[derive(Clone, Debug)]
pub struct QueryRulesPutRuleset<'a, 'b, B> {
    transport: &'a Transport,
    parts: QueryRulesPutRulesetParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> QueryRulesPutRuleset<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [QueryRulesPutRuleset] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: QueryRulesPutRulesetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        QueryRulesPutRuleset {
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
    pub fn body<T>(self, body: T) -> QueryRulesPutRuleset<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        QueryRulesPutRuleset {
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
    #[doc = "Creates an asynchronous call to the Query Rules Put Ruleset API that can be awaited"]
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
#[doc = "Namespace client for QueryRules APIs"]
pub struct QueryRules<'a> {
    transport: &'a Transport,
}
impl<'a> QueryRules<'a> {
    #[doc = "Creates a new instance of [QueryRules]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Query Rules Delete Rule API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-query-rule.html)\n\nDeletes an individual query rule within a ruleset."]
    pub fn delete_rule<'b>(
        &'a self,
        parts: QueryRulesDeleteRuleParts<'b>,
    ) -> QueryRulesDeleteRule<'a, 'b> {
        QueryRulesDeleteRule::new(self.transport(), parts)
    }
    #[doc = "[Query Rules Delete Ruleset API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/delete-query-ruleset.html)\n\nDeletes a query ruleset."]
    pub fn delete_ruleset<'b>(
        &'a self,
        parts: QueryRulesDeleteRulesetParts<'b>,
    ) -> QueryRulesDeleteRuleset<'a, 'b> {
        QueryRulesDeleteRuleset::new(self.transport(), parts)
    }
    #[doc = "[Query Rules Get Rule API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-query-rule.html)\n\nReturns the details about an individual query rule within a ruleset."]
    pub fn get_rule<'b>(&'a self, parts: QueryRulesGetRuleParts<'b>) -> QueryRulesGetRule<'a, 'b> {
        QueryRulesGetRule::new(self.transport(), parts)
    }
    #[doc = "[Query Rules Get Ruleset API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/get-query-ruleset.html)\n\nReturns the details about a query ruleset."]
    pub fn get_ruleset<'b>(
        &'a self,
        parts: QueryRulesGetRulesetParts<'b>,
    ) -> QueryRulesGetRuleset<'a, 'b> {
        QueryRulesGetRuleset::new(self.transport(), parts)
    }
    #[doc = "[Query Rules List Rulesets API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/list-query-rulesets.html)\n\nLists query rulesets."]
    pub fn list_rulesets<'b>(&'a self) -> QueryRulesListRulesets<'a, 'b> {
        QueryRulesListRulesets::new(self.transport())
    }
    #[doc = "[Query Rules Put Rule API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/put-query-rule.html)\n\nCreates or updates a query rule within a ruleset."]
    pub fn put_rule<'b>(
        &'a self,
        parts: QueryRulesPutRuleParts<'b>,
    ) -> QueryRulesPutRule<'a, 'b, ()> {
        QueryRulesPutRule::new(self.transport(), parts)
    }
    #[doc = "[Query Rules Put Ruleset API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/put-query-ruleset.html)\n\nCreates or updates a query ruleset."]
    pub fn put_ruleset<'b>(
        &'a self,
        parts: QueryRulesPutRulesetParts<'b>,
    ) -> QueryRulesPutRuleset<'a, 'b, ()> {
        QueryRulesPutRuleset::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for QueryRules APIs"]
    pub fn query_rules(&self) -> QueryRules {
        QueryRules::new(self.transport())
    }
}
