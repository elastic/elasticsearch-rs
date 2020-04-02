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
#[allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
    params::*,
};
use serde::Serialize;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Bulk API"]
pub enum BulkParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
    #[doc = "Index and Type"]
    IndexType(&'b str, &'b str),
}
impl<'b> BulkParts<'b> {
    #[doc = "Builds a relative URL path to the Bulk API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            BulkParts::None => "/_bulk".into(),
            BulkParts::Index(ref index) => {
                let mut p = String::with_capacity(7usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_bulk");
                p.into()
            }
            BulkParts::IndexType(ref index, ref ty) => {
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
#[doc = "Builder for the [Bulk API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-bulk.html)\n\nAllows to perform multiple index/update/delete operations in a single request."]
pub struct Bulk<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: BulkParts<'b>,
    _source: Option<&'b [&'b str]>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pipeline: Option<&'b str>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    ty: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> Bulk<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Bulk] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: BulkParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Bulk {
            client,
            parts,
            headers,
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
    pub fn _source(mut self, _source: &'b [&'b str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "Default list of fields to exclude from the returned _source field, can be overridden on each sub-request"]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "Default list of fields to extract and return from the _source field, can be overridden on each sub-request"]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: Vec<T>) -> Bulk<'a, 'b, NdBody<T>>
    where
        T: Body,
    {
        Bulk {
            client: self.client,
            parts: self.parts,
            body: Some(NdBody(body)),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pipeline: self.pipeline,
            pretty: self.pretty,
            refresh: self.refresh,
            routing: self.routing,
            source: self.source,
            timeout: self.timeout,
            ty: self.ty,
            wait_for_active_shards: self.wait_for_active_shards,
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
    #[doc = "The pipeline id to preprocess incoming documents with"]
    pub fn pipeline(mut self, pipeline: &'b str) -> Self {
        self.pipeline = Some(pipeline);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
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
    #[doc = "Default document type for items which don't provide one"]
    pub fn ty(mut self, ty: &'b str) -> Self {
        self.ty = Some(ty);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the bulk operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Bulk API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_includes: Option<&'b [&'b str]>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pipeline")]
                pipeline: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "type")]
                ty: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Clear Scroll API"]
pub enum ClearScrollParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "ScrollId"]
    ScrollId(&'b [&'b str]),
}
impl<'b> ClearScrollParts<'b> {
    #[doc = "Builds a relative URL path to the Clear Scroll API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClearScrollParts::None => "/_search/scroll".into(),
            ClearScrollParts::ScrollId(ref scroll_id) => {
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
#[doc = "Builder for the [Clear Scroll API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-request-body.html#_clear_scroll_api)\n\nExplicitly clears the search context for a scroll."]
pub struct ClearScroll<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: ClearScrollParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ClearScroll<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClearScroll] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: ClearScrollParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClearScroll {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClearScroll<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClearScroll {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Clear Scroll API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Count API"]
pub enum CountParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> CountParts<'b> {
    #[doc = "Builds a relative URL path to the Count API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CountParts::None => "/_count".into(),
            CountParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(8usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_count");
                p.into()
            }
            CountParts::IndexType(ref index, ref ty) => {
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
#[doc = "Builder for the [Count API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-count.html)\n\nReturns number of documents matching a query."]
pub struct Count<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: CountParts<'b>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    body: Option<B>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    min_score: Option<i64>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    terminate_after: Option<i64>,
}
impl<'a, 'b, B> Count<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Count] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CountParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Count {
            client,
            parts,
            headers,
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
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "The analyzer to use for the query string"]
    pub fn analyzer(mut self, analyzer: &'b str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Count<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Count {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            default_operator: self.default_operator,
            df: self.df,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
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
        }
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: &'b str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete, expanded or aliased indices should be ignored when throttled"]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Include only documents with a specific `_score` value in the result"]
    pub fn min_score(mut self, min_score: i64) -> Self {
        self.min_score = Some(min_score);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
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
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The maximum count for each shard, upon reaching which the query execution will terminate early"]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Creates an asynchronous call to the Count API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer")]
                analyzer: Option<&'b str>,
                #[serde(rename = "default_operator")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df")]
                df: Option<&'b str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_throttled")]
                ignore_throttled: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient")]
                lenient: Option<bool>,
                #[serde(rename = "min_score")]
                min_score: Option<i64>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "q")]
                q: Option<&'b str>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                routing: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "terminate_after")]
                terminate_after: Option<i64>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Create API"]
pub enum CreateParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
    #[doc = "Index, Type and Id"]
    IndexTypeId(&'b str, &'b str, &'b str),
}
impl<'b> CreateParts<'b> {
    #[doc = "Builds a relative URL path to the Create API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CreateParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(10usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_create/");
                p.push_str(id.as_ref());
                p.into()
            }
            CreateParts::IndexTypeId(ref index, ref ty, ref id) => {
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
#[doc = "Builder for the [Create API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-index_.html)\n\nCreates a new document in the index.\n\nReturns a 409 response when a document with a same ID already exists in the index."]
pub struct Create<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: CreateParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pipeline: Option<&'b str>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> Create<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Create] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CreateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Create {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> Create<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Create {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    #[doc = "The pipeline id to preprocess incoming documents with"]
    pub fn pipeline(mut self, pipeline: &'b str) -> Self {
        self.pipeline = Some(pipeline);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
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
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the index operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Create API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
                #[serde(rename = "pipeline")]
                pipeline: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Delete API"]
pub enum DeleteParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
    #[doc = "Index, Type and Id"]
    IndexTypeId(&'b str, &'b str, &'b str),
}
impl<'b> DeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DeleteParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(7usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc/");
                p.push_str(id.as_ref());
                p.into()
            }
            DeleteParts::IndexTypeId(ref index, ref ty, ref id) => {
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
#[doc = "Builder for the [Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-delete.html)\n\nRemoves a document from the index."]
pub struct Delete<'a, 'b> {
    client: &'a Elasticsearch,
    parts: DeleteParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b> Delete<'a, 'b> {
    #[doc = "Creates a new instance of [Delete] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: DeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Delete {
            client,
            parts,
            headers,
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
    #[doc = "only perform the delete operation if the last operation that has changed the document has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "only perform the delete operation if the last operation that has changed the document has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
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
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the delete operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Delete API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
                #[serde(rename = "if_primary_term")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no")]
                if_seq_no: Option<i64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Delete By Query API"]
pub enum DeleteByQueryParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> DeleteByQueryParts<'b> {
    #[doc = "Builds a relative URL path to the Delete By Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DeleteByQueryParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(18usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_delete_by_query");
                p.into()
            }
            DeleteByQueryParts::IndexType(ref index, ref ty) => {
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
#[doc = "Builder for the [Delete By Query API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-delete-by-query.html)\n\nDeletes documents matching the provided query."]
pub struct DeleteByQuery<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: DeleteByQueryParts<'b>,
    _source: Option<&'b [&'b str]>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    body: Option<B>,
    conflicts: Option<Conflicts>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    max_docs: Option<i64>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    refresh: Option<bool>,
    request_cache: Option<bool>,
    requests_per_second: Option<i64>,
    routing: Option<&'b [&'b str]>,
    scroll: Option<&'b str>,
    scroll_size: Option<i64>,
    search_timeout: Option<&'b str>,
    search_type: Option<SearchType>,
    size: Option<i64>,
    slices: Option<i64>,
    sort: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    stats: Option<&'b [&'b str]>,
    terminate_after: Option<i64>,
    timeout: Option<&'b str>,
    version: Option<bool>,
    wait_for_active_shards: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> DeleteByQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [DeleteByQuery] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: DeleteByQueryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        DeleteByQuery {
            client,
            parts,
            headers,
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
    pub fn _source(mut self, _source: &'b [&'b str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "The analyzer to use for the query string"]
    pub fn analyzer(mut self, analyzer: &'b str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> DeleteByQuery<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        DeleteByQuery {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
            headers: self.headers,
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
        }
    }
    #[doc = "What to do when the delete by query hits version conflicts?"]
    pub fn conflicts(mut self, conflicts: Conflicts) -> Self {
        self.conflicts = Some(conflicts);
        self
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: &'b str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Starting offset (default: 0)"]
    pub fn from(mut self, from: i64) -> Self {
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Maximum number of documents to process (default: all documents)"]
    pub fn max_docs(mut self, max_docs: i64) -> Self {
        self.max_docs = Some(max_docs);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
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
    #[doc = "Should the effected indexes be refreshed?"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specify if request cache should be used for this request or not, defaults to index level setting"]
    pub fn request_cache(mut self, request_cache: bool) -> Self {
        self.request_cache = Some(request_cache);
        self
    }
    #[doc = "The throttle for this request in sub-requests per second. -1 means no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: i64) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: &'b str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "Size on the scroll request powering the delete by query"]
    pub fn scroll_size(mut self, scroll_size: i64) -> Self {
        self.scroll_size = Some(scroll_size);
        self
    }
    #[doc = "Explicit timeout for each search request. Defaults to no timeout."]
    pub fn search_timeout(mut self, search_timeout: &'b str) -> Self {
        self.search_timeout = Some(search_timeout);
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "Deprecated, please use `max_docs` instead"]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The number of slices this task should be divided into. Defaults to 1 meaning the task isn't sliced into subtasks."]
    pub fn slices(mut self, slices: i64) -> Self {
        self.slices = Some(slices);
        self
    }
    #[doc = "A comma-separated list of <field>:<direction> pairs"]
    pub fn sort(mut self, sort: &'b [&'b str]) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specific 'tag' of the request for logging and statistical purposes"]
    pub fn stats(mut self, stats: &'b [&'b str]) -> Self {
        self.stats = Some(stats);
        self
    }
    #[doc = "The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early."]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Time each individual bulk request should wait for shards that are unavailable."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Specify whether to return document version as part of a hit"]
    pub fn version(mut self, version: bool) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the delete by query operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Should the request should block until the delete by query is complete."]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Delete By Query API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_includes: Option<&'b [&'b str]>,
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer")]
                analyzer: Option<&'b str>,
                #[serde(rename = "conflicts")]
                conflicts: Option<Conflicts>,
                #[serde(rename = "default_operator")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df")]
                df: Option<&'b str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "from")]
                from: Option<i64>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient")]
                lenient: Option<bool>,
                #[serde(rename = "max_docs")]
                max_docs: Option<i64>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "q")]
                q: Option<&'b str>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "request_cache")]
                request_cache: Option<bool>,
                #[serde(rename = "requests_per_second")]
                requests_per_second: Option<i64>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                routing: Option<&'b [&'b str]>,
                #[serde(rename = "scroll")]
                scroll: Option<&'b str>,
                #[serde(rename = "scroll_size")]
                scroll_size: Option<i64>,
                #[serde(rename = "search_timeout")]
                search_timeout: Option<&'b str>,
                #[serde(rename = "search_type")]
                search_type: Option<SearchType>,
                #[serde(rename = "size")]
                size: Option<i64>,
                #[serde(rename = "slices")]
                slices: Option<i64>,
                #[serde(rename = "sort", serialize_with = "crate::client::serialize_coll_qs")]
                sort: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "stats", serialize_with = "crate::client::serialize_coll_qs")]
                stats: Option<&'b [&'b str]>,
                #[serde(rename = "terminate_after")]
                terminate_after: Option<i64>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "version")]
                version: Option<bool>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Delete By Query Rethrottle API"]
