// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
use crate::{
    client::{Elasticsearch, Sender},
    enums::*,
    error::ElasticsearchError,
    http_method::HttpMethod,
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
pub enum BulkUrlParts {
    None,
    Index(String),
    IndexType(String, String),
}
impl BulkUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            BulkUrlParts::None => "/_bulk".into(),
            BulkUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(7usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_bulk");
                p.into()
            }
            BulkUrlParts::IndexType(ref index, ref ty) => {
                let mut p = String::with_capacity(8usize + index.len() + ty.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/_bulk");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Bulk<B> {
    client: Elasticsearch,
    parts: BulkUrlParts,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pipeline: Option<String>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<String>,
    source: Option<String>,
    timeout: Option<String>,
    ty: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl<B> Bulk<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: BulkUrlParts) -> Self {
        Bulk {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pipeline: None,
            pretty: None,
            refresh: None,
            routing: None,
            source: None,
            timeout: None,
            ty: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or default list of fields to return, can be overridden on each sub-request"]
    pub fn _source(mut self, _source: Option<Vec<String>>) -> Self {
        self._source = _source;
        self
    }
    #[doc = "Default list of fields to exclude from the returned _source field, can be overridden on each sub-request"]
    pub fn _source_excludes(mut self, _source_excludes: Option<Vec<String>>) -> Self {
        self._source_excludes = _source_excludes;
        self
    }
    #[doc = "Default list of fields to extract and return from the _source field, can be overridden on each sub-request"]
    pub fn _source_includes(mut self, _source_includes: Option<Vec<String>>) -> Self {
        self._source_includes = _source_includes;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "The pipeline id to preprocess incoming documents with"]
    pub fn pipeline(mut self, pipeline: Option<String>) -> Self {
        self.pipeline = pipeline;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "If `true` then refresh the effected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Default document type for items which don't provide one"]
    pub fn ty(mut self, ty: Option<String>) -> Self {
        self.ty = ty;
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the bulk operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl<B> Sender for Bulk<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<Vec<String>>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<Vec<String>>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pipeline", skip_serializing_if = "Option::is_none")]
                pipeline: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
                #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
                ty: Option<String>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pipeline: self.pipeline,
                pretty: self.pretty,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                ty: self.ty,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ClearScrollUrlParts {
    None,
    ScrollId(Vec<String>),
}
impl ClearScrollUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ClearScrollUrlParts::None => "/_search/scroll".into(),
            ClearScrollUrlParts::ScrollId(ref scroll_id) => {
                let scroll_id_str = scroll_id.join(",");
                let mut p = String::with_capacity(16usize + scroll_id_str.len());
                p.push_str("/_search/scroll/");
                p.push_str(scroll_id_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct ClearScroll<B> {
    client: Elasticsearch,
    parts: ClearScrollUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> ClearScroll<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: ClearScrollUrlParts) -> Self {
        ClearScroll {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for ClearScroll<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum CountUrlParts {
    None,
    Index(Vec<String>),
    IndexType(Vec<String>, Vec<String>),
}
impl CountUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CountUrlParts::None => "/_count".into(),
            CountUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(8usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_count");
                p.into()
            }
            CountUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(9usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_count");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Count<B> {
    client: Elasticsearch,
    parts: CountUrlParts,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<String>,
    body: Option<B>,
    default_operator: Option<DefaultOperator>,
    df: Option<String>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    min_score: Option<i64>,
    preference: Option<String>,
    pretty: Option<bool>,
    q: Option<String>,
    routing: Option<Vec<String>>,
    source: Option<String>,
    terminate_after: Option<i64>,
}
impl<B> Count<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: CountUrlParts) -> Self {
        Count {
            client,
            parts,
            allow_no_indices: None,
            analyze_wildcard: None,
            analyzer: None,
            body: None,
            default_operator: None,
            df: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            lenient: None,
            min_score: None,
            preference: None,
            pretty: None,
            q: None,
            routing: None,
            source: None,
            terminate_after: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: Option<bool>) -> Self {
        self.analyze_wildcard = analyze_wildcard;
        self
    }
    #[doc = "The analyzer to use for the query string"]
    pub fn analyzer(mut self, analyzer: Option<String>) -> Self {
        self.analyzer = analyzer;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: Option<DefaultOperator>) -> Self {
        self.default_operator = default_operator;
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: Option<String>) -> Self {
        self.df = df;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether specified concrete, expanded or aliased indices should be ignored when throttled"]
    pub fn ignore_throttled(mut self, ignore_throttled: Option<bool>) -> Self {
        self.ignore_throttled = ignore_throttled;
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: Option<bool>) -> Self {
        self.lenient = lenient;
        self
    }
    #[doc = "Include only documents with a specific `_score` value in the result"]
    pub fn min_score(mut self, min_score: Option<i64>) -> Self {
        self.min_score = min_score;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: Option<String>) -> Self {
        self.q = q;
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: Option<Vec<String>>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "The maximum count for each shard, upon reaching which the query execution will terminate early"]
    pub fn terminate_after(mut self, terminate_after: Option<i64>) -> Self {
        self.terminate_after = terminate_after;
        self
    }
}
impl<B> Sender for Count<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard", skip_serializing_if = "Option::is_none")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer", skip_serializing_if = "Option::is_none")]
                analyzer: Option<String>,
                #[serde(rename = "default_operator", skip_serializing_if = "Option::is_none")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df", skip_serializing_if = "Option::is_none")]
                df: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_throttled", skip_serializing_if = "Option::is_none")]
                ignore_throttled: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient", skip_serializing_if = "Option::is_none")]
                lenient: Option<bool>,
                #[serde(rename = "min_score", skip_serializing_if = "Option::is_none")]
                min_score: Option<i64>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
                q: Option<String>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                routing: Option<Vec<String>>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "terminate_after", skip_serializing_if = "Option::is_none")]
                terminate_after: Option<i64>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                min_score: self.min_score,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                routing: self.routing,
                source: self.source,
                terminate_after: self.terminate_after,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum CreateUrlParts {
    IndexId(String, String),
    IndexTypeId(String, String, String),
}
impl CreateUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CreateUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(10usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_create/");
                p.push_str(id.as_ref());
                p.into()
            }
            CreateUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.push_str("/_create");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Create<B> {
    client: Elasticsearch,
    parts: CreateUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pipeline: Option<String>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<String>,
    source: Option<String>,
    timeout: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<String>,
}
impl<B> Create<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: CreateUrlParts) -> Self {
        Create {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pipeline: None,
            pretty: None,
            refresh: None,
            routing: None,
            source: None,
            timeout: None,
            version: None,
            version_type: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "The pipeline id to preprocess incoming documents with"]
    pub fn pipeline(mut self, pipeline: Option<String>) -> Self {
        self.pipeline = pipeline;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: Option<i64>) -> Self {
        self.version = version;
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: Option<VersionType>) -> Self {
        self.version_type = version_type;
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the index operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl<B> Sender for Create<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pipeline", skip_serializing_if = "Option::is_none")]
                pipeline: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pipeline: self.pipeline,
                pretty: self.pretty,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                version: self.version,
                version_type: self.version_type,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum DeleteUrlParts {
    IndexId(String, String),
    IndexTypeId(String, String, String),
}
impl DeleteUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DeleteUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(7usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc/");
                p.push_str(id.as_ref());
                p.into()
            }
            DeleteUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Delete {
    client: Elasticsearch,
    parts: DeleteUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<String>,
    source: Option<String>,
    timeout: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<String>,
}
impl Delete {
    pub fn new(client: Elasticsearch, parts: DeleteUrlParts) -> Self {
        Delete {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            pretty: None,
            refresh: None,
            routing: None,
            source: None,
            timeout: None,
            version: None,
            version_type: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "only perform the delete operation if the last operation that has changed the document has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: Option<i64>) -> Self {
        self.if_primary_term = if_primary_term;
        self
    }
    #[doc = "only perform the delete operation if the last operation that has changed the document has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: Option<i64>) -> Self {
        self.if_seq_no = if_seq_no;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "If `true` then refresh the effected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: Option<i64>) -> Self {
        self.version = version;
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: Option<VersionType>) -> Self {
        self.version_type = version_type;
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the delete operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl Sender for Delete {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "if_primary_term", skip_serializing_if = "Option::is_none")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no", skip_serializing_if = "Option::is_none")]
                if_seq_no: Option<i64>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                pretty: self.pretty,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                version: self.version,
                version_type: self.version_type,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum DeleteByQueryUrlParts {
    Index(Vec<String>),
    IndexType(Vec<String>, Vec<String>),
}
impl DeleteByQueryUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DeleteByQueryUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(18usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_delete_by_query");
                p.into()
            }
            DeleteByQueryUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(19usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_delete_by_query");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct DeleteByQuery<B> {
    client: Elasticsearch,
    parts: DeleteByQueryUrlParts,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    body: Option<B>,
    conflicts: Option<Conflicts>,
    default_operator: Option<DefaultOperator>,
    df: Option<String>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    from: Option<i64>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    max_docs: Option<i64>,
    preference: Option<String>,
    pretty: Option<bool>,
    q: Option<String>,
    refresh: Option<bool>,
    request_cache: Option<bool>,
    requests_per_second: Option<i64>,
    routing: Option<Vec<String>>,
    scroll: Option<String>,
    scroll_size: Option<i64>,
    search_timeout: Option<String>,
    search_type: Option<SearchType>,
    size: Option<i64>,
    slices: Option<i64>,
    sort: Option<Vec<String>>,
    source: Option<String>,
    stats: Option<Vec<String>>,
    terminate_after: Option<i64>,
    timeout: Option<String>,
    version: Option<bool>,
    wait_for_active_shards: Option<String>,
    wait_for_completion: Option<bool>,
}
impl<B> DeleteByQuery<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: DeleteByQueryUrlParts) -> Self {
        DeleteByQuery {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            allow_no_indices: None,
            analyze_wildcard: None,
            body: None,
            conflicts: None,
            default_operator: None,
            df: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            from: None,
            human: None,
            ignore_unavailable: None,
            lenient: None,
            max_docs: None,
            preference: None,
            pretty: None,
            q: None,
            refresh: None,
            request_cache: None,
            requests_per_second: None,
            routing: None,
            scroll: None,
            scroll_size: None,
            search_timeout: None,
            search_type: None,
            size: None,
            slices: None,
            sort: None,
            source: None,
            stats: None,
            terminate_after: None,
            timeout: None,
            version: None,
            wait_for_active_shards: None,
            wait_for_completion: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: Option<Vec<String>>) -> Self {
        self._source = _source;
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: Option<Vec<String>>) -> Self {
        self._source_excludes = _source_excludes;
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: Option<Vec<String>>) -> Self {
        self._source_includes = _source_includes;
        self
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: Option<bool>) -> Self {
        self.analyze_wildcard = analyze_wildcard;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "What to do when the delete by query hits version conflicts?"]
    pub fn conflicts(mut self, conflicts: Option<Conflicts>) -> Self {
        self.conflicts = conflicts;
        self
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: Option<DefaultOperator>) -> Self {
        self.default_operator = default_operator;
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: Option<String>) -> Self {
        self.df = df;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Starting offset (default: 0)"]
    pub fn from(mut self, from: Option<i64>) -> Self {
        self.from = from;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: Option<bool>) -> Self {
        self.lenient = lenient;
        self
    }
    #[doc = "Maximum number of documents to process (default: all documents)"]
    pub fn max_docs(mut self, max_docs: Option<i64>) -> Self {
        self.max_docs = max_docs;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: Option<String>) -> Self {
        self.q = q;
        self
    }
    #[doc = "Should the effected indexes be refreshed?"]
    pub fn refresh(mut self, refresh: Option<bool>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "Specify if request cache should be used for this request or not, defaults to index level setting"]
    pub fn request_cache(mut self, request_cache: Option<bool>) -> Self {
        self.request_cache = request_cache;
        self
    }
    #[doc = "The throttle for this request in sub-requests per second. -1 means no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: Option<i64>) -> Self {
        self.requests_per_second = requests_per_second;
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: Option<Vec<String>>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: Option<String>) -> Self {
        self.scroll = scroll;
        self
    }
    #[doc = "Size on the scroll request powering the delete by query"]
    pub fn scroll_size(mut self, scroll_size: Option<i64>) -> Self {
        self.scroll_size = scroll_size;
        self
    }
    #[doc = "Explicit timeout for each search request. Defaults to no timeout."]
    pub fn search_timeout(mut self, search_timeout: Option<String>) -> Self {
        self.search_timeout = search_timeout;
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: Option<SearchType>) -> Self {
        self.search_type = search_type;
        self
    }
    #[doc = "Deprecated, please use `max_docs` instead"]
    pub fn size(mut self, size: Option<i64>) -> Self {
        self.size = size;
        self
    }
    #[doc = "The number of slices this task should be divided into. Defaults to 1 meaning the task isn't sliced into subtasks."]
    pub fn slices(mut self, slices: Option<i64>) -> Self {
        self.slices = slices;
        self
    }
    #[doc = "A comma-separated list of <field>:<direction> pairs"]
    pub fn sort(mut self, sort: Option<Vec<String>>) -> Self {
        self.sort = sort;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Specific 'tag' of the request for logging and statistical purposes"]
    pub fn stats(mut self, stats: Option<Vec<String>>) -> Self {
        self.stats = stats;
        self
    }
    #[doc = "The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early."]
    pub fn terminate_after(mut self, terminate_after: Option<i64>) -> Self {
        self.terminate_after = terminate_after;
        self
    }
    #[doc = "Time each individual bulk request should wait for shards that are unavailable."]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Specify whether to return document version as part of a hit"]
    pub fn version(mut self, version: Option<bool>) -> Self {
        self.version = version;
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the delete by query operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
    #[doc = "Should the request should block until the delete by query is complete."]
    pub fn wait_for_completion(mut self, wait_for_completion: Option<bool>) -> Self {
        self.wait_for_completion = wait_for_completion;
        self
    }
}
impl<B> Sender for DeleteByQuery<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<Vec<String>>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<Vec<String>>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard", skip_serializing_if = "Option::is_none")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "conflicts", skip_serializing_if = "Option::is_none")]
                conflicts: Option<Conflicts>,
                #[serde(rename = "default_operator", skip_serializing_if = "Option::is_none")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df", skip_serializing_if = "Option::is_none")]
                df: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i64>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient", skip_serializing_if = "Option::is_none")]
                lenient: Option<bool>,
                #[serde(rename = "max_docs", skip_serializing_if = "Option::is_none")]
                max_docs: Option<i64>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
                q: Option<String>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "request_cache", skip_serializing_if = "Option::is_none")]
                request_cache: Option<bool>,
                #[serde(
                    rename = "requests_per_second",
                    skip_serializing_if = "Option::is_none"
                )]
                requests_per_second: Option<i64>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                routing: Option<Vec<String>>,
                #[serde(rename = "scroll", skip_serializing_if = "Option::is_none")]
                scroll: Option<String>,
                #[serde(rename = "scroll_size", skip_serializing_if = "Option::is_none")]
                scroll_size: Option<i64>,
                #[serde(rename = "search_timeout", skip_serializing_if = "Option::is_none")]
                search_timeout: Option<String>,
                #[serde(rename = "search_type", skip_serializing_if = "Option::is_none")]
                search_type: Option<SearchType>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i64>,
                #[serde(rename = "slices", skip_serializing_if = "Option::is_none")]
                slices: Option<i64>,
                #[serde(
                    rename = "sort",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                sort: Option<Vec<String>>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(
                    rename = "stats",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stats: Option<Vec<String>>,
                #[serde(rename = "terminate_after", skip_serializing_if = "Option::is_none")]
                terminate_after: Option<i64>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<bool>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<String>,
                #[serde(
                    rename = "wait_for_completion",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                allow_no_indices: self.allow_no_indices,
                analyze_wildcard: self.analyze_wildcard,
                conflicts: self.conflicts,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                max_docs: self.max_docs,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                refresh: self.refresh,
                request_cache: self.request_cache,
                requests_per_second: self.requests_per_second,
                routing: self.routing,
                scroll: self.scroll,
                scroll_size: self.scroll_size,
                search_timeout: self.search_timeout,
                search_type: self.search_type,
                size: self.size,
                slices: self.slices,
                sort: self.sort,
                source: self.source,
                stats: self.stats,
                terminate_after: self.terminate_after,
                timeout: self.timeout,
                version: self.version,
                wait_for_active_shards: self.wait_for_active_shards,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum DeleteByQueryRethrottleUrlParts {
    TaskId(String),
}
impl DeleteByQueryRethrottleUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DeleteByQueryRethrottleUrlParts::TaskId(ref task_id) => {
                let mut p = String::with_capacity(30usize + task_id.len());
                p.push_str("/_delete_by_query/");
                p.push_str(task_id.as_ref());
                p.push_str("/_rethrottle");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct DeleteByQueryRethrottle<B> {
    client: Elasticsearch,
    parts: DeleteByQueryRethrottleUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<String>,
}
impl<B> DeleteByQueryRethrottle<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: DeleteByQueryRethrottleUrlParts) -> Self {
        DeleteByQueryRethrottle {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            requests_per_second: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The throttle to set on this request in floating sub-requests per second. -1 means set no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: Option<i64>) -> Self {
        self.requests_per_second = requests_per_second;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for DeleteByQueryRethrottle<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(
                    rename = "requests_per_second",
                    skip_serializing_if = "Option::is_none"
                )]
                requests_per_second: Option<i64>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                requests_per_second: self.requests_per_second,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum DeleteScriptUrlParts {
    Id(String),
}
impl DeleteScriptUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DeleteScriptUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(10usize + id.len());
                p.push_str("/_scripts/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct DeleteScript {
    client: Elasticsearch,
    parts: DeleteScriptUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl DeleteScript {
    pub fn new(client: Elasticsearch, parts: DeleteScriptUrlParts) -> Self {
        DeleteScript {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for DeleteScript {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ExistsUrlParts {
    IndexId(String, String),
    IndexTypeId(String, String, String),
}
impl ExistsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ExistsUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(7usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc/");
                p.push_str(id.as_ref());
                p.into()
            }
            ExistsUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Exists {
    client: Elasticsearch,
    parts: ExistsUrlParts,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    stored_fields: Option<Vec<String>>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl Exists {
    pub fn new(client: Elasticsearch, parts: ExistsUrlParts) -> Self {
        Exists {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            routing: None,
            source: None,
            stored_fields: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: Option<Vec<String>>) -> Self {
        self._source = _source;
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: Option<Vec<String>>) -> Self {
        self._source_excludes = _source_excludes;
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: Option<Vec<String>>) -> Self {
        self._source_includes = _source_includes;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: Option<bool>) -> Self {
        self.realtime = realtime;
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: Option<bool>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "A comma-separated list of stored fields to return in the response"]
    pub fn stored_fields(mut self, stored_fields: Option<Vec<String>>) -> Self {
        self.stored_fields = stored_fields;
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: Option<i64>) -> Self {
        self.version = version;
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: Option<VersionType>) -> Self {
        self.version_type = version_type;
        self
    }
}
impl Sender for Exists {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Head;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<Vec<String>>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<Vec<String>>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stored_fields: Option<Vec<String>>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                stored_fields: self.stored_fields,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ExistsSourceUrlParts {
    IndexId(String, String),
    IndexTypeId(String, String, String),
}
impl ExistsSourceUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ExistsSourceUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(10usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_source/");
                p.push_str(id.as_ref());
                p.into()
            }
            ExistsSourceUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.push_str("/_source");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct ExistsSource {
    client: Elasticsearch,
    parts: ExistsSourceUrlParts,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl ExistsSource {
    pub fn new(client: Elasticsearch, parts: ExistsSourceUrlParts) -> Self {
        ExistsSource {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            routing: None,
            source: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: Option<Vec<String>>) -> Self {
        self._source = _source;
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: Option<Vec<String>>) -> Self {
        self._source_excludes = _source_excludes;
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: Option<Vec<String>>) -> Self {
        self._source_includes = _source_includes;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: Option<bool>) -> Self {
        self.realtime = realtime;
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: Option<bool>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: Option<i64>) -> Self {
        self.version = version;
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: Option<VersionType>) -> Self {
        self.version_type = version_type;
        self
    }
}
impl Sender for ExistsSource {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Head;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<Vec<String>>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<Vec<String>>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ExplainUrlParts {
    IndexId(String, String),
    IndexTypeId(String, String, String),
}
impl ExplainUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ExplainUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(11usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_explain/");
                p.push_str(id.as_ref());
                p.into()
            }
            ExplainUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(12usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.push_str("/_explain");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Explain<B> {
    client: Elasticsearch,
    parts: ExplainUrlParts,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<String>,
    body: Option<B>,
    default_operator: Option<DefaultOperator>,
    df: Option<String>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    lenient: Option<bool>,
    preference: Option<String>,
    pretty: Option<bool>,
    q: Option<String>,
    routing: Option<String>,
    source: Option<String>,
    stored_fields: Option<Vec<String>>,
}
impl<B> Explain<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: ExplainUrlParts) -> Self {
        Explain {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            analyze_wildcard: None,
            analyzer: None,
            body: None,
            default_operator: None,
            df: None,
            error_trace: None,
            filter_path: None,
            human: None,
            lenient: None,
            preference: None,
            pretty: None,
            q: None,
            routing: None,
            source: None,
            stored_fields: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: Option<Vec<String>>) -> Self {
        self._source = _source;
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: Option<Vec<String>>) -> Self {
        self._source_excludes = _source_excludes;
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: Option<Vec<String>>) -> Self {
        self._source_includes = _source_includes;
        self
    }
    #[doc = "Specify whether wildcards and prefix queries in the query string query should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: Option<bool>) -> Self {
        self.analyze_wildcard = analyze_wildcard;
        self
    }
    #[doc = "The analyzer for the query string query"]
    pub fn analyzer(mut self, analyzer: Option<String>) -> Self {
        self.analyzer = analyzer;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: Option<DefaultOperator>) -> Self {
        self.default_operator = default_operator;
        self
    }
    #[doc = "The default field for query string query (default: _all)"]
    pub fn df(mut self, df: Option<String>) -> Self {
        self.df = df;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: Option<bool>) -> Self {
        self.lenient = lenient;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: Option<String>) -> Self {
        self.q = q;
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "A comma-separated list of stored fields to return in the response"]
    pub fn stored_fields(mut self, stored_fields: Option<Vec<String>>) -> Self {
        self.stored_fields = stored_fields;
        self
    }
}
impl<B> Sender for Explain<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<Vec<String>>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<Vec<String>>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "analyze_wildcard", skip_serializing_if = "Option::is_none")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer", skip_serializing_if = "Option::is_none")]
                analyzer: Option<String>,
                #[serde(rename = "default_operator", skip_serializing_if = "Option::is_none")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df", skip_serializing_if = "Option::is_none")]
                df: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "lenient", skip_serializing_if = "Option::is_none")]
                lenient: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
                q: Option<String>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stored_fields: Option<Vec<String>>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                lenient: self.lenient,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                routing: self.routing,
                source: self.source,
                stored_fields: self.stored_fields,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum FieldCapsUrlParts {
    None,
    Index(Vec<String>),
}
impl FieldCapsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            FieldCapsUrlParts::None => "/_field_caps".into(),
            FieldCapsUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(13usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_field_caps");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct FieldCaps<B> {
    client: Elasticsearch,
    parts: FieldCapsUrlParts,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    fields: Option<Vec<String>>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_unmapped: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> FieldCaps<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: FieldCapsUrlParts) -> Self {
        FieldCaps {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            fields: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            include_unmapped: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of field names"]
    pub fn fields(mut self, fields: Option<Vec<String>>) -> Self {
        self.fields = fields;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Indicates whether unmapped fields should be included in the response."]
    pub fn include_unmapped(mut self, include_unmapped: Option<bool>) -> Self {
        self.include_unmapped = include_unmapped;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for FieldCaps<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "fields",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                fields: Option<Vec<String>>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_unmapped", skip_serializing_if = "Option::is_none")]
                include_unmapped: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                fields: self.fields,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_unmapped: self.include_unmapped,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum GetUrlParts {
    IndexId(String, String),
    IndexTypeId(String, String, String),
}
impl GetUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            GetUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(7usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc/");
                p.push_str(id.as_ref());
                p.into()
            }
            GetUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Get {
    client: Elasticsearch,
    parts: GetUrlParts,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    stored_fields: Option<Vec<String>>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl Get {
    pub fn new(client: Elasticsearch, parts: GetUrlParts) -> Self {
        Get {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            routing: None,
            source: None,
            stored_fields: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: Option<Vec<String>>) -> Self {
        self._source = _source;
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: Option<Vec<String>>) -> Self {
        self._source_excludes = _source_excludes;
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: Option<Vec<String>>) -> Self {
        self._source_includes = _source_includes;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: Option<bool>) -> Self {
        self.realtime = realtime;
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: Option<bool>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "A comma-separated list of stored fields to return in the response"]
    pub fn stored_fields(mut self, stored_fields: Option<Vec<String>>) -> Self {
        self.stored_fields = stored_fields;
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: Option<i64>) -> Self {
        self.version = version;
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: Option<VersionType>) -> Self {
        self.version_type = version_type;
        self
    }
}
impl Sender for Get {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<Vec<String>>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<Vec<String>>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stored_fields: Option<Vec<String>>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                stored_fields: self.stored_fields,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum GetScriptUrlParts {
    Id(String),
}
impl GetScriptUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            GetScriptUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(10usize + id.len());
                p.push_str("/_scripts/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct GetScript {
    client: Elasticsearch,
    parts: GetScriptUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl GetScript {
    pub fn new(client: Elasticsearch, parts: GetScriptUrlParts) -> Self {
        GetScript {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for GetScript {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum GetSourceUrlParts {
    IndexId(String, String),
    IndexTypeId(String, String, String),
}
impl GetSourceUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            GetSourceUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(10usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_source/");
                p.push_str(id.as_ref());
                p.into()
            }
            GetSourceUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.push_str("/_source");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct GetSource {
    client: Elasticsearch,
    parts: GetSourceUrlParts,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl GetSource {
    pub fn new(client: Elasticsearch, parts: GetSourceUrlParts) -> Self {
        GetSource {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            routing: None,
            source: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: Option<Vec<String>>) -> Self {
        self._source = _source;
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: Option<Vec<String>>) -> Self {
        self._source_excludes = _source_excludes;
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: Option<Vec<String>>) -> Self {
        self._source_includes = _source_includes;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: Option<bool>) -> Self {
        self.realtime = realtime;
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: Option<bool>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: Option<i64>) -> Self {
        self.version = version;
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: Option<VersionType>) -> Self {
        self.version_type = version_type;
        self
    }
}
impl Sender for GetSource {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<Vec<String>>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<Vec<String>>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum IndexUrlParts {
    IndexId(String, String),
    Index(String),
    IndexType(String, String),
    IndexTypeId(String, String, String),
}
impl IndexUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndexUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(7usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc/");
                p.push_str(id.as_ref());
                p.into()
            }
            IndexUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(6usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc");
                p.into()
            }
            IndexUrlParts::IndexType(ref index, ref ty) => {
                let mut p = String::with_capacity(2usize + index.len() + ty.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.into()
            }
            IndexUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Index<B> {
    client: Elasticsearch,
    parts: IndexUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    op_type: Option<OpType>,
    pipeline: Option<String>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<String>,
    source: Option<String>,
    timeout: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<String>,
}
impl<B> Index<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: IndexUrlParts) -> Self {
        Index {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            op_type: None,
            pipeline: None,
            pretty: None,
            refresh: None,
            routing: None,
            source: None,
            timeout: None,
            version: None,
            version_type: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "only perform the index operation if the last operation that has changed the document has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: Option<i64>) -> Self {
        self.if_primary_term = if_primary_term;
        self
    }
    #[doc = "only perform the index operation if the last operation that has changed the document has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: Option<i64>) -> Self {
        self.if_seq_no = if_seq_no;
        self
    }
    #[doc = "Explicit operation type"]
    pub fn op_type(mut self, op_type: Option<OpType>) -> Self {
        self.op_type = op_type;
        self
    }
    #[doc = "The pipeline id to preprocess incoming documents with"]
    pub fn pipeline(mut self, pipeline: Option<String>) -> Self {
        self.pipeline = pipeline;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: Option<i64>) -> Self {
        self.version = version;
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: Option<VersionType>) -> Self {
        self.version_type = version_type;
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the index operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl<B> Sender for Index<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "if_primary_term", skip_serializing_if = "Option::is_none")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no", skip_serializing_if = "Option::is_none")]
                if_seq_no: Option<i64>,
                #[serde(rename = "op_type", skip_serializing_if = "Option::is_none")]
                op_type: Option<OpType>,
                #[serde(rename = "pipeline", skip_serializing_if = "Option::is_none")]
                pipeline: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                op_type: self.op_type,
                pipeline: self.pipeline,
                pretty: self.pretty,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                version: self.version,
                version_type: self.version_type,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum InfoUrlParts {
    None,
}
impl InfoUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            InfoUrlParts::None => "/".into(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Info {
    client: Elasticsearch,
    parts: InfoUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl Info {
    pub fn new(client: Elasticsearch) -> Self {
        Info {
            client,
            parts: InfoUrlParts::None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for Info {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum MgetUrlParts {
    None,
    Index(String),
    IndexType(String, String),
}
impl MgetUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MgetUrlParts::None => "/_mget".into(),
            MgetUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(7usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_mget");
                p.into()
            }
            MgetUrlParts::IndexType(ref index, ref ty) => {
                let mut p = String::with_capacity(8usize + index.len() + ty.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/_mget");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Mget<B> {
    client: Elasticsearch,
    parts: MgetUrlParts,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    stored_fields: Option<Vec<String>>,
}
impl<B> Mget<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MgetUrlParts) -> Self {
        Mget {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            routing: None,
            source: None,
            stored_fields: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: Option<Vec<String>>) -> Self {
        self._source = _source;
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: Option<Vec<String>>) -> Self {
        self._source_excludes = _source_excludes;
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: Option<Vec<String>>) -> Self {
        self._source_includes = _source_includes;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: Option<bool>) -> Self {
        self.realtime = realtime;
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: Option<bool>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "A comma-separated list of stored fields to return in the response"]
    pub fn stored_fields(mut self, stored_fields: Option<Vec<String>>) -> Self {
        self.stored_fields = stored_fields;
        self
    }
}
impl<B> Sender for Mget<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<Vec<String>>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<Vec<String>>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stored_fields: Option<Vec<String>>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                stored_fields: self.stored_fields,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum MsearchUrlParts {
    None,
    Index(Vec<String>),
    IndexType(Vec<String>, Vec<String>),
}
impl MsearchUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MsearchUrlParts::None => "/_msearch".into(),
            MsearchUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_msearch");
                p.into()
            }
            MsearchUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(11usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_msearch");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Msearch<B> {
    client: Elasticsearch,
    parts: MsearchUrlParts,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    max_concurrent_searches: Option<i64>,
    max_concurrent_shard_requests: Option<i64>,
    pre_filter_shard_size: Option<i64>,
    pretty: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    search_type: Option<SearchType>,
    source: Option<String>,
    typed_keys: Option<bool>,
}
impl<B> Msearch<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MsearchUrlParts) -> Self {
        Msearch {
            client,
            parts,
            body: None,
            ccs_minimize_roundtrips: None,
            error_trace: None,
            filter_path: None,
            human: None,
            max_concurrent_searches: None,
            max_concurrent_shard_requests: None,
            pre_filter_shard_size: None,
            pretty: None,
            rest_total_hits_as_int: None,
            search_type: None,
            source: None,
            typed_keys: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution"]
    pub fn ccs_minimize_roundtrips(mut self, ccs_minimize_roundtrips: Option<bool>) -> Self {
        self.ccs_minimize_roundtrips = ccs_minimize_roundtrips;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Controls the maximum number of concurrent searches the multi search api will execute"]
    pub fn max_concurrent_searches(mut self, max_concurrent_searches: Option<i64>) -> Self {
        self.max_concurrent_searches = max_concurrent_searches;
        self
    }
    #[doc = "The number of concurrent shard requests each sub search executes concurrently per node. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests"]
    pub fn max_concurrent_shard_requests(
        mut self,
        max_concurrent_shard_requests: Option<i64>,
    ) -> Self {
        self.max_concurrent_shard_requests = max_concurrent_shard_requests;
        self
    }
    #[doc = "A threshold that enforces a pre-filter roundtrip to prefilter search shards based on query rewriting if the\u{a0}number of shards the search request expands to exceeds the threshold. This filter roundtrip can limit the number of shards significantly if for instance a shard can not match any documents based on it's rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint."]
    pub fn pre_filter_shard_size(mut self, pre_filter_shard_size: Option<i64>) -> Self {
        self.pre_filter_shard_size = pre_filter_shard_size;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: Option<bool>) -> Self {
        self.rest_total_hits_as_int = rest_total_hits_as_int;
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: Option<SearchType>) -> Self {
        self.search_type = search_type;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: Option<bool>) -> Self {
        self.typed_keys = typed_keys;
        self
    }
}
impl<B> Sender for Msearch<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "ccs_minimize_roundtrips",
                    skip_serializing_if = "Option::is_none"
                )]
                ccs_minimize_roundtrips: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(
                    rename = "max_concurrent_searches",
                    skip_serializing_if = "Option::is_none"
                )]
                max_concurrent_searches: Option<i64>,
                #[serde(
                    rename = "max_concurrent_shard_requests",
                    skip_serializing_if = "Option::is_none"
                )]
                max_concurrent_shard_requests: Option<i64>,
                #[serde(
                    rename = "pre_filter_shard_size",
                    skip_serializing_if = "Option::is_none"
                )]
                pre_filter_shard_size: Option<i64>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(
                    rename = "rest_total_hits_as_int",
                    skip_serializing_if = "Option::is_none"
                )]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "search_type", skip_serializing_if = "Option::is_none")]
                search_type: Option<SearchType>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "typed_keys", skip_serializing_if = "Option::is_none")]
                typed_keys: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                max_concurrent_searches: self.max_concurrent_searches,
                max_concurrent_shard_requests: self.max_concurrent_shard_requests,
                pre_filter_shard_size: self.pre_filter_shard_size,
                pretty: self.pretty,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                search_type: self.search_type,
                source: self.source,
                typed_keys: self.typed_keys,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum MsearchTemplateUrlParts {
    None,
    Index(Vec<String>),
    IndexType(Vec<String>, Vec<String>),
}
impl MsearchTemplateUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MsearchTemplateUrlParts::None => "/_msearch/template".into(),
            MsearchTemplateUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(19usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_msearch/template");
                p.into()
            }
            MsearchTemplateUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(20usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_msearch/template");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct MsearchTemplate<B> {
    client: Elasticsearch,
    parts: MsearchTemplateUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    max_concurrent_searches: Option<i64>,
    pretty: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    search_type: Option<SearchType>,
    source: Option<String>,
    typed_keys: Option<bool>,
}
impl<B> MsearchTemplate<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MsearchTemplateUrlParts) -> Self {
        MsearchTemplate {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            max_concurrent_searches: None,
            pretty: None,
            rest_total_hits_as_int: None,
            search_type: None,
            source: None,
            typed_keys: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Controls the maximum number of concurrent searches the multi search api will execute"]
    pub fn max_concurrent_searches(mut self, max_concurrent_searches: Option<i64>) -> Self {
        self.max_concurrent_searches = max_concurrent_searches;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: Option<bool>) -> Self {
        self.rest_total_hits_as_int = rest_total_hits_as_int;
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: Option<SearchType>) -> Self {
        self.search_type = search_type;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: Option<bool>) -> Self {
        self.typed_keys = typed_keys;
        self
    }
}
impl<B> Sender for MsearchTemplate<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(
                    rename = "max_concurrent_searches",
                    skip_serializing_if = "Option::is_none"
                )]
                max_concurrent_searches: Option<i64>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(
                    rename = "rest_total_hits_as_int",
                    skip_serializing_if = "Option::is_none"
                )]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "search_type", skip_serializing_if = "Option::is_none")]
                search_type: Option<SearchType>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "typed_keys", skip_serializing_if = "Option::is_none")]
                typed_keys: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                max_concurrent_searches: self.max_concurrent_searches,
                pretty: self.pretty,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                search_type: self.search_type,
                source: self.source,
                typed_keys: self.typed_keys,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum MtermvectorsUrlParts {
    None,
    Index(String),
    IndexType(String, String),
}
impl MtermvectorsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MtermvectorsUrlParts::None => "/_mtermvectors".into(),
            MtermvectorsUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(15usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_mtermvectors");
                p.into()
            }
            MtermvectorsUrlParts::IndexType(ref index, ref ty) => {
                let mut p = String::with_capacity(16usize + index.len() + ty.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/_mtermvectors");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Mtermvectors<B> {
    client: Elasticsearch,
    parts: MtermvectorsUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    field_statistics: Option<bool>,
    fields: Option<Vec<String>>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ids: Option<Vec<String>>,
    offsets: Option<bool>,
    payloads: Option<bool>,
    positions: Option<bool>,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    term_statistics: Option<bool>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<B> Mtermvectors<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MtermvectorsUrlParts) -> Self {
        Mtermvectors {
            client,
            parts,
            body: None,
            error_trace: None,
            field_statistics: None,
            fields: None,
            filter_path: None,
            human: None,
            ids: None,
            offsets: None,
            payloads: None,
            positions: None,
            preference: None,
            pretty: None,
            realtime: None,
            routing: None,
            source: None,
            term_statistics: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn field_statistics(mut self, field_statistics: Option<bool>) -> Self {
        self.field_statistics = field_statistics;
        self
    }
    #[doc = "A comma-separated list of fields to return. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn fields(mut self, fields: Option<Vec<String>>) -> Self {
        self.fields = fields;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "A comma-separated list of documents ids. You must define ids as parameter or set \"ids\" or \"docs\" in the request body"]
    pub fn ids(mut self, ids: Option<Vec<String>>) -> Self {
        self.ids = ids;
        self
    }
    #[doc = "Specifies if term offsets should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn offsets(mut self, offsets: Option<bool>) -> Self {
        self.offsets = offsets;
        self
    }
    #[doc = "Specifies if term payloads should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn payloads(mut self, payloads: Option<bool>) -> Self {
        self.payloads = payloads;
        self
    }
    #[doc = "Specifies if term positions should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn positions(mut self, positions: Option<bool>) -> Self {
        self.positions = positions;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random) .Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Specifies if requests are real-time as opposed to near-real-time (default: true)."]
    pub fn realtime(mut self, realtime: Option<bool>) -> Self {
        self.realtime = realtime;
        self
    }
    #[doc = "Specific routing value. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Specifies if total term frequency and document frequency should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn term_statistics(mut self, term_statistics: Option<bool>) -> Self {
        self.term_statistics = term_statistics;
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: Option<i64>) -> Self {
        self.version = version;
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: Option<VersionType>) -> Self {
        self.version_type = version_type;
        self
    }
}
impl<B> Sender for Mtermvectors<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "field_statistics", skip_serializing_if = "Option::is_none")]
                field_statistics: Option<bool>,
                #[serde(
                    rename = "fields",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                fields: Option<Vec<String>>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(
                    rename = "ids",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                ids: Option<Vec<String>>,
                #[serde(rename = "offsets", skip_serializing_if = "Option::is_none")]
                offsets: Option<bool>,
                #[serde(rename = "payloads", skip_serializing_if = "Option::is_none")]
                payloads: Option<bool>,
                #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
                positions: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "term_statistics", skip_serializing_if = "Option::is_none")]
                term_statistics: Option<bool>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                field_statistics: self.field_statistics,
                fields: self.fields,
                filter_path: self.filter_path,
                human: self.human,
                ids: self.ids,
                offsets: self.offsets,
                payloads: self.payloads,
                positions: self.positions,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                routing: self.routing,
                source: self.source,
                term_statistics: self.term_statistics,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum PingUrlParts {
    None,
}
impl PingUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            PingUrlParts::None => "/".into(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Ping {
    client: Elasticsearch,
    parts: PingUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl Ping {
    pub fn new(client: Elasticsearch) -> Self {
        Ping {
            client,
            parts: PingUrlParts::None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for Ping {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Head;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum PutScriptUrlParts {
    Id(String),
    IdContext(String, String),
}
impl PutScriptUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            PutScriptUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(10usize + id.len());
                p.push_str("/_scripts/");
                p.push_str(id.as_ref());
                p.into()
            }
            PutScriptUrlParts::IdContext(ref id, ref context) => {
                let mut p = String::with_capacity(11usize + id.len() + context.len());
                p.push_str("/_scripts/");
                p.push_str(id.as_ref());
                p.push_str("/");
                p.push_str(context.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct PutScript<B> {
    client: Elasticsearch,
    parts: PutScriptUrlParts,
    body: Option<B>,
    context: Option<String>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> PutScript<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: PutScriptUrlParts) -> Self {
        PutScript {
            client,
            parts,
            body: None,
            context: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Context name to compile script against"]
    pub fn context(mut self, context: Option<String>) -> Self {
        self.context = context;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl<B> Sender for PutScript<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
                context: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "master_timeout", skip_serializing_if = "Option::is_none")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                context: self.context,
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum RankEvalUrlParts {
    None,
    Index(Vec<String>),
}
impl RankEvalUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            RankEvalUrlParts::None => "/_rank_eval".into(),
            RankEvalUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(12usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_rank_eval");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct RankEval<B> {
    client: Elasticsearch,
    parts: RankEvalUrlParts,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> RankEval<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: RankEvalUrlParts) -> Self {
        RankEval {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for RankEval<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ReindexUrlParts {
    None,
}
impl ReindexUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ReindexUrlParts::None => "/_reindex".into(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Reindex<B> {
    client: Elasticsearch,
    parts: ReindexUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    max_docs: Option<i64>,
    pretty: Option<bool>,
    refresh: Option<bool>,
    requests_per_second: Option<i64>,
    scroll: Option<String>,
    slices: Option<i64>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
    wait_for_completion: Option<bool>,
}
impl<B> Reindex<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        Reindex {
            client,
            parts: ReindexUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            max_docs: None,
            pretty: None,
            refresh: None,
            requests_per_second: None,
            scroll: None,
            slices: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
            wait_for_completion: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Maximum number of documents to process (default: all documents)"]
    pub fn max_docs(mut self, max_docs: Option<i64>) -> Self {
        self.max_docs = max_docs;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Should the effected indexes be refreshed?"]
    pub fn refresh(mut self, refresh: Option<bool>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "The throttle to set on this request in sub-requests per second. -1 means no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: Option<i64>) -> Self {
        self.requests_per_second = requests_per_second;
        self
    }
    #[doc = "Control how long to keep the search context alive"]
    pub fn scroll(mut self, scroll: Option<String>) -> Self {
        self.scroll = scroll;
        self
    }
    #[doc = "The number of slices this task should be divided into. Defaults to 1 meaning the task isn't sliced into subtasks."]
    pub fn slices(mut self, slices: Option<i64>) -> Self {
        self.slices = slices;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Time each individual bulk request should wait for shards that are unavailable."]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the reindex operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
    #[doc = "Should the request should block until the reindex is complete."]
    pub fn wait_for_completion(mut self, wait_for_completion: Option<bool>) -> Self {
        self.wait_for_completion = wait_for_completion;
        self
    }
}
impl<B> Sender for Reindex<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "max_docs", skip_serializing_if = "Option::is_none")]
                max_docs: Option<i64>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(
                    rename = "requests_per_second",
                    skip_serializing_if = "Option::is_none"
                )]
                requests_per_second: Option<i64>,
                #[serde(rename = "scroll", skip_serializing_if = "Option::is_none")]
                scroll: Option<String>,
                #[serde(rename = "slices", skip_serializing_if = "Option::is_none")]
                slices: Option<i64>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<String>,
                #[serde(
                    rename = "wait_for_completion",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                max_docs: self.max_docs,
                pretty: self.pretty,
                refresh: self.refresh,
                requests_per_second: self.requests_per_second,
                scroll: self.scroll,
                slices: self.slices,
                source: self.source,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ReindexRethrottleUrlParts {
    TaskId(String),
}
impl ReindexRethrottleUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ReindexRethrottleUrlParts::TaskId(ref task_id) => {
                let mut p = String::with_capacity(22usize + task_id.len());
                p.push_str("/_reindex/");
                p.push_str(task_id.as_ref());
                p.push_str("/_rethrottle");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct ReindexRethrottle<B> {
    client: Elasticsearch,
    parts: ReindexRethrottleUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<String>,
}
impl<B> ReindexRethrottle<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: ReindexRethrottleUrlParts) -> Self {
        ReindexRethrottle {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            requests_per_second: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The throttle to set on this request in floating sub-requests per second. -1 means set no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: Option<i64>) -> Self {
        self.requests_per_second = requests_per_second;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for ReindexRethrottle<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(
                    rename = "requests_per_second",
                    skip_serializing_if = "Option::is_none"
                )]
                requests_per_second: Option<i64>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                requests_per_second: self.requests_per_second,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum RenderSearchTemplateUrlParts {
    None,
    Id(String),
}
impl RenderSearchTemplateUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            RenderSearchTemplateUrlParts::None => "/_render/template".into(),
            RenderSearchTemplateUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(18usize + id.len());
                p.push_str("/_render/template/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct RenderSearchTemplate<B> {
    client: Elasticsearch,
    parts: RenderSearchTemplateUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> RenderSearchTemplate<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: RenderSearchTemplateUrlParts) -> Self {
        RenderSearchTemplate {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for RenderSearchTemplate<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ScriptsPainlessExecuteUrlParts {
    None,
}
impl ScriptsPainlessExecuteUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ScriptsPainlessExecuteUrlParts::None => "/_scripts/painless/_execute".into(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct ScriptsPainlessExecute<B> {
    client: Elasticsearch,
    parts: ScriptsPainlessExecuteUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> ScriptsPainlessExecute<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        ScriptsPainlessExecute {
            client,
            parts: ScriptsPainlessExecuteUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for ScriptsPainlessExecute<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ScrollUrlParts {
    None,
    ScrollId(String),
}
impl ScrollUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            ScrollUrlParts::None => "/_search/scroll".into(),
            ScrollUrlParts::ScrollId(ref scroll_id) => {
                let mut p = String::with_capacity(16usize + scroll_id.len());
                p.push_str("/_search/scroll/");
                p.push_str(scroll_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Scroll<B> {
    client: Elasticsearch,
    parts: ScrollUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    scroll: Option<String>,
    scroll_id: Option<String>,
    source: Option<String>,
}
impl<B> Scroll<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: ScrollUrlParts) -> Self {
        Scroll {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            rest_total_hits_as_int: None,
            scroll: None,
            scroll_id: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: Option<bool>) -> Self {
        self.rest_total_hits_as_int = rest_total_hits_as_int;
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: Option<String>) -> Self {
        self.scroll = scroll;
        self
    }
    #[doc = "The scroll ID for scrolled search"]
    pub fn scroll_id(mut self, scroll_id: Option<String>) -> Self {
        self.scroll_id = scroll_id;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for Scroll<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(
                    rename = "rest_total_hits_as_int",
                    skip_serializing_if = "Option::is_none"
                )]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "scroll", skip_serializing_if = "Option::is_none")]
                scroll: Option<String>,
                #[serde(rename = "scroll_id", skip_serializing_if = "Option::is_none")]
                scroll_id: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                scroll: self.scroll,
                scroll_id: self.scroll_id,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum SearchUrlParts {
    None,
    Index(Vec<String>),
    IndexType(Vec<String>, Vec<String>),
}
impl SearchUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SearchUrlParts::None => "/_search".into(),
            SearchUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(9usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_search");
                p.into()
            }
            SearchUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(10usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_search");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Search<B> {
    client: Elasticsearch,
    parts: SearchUrlParts,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    allow_no_indices: Option<bool>,
    allow_partial_search_results: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<String>,
    batched_reduce_size: Option<i64>,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    default_operator: Option<DefaultOperator>,
    df: Option<String>,
    docvalue_fields: Option<Vec<String>>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    explain: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i64>,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    max_concurrent_shard_requests: Option<i64>,
    pre_filter_shard_size: Option<i64>,
    preference: Option<String>,
    pretty: Option<bool>,
    q: Option<String>,
    request_cache: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    routing: Option<Vec<String>>,
    scroll: Option<String>,
    search_type: Option<SearchType>,
    seq_no_primary_term: Option<bool>,
    size: Option<i64>,
    sort: Option<Vec<String>>,
    source: Option<String>,
    stats: Option<Vec<String>>,
    stored_fields: Option<Vec<String>>,
    suggest_field: Option<String>,
    suggest_mode: Option<SuggestMode>,
    suggest_size: Option<i64>,
    suggest_text: Option<String>,
    terminate_after: Option<i64>,
    timeout: Option<String>,
    track_scores: Option<bool>,
    track_total_hits: Option<bool>,
    typed_keys: Option<bool>,
    version: Option<bool>,
}
impl<B> Search<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SearchUrlParts) -> Self {
        Search {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            allow_no_indices: None,
            allow_partial_search_results: None,
            analyze_wildcard: None,
            analyzer: None,
            batched_reduce_size: None,
            body: None,
            ccs_minimize_roundtrips: None,
            default_operator: None,
            df: None,
            docvalue_fields: None,
            error_trace: None,
            expand_wildcards: None,
            explain: None,
            filter_path: None,
            from: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            lenient: None,
            max_concurrent_shard_requests: None,
            pre_filter_shard_size: None,
            preference: None,
            pretty: None,
            q: None,
            request_cache: None,
            rest_total_hits_as_int: None,
            routing: None,
            scroll: None,
            search_type: None,
            seq_no_primary_term: None,
            size: None,
            sort: None,
            source: None,
            stats: None,
            stored_fields: None,
            suggest_field: None,
            suggest_mode: None,
            suggest_size: None,
            suggest_text: None,
            terminate_after: None,
            timeout: None,
            track_scores: None,
            track_total_hits: None,
            typed_keys: None,
            version: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: Option<Vec<String>>) -> Self {
        self._source = _source;
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: Option<Vec<String>>) -> Self {
        self._source_excludes = _source_excludes;
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: Option<Vec<String>>) -> Self {
        self._source_includes = _source_includes;
        self
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Indicate if an error should be returned if there is a partial search failure or timeout"]
    pub fn allow_partial_search_results(
        mut self,
        allow_partial_search_results: Option<bool>,
    ) -> Self {
        self.allow_partial_search_results = allow_partial_search_results;
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: Option<bool>) -> Self {
        self.analyze_wildcard = analyze_wildcard;
        self
    }
    #[doc = "The analyzer to use for the query string"]
    pub fn analyzer(mut self, analyzer: Option<String>) -> Self {
        self.analyzer = analyzer;
        self
    }
    #[doc = "The number of shard results that should be reduced at once on the coordinating node. This value should be used as a protection mechanism to reduce the memory overhead per search request if the potential number of shards in the request can be large."]
    pub fn batched_reduce_size(mut self, batched_reduce_size: Option<i64>) -> Self {
        self.batched_reduce_size = batched_reduce_size;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution"]
    pub fn ccs_minimize_roundtrips(mut self, ccs_minimize_roundtrips: Option<bool>) -> Self {
        self.ccs_minimize_roundtrips = ccs_minimize_roundtrips;
        self
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: Option<DefaultOperator>) -> Self {
        self.default_operator = default_operator;
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: Option<String>) -> Self {
        self.df = df;
        self
    }
    #[doc = "A comma-separated list of fields to return as the docvalue representation of a field for each hit"]
    pub fn docvalue_fields(mut self, docvalue_fields: Option<Vec<String>>) -> Self {
        self.docvalue_fields = docvalue_fields;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "Specify whether to return detailed information about score computation as part of a hit"]
    pub fn explain(mut self, explain: Option<bool>) -> Self {
        self.explain = explain;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Starting offset (default: 0)"]
    pub fn from(mut self, from: Option<i64>) -> Self {
        self.from = from;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether specified concrete, expanded or aliased indices should be ignored when throttled"]
    pub fn ignore_throttled(mut self, ignore_throttled: Option<bool>) -> Self {
        self.ignore_throttled = ignore_throttled;
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: Option<bool>) -> Self {
        self.lenient = lenient;
        self
    }
    #[doc = "The number of concurrent shard requests per node this search executes concurrently. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests"]
    pub fn max_concurrent_shard_requests(
        mut self,
        max_concurrent_shard_requests: Option<i64>,
    ) -> Self {
        self.max_concurrent_shard_requests = max_concurrent_shard_requests;
        self
    }
    #[doc = "A threshold that enforces a pre-filter roundtrip to prefilter search shards based on query rewriting if the\u{a0}number of shards the search request expands to exceeds the threshold. This filter roundtrip can limit the number of shards significantly if for instance a shard can not match any documents based on it's rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint."]
    pub fn pre_filter_shard_size(mut self, pre_filter_shard_size: Option<i64>) -> Self {
        self.pre_filter_shard_size = pre_filter_shard_size;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: Option<String>) -> Self {
        self.q = q;
        self
    }
    #[doc = "Specify if request cache should be used for this request or not, defaults to index level setting"]
    pub fn request_cache(mut self, request_cache: Option<bool>) -> Self {
        self.request_cache = request_cache;
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: Option<bool>) -> Self {
        self.rest_total_hits_as_int = rest_total_hits_as_int;
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: Option<Vec<String>>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: Option<String>) -> Self {
        self.scroll = scroll;
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: Option<SearchType>) -> Self {
        self.search_type = search_type;
        self
    }
    #[doc = "Specify whether to return sequence number and primary term of the last modification of each hit"]
    pub fn seq_no_primary_term(mut self, seq_no_primary_term: Option<bool>) -> Self {
        self.seq_no_primary_term = seq_no_primary_term;
        self
    }
    #[doc = "Number of hits to return (default: 10)"]
    pub fn size(mut self, size: Option<i64>) -> Self {
        self.size = size;
        self
    }
    #[doc = "A comma-separated list of <field>:<direction> pairs"]
    pub fn sort(mut self, sort: Option<Vec<String>>) -> Self {
        self.sort = sort;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Specific 'tag' of the request for logging and statistical purposes"]
    pub fn stats(mut self, stats: Option<Vec<String>>) -> Self {
        self.stats = stats;
        self
    }
    #[doc = "A comma-separated list of stored fields to return as part of a hit"]
    pub fn stored_fields(mut self, stored_fields: Option<Vec<String>>) -> Self {
        self.stored_fields = stored_fields;
        self
    }
    #[doc = "Specify which field to use for suggestions"]
    pub fn suggest_field(mut self, suggest_field: Option<String>) -> Self {
        self.suggest_field = suggest_field;
        self
    }
    #[doc = "Specify suggest mode"]
    pub fn suggest_mode(mut self, suggest_mode: Option<SuggestMode>) -> Self {
        self.suggest_mode = suggest_mode;
        self
    }
    #[doc = "How many suggestions to return in response"]
    pub fn suggest_size(mut self, suggest_size: Option<i64>) -> Self {
        self.suggest_size = suggest_size;
        self
    }
    #[doc = "The source text for which the suggestions should be returned"]
    pub fn suggest_text(mut self, suggest_text: Option<String>) -> Self {
        self.suggest_text = suggest_text;
        self
    }
    #[doc = "The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early."]
    pub fn terminate_after(mut self, terminate_after: Option<i64>) -> Self {
        self.terminate_after = terminate_after;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Whether to calculate and return scores even if they are not used for sorting"]
    pub fn track_scores(mut self, track_scores: Option<bool>) -> Self {
        self.track_scores = track_scores;
        self
    }
    #[doc = "Indicate if the number of documents that match the query should be tracked"]
    pub fn track_total_hits(mut self, track_total_hits: Option<bool>) -> Self {
        self.track_total_hits = track_total_hits;
        self
    }
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: Option<bool>) -> Self {
        self.typed_keys = typed_keys;
        self
    }
    #[doc = "Specify whether to return document version as part of a hit"]
    pub fn version(mut self, version: Option<bool>) -> Self {
        self.version = version;
        self
    }
}
impl<B> Sender for Search<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<Vec<String>>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<Vec<String>>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(
                    rename = "allow_partial_search_results",
                    skip_serializing_if = "Option::is_none"
                )]
                allow_partial_search_results: Option<bool>,
                #[serde(rename = "analyze_wildcard", skip_serializing_if = "Option::is_none")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer", skip_serializing_if = "Option::is_none")]
                analyzer: Option<String>,
                #[serde(
                    rename = "batched_reduce_size",
                    skip_serializing_if = "Option::is_none"
                )]
                batched_reduce_size: Option<i64>,
                #[serde(
                    rename = "ccs_minimize_roundtrips",
                    skip_serializing_if = "Option::is_none"
                )]
                ccs_minimize_roundtrips: Option<bool>,
                #[serde(rename = "default_operator", skip_serializing_if = "Option::is_none")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df", skip_serializing_if = "Option::is_none")]
                df: Option<String>,
                #[serde(
                    rename = "docvalue_fields",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                docvalue_fields: Option<Vec<String>>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "explain", skip_serializing_if = "Option::is_none")]
                explain: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i64>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_throttled", skip_serializing_if = "Option::is_none")]
                ignore_throttled: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient", skip_serializing_if = "Option::is_none")]
                lenient: Option<bool>,
                #[serde(
                    rename = "max_concurrent_shard_requests",
                    skip_serializing_if = "Option::is_none"
                )]
                max_concurrent_shard_requests: Option<i64>,
                #[serde(
                    rename = "pre_filter_shard_size",
                    skip_serializing_if = "Option::is_none"
                )]
                pre_filter_shard_size: Option<i64>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
                q: Option<String>,
                #[serde(rename = "request_cache", skip_serializing_if = "Option::is_none")]
                request_cache: Option<bool>,
                #[serde(
                    rename = "rest_total_hits_as_int",
                    skip_serializing_if = "Option::is_none"
                )]
                rest_total_hits_as_int: Option<bool>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                routing: Option<Vec<String>>,
                #[serde(rename = "scroll", skip_serializing_if = "Option::is_none")]
                scroll: Option<String>,
                #[serde(rename = "search_type", skip_serializing_if = "Option::is_none")]
                search_type: Option<SearchType>,
                #[serde(
                    rename = "seq_no_primary_term",
                    skip_serializing_if = "Option::is_none"
                )]
                seq_no_primary_term: Option<bool>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i64>,
                #[serde(
                    rename = "sort",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                sort: Option<Vec<String>>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(
                    rename = "stats",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stats: Option<Vec<String>>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stored_fields: Option<Vec<String>>,
                #[serde(rename = "suggest_field", skip_serializing_if = "Option::is_none")]
                suggest_field: Option<String>,
                #[serde(rename = "suggest_mode", skip_serializing_if = "Option::is_none")]
                suggest_mode: Option<SuggestMode>,
                #[serde(rename = "suggest_size", skip_serializing_if = "Option::is_none")]
                suggest_size: Option<i64>,
                #[serde(rename = "suggest_text", skip_serializing_if = "Option::is_none")]
                suggest_text: Option<String>,
                #[serde(rename = "terminate_after", skip_serializing_if = "Option::is_none")]
                terminate_after: Option<i64>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
                #[serde(rename = "track_scores", skip_serializing_if = "Option::is_none")]
                track_scores: Option<bool>,
                #[serde(rename = "track_total_hits", skip_serializing_if = "Option::is_none")]
                track_total_hits: Option<bool>,
                #[serde(rename = "typed_keys", skip_serializing_if = "Option::is_none")]
                typed_keys: Option<bool>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                allow_no_indices: self.allow_no_indices,
                allow_partial_search_results: self.allow_partial_search_results,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                batched_reduce_size: self.batched_reduce_size,
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
                default_operator: self.default_operator,
                df: self.df,
                docvalue_fields: self.docvalue_fields,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                explain: self.explain,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                max_concurrent_shard_requests: self.max_concurrent_shard_requests,
                pre_filter_shard_size: self.pre_filter_shard_size,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                request_cache: self.request_cache,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                routing: self.routing,
                scroll: self.scroll,
                search_type: self.search_type,
                seq_no_primary_term: self.seq_no_primary_term,
                size: self.size,
                sort: self.sort,
                source: self.source,
                stats: self.stats,
                stored_fields: self.stored_fields,
                suggest_field: self.suggest_field,
                suggest_mode: self.suggest_mode,
                suggest_size: self.suggest_size,
                suggest_text: self.suggest_text,
                terminate_after: self.terminate_after,
                timeout: self.timeout,
                track_scores: self.track_scores,
                track_total_hits: self.track_total_hits,
                typed_keys: self.typed_keys,
                version: self.version,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum SearchShardsUrlParts {
    None,
    Index(Vec<String>),
}
impl SearchShardsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SearchShardsUrlParts::None => "/_search_shards".into(),
            SearchShardsUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(16usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_search_shards");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct SearchShards<B> {
    client: Elasticsearch,
    parts: SearchShardsUrlParts,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    preference: Option<String>,
    pretty: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
}
impl<B> SearchShards<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SearchShardsUrlParts) -> Self {
        SearchShards {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            local: None,
            preference: None,
            pretty: None,
            routing: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for SearchShards<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
                local: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
                preference: self.preference,
                pretty: self.pretty,
                routing: self.routing,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum SearchTemplateUrlParts {
    None,
    Index(Vec<String>),
    IndexType(Vec<String>, Vec<String>),
}
impl SearchTemplateUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SearchTemplateUrlParts::None => "/_search/template".into(),
            SearchTemplateUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(18usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_search/template");
                p.into()
            }
            SearchTemplateUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(19usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_search/template");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct SearchTemplate<B> {
    client: Elasticsearch,
    parts: SearchTemplateUrlParts,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    explain: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    preference: Option<String>,
    pretty: Option<bool>,
    profile: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    routing: Option<Vec<String>>,
    scroll: Option<String>,
    search_type: Option<SearchType>,
    source: Option<String>,
    typed_keys: Option<bool>,
}
impl<B> SearchTemplate<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SearchTemplateUrlParts) -> Self {
        SearchTemplate {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            explain: None,
            filter_path: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            preference: None,
            pretty: None,
            profile: None,
            rest_total_hits_as_int: None,
            routing: None,
            scroll: None,
            search_type: None,
            source: None,
            typed_keys: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "Specify whether to return detailed information about score computation as part of a hit"]
    pub fn explain(mut self, explain: Option<bool>) -> Self {
        self.explain = explain;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether specified concrete, expanded or aliased indices should be ignored when throttled"]
    pub fn ignore_throttled(mut self, ignore_throttled: Option<bool>) -> Self {
        self.ignore_throttled = ignore_throttled;
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Specify whether to profile the query execution"]
    pub fn profile(mut self, profile: Option<bool>) -> Self {
        self.profile = profile;
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: Option<bool>) -> Self {
        self.rest_total_hits_as_int = rest_total_hits_as_int;
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: Option<Vec<String>>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: Option<String>) -> Self {
        self.scroll = scroll;
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: Option<SearchType>) -> Self {
        self.search_type = search_type;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: Option<bool>) -> Self {
        self.typed_keys = typed_keys;
        self
    }
}
impl<B> Sender for SearchTemplate<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "explain", skip_serializing_if = "Option::is_none")]
                explain: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_throttled", skip_serializing_if = "Option::is_none")]
                ignore_throttled: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
                profile: Option<bool>,
                #[serde(
                    rename = "rest_total_hits_as_int",
                    skip_serializing_if = "Option::is_none"
                )]
                rest_total_hits_as_int: Option<bool>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                routing: Option<Vec<String>>,
                #[serde(rename = "scroll", skip_serializing_if = "Option::is_none")]
                scroll: Option<String>,
                #[serde(rename = "search_type", skip_serializing_if = "Option::is_none")]
                search_type: Option<SearchType>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "typed_keys", skip_serializing_if = "Option::is_none")]
                typed_keys: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                explain: self.explain,
                filter_path: self.filter_path,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                preference: self.preference,
                pretty: self.pretty,
                profile: self.profile,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                routing: self.routing,
                scroll: self.scroll,
                search_type: self.search_type,
                source: self.source,
                typed_keys: self.typed_keys,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TermvectorsUrlParts {
    IndexId(String, String),
    Index(String),
    IndexTypeId(String, String, String),
    IndexType(String, String),
}
impl TermvectorsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            TermvectorsUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(15usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_termvectors/");
                p.push_str(id.as_ref());
                p.into()
            }
            TermvectorsUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(14usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_termvectors");
                p.into()
            }
            TermvectorsUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(16usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.push_str("/_termvectors");
                p.into()
            }
            TermvectorsUrlParts::IndexType(ref index, ref ty) => {
                let mut p = String::with_capacity(15usize + index.len() + ty.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/_termvectors");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Termvectors<B> {
    client: Elasticsearch,
    parts: TermvectorsUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    field_statistics: Option<bool>,
    fields: Option<Vec<String>>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    offsets: Option<bool>,
    payloads: Option<bool>,
    positions: Option<bool>,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    term_statistics: Option<bool>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<B> Termvectors<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: TermvectorsUrlParts) -> Self {
        Termvectors {
            client,
            parts,
            body: None,
            error_trace: None,
            field_statistics: None,
            fields: None,
            filter_path: None,
            human: None,
            offsets: None,
            payloads: None,
            positions: None,
            preference: None,
            pretty: None,
            realtime: None,
            routing: None,
            source: None,
            term_statistics: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned."]
    pub fn field_statistics(mut self, field_statistics: Option<bool>) -> Self {
        self.field_statistics = field_statistics;
        self
    }
    #[doc = "A comma-separated list of fields to return."]
    pub fn fields(mut self, fields: Option<Vec<String>>) -> Self {
        self.fields = fields;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Specifies if term offsets should be returned."]
    pub fn offsets(mut self, offsets: Option<bool>) -> Self {
        self.offsets = offsets;
        self
    }
    #[doc = "Specifies if term payloads should be returned."]
    pub fn payloads(mut self, payloads: Option<bool>) -> Self {
        self.payloads = payloads;
        self
    }
    #[doc = "Specifies if term positions should be returned."]
    pub fn positions(mut self, positions: Option<bool>) -> Self {
        self.positions = positions;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)."]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Specifies if request is real-time as opposed to near-real-time (default: true)."]
    pub fn realtime(mut self, realtime: Option<bool>) -> Self {
        self.realtime = realtime;
        self
    }
    #[doc = "Specific routing value."]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Specifies if total term frequency and document frequency should be returned."]
    pub fn term_statistics(mut self, term_statistics: Option<bool>) -> Self {
        self.term_statistics = term_statistics;
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: Option<i64>) -> Self {
        self.version = version;
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: Option<VersionType>) -> Self {
        self.version_type = version_type;
        self
    }
}
impl<B> Sender for Termvectors<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "field_statistics", skip_serializing_if = "Option::is_none")]
                field_statistics: Option<bool>,
                #[serde(
                    rename = "fields",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                fields: Option<Vec<String>>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "offsets", skip_serializing_if = "Option::is_none")]
                offsets: Option<bool>,
                #[serde(rename = "payloads", skip_serializing_if = "Option::is_none")]
                payloads: Option<bool>,
                #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
                positions: Option<bool>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
                realtime: Option<bool>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "term_statistics", skip_serializing_if = "Option::is_none")]
                term_statistics: Option<bool>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<i64>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                field_statistics: self.field_statistics,
                fields: self.fields,
                filter_path: self.filter_path,
                human: self.human,
                offsets: self.offsets,
                payloads: self.payloads,
                positions: self.positions,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                routing: self.routing,
                source: self.source,
                term_statistics: self.term_statistics,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateUrlParts {
    IndexId(String, String),
    IndexTypeId(String, String, String),
}
impl UpdateUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            UpdateUrlParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(10usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_update/");
                p.push_str(id.as_ref());
                p.into()
            }
            UpdateUrlParts::IndexTypeId(ref index, ref ty, ref id) => {
                let mut p = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/");
                p.push_str(id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Update<B> {
    client: Elasticsearch,
    parts: UpdateUrlParts,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    lang: Option<String>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    retry_on_conflict: Option<i64>,
    routing: Option<String>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl<B> Update<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: UpdateUrlParts) -> Self {
        Update {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            lang: None,
            pretty: None,
            refresh: None,
            retry_on_conflict: None,
            routing: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: Option<Vec<String>>) -> Self {
        self._source = _source;
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: Option<Vec<String>>) -> Self {
        self._source_excludes = _source_excludes;
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: Option<Vec<String>>) -> Self {
        self._source_includes = _source_includes;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "only perform the update operation if the last operation that has changed the document has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: Option<i64>) -> Self {
        self.if_primary_term = if_primary_term;
        self
    }
    #[doc = "only perform the update operation if the last operation that has changed the document has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: Option<i64>) -> Self {
        self.if_seq_no = if_seq_no;
        self
    }
    #[doc = "The script language (default: painless)"]
    pub fn lang(mut self, lang: Option<String>) -> Self {
        self.lang = lang;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "If `true` then refresh the effected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "Specify how many times should the operation be retried when a conflict occurs (default: 0)"]
    pub fn retry_on_conflict(mut self, retry_on_conflict: Option<i64>) -> Self {
        self.retry_on_conflict = retry_on_conflict;
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the update operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl<B> Sender for Update<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<Vec<String>>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<Vec<String>>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "if_primary_term", skip_serializing_if = "Option::is_none")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no", skip_serializing_if = "Option::is_none")]
                if_seq_no: Option<i64>,
                #[serde(rename = "lang", skip_serializing_if = "Option::is_none")]
                lang: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<Refresh>,
                #[serde(rename = "retry_on_conflict", skip_serializing_if = "Option::is_none")]
                retry_on_conflict: Option<i64>,
                #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
                routing: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                lang: self.lang,
                pretty: self.pretty,
                refresh: self.refresh,
                retry_on_conflict: self.retry_on_conflict,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateByQueryUrlParts {
    Index(Vec<String>),
    IndexType(Vec<String>, Vec<String>),
}
impl UpdateByQueryUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            UpdateByQueryUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(18usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_update_by_query");
                p.into()
            }
            UpdateByQueryUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(19usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_update_by_query");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct UpdateByQuery<B> {
    client: Elasticsearch,
    parts: UpdateByQueryUrlParts,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<String>,
    body: Option<B>,
    conflicts: Option<Conflicts>,
    default_operator: Option<DefaultOperator>,
    df: Option<String>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    from: Option<i64>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    max_docs: Option<i64>,
    pipeline: Option<String>,
    preference: Option<String>,
    pretty: Option<bool>,
    q: Option<String>,
    refresh: Option<bool>,
    request_cache: Option<bool>,
    requests_per_second: Option<i64>,
    routing: Option<Vec<String>>,
    scroll: Option<String>,
    scroll_size: Option<i64>,
    search_timeout: Option<String>,
    search_type: Option<SearchType>,
    size: Option<i64>,
    slices: Option<i64>,
    sort: Option<Vec<String>>,
    source: Option<String>,
    stats: Option<Vec<String>>,
    terminate_after: Option<i64>,
    timeout: Option<String>,
    version: Option<bool>,
    version_type: Option<bool>,
    wait_for_active_shards: Option<String>,
    wait_for_completion: Option<bool>,
}
impl<B> UpdateByQuery<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: UpdateByQueryUrlParts) -> Self {
        UpdateByQuery {
            client,
            parts,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            allow_no_indices: None,
            analyze_wildcard: None,
            analyzer: None,
            body: None,
            conflicts: None,
            default_operator: None,
            df: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            from: None,
            human: None,
            ignore_unavailable: None,
            lenient: None,
            max_docs: None,
            pipeline: None,
            preference: None,
            pretty: None,
            q: None,
            refresh: None,
            request_cache: None,
            requests_per_second: None,
            routing: None,
            scroll: None,
            scroll_size: None,
            search_timeout: None,
            search_type: None,
            size: None,
            slices: None,
            sort: None,
            source: None,
            stats: None,
            terminate_after: None,
            timeout: None,
            version: None,
            version_type: None,
            wait_for_active_shards: None,
            wait_for_completion: None,
        }
    }
    #[doc = "True or false to return the _source field or not, or a list of fields to return"]
    pub fn _source(mut self, _source: Option<Vec<String>>) -> Self {
        self._source = _source;
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: Option<Vec<String>>) -> Self {
        self._source_excludes = _source_excludes;
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: Option<Vec<String>>) -> Self {
        self._source_includes = _source_includes;
        self
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: Option<bool>) -> Self {
        self.analyze_wildcard = analyze_wildcard;
        self
    }
    #[doc = "The analyzer to use for the query string"]
    pub fn analyzer(mut self, analyzer: Option<String>) -> Self {
        self.analyzer = analyzer;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "What to do when the update by query hits version conflicts?"]
    pub fn conflicts(mut self, conflicts: Option<Conflicts>) -> Self {
        self.conflicts = conflicts;
        self
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: Option<DefaultOperator>) -> Self {
        self.default_operator = default_operator;
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: Option<String>) -> Self {
        self.df = df;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Starting offset (default: 0)"]
    pub fn from(mut self, from: Option<i64>) -> Self {
        self.from = from;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: Option<bool>) -> Self {
        self.lenient = lenient;
        self
    }
    #[doc = "Maximum number of documents to process (default: all documents)"]
    pub fn max_docs(mut self, max_docs: Option<i64>) -> Self {
        self.max_docs = max_docs;
        self
    }
    #[doc = "Ingest pipeline to set on index requests made by this action. (default: none)"]
    pub fn pipeline(mut self, pipeline: Option<String>) -> Self {
        self.pipeline = pipeline;
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: Option<String>) -> Self {
        self.preference = preference;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: Option<String>) -> Self {
        self.q = q;
        self
    }
    #[doc = "Should the effected indexes be refreshed?"]
    pub fn refresh(mut self, refresh: Option<bool>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "Specify if request cache should be used for this request or not, defaults to index level setting"]
    pub fn request_cache(mut self, request_cache: Option<bool>) -> Self {
        self.request_cache = request_cache;
        self
    }
    #[doc = "The throttle to set on this request in sub-requests per second. -1 means no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: Option<i64>) -> Self {
        self.requests_per_second = requests_per_second;
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: Option<Vec<String>>) -> Self {
        self.routing = routing;
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: Option<String>) -> Self {
        self.scroll = scroll;
        self
    }
    #[doc = "Size on the scroll request powering the update by query"]
    pub fn scroll_size(mut self, scroll_size: Option<i64>) -> Self {
        self.scroll_size = scroll_size;
        self
    }
    #[doc = "Explicit timeout for each search request. Defaults to no timeout."]
    pub fn search_timeout(mut self, search_timeout: Option<String>) -> Self {
        self.search_timeout = search_timeout;
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: Option<SearchType>) -> Self {
        self.search_type = search_type;
        self
    }
    #[doc = "Deprecated, please use `max_docs` instead"]
    pub fn size(mut self, size: Option<i64>) -> Self {
        self.size = size;
        self
    }
    #[doc = "The number of slices this task should be divided into. Defaults to 1 meaning the task isn't sliced into subtasks."]
    pub fn slices(mut self, slices: Option<i64>) -> Self {
        self.slices = slices;
        self
    }
    #[doc = "A comma-separated list of <field>:<direction> pairs"]
    pub fn sort(mut self, sort: Option<Vec<String>>) -> Self {
        self.sort = sort;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Specific 'tag' of the request for logging and statistical purposes"]
    pub fn stats(mut self, stats: Option<Vec<String>>) -> Self {
        self.stats = stats;
        self
    }
    #[doc = "The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early."]
    pub fn terminate_after(mut self, terminate_after: Option<i64>) -> Self {
        self.terminate_after = terminate_after;
        self
    }
    #[doc = "Time each individual bulk request should wait for shards that are unavailable."]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Specify whether to return document version as part of a hit"]
    pub fn version(mut self, version: Option<bool>) -> Self {
        self.version = version;
        self
    }
    #[doc = "Should the document increment the version number (internal) on hit or not (reindex)"]
    pub fn version_type(mut self, version_type: Option<bool>) -> Self {
        self.version_type = version_type;
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the update by query operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
    #[doc = "Should the request should block until the update by query operation is complete."]
    pub fn wait_for_completion(mut self, wait_for_completion: Option<bool>) -> Self {
        self.wait_for_completion = wait_for_completion;
        self
    }
}
impl<B> Sender for UpdateByQuery<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source: Option<Vec<String>>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_excludes: Option<Vec<String>>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "allow_no_indices", skip_serializing_if = "Option::is_none")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard", skip_serializing_if = "Option::is_none")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer", skip_serializing_if = "Option::is_none")]
                analyzer: Option<String>,
                #[serde(rename = "conflicts", skip_serializing_if = "Option::is_none")]
                conflicts: Option<Conflicts>,
                #[serde(rename = "default_operator", skip_serializing_if = "Option::is_none")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df", skip_serializing_if = "Option::is_none")]
                df: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards", skip_serializing_if = "Option::is_none")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i64>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable", skip_serializing_if = "Option::is_none")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient", skip_serializing_if = "Option::is_none")]
                lenient: Option<bool>,
                #[serde(rename = "max_docs", skip_serializing_if = "Option::is_none")]
                max_docs: Option<i64>,
                #[serde(rename = "pipeline", skip_serializing_if = "Option::is_none")]
                pipeline: Option<String>,
                #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
                preference: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
                q: Option<String>,
                #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
                refresh: Option<bool>,
                #[serde(rename = "request_cache", skip_serializing_if = "Option::is_none")]
                request_cache: Option<bool>,
                #[serde(
                    rename = "requests_per_second",
                    skip_serializing_if = "Option::is_none"
                )]
                requests_per_second: Option<i64>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                routing: Option<Vec<String>>,
                #[serde(rename = "scroll", skip_serializing_if = "Option::is_none")]
                scroll: Option<String>,
                #[serde(rename = "scroll_size", skip_serializing_if = "Option::is_none")]
                scroll_size: Option<i64>,
                #[serde(rename = "search_timeout", skip_serializing_if = "Option::is_none")]
                search_timeout: Option<String>,
                #[serde(rename = "search_type", skip_serializing_if = "Option::is_none")]
                search_type: Option<SearchType>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i64>,
                #[serde(rename = "slices", skip_serializing_if = "Option::is_none")]
                slices: Option<i64>,
                #[serde(
                    rename = "sort",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                sort: Option<Vec<String>>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(
                    rename = "stats",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                stats: Option<Vec<String>>,
                #[serde(rename = "terminate_after", skip_serializing_if = "Option::is_none")]
                terminate_after: Option<i64>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
                #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
                version: Option<bool>,
                #[serde(rename = "version_type", skip_serializing_if = "Option::is_none")]
                version_type: Option<bool>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<String>,
                #[serde(
                    rename = "wait_for_completion",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                allow_no_indices: self.allow_no_indices,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                conflicts: self.conflicts,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                max_docs: self.max_docs,
                pipeline: self.pipeline,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                refresh: self.refresh,
                request_cache: self.request_cache,
                requests_per_second: self.requests_per_second,
                routing: self.routing,
                scroll: self.scroll,
                scroll_size: self.scroll_size,
                search_timeout: self.search_timeout,
                search_type: self.search_type,
                size: self.size,
                slices: self.slices,
                sort: self.sort,
                source: self.source,
                stats: self.stats,
                terminate_after: self.terminate_after,
                timeout: self.timeout,
                version: self.version,
                version_type: self.version_type,
                wait_for_active_shards: self.wait_for_active_shards,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateByQueryRethrottleUrlParts {
    TaskId(String),
}
impl UpdateByQueryRethrottleUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            UpdateByQueryRethrottleUrlParts::TaskId(ref task_id) => {
                let mut p = String::with_capacity(30usize + task_id.len());
                p.push_str("/_update_by_query/");
                p.push_str(task_id.as_ref());
                p.push_str("/_rethrottle");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct UpdateByQueryRethrottle<B> {
    client: Elasticsearch,
    parts: UpdateByQueryRethrottleUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<String>,
}
impl<B> UpdateByQueryRethrottle<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: UpdateByQueryRethrottleUrlParts) -> Self {
        UpdateByQueryRethrottle {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            requests_per_second: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The throttle to set on this request in floating sub-requests per second. -1 means set no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: Option<i64>) -> Self {
        self.requests_per_second = requests_per_second;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for UpdateByQueryRethrottle<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(
                    rename = "requests_per_second",
                    skip_serializing_if = "Option::is_none"
                )]
                requests_per_second: Option<i64>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                requests_per_second: self.requests_per_second,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
impl Elasticsearch {
    #[doc = "Allows to perform multiple index/update/delete operations in a single request."]
    pub fn bulk<B>(&self, parts: BulkUrlParts) -> Bulk<B>
    where
        B: Serialize,
    {
        Bulk::new(self.clone(), parts)
    }
    #[doc = "Explicitly clears the search context for a scroll."]
    pub fn clear_scroll<B>(&self, parts: ClearScrollUrlParts) -> ClearScroll<B>
    where
        B: Serialize,
    {
        ClearScroll::new(self.clone(), parts)
    }
    #[doc = "Returns number of documents matching a query."]
    pub fn count<B>(&self, parts: CountUrlParts) -> Count<B>
    where
        B: Serialize,
    {
        Count::new(self.clone(), parts)
    }
    #[doc = "Creates a new document in the index.\n\nReturns a 409 response when a document with a same ID already exists in the index."]
    pub fn create<B>(&self, parts: CreateUrlParts) -> Create<B>
    where
        B: Serialize,
    {
        Create::new(self.clone(), parts)
    }
    #[doc = "Removes a document from the index."]
    pub fn delete(&self, parts: DeleteUrlParts) -> Delete {
        Delete::new(self.clone(), parts)
    }
    #[doc = "Deletes documents matching the provided query."]
    pub fn delete_by_query<B>(&self, parts: DeleteByQueryUrlParts) -> DeleteByQuery<B>
    where
        B: Serialize,
    {
        DeleteByQuery::new(self.clone(), parts)
    }
    #[doc = "Changes the number of requests per second for a particular Delete By Query operation."]
    pub fn delete_by_query_rethrottle<B>(
        &self,
        parts: DeleteByQueryRethrottleUrlParts,
    ) -> DeleteByQueryRethrottle<B>
    where
        B: Serialize,
    {
        DeleteByQueryRethrottle::new(self.clone(), parts)
    }
    #[doc = "Deletes a script."]
    pub fn delete_script(&self, parts: DeleteScriptUrlParts) -> DeleteScript {
        DeleteScript::new(self.clone(), parts)
    }
    #[doc = "Returns information about whether a document exists in an index."]
    pub fn exists(&self, parts: ExistsUrlParts) -> Exists {
        Exists::new(self.clone(), parts)
    }
    #[doc = "Returns information about whether a document source exists in an index."]
    pub fn exists_source(&self, parts: ExistsSourceUrlParts) -> ExistsSource {
        ExistsSource::new(self.clone(), parts)
    }
    #[doc = "Returns information about why a specific matches (or doesn't match) a query."]
    pub fn explain<B>(&self, parts: ExplainUrlParts) -> Explain<B>
    where
        B: Serialize,
    {
        Explain::new(self.clone(), parts)
    }
    #[doc = "Returns the information about the capabilities of fields among multiple indices."]
    pub fn field_caps<B>(&self, parts: FieldCapsUrlParts) -> FieldCaps<B>
    where
        B: Serialize,
    {
        FieldCaps::new(self.clone(), parts)
    }
    #[doc = "Returns a document."]
    pub fn get(&self, parts: GetUrlParts) -> Get {
        Get::new(self.clone(), parts)
    }
    #[doc = "Returns a script."]
    pub fn get_script(&self, parts: GetScriptUrlParts) -> GetScript {
        GetScript::new(self.clone(), parts)
    }
    #[doc = "Returns the source of a document."]
    pub fn get_source(&self, parts: GetSourceUrlParts) -> GetSource {
        GetSource::new(self.clone(), parts)
    }
    #[doc = "Creates or updates a document in an index."]
    pub fn index<B>(&self, parts: IndexUrlParts) -> Index<B>
    where
        B: Serialize,
    {
        Index::new(self.clone(), parts)
    }
    #[doc = "Returns basic information about the cluster."]
    pub fn info(&self) -> Info {
        Info::new(self.clone())
    }
    #[doc = "Allows to get multiple documents in one request."]
    pub fn mget<B>(&self, parts: MgetUrlParts) -> Mget<B>
    where
        B: Serialize,
    {
        Mget::new(self.clone(), parts)
    }
    #[doc = "Allows to execute several search operations in one request."]
    pub fn msearch<B>(&self, parts: MsearchUrlParts) -> Msearch<B>
    where
        B: Serialize,
    {
        Msearch::new(self.clone(), parts)
    }
    #[doc = "Allows to execute several search template operations in one request."]
    pub fn msearch_template<B>(&self, parts: MsearchTemplateUrlParts) -> MsearchTemplate<B>
    where
        B: Serialize,
    {
        MsearchTemplate::new(self.clone(), parts)
    }
    #[doc = "Returns multiple termvectors in one request."]
    pub fn mtermvectors<B>(&self, parts: MtermvectorsUrlParts) -> Mtermvectors<B>
    where
        B: Serialize,
    {
        Mtermvectors::new(self.clone(), parts)
    }
    #[doc = "Returns whether the cluster is running."]
    pub fn ping(&self) -> Ping {
        Ping::new(self.clone())
    }
    #[doc = "Creates or updates a script."]
    pub fn put_script<B>(&self, parts: PutScriptUrlParts) -> PutScript<B>
    where
        B: Serialize,
    {
        PutScript::new(self.clone(), parts)
    }
    #[doc = "Allows to evaluate the quality of ranked search results over a set of typical search queries"]
    pub fn rank_eval<B>(&self, parts: RankEvalUrlParts) -> RankEval<B>
    where
        B: Serialize,
    {
        RankEval::new(self.clone(), parts)
    }
    #[doc = "Allows to copy documents from one index to another, optionally filtering the source\ndocuments by a query, changing the destination index settings, or fetching the\ndocuments from a remote cluster."]
    pub fn reindex<B>(&self) -> Reindex<B>
    where
        B: Serialize,
    {
        Reindex::new(self.clone())
    }
    #[doc = "Changes the number of requests per second for a particular Reindex operation."]
    pub fn reindex_rethrottle<B>(&self, parts: ReindexRethrottleUrlParts) -> ReindexRethrottle<B>
    where
        B: Serialize,
    {
        ReindexRethrottle::new(self.clone(), parts)
    }
    #[doc = "Allows to use the Mustache language to pre-render a search definition."]
    pub fn render_search_template<B>(
        &self,
        parts: RenderSearchTemplateUrlParts,
    ) -> RenderSearchTemplate<B>
    where
        B: Serialize,
    {
        RenderSearchTemplate::new(self.clone(), parts)
    }
    #[doc = "Allows an arbitrary script to be executed and a result to be returned"]
    pub fn scripts_painless_execute<B>(&self) -> ScriptsPainlessExecute<B>
    where
        B: Serialize,
    {
        ScriptsPainlessExecute::new(self.clone())
    }
    #[doc = "Allows to retrieve a large numbers of results from a single search request."]
    pub fn scroll<B>(&self, parts: ScrollUrlParts) -> Scroll<B>
    where
        B: Serialize,
    {
        Scroll::new(self.clone(), parts)
    }
    #[doc = "Returns results matching a query."]
    pub fn search<B>(&self, parts: SearchUrlParts) -> Search<B>
    where
        B: Serialize,
    {
        Search::new(self.clone(), parts)
    }
    #[doc = "Returns information about the indices and shards that a search request would be executed against."]
    pub fn search_shards<B>(&self, parts: SearchShardsUrlParts) -> SearchShards<B>
    where
        B: Serialize,
    {
        SearchShards::new(self.clone(), parts)
    }
    #[doc = "Allows to use the Mustache language to pre-render a search definition."]
    pub fn search_template<B>(&self, parts: SearchTemplateUrlParts) -> SearchTemplate<B>
    where
        B: Serialize,
    {
        SearchTemplate::new(self.clone(), parts)
    }
    #[doc = "Returns information and statistics about terms in the fields of a particular document."]
    pub fn termvectors<B>(&self, parts: TermvectorsUrlParts) -> Termvectors<B>
    where
        B: Serialize,
    {
        Termvectors::new(self.clone(), parts)
    }
    #[doc = "Updates a document with a script or partial document."]
    pub fn update<B>(&self, parts: UpdateUrlParts) -> Update<B>
    where
        B: Serialize,
    {
        Update::new(self.clone(), parts)
    }
    #[doc = "Performs an update on every document in the index without changing the source,\nfor example to pick up a mapping change."]
    pub fn update_by_query<B>(&self, parts: UpdateByQueryUrlParts) -> UpdateByQuery<B>
    where
        B: Serialize,
    {
        UpdateByQuery::new(self.clone(), parts)
    }
    #[doc = "Changes the number of requests per second for a particular Update By Query operation."]
    pub fn update_by_query_rethrottle<B>(
        &self,
        parts: UpdateByQueryRethrottleUrlParts,
    ) -> UpdateByQueryRethrottle<B>
    where
        B: Serialize,
    {
        UpdateByQueryRethrottle::new(self.clone(), parts)
    }
}