pub enum DeleteByQueryRethrottleParts<'b> {
    #[doc = "TaskId"]
    TaskId(&'b str),
}
impl<'b> DeleteByQueryRethrottleParts<'b> {
    #[doc = "Builds a relative URL path to the Delete By Query Rethrottle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DeleteByQueryRethrottleParts::TaskId(ref task_id) => {
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
#[doc = "Builder for the [Delete By Query Rethrottle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-delete-by-query.html)\n\nChanges the number of requests per second for a particular Delete By Query operation."]
pub struct DeleteByQueryRethrottle<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: DeleteByQueryRethrottleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> DeleteByQueryRethrottle<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [DeleteByQueryRethrottle] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: DeleteByQueryRethrottleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        DeleteByQueryRethrottle {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> DeleteByQueryRethrottle<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        DeleteByQueryRethrottle {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            requests_per_second: self.requests_per_second,
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
    #[doc = "The throttle to set on this request in floating sub-requests per second. -1 means set no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: i64) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Delete By Query Rethrottle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
                #[serde(rename = "requests_per_second")]
                requests_per_second: Option<i64>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Delete Script API"]
pub enum DeleteScriptParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> DeleteScriptParts<'b> {
    #[doc = "Builds a relative URL path to the Delete Script API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DeleteScriptParts::Id(ref id) => {
                let mut p = String::with_capacity(10usize + id.len());
                p.push_str("/_scripts/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Delete Script API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/modules-scripting.html)\n\nDeletes a script."]
pub struct DeleteScript<'a, 'b> {
    client: &'a Elasticsearch,
    parts: DeleteScriptParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> DeleteScript<'a, 'b> {
    #[doc = "Creates a new instance of [DeleteScript] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: DeleteScriptParts<'b>) -> Self {
        let headers = HeaderMap::new();
        DeleteScript {
            client,
            parts,
            headers,
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
    #[doc = "Creates an asynchronous call to the Delete Script API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Exists API"]
pub enum ExistsParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
    #[doc = "Index, Type and Id"]
    IndexTypeId(&'b str, &'b str, &'b str),
}
impl<'b> ExistsParts<'b> {
    #[doc = "Builds a relative URL path to the Exists API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ExistsParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(7usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc/");
                p.push_str(id.as_ref());
                p.into()
            }
            ExistsParts::IndexTypeId(ref index, ref ty, ref id) => {
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
#[doc = "Builder for the [Exists API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-get.html)\n\nReturns information about whether a document exists in an index."]
pub struct Exists<'a, 'b> {
    client: &'a Elasticsearch,
    parts: ExistsParts<'b>,
    _source: Option<&'b [&'b str]>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    stored_fields: Option<&'b [&'b str]>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, 'b> Exists<'a, 'b> {
    #[doc = "Creates a new instance of [Exists] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: ExistsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Exists {
            client,
            parts,
            headers,
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
    pub fn _source(mut self, _source: &'b [&'b str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
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
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "A comma-separated list of stored fields to return in the response"]
    pub fn stored_fields(mut self, stored_fields: &'b [&'b str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous call to the Exists API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_includes: Option<&'b [&'b str]>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                stored_fields: Option<&'b [&'b str]>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Exists Source API"]
pub enum ExistsSourceParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
    #[doc = "Index, Type and Id"]
    IndexTypeId(&'b str, &'b str, &'b str),
}
impl<'b> ExistsSourceParts<'b> {
    #[doc = "Builds a relative URL path to the Exists Source API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ExistsSourceParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(10usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_source/");
                p.push_str(id.as_ref());
                p.into()
            }
            ExistsSourceParts::IndexTypeId(ref index, ref ty, ref id) => {
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
#[doc = "Builder for the [Exists Source API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-get.html)\n\nReturns information about whether a document source exists in an index."]
pub struct ExistsSource<'a, 'b> {
    client: &'a Elasticsearch,
    parts: ExistsSourceParts<'b>,
    _source: Option<&'b [&'b str]>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, 'b> ExistsSource<'a, 'b> {
    #[doc = "Creates a new instance of [ExistsSource] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: ExistsSourceParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ExistsSource {
            client,
            parts,
            headers,
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
    pub fn _source(mut self, _source: &'b [&'b str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
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
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous call to the Exists Source API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_includes: Option<&'b [&'b str]>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Explain API"]
pub enum ExplainParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
    #[doc = "Index, Type and Id"]
    IndexTypeId(&'b str, &'b str, &'b str),
}
impl<'b> ExplainParts<'b> {
    #[doc = "Builds a relative URL path to the Explain API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ExplainParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(11usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_explain/");
                p.push_str(id.as_ref());
                p.into()
            }
            ExplainParts::IndexTypeId(ref index, ref ty, ref id) => {
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
#[doc = "Builder for the [Explain API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-explain.html)\n\nReturns information about why a specific matches (or doesn't match) a query."]
pub struct Explain<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: ExplainParts<'b>,
    _source: Option<&'b [&'b str]>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    body: Option<B>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    lenient: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    stored_fields: Option<&'b [&'b str]>,
}
impl<'a, 'b, B> Explain<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Explain] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: ExplainParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Explain {
            client,
            parts,
            headers,
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
    pub fn _source(mut self, _source: &'b [&'b str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Specify whether wildcards and prefix queries in the query string query should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "The analyzer for the query string query"]
    pub fn analyzer(mut self, analyzer: &'b str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Explain<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Explain {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            default_operator: self.default_operator,
            df: self.df,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            lenient: self.lenient,
            preference: self.preference,
            pretty: self.pretty,
            q: self.q,
            routing: self.routing,
            source: self.source,
            stored_fields: self.stored_fields,
        }
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "The default field for query string query (default: _all)"]
    pub fn df(mut self, df: &'b str) -> Self {
        self.df = Some(df);
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
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
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
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "A comma-separated list of stored fields to return in the response"]
    pub fn stored_fields(mut self, stored_fields: &'b [&'b str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Creates an asynchronous call to the Explain API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_includes: Option<&'b [&'b str]>,
                #[serde(rename = "analyze_wildcard")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer")]
                analyzer: Option<&'b str>,
                #[serde(rename = "default_operator")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df")]
                df: Option<&'b str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "lenient")]
                lenient: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "q")]
                q: Option<&'b str>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                stored_fields: Option<&'b [&'b str]>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Field Caps API"]
pub enum FieldCapsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> FieldCapsParts<'b> {
    #[doc = "Builds a relative URL path to the Field Caps API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FieldCapsParts::None => "/_field_caps".into(),
            FieldCapsParts::Index(ref index) => {
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
#[doc = "Builder for the [Field Caps API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-field-caps.html)\n\nReturns the information about the capabilities of fields among multiple indices."]
pub struct FieldCaps<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: FieldCapsParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    fields: Option<&'b [&'b str]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_unmapped: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> FieldCaps<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [FieldCaps] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: FieldCapsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        FieldCaps {
            client,
            parts,
            headers,
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
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> FieldCaps<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        FieldCaps {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            fields: self.fields,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            include_unmapped: self.include_unmapped,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of field names"]
    pub fn fields(mut self, fields: &'b [&'b str]) -> Self {
        self.fields = Some(fields);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Indicates whether unmapped fields should be included in the response."]
    pub fn include_unmapped(mut self, include_unmapped: bool) -> Self {
        self.include_unmapped = Some(include_unmapped);
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
    #[doc = "Creates an asynchronous call to the Field Caps API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "fields", serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'b [&'b str]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_unmapped")]
                include_unmapped: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Get API"]
pub enum GetParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
    #[doc = "Index, Type and Id"]
    IndexTypeId(&'b str, &'b str, &'b str),
}
impl<'b> GetParts<'b> {
    #[doc = "Builds a relative URL path to the Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GetParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(7usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc/");
                p.push_str(id.as_ref());
                p.into()
            }
            GetParts::IndexTypeId(ref index, ref ty, ref id) => {
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
#[doc = "Builder for the [Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-get.html)\n\nReturns a document."]
pub struct Get<'a, 'b> {
    client: &'a Elasticsearch,
    parts: GetParts<'b>,
    _source: Option<&'b [&'b str]>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    stored_fields: Option<&'b [&'b str]>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, 'b> Get<'a, 'b> {
    #[doc = "Creates a new instance of [Get] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: GetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Get {
            client,
            parts,
            headers,
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
    pub fn _source(mut self, _source: &'b [&'b str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
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
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "A comma-separated list of stored fields to return in the response"]
    pub fn stored_fields(mut self, stored_fields: &'b [&'b str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous call to the Get API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_includes: Option<&'b [&'b str]>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                stored_fields: Option<&'b [&'b str]>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Get Script API"]
pub enum GetScriptParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> GetScriptParts<'b> {
    #[doc = "Builds a relative URL path to the Get Script API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GetScriptParts::Id(ref id) => {
                let mut p = String::with_capacity(10usize + id.len());
                p.push_str("/_scripts/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Get Script API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/modules-scripting.html)\n\nReturns a script."]
pub struct GetScript<'a, 'b> {
    client: &'a Elasticsearch,
    parts: GetScriptParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> GetScript<'a, 'b> {
    #[doc = "Creates a new instance of [GetScript] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: GetScriptParts<'b>) -> Self {
        let headers = HeaderMap::new();
        GetScript {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
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
    #[doc = "Creates an asynchronous call to the Get Script API that can be awaited"]
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Get Source API"]
pub enum GetSourceParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
    #[doc = "Index, Type and Id"]
    IndexTypeId(&'b str, &'b str, &'b str),
}
impl<'b> GetSourceParts<'b> {
    #[doc = "Builds a relative URL path to the Get Source API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GetSourceParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(10usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_source/");
                p.push_str(id.as_ref());
                p.into()
            }
            GetSourceParts::IndexTypeId(ref index, ref ty, ref id) => {
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
#[doc = "Builder for the [Get Source API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-get.html)\n\nReturns the source of a document."]
pub struct GetSource<'a, 'b> {
    client: &'a Elasticsearch,
    parts: GetSourceParts<'b>,
    _source: Option<&'b [&'b str]>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, 'b> GetSource<'a, 'b> {
    #[doc = "Creates a new instance of [GetSource] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: GetSourceParts<'b>) -> Self {
        let headers = HeaderMap::new();
        GetSource {
            client,
            parts,
            headers,
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
    pub fn _source(mut self, _source: &'b [&'b str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
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
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous call to the Get Source API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_includes: Option<&'b [&'b str]>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Index API"]
pub enum IndexParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
    #[doc = "Index"]
    Index(&'b str),
    #[doc = "Index and Type"]
    IndexType(&'b str, &'b str),
    #[doc = "Index, Type and Id"]
    IndexTypeId(&'b str, &'b str, &'b str),
}
impl<'b> IndexParts<'b> {
    #[doc = "Builds a relative URL path to the Index API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndexParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(7usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc/");
                p.push_str(id.as_ref());
                p.into()
            }
            IndexParts::Index(ref index) => {
                let mut p = String::with_capacity(6usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_doc");
                p.into()
            }
            IndexParts::IndexType(ref index, ref ty) => {
                let mut p = String::with_capacity(2usize + index.len() + ty.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.into()
            }
            IndexParts::IndexTypeId(ref index, ref ty, ref id) => {
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
#[doc = "Builder for the [Index API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-index_.html)\n\nCreates or updates a document in an index."]
pub struct Index<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndexParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    op_type: Option<OpType>,
    pipeline: Option<&'b str>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> Index<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Index] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndexParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Index {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> Index<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Index {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    #[doc = "only perform the index operation if the last operation that has changed the document has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "only perform the index operation if the last operation that has changed the document has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
        self
    }
    #[doc = "Explicit operation type. Defaults to `index` for requests with an explicit document ID, and to `create`for requests without an explicit document ID"]
    pub fn op_type(mut self, op_type: OpType) -> Self {
        self.op_type = Some(op_type);
        self
    }
    #[doc = "The pipeline id to preprocess incoming documents with"]
    pub fn pipeline(mut self, pipeline: &'b str) -> Self {
        self.pipeline = Some(pipeline);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
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
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the index operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Index API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
                #[serde(rename = "if_primary_term")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no")]
                if_seq_no: Option<i64>,
                #[serde(rename = "op_type")]
                op_type: Option<OpType>,
                #[serde(rename = "pipeline")]
                pipeline: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Info API"]
pub enum InfoParts {
    #[doc = "No parts"]
    None,
}
impl InfoParts {
    #[doc = "Builds a relative URL path to the Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InfoParts::None => "/".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Info API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/index.html)\n\nReturns basic information about the cluster."]
pub struct Info<'a, 'b> {
    client: &'a Elasticsearch,
    parts: InfoParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> Info<'a, 'b> {
    #[doc = "Creates a new instance of [Info]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        Info {
            client,
            parts: InfoParts::None,
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
    #[doc = "Creates an asynchronous call to the Info API that can be awaited"]
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
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Mget API"]
pub enum MgetParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
    #[doc = "Index and Type"]
    IndexType(&'b str, &'b str),
}
impl<'b> MgetParts<'b> {
    #[doc = "Builds a relative URL path to the Mget API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MgetParts::None => "/_mget".into(),
            MgetParts::Index(ref index) => {
                let mut p = String::with_capacity(7usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_mget");
                p.into()
            }
            MgetParts::IndexType(ref index, ref ty) => {
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
#[doc = "Builder for the [Mget API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-multi-get.html)\n\nAllows to get multiple documents in one request."]
pub struct Mget<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: MgetParts<'b>,
    _source: Option<&'b [&'b str]>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    stored_fields: Option<&'b [&'b str]>,
}
impl<'a, 'b, B> Mget<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Mget] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: MgetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Mget {
            client,
            parts,
            headers,
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
    pub fn _source(mut self, _source: &'b [&'b str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Mget<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Mget {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            preference: self.preference,
            pretty: self.pretty,
            realtime: self.realtime,
            refresh: self.refresh,
            routing: self.routing,
            source: self.source,
            stored_fields: self.stored_fields,
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
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specify whether to perform the operation in realtime or search mode"]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Refresh the shard containing the document before performing the operation"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "A comma-separated list of stored fields to return in the response"]
    pub fn stored_fields(mut self, stored_fields: &'b [&'b str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Creates an asynchronous call to the Mget API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_includes: Option<&'b [&'b str]>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                stored_fields: Option<&'b [&'b str]>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Msearch API"]
pub enum MsearchParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> MsearchParts<'b> {
    #[doc = "Builds a relative URL path to the Msearch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MsearchParts::None => "/_msearch".into(),
            MsearchParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_msearch");
                p.into()
            }
            MsearchParts::IndexType(ref index, ref ty) => {
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
#[doc = "Builder for the [Msearch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-multi-search.html)\n\nAllows to execute several search operations in one request."]
pub struct Msearch<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: MsearchParts<'b>,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    max_concurrent_searches: Option<i64>,
    max_concurrent_shard_requests: Option<i64>,
    pre_filter_shard_size: Option<i64>,
    pretty: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    search_type: Option<SearchType>,
    source: Option<&'b str>,
    typed_keys: Option<bool>,
}
impl<'a, 'b, B> Msearch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Msearch] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: MsearchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Msearch {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: Vec<T>) -> Msearch<'a, 'b, NdBody<T>>
    where
        T: Body,
    {
        Msearch {
            client: self.client,
            parts: self.parts,
            body: Some(NdBody(body)),
            ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            max_concurrent_searches: self.max_concurrent_searches,
            max_concurrent_shard_requests: self.max_concurrent_shard_requests,
            pre_filter_shard_size: self.pre_filter_shard_size,
            pretty: self.pretty,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            search_type: self.search_type,
            source: self.source,
            typed_keys: self.typed_keys,
        }
    }
    #[doc = "Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution"]
    pub fn ccs_minimize_roundtrips(mut self, ccs_minimize_roundtrips: bool) -> Self {
        self.ccs_minimize_roundtrips = Some(ccs_minimize_roundtrips);
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
    #[doc = "Controls the maximum number of concurrent searches the multi search api will execute"]
    pub fn max_concurrent_searches(mut self, max_concurrent_searches: i64) -> Self {
        self.max_concurrent_searches = Some(max_concurrent_searches);
        self
    }
    #[doc = "The number of concurrent shard requests each sub search executes concurrently per node. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests"]
    pub fn max_concurrent_shard_requests(mut self, max_concurrent_shard_requests: i64) -> Self {
        self.max_concurrent_shard_requests = Some(max_concurrent_shard_requests);
        self
    }
    #[doc = "A threshold that enforces a pre-filter roundtrip to prefilter search shards based on query rewriting if the\u{a0}number of shards the search request expands to exceeds the threshold. This filter roundtrip can limit the number of shards significantly if for instance a shard can not match any documents based on it's rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint."]
    pub fn pre_filter_shard_size(mut self, pre_filter_shard_size: i64) -> Self {
        self.pre_filter_shard_size = Some(pre_filter_shard_size);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
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
    #[doc = "Creates an asynchronous call to the Msearch API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "ccs_minimize_roundtrips")]
                ccs_minimize_roundtrips: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "max_concurrent_searches")]
                max_concurrent_searches: Option<i64>,
                #[serde(rename = "max_concurrent_shard_requests")]
                max_concurrent_shard_requests: Option<i64>,
                #[serde(rename = "pre_filter_shard_size")]
                pre_filter_shard_size: Option<i64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "rest_total_hits_as_int")]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "search_type")]
                search_type: Option<SearchType>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "typed_keys")]
                typed_keys: Option<bool>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Msearch Template API"]
pub enum MsearchTemplateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> MsearchTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Msearch Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MsearchTemplateParts::None => "/_msearch/template".into(),
            MsearchTemplateParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(19usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_msearch/template");
                p.into()
            }
            MsearchTemplateParts::IndexType(ref index, ref ty) => {
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
#[doc = "Builder for the [Msearch Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-multi-search.html)\n\nAllows to execute several search template operations in one request."]
pub struct MsearchTemplate<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: MsearchTemplateParts<'b>,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    max_concurrent_searches: Option<i64>,
    pretty: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    search_type: Option<SearchType>,
    source: Option<&'b str>,
    typed_keys: Option<bool>,
}
impl<'a, 'b, B> MsearchTemplate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MsearchTemplate] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: MsearchTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MsearchTemplate {
            client,
            parts,
            headers,
            body: None,
            ccs_minimize_roundtrips: None,
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
    pub fn body<T>(self, body: Vec<T>) -> MsearchTemplate<'a, 'b, NdBody<T>>
    where
        T: Body,
    {
        MsearchTemplate {
            client: self.client,
            parts: self.parts,
            body: Some(NdBody(body)),
            ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            max_concurrent_searches: self.max_concurrent_searches,
            pretty: self.pretty,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            search_type: self.search_type,
            source: self.source,
            typed_keys: self.typed_keys,
        }
    }
    #[doc = "Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution"]
    pub fn ccs_minimize_roundtrips(mut self, ccs_minimize_roundtrips: bool) -> Self {
        self.ccs_minimize_roundtrips = Some(ccs_minimize_roundtrips);
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
    #[doc = "Controls the maximum number of concurrent searches the multi search api will execute"]
    pub fn max_concurrent_searches(mut self, max_concurrent_searches: i64) -> Self {
        self.max_concurrent_searches = Some(max_concurrent_searches);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
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
    #[doc = "Creates an asynchronous call to the Msearch Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "ccs_minimize_roundtrips")]
                ccs_minimize_roundtrips: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "max_concurrent_searches")]
                max_concurrent_searches: Option<i64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "rest_total_hits_as_int")]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "search_type")]
                search_type: Option<SearchType>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "typed_keys")]
                typed_keys: Option<bool>,
            }
            let query_params = QueryParams {
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Mtermvectors API"]
pub enum MtermvectorsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
    #[doc = "Index and Type"]
    IndexType(&'b str, &'b str),
}
impl<'b> MtermvectorsParts<'b> {
    #[doc = "Builds a relative URL path to the Mtermvectors API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MtermvectorsParts::None => "/_mtermvectors".into(),
            MtermvectorsParts::Index(ref index) => {
                let mut p = String::with_capacity(15usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_mtermvectors");
                p.into()
            }
            MtermvectorsParts::IndexType(ref index, ref ty) => {
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
#[doc = "Builder for the [Mtermvectors API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-multi-termvectors.html)\n\nReturns multiple termvectors in one request."]
pub struct Mtermvectors<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: MtermvectorsParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    field_statistics: Option<bool>,
    fields: Option<&'b [&'b str]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ids: Option<&'b [&'b str]>,
    offsets: Option<bool>,
    payloads: Option<bool>,
    positions: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    term_statistics: Option<bool>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, 'b, B> Mtermvectors<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Mtermvectors] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: MtermvectorsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Mtermvectors {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> Mtermvectors<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Mtermvectors {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            field_statistics: self.field_statistics,
            fields: self.fields,
            filter_path: self.filter_path,
            headers: self.headers,
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
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn field_statistics(mut self, field_statistics: bool) -> Self {
        self.field_statistics = Some(field_statistics);
        self
    }
    #[doc = "A comma-separated list of fields to return. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn fields(mut self, fields: &'b [&'b str]) -> Self {
        self.fields = Some(fields);
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
    #[doc = "A comma-separated list of documents ids. You must define ids as parameter or set \"ids\" or \"docs\" in the request body"]
    pub fn ids(mut self, ids: &'b [&'b str]) -> Self {
        self.ids = Some(ids);
        self
    }
    #[doc = "Specifies if term offsets should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn offsets(mut self, offsets: bool) -> Self {
        self.offsets = Some(offsets);
        self
    }
    #[doc = "Specifies if term payloads should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn payloads(mut self, payloads: bool) -> Self {
        self.payloads = Some(payloads);
        self
    }
    #[doc = "Specifies if term positions should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn positions(mut self, positions: bool) -> Self {
        self.positions = Some(positions);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random) .Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specifies if requests are real-time as opposed to near-real-time (default: true)."]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Specific routing value. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specifies if total term frequency and document frequency should be returned. Applies to all returned documents unless otherwise specified in body \"params\" or \"docs\"."]
    pub fn term_statistics(mut self, term_statistics: bool) -> Self {
        self.term_statistics = Some(term_statistics);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous call to the Mtermvectors API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "field_statistics")]
                field_statistics: Option<bool>,
                #[serde(rename = "fields", serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'b [&'b str]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ids", serialize_with = "crate::client::serialize_coll_qs")]
                ids: Option<&'b [&'b str]>,
                #[serde(rename = "offsets")]
                offsets: Option<bool>,
                #[serde(rename = "payloads")]
                payloads: Option<bool>,
                #[serde(rename = "positions")]
                positions: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "term_statistics")]
                term_statistics: Option<bool>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ping API"]
pub enum PingParts {
    #[doc = "No parts"]
    None,
}
impl PingParts {
    #[doc = "Builds a relative URL path to the Ping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            PingParts::None => "/".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/index.html)\n\nReturns whether the cluster is running."]
pub struct Ping<'a, 'b> {
    client: &'a Elasticsearch,
    parts: PingParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> Ping<'a, 'b> {
    #[doc = "Creates a new instance of [Ping]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        Ping {
            client,
            parts: PingParts::None,
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
    #[doc = "Creates an asynchronous call to the Ping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
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
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Put Script API"]
pub enum PutScriptParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "Id and Context"]
    IdContext(&'b str, &'b str),
}
impl<'b> PutScriptParts<'b> {
    #[doc = "Builds a relative URL path to the Put Script API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            PutScriptParts::Id(ref id) => {
                let mut p = String::with_capacity(10usize + id.len());
                p.push_str("/_scripts/");
                p.push_str(id.as_ref());
                p.into()
            }
            PutScriptParts::IdContext(ref id, ref context) => {
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
#[doc = "Builder for the [Put Script API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/modules-scripting.html)\n\nCreates or updates a script."]
pub struct PutScript<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: PutScriptParts<'b>,
    body: Option<B>,
    context: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> PutScript<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [PutScript] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: PutScriptParts<'b>) -> Self {
        let headers = HeaderMap::new();
        PutScript {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> PutScript<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        PutScript {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            context: self.context,
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
    #[doc = "Context name to compile script against"]
    pub fn context(mut self, context: &'b str) -> Self {
        self.context = Some(context);
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
    #[doc = "Creates an asynchronous call to the Put Script API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "context")]
                context: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Reindex API"]
pub enum ReindexParts {
    #[doc = "No parts"]
    None,
}
impl ReindexParts {
    #[doc = "Builds a relative URL path to the Reindex API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReindexParts::None => "/_reindex".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Reindex API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-reindex.html)\n\nAllows to copy documents from one index to another, optionally filtering the source\ndocuments by a query, changing the destination index settings, or fetching the\ndocuments from a remote cluster."]
pub struct Reindex<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: ReindexParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    max_docs: Option<i64>,
    pretty: Option<bool>,
    refresh: Option<bool>,
    requests_per_second: Option<i64>,
    scroll: Option<&'b str>,
    slices: Option<i64>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> Reindex<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Reindex]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        Reindex {
            client,
            parts: ReindexParts::None,
            headers,
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
    pub fn body<T>(self, body: T) -> Reindex<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Reindex {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    #[doc = "Maximum number of documents to process (default: all documents)"]
    pub fn max_docs(mut self, max_docs: i64) -> Self {
        self.max_docs = Some(max_docs);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Should the affected indexes be refreshed?"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The throttle to set on this request in sub-requests per second. -1 means no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: i64) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "Control how long to keep the search context alive"]
    pub fn scroll(mut self, scroll: &'b str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "The number of slices this task should be divided into. Defaults to 1 meaning the task isn't sliced into subtasks."]
    pub fn slices(mut self, slices: i64) -> Self {
        self.slices = Some(slices);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Time each individual bulk request should wait for shards that are unavailable."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the reindex operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Should the request should block until the reindex is complete."]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Reindex API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
                #[serde(rename = "max_docs")]
                max_docs: Option<i64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "requests_per_second")]
                requests_per_second: Option<i64>,
                #[serde(rename = "scroll")]
                scroll: Option<&'b str>,
                #[serde(rename = "slices")]
                slices: Option<i64>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Reindex Rethrottle API"]
pub enum ReindexRethrottleParts<'b> {
    #[doc = "TaskId"]
    TaskId(&'b str),
}
impl<'b> ReindexRethrottleParts<'b> {
    #[doc = "Builds a relative URL path to the Reindex Rethrottle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReindexRethrottleParts::TaskId(ref task_id) => {
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
#[doc = "Builder for the [Reindex Rethrottle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-reindex.html)\n\nChanges the number of requests per second for a particular Reindex operation."]
pub struct ReindexRethrottle<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: ReindexRethrottleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ReindexRethrottle<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ReindexRethrottle] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: ReindexRethrottleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ReindexRethrottle {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> ReindexRethrottle<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ReindexRethrottle {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            requests_per_second: self.requests_per_second,
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
    #[doc = "The throttle to set on this request in floating sub-requests per second. -1 means set no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: i64) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Reindex Rethrottle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
                #[serde(rename = "requests_per_second")]
                requests_per_second: Option<i64>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Render Search Template API"]
pub enum RenderSearchTemplateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> RenderSearchTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Render Search Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RenderSearchTemplateParts::None => "/_render/template".into(),
            RenderSearchTemplateParts::Id(ref id) => {
                let mut p = String::with_capacity(18usize + id.len());
                p.push_str("/_render/template/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Render Search Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-template.html#_validating_templates)\n\nAllows to use the Mustache language to pre-render a search definition."]
pub struct RenderSearchTemplate<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: RenderSearchTemplateParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> RenderSearchTemplate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [RenderSearchTemplate] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: RenderSearchTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        RenderSearchTemplate {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> RenderSearchTemplate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        RenderSearchTemplate {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Render Search Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Scroll API"]
pub enum ScrollParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "ScrollId"]
    ScrollId(&'b str),
}
impl<'b> ScrollParts<'b> {
    #[doc = "Builds a relative URL path to the Scroll API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ScrollParts::None => "/_search/scroll".into(),
            ScrollParts::ScrollId(ref scroll_id) => {
                let mut p = String::with_capacity(16usize + scroll_id.len());
                p.push_str("/_search/scroll/");
                p.push_str(scroll_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Scroll API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-request-body.html#request-body-search-scroll)\n\nAllows to retrieve a large numbers of results from a single search request."]
pub struct Scroll<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: ScrollParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    scroll: Option<&'b str>,
    scroll_id: Option<&'b str>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> Scroll<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Scroll] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: ScrollParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Scroll {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> Scroll<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Scroll {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            scroll: self.scroll,
            scroll_id: self.scroll_id,
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
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: &'b str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "The scroll ID for scrolled search"]
    pub fn scroll_id(mut self, scroll_id: &'b str) -> Self {
        self.scroll_id = Some(scroll_id);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Scroll API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
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
                #[serde(rename = "rest_total_hits_as_int")]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "scroll")]
                scroll: Option<&'b str>,
                #[serde(rename = "scroll_id")]
                scroll_id: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Search API"]
pub enum SearchParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> SearchParts<'b> {
    #[doc = "Builds a relative URL path to the Search API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchParts::None => "/_search".into(),
            SearchParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(9usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_search");
                p.into()
            }
            SearchParts::IndexType(ref index, ref ty) => {
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
#[doc = "Builder for the [Search API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-search.html)\n\nReturns results matching a query."]
pub struct Search<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SearchParts<'b>,
    _source: Option<&'b [&'b str]>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    allow_no_indices: Option<bool>,
    allow_partial_search_results: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    batched_reduce_size: Option<i64>,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    docvalue_fields: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    explain: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    max_concurrent_shard_requests: Option<i64>,
    pre_filter_shard_size: Option<i64>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    request_cache: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    routing: Option<&'b [&'b str]>,
    scroll: Option<&'b str>,
    search_type: Option<SearchType>,
    seq_no_primary_term: Option<bool>,
    size: Option<i64>,
    sort: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    stats: Option<&'b [&'b str]>,
    stored_fields: Option<&'b [&'b str]>,
    suggest_field: Option<&'b str>,
    suggest_mode: Option<SuggestMode>,
    suggest_size: Option<i64>,
    suggest_text: Option<&'b str>,
    terminate_after: Option<i64>,
    timeout: Option<&'b str>,
    track_scores: Option<bool>,
    track_total_hits: Option<TrackTotalHits>,
    typed_keys: Option<bool>,
    version: Option<bool>,
}
impl<'a, 'b, B> Search<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Search] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SearchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Search {
            client,
            parts,
            headers,
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
    pub fn _source(mut self, _source: &'b [&'b str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Indicate if an error should be returned if there is a partial search failure or timeout"]
    pub fn allow_partial_search_results(mut self, allow_partial_search_results: bool) -> Self {
        self.allow_partial_search_results = Some(allow_partial_search_results);
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "The analyzer to use for the query string"]
    pub fn analyzer(mut self, analyzer: &'b str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The number of shard results that should be reduced at once on the coordinating node. This value should be used as a protection mechanism to reduce the memory overhead per search request if the potential number of shards in the request can be large."]
    pub fn batched_reduce_size(mut self, batched_reduce_size: i64) -> Self {
        self.batched_reduce_size = Some(batched_reduce_size);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Search<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Search {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
            headers: self.headers,
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
        }
    }
    #[doc = "Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution"]
    pub fn ccs_minimize_roundtrips(mut self, ccs_minimize_roundtrips: bool) -> Self {
        self.ccs_minimize_roundtrips = Some(ccs_minimize_roundtrips);
        self
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: &'b str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "A comma-separated list of fields to return as the docvalue representation of a field for each hit"]
    pub fn docvalue_fields(mut self, docvalue_fields: &'b [&'b str]) -> Self {
        self.docvalue_fields = Some(docvalue_fields);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "Specify whether to return detailed information about score computation as part of a hit"]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Starting offset (default: 0)"]
    pub fn from(mut self, from: i64) -> Self {
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
    #[doc = "Whether specified concrete, expanded or aliased indices should be ignored when throttled"]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "The number of concurrent shard requests per node this search executes concurrently. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests"]
    pub fn max_concurrent_shard_requests(mut self, max_concurrent_shard_requests: i64) -> Self {
        self.max_concurrent_shard_requests = Some(max_concurrent_shard_requests);
        self
    }
    #[doc = "A threshold that enforces a pre-filter roundtrip to prefilter search shards based on query rewriting if the\u{a0}number of shards the search request expands to exceeds the threshold. This filter roundtrip can limit the number of shards significantly if for instance a shard can not match any documents based on it's rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint."]
    pub fn pre_filter_shard_size(mut self, pre_filter_shard_size: i64) -> Self {
        self.pre_filter_shard_size = Some(pre_filter_shard_size);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
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
    #[doc = "Specify if request cache should be used for this request or not, defaults to index level setting"]
    pub fn request_cache(mut self, request_cache: bool) -> Self {
        self.request_cache = Some(request_cache);
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: &'b str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "Specify whether to return sequence number and primary term of the last modification of each hit"]
    pub fn seq_no_primary_term(mut self, seq_no_primary_term: bool) -> Self {
        self.seq_no_primary_term = Some(seq_no_primary_term);
        self
    }
    #[doc = "Number of hits to return (default: 10)"]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "A comma-separated list of <field>:<direction> pairs"]
    pub fn sort(mut self, sort: &'b [&'b str]) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specific 'tag' of the request for logging and statistical purposes"]
    pub fn stats(mut self, stats: &'b [&'b str]) -> Self {
        self.stats = Some(stats);
        self
    }
    #[doc = "A comma-separated list of stored fields to return as part of a hit"]
    pub fn stored_fields(mut self, stored_fields: &'b [&'b str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Specify which field to use for suggestions"]
    pub fn suggest_field(mut self, suggest_field: &'b str) -> Self {
        self.suggest_field = Some(suggest_field);
        self
    }
    #[doc = "Specify suggest mode"]
    pub fn suggest_mode(mut self, suggest_mode: SuggestMode) -> Self {
        self.suggest_mode = Some(suggest_mode);
        self
    }
    #[doc = "How many suggestions to return in response"]
    pub fn suggest_size(mut self, suggest_size: i64) -> Self {
        self.suggest_size = Some(suggest_size);
        self
    }
    #[doc = "The source text for which the suggestions should be returned"]
    pub fn suggest_text(mut self, suggest_text: &'b str) -> Self {
        self.suggest_text = Some(suggest_text);
        self
    }
    #[doc = "The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early."]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Whether to calculate and return scores even if they are not used for sorting"]
    pub fn track_scores(mut self, track_scores: bool) -> Self {
        self.track_scores = Some(track_scores);
        self
    }
    #[doc = "Indicate if the number of documents that match the query should be tracked"]
    pub fn track_total_hits(mut self, track_total_hits: TrackTotalHits) -> Self {
        self.track_total_hits = Some(track_total_hits);
        self
    }
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "Specify whether to return document version as part of a hit"]
    pub fn version(mut self, version: bool) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Creates an asynchronous call to the Search API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_includes: Option<&'b [&'b str]>,
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "allow_partial_search_results")]
                allow_partial_search_results: Option<bool>,
                #[serde(rename = "analyze_wildcard")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer")]
                analyzer: Option<&'b str>,
                #[serde(rename = "batched_reduce_size")]
                batched_reduce_size: Option<i64>,
                #[serde(rename = "ccs_minimize_roundtrips")]
                ccs_minimize_roundtrips: Option<bool>,
                #[serde(rename = "default_operator")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df")]
                df: Option<&'b str>,
                #[serde(
                    rename = "docvalue_fields",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                docvalue_fields: Option<&'b [&'b str]>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "explain")]
                explain: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "from")]
                from: Option<i64>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_throttled")]
                ignore_throttled: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient")]
                lenient: Option<bool>,
                #[serde(rename = "max_concurrent_shard_requests")]
                max_concurrent_shard_requests: Option<i64>,
                #[serde(rename = "pre_filter_shard_size")]
                pre_filter_shard_size: Option<i64>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "q")]
                q: Option<&'b str>,
                #[serde(rename = "request_cache")]
                request_cache: Option<bool>,
                #[serde(rename = "rest_total_hits_as_int")]
                rest_total_hits_as_int: Option<bool>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                routing: Option<&'b [&'b str]>,
                #[serde(rename = "scroll")]
                scroll: Option<&'b str>,
                #[serde(rename = "search_type")]
                search_type: Option<SearchType>,
                #[serde(rename = "seq_no_primary_term")]
                seq_no_primary_term: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i64>,
                #[serde(rename = "sort", serialize_with = "crate::client::serialize_coll_qs")]
                sort: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "stats", serialize_with = "crate::client::serialize_coll_qs")]
                stats: Option<&'b [&'b str]>,
                #[serde(
                    rename = "stored_fields",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                stored_fields: Option<&'b [&'b str]>,
                #[serde(rename = "suggest_field")]
                suggest_field: Option<&'b str>,
                #[serde(rename = "suggest_mode")]
                suggest_mode: Option<SuggestMode>,
                #[serde(rename = "suggest_size")]
                suggest_size: Option<i64>,
                #[serde(rename = "suggest_text")]
                suggest_text: Option<&'b str>,
                #[serde(rename = "terminate_after")]
                terminate_after: Option<i64>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "track_scores")]
                track_scores: Option<bool>,
                #[serde(rename = "track_total_hits")]
                track_total_hits: Option<TrackTotalHits>,
                #[serde(rename = "typed_keys")]
                typed_keys: Option<bool>,
                #[serde(rename = "version")]
                version: Option<bool>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Search Shards API"]
pub enum SearchShardsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> SearchShardsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Shards API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchShardsParts::None => "/_search_shards".into(),
            SearchShardsParts::Index(ref index) => {
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
#[doc = "Builder for the [Search Shards API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-shards.html)\n\nReturns information about the indices and shards that a search request would be executed against."]
pub struct SearchShards<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SearchShardsParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SearchShards<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchShards] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SearchShardsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchShards {
            client,
            parts,
            headers,
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
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SearchShards<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchShards {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            local: self.local,
            preference: self.preference,
            pretty: self.pretty,
            routing: self.routing,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Search Shards API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Search Template API"]
pub enum SearchTemplateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> SearchTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Search Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchTemplateParts::None => "/_search/template".into(),
            SearchTemplateParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(18usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_search/template");
                p.into()
            }
            SearchTemplateParts::IndexType(ref index, ref ty) => {
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
#[doc = "Builder for the [Search Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-template.html)\n\nAllows to use the Mustache language to pre-render a search definition."]
pub struct SearchTemplate<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SearchTemplateParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    explain: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    profile: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    routing: Option<&'b [&'b str]>,
    scroll: Option<&'b str>,
    search_type: Option<SearchType>,
    source: Option<&'b str>,
    typed_keys: Option<bool>,
}
impl<'a, 'b, B> SearchTemplate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchTemplate] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SearchTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchTemplate {
            client,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            ccs_minimize_roundtrips: None,
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
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SearchTemplate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchTemplate {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            explain: self.explain,
            filter_path: self.filter_path,
            headers: self.headers,
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
        }
    }
    #[doc = "Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution"]
    pub fn ccs_minimize_roundtrips(mut self, ccs_minimize_roundtrips: bool) -> Self {
        self.ccs_minimize_roundtrips = Some(ccs_minimize_roundtrips);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "Specify whether to return detailed information about score computation as part of a hit"]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
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
    #[doc = "Whether specified concrete, expanded or aliased indices should be ignored when throttled"]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specify whether to profile the query execution"]
    pub fn profile(mut self, profile: bool) -> Self {
        self.profile = Some(profile);
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: &'b str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
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
    #[doc = "Creates an asynchronous call to the Search Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "ccs_minimize_roundtrips")]
                ccs_minimize_roundtrips: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "explain")]
                explain: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_throttled")]
                ignore_throttled: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "profile")]
                profile: Option<bool>,
                #[serde(rename = "rest_total_hits_as_int")]
                rest_total_hits_as_int: Option<bool>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                routing: Option<&'b [&'b str]>,
                #[serde(rename = "scroll")]
                scroll: Option<&'b str>,
                #[serde(rename = "search_type")]
                search_type: Option<SearchType>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "typed_keys")]
                typed_keys: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Termvectors API"]
pub enum TermvectorsParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
    #[doc = "Index"]
    Index(&'b str),
    #[doc = "Index, Type and Id"]
    IndexTypeId(&'b str, &'b str, &'b str),
    #[doc = "Index and Type"]
    IndexType(&'b str, &'b str),
}
impl<'b> TermvectorsParts<'b> {
    #[doc = "Builds a relative URL path to the Termvectors API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TermvectorsParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(15usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_termvectors/");
                p.push_str(id.as_ref());
                p.into()
            }
            TermvectorsParts::Index(ref index) => {
                let mut p = String::with_capacity(14usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_termvectors");
                p.into()
            }
            TermvectorsParts::IndexTypeId(ref index, ref ty, ref id) => {
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
            TermvectorsParts::IndexType(ref index, ref ty) => {
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
#[doc = "Builder for the [Termvectors API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-termvectors.html)\n\nReturns information and statistics about terms in the fields of a particular document."]
pub struct Termvectors<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: TermvectorsParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    field_statistics: Option<bool>,
    fields: Option<&'b [&'b str]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    offsets: Option<bool>,
    payloads: Option<bool>,
    positions: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    term_statistics: Option<bool>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, 'b, B> Termvectors<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Termvectors] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: TermvectorsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Termvectors {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> Termvectors<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Termvectors {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            field_statistics: self.field_statistics,
            fields: self.fields,
            filter_path: self.filter_path,
            headers: self.headers,
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
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned."]
    pub fn field_statistics(mut self, field_statistics: bool) -> Self {
        self.field_statistics = Some(field_statistics);
        self
    }
    #[doc = "A comma-separated list of fields to return."]
    pub fn fields(mut self, fields: &'b [&'b str]) -> Self {
        self.fields = Some(fields);
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
    #[doc = "Specifies if term offsets should be returned."]
    pub fn offsets(mut self, offsets: bool) -> Self {
        self.offsets = Some(offsets);
        self
    }
    #[doc = "Specifies if term payloads should be returned."]
    pub fn payloads(mut self, payloads: bool) -> Self {
        self.payloads = Some(payloads);
        self
    }
    #[doc = "Specifies if term positions should be returned."]
    pub fn positions(mut self, positions: bool) -> Self {
        self.positions = Some(positions);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specifies if request is real-time as opposed to near-real-time (default: true)."]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Specific routing value."]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specifies if total term frequency and document frequency should be returned."]
    pub fn term_statistics(mut self, term_statistics: bool) -> Self {
        self.term_statistics = Some(term_statistics);
        self
    }
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Specific version type"]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous call to the Termvectors API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "field_statistics")]
                field_statistics: Option<bool>,
                #[serde(rename = "fields", serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'b [&'b str]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "offsets")]
                offsets: Option<bool>,
                #[serde(rename = "payloads")]
                payloads: Option<bool>,
                #[serde(rename = "positions")]
                positions: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "term_statistics")]
                term_statistics: Option<bool>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Update API"]
pub enum UpdateParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
    #[doc = "Index, Type and Id"]
    IndexTypeId(&'b str, &'b str, &'b str),
}
impl<'b> UpdateParts<'b> {
    #[doc = "Builds a relative URL path to the Update API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            UpdateParts::IndexId(ref index, ref id) => {
                let mut p = String::with_capacity(10usize + index.len() + id.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_update/");
                p.push_str(id.as_ref());
                p.into()
            }
            UpdateParts::IndexTypeId(ref index, ref ty, ref id) => {
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
#[doc = "Builder for the [Update API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-update.html)\n\nUpdates a document with a script or partial document."]
pub struct Update<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: UpdateParts<'b>,
    _source: Option<&'b [&'b str]>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    lang: Option<&'b str>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    retry_on_conflict: Option<i64>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> Update<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Update] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: UpdateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Update {
            client,
            parts,
            headers,
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
    pub fn _source(mut self, _source: &'b [&'b str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Update<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Update {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    #[doc = "only perform the update operation if the last operation that has changed the document has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "only perform the update operation if the last operation that has changed the document has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
        self
    }
    #[doc = "The script language (default: painless)"]
    pub fn lang(mut self, lang: &'b str) -> Self {
        self.lang = Some(lang);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specify how many times should the operation be retried when a conflict occurs (default: 0)"]
    pub fn retry_on_conflict(mut self, retry_on_conflict: i64) -> Self {
        self.retry_on_conflict = Some(retry_on_conflict);
        self
    }
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
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
    #[doc = "Sets the number of shard copies that must be active before proceeding with the update operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Update API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_includes: Option<&'b [&'b str]>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "if_primary_term")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no")]
                if_seq_no: Option<i64>,
                #[serde(rename = "lang")]
                lang: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "retry_on_conflict")]
                retry_on_conflict: Option<i64>,
                #[serde(rename = "routing")]
                routing: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Update By Query API"]
pub enum UpdateByQueryParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> UpdateByQueryParts<'b> {
    #[doc = "Builds a relative URL path to the Update By Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            UpdateByQueryParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(18usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_update_by_query");
                p.into()
            }
            UpdateByQueryParts::IndexType(ref index, ref ty) => {
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
#[doc = "Builder for the [Update By Query API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-update-by-query.html)\n\nPerforms an update on every document in the index without changing the source,\nfor example to pick up a mapping change."]
pub struct UpdateByQuery<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: UpdateByQueryParts<'b>,
    _source: Option<&'b [&'b str]>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    body: Option<B>,
    conflicts: Option<Conflicts>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    max_docs: Option<i64>,
    pipeline: Option<&'b str>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    refresh: Option<bool>,
    request_cache: Option<bool>,
    requests_per_second: Option<i64>,
    routing: Option<&'b [&'b str]>,
    scroll: Option<&'b str>,
    scroll_size: Option<i64>,
    search_timeout: Option<&'b str>,
    search_type: Option<SearchType>,
    size: Option<i64>,
    slices: Option<i64>,
    sort: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    stats: Option<&'b [&'b str]>,
    terminate_after: Option<i64>,
    timeout: Option<&'b str>,
    version: Option<bool>,
    version_type: Option<bool>,
    wait_for_active_shards: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> UpdateByQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [UpdateByQuery] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: UpdateByQueryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        UpdateByQuery {
            client,
            parts,
            headers,
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
    pub fn _source(mut self, _source: &'b [&'b str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A list of fields to exclude from the returned _source field"]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A list of fields to extract and return from the _source field"]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "The analyzer to use for the query string"]
    pub fn analyzer(mut self, analyzer: &'b str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> UpdateByQuery<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        UpdateByQuery {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
            headers: self.headers,
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
        }
    }
    #[doc = "What to do when the update by query hits version conflicts?"]
    pub fn conflicts(mut self, conflicts: Conflicts) -> Self {
        self.conflicts = Some(conflicts);
        self
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: &'b str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Starting offset (default: 0)"]
    pub fn from(mut self, from: i64) -> Self {
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Maximum number of documents to process (default: all documents)"]
    pub fn max_docs(mut self, max_docs: i64) -> Self {
        self.max_docs = Some(max_docs);
        self
    }
    #[doc = "Ingest pipeline to set on index requests made by this action. (default: none)"]
    pub fn pipeline(mut self, pipeline: &'b str) -> Self {
        self.pipeline = Some(pipeline);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on (default: random)"]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
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
    #[doc = "Should the affected indexes be refreshed?"]
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Specify if request cache should be used for this request or not, defaults to index level setting"]
    pub fn request_cache(mut self, request_cache: bool) -> Self {
        self.request_cache = Some(request_cache);
        self
    }
    #[doc = "The throttle to set on this request in sub-requests per second. -1 means no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: i64) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "A comma-separated list of specific routing values"]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search"]
    pub fn scroll(mut self, scroll: &'b str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "Size on the scroll request powering the update by query"]
    pub fn scroll_size(mut self, scroll_size: i64) -> Self {
        self.scroll_size = Some(scroll_size);
        self
    }
    #[doc = "Explicit timeout for each search request. Defaults to no timeout."]
    pub fn search_timeout(mut self, search_timeout: &'b str) -> Self {
        self.search_timeout = Some(search_timeout);
        self
    }
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "Deprecated, please use `max_docs` instead"]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The number of slices this task should be divided into. Defaults to 1 meaning the task isn't sliced into subtasks."]
    pub fn slices(mut self, slices: i64) -> Self {
        self.slices = Some(slices);
        self
    }
    #[doc = "A comma-separated list of <field>:<direction> pairs"]
    pub fn sort(mut self, sort: &'b [&'b str]) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specific 'tag' of the request for logging and statistical purposes"]
    pub fn stats(mut self, stats: &'b [&'b str]) -> Self {
        self.stats = Some(stats);
        self
    }
    #[doc = "The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early."]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Time each individual bulk request should wait for shards that are unavailable."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Specify whether to return document version as part of a hit"]
    pub fn version(mut self, version: bool) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Should the document increment the version number (internal) on hit or not (reindex)"]
    pub fn version_type(mut self, version_type: bool) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Sets the number of shard copies that must be active before proceeding with the update by query operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Should the request should block until the update by query operation is complete."]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Update By Query API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(
                    rename = "_source",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_excludes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(
                    rename = "_source_includes",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                _source_includes: Option<&'b [&'b str]>,
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer")]
                analyzer: Option<&'b str>,
                #[serde(rename = "conflicts")]
                conflicts: Option<Conflicts>,
                #[serde(rename = "default_operator")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df")]
                df: Option<&'b str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "from")]
                from: Option<i64>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient")]
                lenient: Option<bool>,
                #[serde(rename = "max_docs")]
                max_docs: Option<i64>,
                #[serde(rename = "pipeline")]
                pipeline: Option<&'b str>,
                #[serde(rename = "preference")]
                preference: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "q")]
                q: Option<&'b str>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "request_cache")]
                request_cache: Option<bool>,
                #[serde(rename = "requests_per_second")]
                requests_per_second: Option<i64>,
                #[serde(
                    rename = "routing",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                routing: Option<&'b [&'b str]>,
                #[serde(rename = "scroll")]
                scroll: Option<&'b str>,
                #[serde(rename = "scroll_size")]
                scroll_size: Option<i64>,
                #[serde(rename = "search_timeout")]
                search_timeout: Option<&'b str>,
                #[serde(rename = "search_type")]
                search_type: Option<SearchType>,
                #[serde(rename = "size")]
                size: Option<i64>,
                #[serde(rename = "slices")]
                slices: Option<i64>,
                #[serde(rename = "sort", serialize_with = "crate::client::serialize_coll_qs")]
                sort: Option<&'b [&'b str]>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "stats", serialize_with = "crate::client::serialize_coll_qs")]
                stats: Option<&'b [&'b str]>,
                #[serde(rename = "terminate_after")]
                terminate_after: Option<i64>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "version")]
                version: Option<bool>,
                #[serde(rename = "version_type")]
                version_type: Option<bool>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Update By Query Rethrottle API"]
pub enum UpdateByQueryRethrottleParts<'b> {
    #[doc = "TaskId"]
    TaskId(&'b str),
}
impl<'b> UpdateByQueryRethrottleParts<'b> {
    #[doc = "Builds a relative URL path to the Update By Query Rethrottle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            UpdateByQueryRethrottleParts::TaskId(ref task_id) => {
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
#[doc = "Builder for the [Update By Query Rethrottle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-update-by-query.html)\n\nChanges the number of requests per second for a particular Update By Query operation."]
pub struct UpdateByQueryRethrottle<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: UpdateByQueryRethrottleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> UpdateByQueryRethrottle<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [UpdateByQueryRethrottle] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: UpdateByQueryRethrottleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        UpdateByQueryRethrottle {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> UpdateByQueryRethrottle<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        UpdateByQueryRethrottle {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            requests_per_second: self.requests_per_second,
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
    #[doc = "The throttle to set on this request in floating sub-requests per second. -1 means set no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: i64) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Update By Query Rethrottle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
                #[serde(rename = "requests_per_second")]
                requests_per_second: Option<i64>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
impl Elasticsearch {
    #[doc = "[Bulk API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-bulk.html)\n\nAllows to perform multiple index/update/delete operations in a single request."]
    pub fn bulk<'a, 'b>(&'a self, parts: BulkParts<'b>) -> Bulk<'a, 'b, ()> {
        Bulk::new(&self, parts)
    }
    #[doc = "[Clear Scroll API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-request-body.html#_clear_scroll_api)\n\nExplicitly clears the search context for a scroll."]
    pub fn clear_scroll<'a, 'b>(&'a self, parts: ClearScrollParts<'b>) -> ClearScroll<'a, 'b, ()> {
        ClearScroll::new(&self, parts)
    }
    #[doc = "[Count API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-count.html)\n\nReturns number of documents matching a query."]
    pub fn count<'a, 'b>(&'a self, parts: CountParts<'b>) -> Count<'a, 'b, ()> {
        Count::new(&self, parts)
    }
    #[doc = "[Create API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-index_.html)\n\nCreates a new document in the index.\n\nReturns a 409 response when a document with a same ID already exists in the index."]
    pub fn create<'a, 'b>(&'a self, parts: CreateParts<'b>) -> Create<'a, 'b, ()> {
        Create::new(&self, parts)
    }
    #[doc = "[Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-delete.html)\n\nRemoves a document from the index."]
    pub fn delete<'a, 'b>(&'a self, parts: DeleteParts<'b>) -> Delete<'a, 'b> {
        Delete::new(&self, parts)
    }
    #[doc = "[Delete By Query API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-delete-by-query.html)\n\nDeletes documents matching the provided query."]
    pub fn delete_by_query<'a, 'b>(
        &'a self,
        parts: DeleteByQueryParts<'b>,
    ) -> DeleteByQuery<'a, 'b, ()> {
        DeleteByQuery::new(&self, parts)
    }
    #[doc = "[Delete By Query Rethrottle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-delete-by-query.html)\n\nChanges the number of requests per second for a particular Delete By Query operation."]
    pub fn delete_by_query_rethrottle<'a, 'b>(
        &'a self,
        parts: DeleteByQueryRethrottleParts<'b>,
    ) -> DeleteByQueryRethrottle<'a, 'b, ()> {
        DeleteByQueryRethrottle::new(&self, parts)
    }
    #[doc = "[Delete Script API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/modules-scripting.html)\n\nDeletes a script."]
    pub fn delete_script<'a, 'b>(&'a self, parts: DeleteScriptParts<'b>) -> DeleteScript<'a, 'b> {
        DeleteScript::new(&self, parts)
    }
    #[doc = "[Exists API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-get.html)\n\nReturns information about whether a document exists in an index."]
    pub fn exists<'a, 'b>(&'a self, parts: ExistsParts<'b>) -> Exists<'a, 'b> {
        Exists::new(&self, parts)
    }
    #[doc = "[Exists Source API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-get.html)\n\nReturns information about whether a document source exists in an index."]
    pub fn exists_source<'a, 'b>(&'a self, parts: ExistsSourceParts<'b>) -> ExistsSource<'a, 'b> {
        ExistsSource::new(&self, parts)
    }
    #[doc = "[Explain API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-explain.html)\n\nReturns information about why a specific matches (or doesn't match) a query."]
    pub fn explain<'a, 'b>(&'a self, parts: ExplainParts<'b>) -> Explain<'a, 'b, ()> {
        Explain::new(&self, parts)
    }
    #[doc = "[Field Caps API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-field-caps.html)\n\nReturns the information about the capabilities of fields among multiple indices."]
    pub fn field_caps<'a, 'b>(&'a self, parts: FieldCapsParts<'b>) -> FieldCaps<'a, 'b, ()> {
        FieldCaps::new(&self, parts)
    }
    #[doc = "[Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-get.html)\n\nReturns a document."]
    pub fn get<'a, 'b>(&'a self, parts: GetParts<'b>) -> Get<'a, 'b> {
        Get::new(&self, parts)
    }
    #[doc = "[Get Script API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/modules-scripting.html)\n\nReturns a script."]
    pub fn get_script<'a, 'b>(&'a self, parts: GetScriptParts<'b>) -> GetScript<'a, 'b> {
        GetScript::new(&self, parts)
    }
    #[doc = "[Get Source API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-get.html)\n\nReturns the source of a document."]
    pub fn get_source<'a, 'b>(&'a self, parts: GetSourceParts<'b>) -> GetSource<'a, 'b> {
        GetSource::new(&self, parts)
    }
    #[doc = "[Index API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-index_.html)\n\nCreates or updates a document in an index."]
    pub fn index<'a, 'b>(&'a self, parts: IndexParts<'b>) -> Index<'a, 'b, ()> {
        Index::new(&self, parts)
    }
    #[doc = "[Info API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/index.html)\n\nReturns basic information about the cluster."]
    pub fn info<'a, 'b>(&'a self) -> Info<'a, 'b> {
        Info::new(&self)
    }
    #[doc = "[Mget API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-multi-get.html)\n\nAllows to get multiple documents in one request."]
    pub fn mget<'a, 'b>(&'a self, parts: MgetParts<'b>) -> Mget<'a, 'b, ()> {
        Mget::new(&self, parts)
    }
    #[doc = "[Msearch API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-multi-search.html)\n\nAllows to execute several search operations in one request."]
    pub fn msearch<'a, 'b>(&'a self, parts: MsearchParts<'b>) -> Msearch<'a, 'b, ()> {
        Msearch::new(&self, parts)
    }
    #[doc = "[Msearch Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-multi-search.html)\n\nAllows to execute several search template operations in one request."]
    pub fn msearch_template<'a, 'b>(
        &'a self,
        parts: MsearchTemplateParts<'b>,
    ) -> MsearchTemplate<'a, 'b, ()> {
        MsearchTemplate::new(&self, parts)
    }
    #[doc = "[Mtermvectors API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-multi-termvectors.html)\n\nReturns multiple termvectors in one request."]
    pub fn mtermvectors<'a, 'b>(
        &'a self,
        parts: MtermvectorsParts<'b>,
    ) -> Mtermvectors<'a, 'b, ()> {
        Mtermvectors::new(&self, parts)
    }
    #[doc = "[Ping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/index.html)\n\nReturns whether the cluster is running."]
    pub fn ping<'a, 'b>(&'a self) -> Ping<'a, 'b> {
        Ping::new(&self)
    }
    #[doc = "[Put Script API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/modules-scripting.html)\n\nCreates or updates a script."]
    pub fn put_script<'a, 'b>(&'a self, parts: PutScriptParts<'b>) -> PutScript<'a, 'b, ()> {
        PutScript::new(&self, parts)
    }
    #[doc = "[Reindex API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-reindex.html)\n\nAllows to copy documents from one index to another, optionally filtering the source\ndocuments by a query, changing the destination index settings, or fetching the\ndocuments from a remote cluster."]
    pub fn reindex<'a, 'b>(&'a self) -> Reindex<'a, 'b, ()> {
        Reindex::new(&self)
    }
    #[doc = "[Reindex Rethrottle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-reindex.html)\n\nChanges the number of requests per second for a particular Reindex operation."]
    pub fn reindex_rethrottle<'a, 'b>(
        &'a self,
        parts: ReindexRethrottleParts<'b>,
    ) -> ReindexRethrottle<'a, 'b, ()> {
        ReindexRethrottle::new(&self, parts)
    }
    #[doc = "[Render Search Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-template.html#_validating_templates)\n\nAllows to use the Mustache language to pre-render a search definition."]
    pub fn render_search_template<'a, 'b>(
        &'a self,
        parts: RenderSearchTemplateParts<'b>,
    ) -> RenderSearchTemplate<'a, 'b, ()> {
        RenderSearchTemplate::new(&self, parts)
    }
    #[doc = "[Scroll API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-request-body.html#request-body-search-scroll)\n\nAllows to retrieve a large numbers of results from a single search request.\n\n# Examples\n\nTo initiate a scroll, make search API call with a specified `scroll` timeout,\nthen fetch the next set of hits using the `_scroll_id` returned in\nthe response. Once no more hits are returned, clear the scroll.\n\n```rust,norun\n# use elasticsearch::{Elasticsearch, SearchParts, ScrollParts, ClearScrollParts};\n# use serde_json::{json, Value};\n# async fn run() -> Result<(), Box<dyn std::error::Error>> { \n# let client = Elasticsearch::default();\nfn print_hits(hits: &[Value]) {\n    for hit in hits {\n        println!(\n            \"id: '{}', source: '{}', score: '{}'\",\n            hit[\"_id\"].as_str().unwrap(),\n            hit[\"_source\"],\n            hit[\"_score\"].as_f64().unwrap()\n        );\n    }\n}\n\nlet scroll = \"1m\";\nlet mut response = client\n    .search(SearchParts::Index(&[\"tweets\"]))\n    .scroll(scroll)\n    .body(json!({\n        \"query\": {\n            \"match\": {\n                \"body\": {\n                    \"query\": \"Elasticsearch rust\",\n                    \"operator\": \"AND\"\n                }\n            }\n        }\n    }))\n    .send()\n    .await?;\n\nlet mut response_body = response.json::<Value>().await?;\nlet mut scroll_id = response_body[\"_scroll_id\"].as_str().unwrap();\nlet mut hits = response_body[\"hits\"][\"hits\"].as_array().unwrap();\n\nprint_hits(hits);\n\nwhile hits.len() > 0 {\n    response = client\n        .scroll(ScrollParts::None)\n        .body(json!({\n            \"scroll\": scroll,\n            \"scroll_id\": scroll_id\n        }))\n        .send()\n        .await?;\n\n    response_body = response.json::<Value>().await?;\n    scroll_id = response_body[\"_scroll_id\"].as_str().unwrap();\n    hits = response_body[\"hits\"][\"hits\"].as_array().unwrap();\n    print_hits(hits);\n}\n\nresponse = client\n    .clear_scroll(ClearScrollParts::None)\n    .body(json!({\n        \"scroll_id\": scroll_id\n    }))\n    .send()\n    .await?;\n    \n# Ok(())\n# }\n```"]
    pub fn scroll<'a, 'b>(&'a self, parts: ScrollParts<'b>) -> Scroll<'a, 'b, ()> {
        Scroll::new(&self, parts)
    }
    #[doc = "[Search API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-search.html)\n\nReturns results matching a query."]
    pub fn search<'a, 'b>(&'a self, parts: SearchParts<'b>) -> Search<'a, 'b, ()> {
        Search::new(&self, parts)
    }
    #[doc = "[Search Shards API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-shards.html)\n\nReturns information about the indices and shards that a search request would be executed against."]
    pub fn search_shards<'a, 'b>(
        &'a self,
        parts: SearchShardsParts<'b>,
    ) -> SearchShards<'a, 'b, ()> {
        SearchShards::new(&self, parts)
    }
    #[doc = "[Search Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/search-template.html)\n\nAllows to use the Mustache language to pre-render a search definition."]
    pub fn search_template<'a, 'b>(
        &'a self,
        parts: SearchTemplateParts<'b>,
    ) -> SearchTemplate<'a, 'b, ()> {
        SearchTemplate::new(&self, parts)
    }
    #[doc = "[Termvectors API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-termvectors.html)\n\nReturns information and statistics about terms in the fields of a particular document."]
    pub fn termvectors<'a, 'b>(&'a self, parts: TermvectorsParts<'b>) -> Termvectors<'a, 'b, ()> {
        Termvectors::new(&self, parts)
    }
    #[doc = "[Update API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-update.html)\n\nUpdates a document with a script or partial document."]
    pub fn update<'a, 'b>(&'a self, parts: UpdateParts<'b>) -> Update<'a, 'b, ()> {
        Update::new(&self, parts)
    }
    #[doc = "[Update By Query API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-update-by-query.html)\n\nPerforms an update on every document in the index without changing the source,\nfor example to pick up a mapping change."]
    pub fn update_by_query<'a, 'b>(
        &'a self,
        parts: UpdateByQueryParts<'b>,
    ) -> UpdateByQuery<'a, 'b, ()> {
        UpdateByQuery::new(&self, parts)
    }
    #[doc = "[Update By Query Rethrottle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/docs-update-by-query.html)\n\nChanges the number of requests per second for a particular Update By Query operation."]
    pub fn update_by_query_rethrottle<'a, 'b>(
        &'a self,
        parts: UpdateByQueryRethrottleParts<'b>,
    ) -> UpdateByQueryRethrottle<'a, 'b, ()> {
        UpdateByQueryRethrottle::new(&self, parts)
    }
}
