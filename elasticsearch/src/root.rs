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
pub struct Bulk<B> {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Option<String>,
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
    pub fn new(client: Elasticsearch) -> Self {
        Bulk {
            client,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            index: None,
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
        let path = "/_bulk";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "_source")]
                _source: Option<Vec<String>>,
                #[serde(rename = "_source_excludes")]
                _source_excludes: Option<Vec<String>>,
                #[serde(rename = "_source_includes")]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "pipeline")]
                pipeline: Option<String>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "type")]
                ty: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                pipeline: self.pipeline,
                refresh: self.refresh,
                routing: self.routing,
                timeout: self.timeout,
                ty: self.ty,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct ClearScroll<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    scroll_id: Option<Vec<String>>,
    source: Option<String>,
}
impl<B> ClearScroll<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        ClearScroll {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
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
        let path = "/_search/scroll";
        let method = HttpMethod::Delete;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Count<B> {
    client: Elasticsearch,
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
    index: Option<Vec<String>>,
    lenient: Option<bool>,
    min_score: Option<i64>,
    preference: Option<String>,
    pretty: Option<bool>,
    q: Option<String>,
    routing: Option<Vec<String>>,
    source: Option<String>,
    terminate_after: Option<i64>,
    ty: Option<Vec<String>>,
}
impl<B> Count<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        Count {
            client,
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
            index: None,
            lenient: None,
            min_score: None,
            preference: None,
            pretty: None,
            q: None,
            routing: None,
            source: None,
            terminate_after: None,
            ty: None,
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
        let path = "/_count";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer")]
                analyzer: Option<String>,
                #[serde(rename = "default_operator")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df")]
                df: Option<String>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_throttled")]
                ignore_throttled: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient")]
                lenient: Option<bool>,
                #[serde(rename = "min_score")]
                min_score: Option<i64>,
                #[serde(rename = "preference")]
                preference: Option<String>,
                #[serde(rename = "q")]
                q: Option<String>,
                #[serde(rename = "routing")]
                routing: Option<Vec<String>>,
                #[serde(rename = "terminate_after")]
                terminate_after: Option<i64>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                default_operator: self.default_operator,
                df: self.df,
                expand_wildcards: self.expand_wildcards,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                min_score: self.min_score,
                preference: self.preference,
                q: self.q,
                routing: self.routing,
                terminate_after: self.terminate_after,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Create<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    index: String,
    pipeline: Option<String>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<String>,
    source: Option<String>,
    timeout: Option<String>,
    ty: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<String>,
}
impl<B> Create<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String, id: String) -> Self {
        Create {
            client,
            index: index,
            id: id,
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
        let path = "/{index}/_create/{id}";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "pipeline")]
                pipeline: Option<String>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                pipeline: self.pipeline,
                refresh: self.refresh,
                routing: self.routing,
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
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Delete {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    index: String,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<String>,
    source: Option<String>,
    timeout: Option<String>,
    ty: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<String>,
}
impl Delete {
    pub fn new(client: Elasticsearch, index: String, id: String) -> Self {
        Delete {
            client,
            index: index,
            id: id,
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
            ty: None,
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
        let path = "/{index}/_doc/{id}";
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "if_primary_term")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no")]
                if_seq_no: Option<i64>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                refresh: self.refresh,
                routing: self.routing,
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
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct DeleteByQuery<B> {
    client: Elasticsearch,
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
    index: Vec<String>,
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
    ty: Option<Vec<String>>,
    version: Option<bool>,
    wait_for_active_shards: Option<String>,
    wait_for_completion: Option<bool>,
}
impl<B> DeleteByQuery<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        DeleteByQuery {
            client,
            index: index,
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
            ty: None,
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
        let path = "/{index}/_delete_by_query";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "_source")]
                _source: Option<Vec<String>>,
                #[serde(rename = "_source_excludes")]
                _source_excludes: Option<Vec<String>>,
                #[serde(rename = "_source_includes")]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer")]
                analyzer: Option<String>,
                #[serde(rename = "conflicts")]
                conflicts: Option<Conflicts>,
                #[serde(rename = "default_operator")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df")]
                df: Option<String>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "from")]
                from: Option<i64>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient")]
                lenient: Option<bool>,
                #[serde(rename = "max_docs")]
                max_docs: Option<i64>,
                #[serde(rename = "preference")]
                preference: Option<String>,
                #[serde(rename = "q")]
                q: Option<String>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "request_cache")]
                request_cache: Option<bool>,
                #[serde(rename = "requests_per_second")]
                requests_per_second: Option<i64>,
                #[serde(rename = "routing")]
                routing: Option<Vec<String>>,
                #[serde(rename = "scroll")]
                scroll: Option<String>,
                #[serde(rename = "scroll_size")]
                scroll_size: Option<i64>,
                #[serde(rename = "search_timeout")]
                search_timeout: Option<String>,
                #[serde(rename = "search_type")]
                search_type: Option<SearchType>,
                #[serde(rename = "size")]
                size: Option<i64>,
                #[serde(rename = "slices")]
                slices: Option<i64>,
                #[serde(rename = "sort")]
                sort: Option<Vec<String>>,
                #[serde(rename = "stats")]
                stats: Option<Vec<String>>,
                #[serde(rename = "terminate_after")]
                terminate_after: Option<i64>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "version")]
                version: Option<bool>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
                #[serde(rename = "wait_for_completion")]
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
                expand_wildcards: self.expand_wildcards,
                from: self.from,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                max_docs: self.max_docs,
                preference: self.preference,
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
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct DeleteByQueryRethrottle<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<String>,
    task_id: String,
}
impl<B> DeleteByQueryRethrottle<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, task_id: String) -> Self {
        DeleteByQueryRethrottle {
            client,
            task_id: task_id,
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
        let path = "/_delete_by_query/{task_id}/_rethrottle";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "requests_per_second")]
                requests_per_second: Option<i64>,
            }
            let query_params = QueryParamsStruct {
                requests_per_second: self.requests_per_second,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct DeleteScript {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl DeleteScript {
    pub fn new(client: Elasticsearch, id: String) -> Self {
        DeleteScript {
            client,
            id: id,
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
        let path = "/_scripts/{id}";
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                master_timeout: self.master_timeout,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Exists {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    index: String,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    stored_fields: Option<Vec<String>>,
    ty: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl Exists {
    pub fn new(client: Elasticsearch, index: String, id: String) -> Self {
        Exists {
            client,
            index: index,
            id: id,
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
            ty: None,
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
        let path = "/{index}/_doc/{id}";
        let method = HttpMethod::Head;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "_source")]
                _source: Option<Vec<String>>,
                #[serde(rename = "_source_excludes")]
                _source_excludes: Option<Vec<String>>,
                #[serde(rename = "_source_includes")]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "preference")]
                preference: Option<String>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "stored_fields")]
                stored_fields: Option<Vec<String>>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                preference: self.preference,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                stored_fields: self.stored_fields,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct ExistsSource {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    index: String,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    ty: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl ExistsSource {
    pub fn new(client: Elasticsearch, index: String, id: String) -> Self {
        ExistsSource {
            client,
            index: index,
            id: id,
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
            ty: None,
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
        let path = "/{index}/_source/{id}";
        let method = HttpMethod::Head;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "_source")]
                _source: Option<Vec<String>>,
                #[serde(rename = "_source_excludes")]
                _source_excludes: Option<Vec<String>>,
                #[serde(rename = "_source_includes")]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "preference")]
                preference: Option<String>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                preference: self.preference,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Explain<B> {
    client: Elasticsearch,
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
    id: String,
    index: String,
    lenient: Option<bool>,
    preference: Option<String>,
    pretty: Option<bool>,
    q: Option<String>,
    routing: Option<String>,
    source: Option<String>,
    stored_fields: Option<Vec<String>>,
    ty: Option<String>,
}
impl<B> Explain<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String, id: String) -> Self {
        Explain {
            client,
            index: index,
            id: id,
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
            ty: None,
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
        let path = "/{index}/_explain/{id}";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "_source")]
                _source: Option<Vec<String>>,
                #[serde(rename = "_source_excludes")]
                _source_excludes: Option<Vec<String>>,
                #[serde(rename = "_source_includes")]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "analyze_wildcard")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer")]
                analyzer: Option<String>,
                #[serde(rename = "default_operator")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df")]
                df: Option<String>,
                #[serde(rename = "lenient")]
                lenient: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<String>,
                #[serde(rename = "q")]
                q: Option<String>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "stored_fields")]
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
                lenient: self.lenient,
                preference: self.preference,
                q: self.q,
                routing: self.routing,
                stored_fields: self.stored_fields,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct FieldCaps<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    fields: Option<Vec<String>>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_unmapped: Option<bool>,
    index: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> FieldCaps<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        FieldCaps {
            client,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            fields: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            include_unmapped: None,
            index: None,
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
        let path = "/_field_caps";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "fields")]
                fields: Option<Vec<String>>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_unmapped")]
                include_unmapped: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                fields: self.fields,
                ignore_unavailable: self.ignore_unavailable,
                include_unmapped: self.include_unmapped,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Get {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    index: String,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    stored_fields: Option<Vec<String>>,
    ty: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl Get {
    pub fn new(client: Elasticsearch, index: String, id: String) -> Self {
        Get {
            client,
            index: index,
            id: id,
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
            ty: None,
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
        let path = "/{index}/_doc/{id}";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "_source")]
                _source: Option<Vec<String>>,
                #[serde(rename = "_source_excludes")]
                _source_excludes: Option<Vec<String>>,
                #[serde(rename = "_source_includes")]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "preference")]
                preference: Option<String>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "stored_fields")]
                stored_fields: Option<Vec<String>>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                preference: self.preference,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                stored_fields: self.stored_fields,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct GetScript {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl GetScript {
    pub fn new(client: Elasticsearch, id: String) -> Self {
        GetScript {
            client,
            id: id,
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
        let path = "/_scripts/{id}";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                master_timeout: self.master_timeout,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct GetSource {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    index: String,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    ty: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl GetSource {
    pub fn new(client: Elasticsearch, index: String, id: String) -> Self {
        GetSource {
            client,
            index: index,
            id: id,
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
            ty: None,
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
        let path = "/{index}/_source/{id}";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "_source")]
                _source: Option<Vec<String>>,
                #[serde(rename = "_source_excludes")]
                _source_excludes: Option<Vec<String>>,
                #[serde(rename = "_source_includes")]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "preference")]
                preference: Option<String>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                preference: self.preference,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Index<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: Option<String>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    index: String,
    op_type: Option<OpType>,
    pipeline: Option<String>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    routing: Option<String>,
    source: Option<String>,
    timeout: Option<String>,
    ty: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<String>,
}
impl<B> Index<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String) -> Self {
        Index {
            client,
            index: index,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            id: None,
            if_primary_term: None,
            if_seq_no: None,
            op_type: None,
            pipeline: None,
            pretty: None,
            refresh: None,
            routing: None,
            source: None,
            timeout: None,
            ty: None,
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
        let path = "/{index}/_doc/{id}";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "if_primary_term")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no")]
                if_seq_no: Option<i64>,
                #[serde(rename = "op_type")]
                op_type: Option<OpType>,
                #[serde(rename = "pipeline")]
                pipeline: Option<String>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                op_type: self.op_type,
                pipeline: self.pipeline,
                refresh: self.refresh,
                routing: self.routing,
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
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Info {
    client: Elasticsearch,
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
        let path = "/";
        let method = HttpMethod::Get;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Mget<B> {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Option<String>,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    stored_fields: Option<Vec<String>>,
    ty: Option<String>,
}
impl<B> Mget<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        Mget {
            client,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            index: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            routing: None,
            source: None,
            stored_fields: None,
            ty: None,
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
        let path = "/_mget";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "_source")]
                _source: Option<Vec<String>>,
                #[serde(rename = "_source_excludes")]
                _source_excludes: Option<Vec<String>>,
                #[serde(rename = "_source_includes")]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "preference")]
                preference: Option<String>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "stored_fields")]
                stored_fields: Option<Vec<String>>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                preference: self.preference,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                stored_fields: self.stored_fields,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Msearch<B> {
    client: Elasticsearch,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Option<Vec<String>>,
    max_concurrent_searches: Option<i64>,
    max_concurrent_shard_requests: Option<i64>,
    pre_filter_shard_size: Option<i64>,
    pretty: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    search_type: Option<SearchType>,
    source: Option<String>,
    ty: Option<Vec<String>>,
    typed_keys: Option<bool>,
}
impl<B> Msearch<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        Msearch {
            client,
            body: None,
            ccs_minimize_roundtrips: None,
            error_trace: None,
            filter_path: None,
            human: None,
            index: None,
            max_concurrent_searches: None,
            max_concurrent_shard_requests: None,
            pre_filter_shard_size: None,
            pretty: None,
            rest_total_hits_as_int: None,
            search_type: None,
            source: None,
            ty: None,
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
        let path = "/_msearch";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "ccs_minimize_roundtrips")]
                ccs_minimize_roundtrips: Option<bool>,
                #[serde(rename = "max_concurrent_searches")]
                max_concurrent_searches: Option<i64>,
                #[serde(rename = "max_concurrent_shard_requests")]
                max_concurrent_shard_requests: Option<i64>,
                #[serde(rename = "pre_filter_shard_size")]
                pre_filter_shard_size: Option<i64>,
                #[serde(rename = "rest_total_hits_as_int")]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "search_type")]
                search_type: Option<SearchType>,
                #[serde(rename = "typed_keys")]
                typed_keys: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
                max_concurrent_searches: self.max_concurrent_searches,
                max_concurrent_shard_requests: self.max_concurrent_shard_requests,
                pre_filter_shard_size: self.pre_filter_shard_size,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                search_type: self.search_type,
                typed_keys: self.typed_keys,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MsearchTemplate<B> {
    client: Elasticsearch,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Option<Vec<String>>,
    max_concurrent_searches: Option<i64>,
    pretty: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    search_type: Option<SearchType>,
    source: Option<String>,
    ty: Option<Vec<String>>,
    typed_keys: Option<bool>,
}
impl<B> MsearchTemplate<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        MsearchTemplate {
            client,
            body: None,
            ccs_minimize_roundtrips: None,
            error_trace: None,
            filter_path: None,
            human: None,
            index: None,
            max_concurrent_searches: None,
            pretty: None,
            rest_total_hits_as_int: None,
            search_type: None,
            source: None,
            ty: None,
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
        let path = "/_msearch/template";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "ccs_minimize_roundtrips")]
                ccs_minimize_roundtrips: Option<bool>,
                #[serde(rename = "max_concurrent_searches")]
                max_concurrent_searches: Option<i64>,
                #[serde(rename = "rest_total_hits_as_int")]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "search_type")]
                search_type: Option<SearchType>,
                #[serde(rename = "typed_keys")]
                typed_keys: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
                max_concurrent_searches: self.max_concurrent_searches,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                search_type: self.search_type,
                typed_keys: self.typed_keys,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Mtermvectors<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    field_statistics: Option<bool>,
    fields: Option<Vec<String>>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ids: Option<Vec<String>>,
    index: Option<String>,
    offsets: Option<bool>,
    payloads: Option<bool>,
    positions: Option<bool>,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    term_statistics: Option<bool>,
    ty: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<B> Mtermvectors<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        Mtermvectors {
            client,
            body: None,
            error_trace: None,
            field_statistics: None,
            fields: None,
            filter_path: None,
            human: None,
            ids: None,
            index: None,
            offsets: None,
            payloads: None,
            positions: None,
            preference: None,
            pretty: None,
            realtime: None,
            routing: None,
            source: None,
            term_statistics: None,
            ty: None,
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
        let path = "/_mtermvectors";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "field_statistics")]
                field_statistics: Option<bool>,
                #[serde(rename = "fields")]
                fields: Option<Vec<String>>,
                #[serde(rename = "ids")]
                ids: Option<Vec<String>>,
                #[serde(rename = "offsets")]
                offsets: Option<bool>,
                #[serde(rename = "payloads")]
                payloads: Option<bool>,
                #[serde(rename = "positions")]
                positions: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<String>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "term_statistics")]
                term_statistics: Option<bool>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                field_statistics: self.field_statistics,
                fields: self.fields,
                ids: self.ids,
                offsets: self.offsets,
                payloads: self.payloads,
                positions: self.positions,
                preference: self.preference,
                realtime: self.realtime,
                routing: self.routing,
                term_statistics: self.term_statistics,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Ping {
    client: Elasticsearch,
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
        let path = "/";
        let method = HttpMethod::Head;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct PutScript<B> {
    client: Elasticsearch,
    body: Option<B>,
    context: Option<String>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> PutScript<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, id: String) -> Self {
        PutScript {
            client,
            id: id,
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
        let path = "/_scripts/{id}";
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "context")]
                context: Option<String>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                context: self.context,
                master_timeout: self.master_timeout,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct RankEval<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> RankEval<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        RankEval {
            client,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            index: None,
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
        let path = "/_rank_eval";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Reindex<B> {
    client: Elasticsearch,
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
        let path = "/_reindex";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "max_docs")]
                max_docs: Option<i64>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "requests_per_second")]
                requests_per_second: Option<i64>,
                #[serde(rename = "scroll")]
                scroll: Option<String>,
                #[serde(rename = "slices")]
                slices: Option<i64>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                max_docs: self.max_docs,
                refresh: self.refresh,
                requests_per_second: self.requests_per_second,
                scroll: self.scroll,
                slices: self.slices,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct ReindexRethrottle<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<String>,
    task_id: String,
}
impl<B> ReindexRethrottle<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, task_id: String) -> Self {
        ReindexRethrottle {
            client,
            task_id: task_id,
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
        let path = "/_reindex/{task_id}/_rethrottle";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "requests_per_second")]
                requests_per_second: Option<i64>,
            }
            let query_params = QueryParamsStruct {
                requests_per_second: self.requests_per_second,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct RenderSearchTemplate<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> RenderSearchTemplate<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        RenderSearchTemplate {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            id: None,
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
        let path = "/_render/template";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct ScriptsPainlessExecute<B> {
    client: Elasticsearch,
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
        let path = "/_scripts/painless/_execute";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Scroll<B> {
    client: Elasticsearch,
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
    pub fn new(client: Elasticsearch) -> Self {
        Scroll {
            client,
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
        let path = "/_search/scroll";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "rest_total_hits_as_int")]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "scroll")]
                scroll: Option<String>,
                #[serde(rename = "scroll_id")]
                scroll_id: Option<String>,
            }
            let query_params = QueryParamsStruct {
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                scroll: self.scroll,
                scroll_id: self.scroll_id,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Search<B> {
    client: Elasticsearch,
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
    index: Option<Vec<String>>,
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
    ty: Option<Vec<String>>,
    typed_keys: Option<bool>,
    version: Option<bool>,
}
impl<B> Search<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        Search {
            client,
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
            index: None,
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
            ty: None,
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
        let path = "/_search";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "_source")]
                _source: Option<Vec<String>>,
                #[serde(rename = "_source_excludes")]
                _source_excludes: Option<Vec<String>>,
                #[serde(rename = "_source_includes")]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "allow_partial_search_results")]
                allow_partial_search_results: Option<bool>,
                #[serde(rename = "analyze_wildcard")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer")]
                analyzer: Option<String>,
                #[serde(rename = "batched_reduce_size")]
                batched_reduce_size: Option<i64>,
                #[serde(rename = "ccs_minimize_roundtrips")]
                ccs_minimize_roundtrips: Option<bool>,
                #[serde(rename = "default_operator")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df")]
                df: Option<String>,
                #[serde(rename = "docvalue_fields")]
                docvalue_fields: Option<Vec<String>>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "explain")]
                explain: Option<bool>,
                #[serde(rename = "from")]
                from: Option<i64>,
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
                preference: Option<String>,
                #[serde(rename = "q")]
                q: Option<String>,
                #[serde(rename = "request_cache")]
                request_cache: Option<bool>,
                #[serde(rename = "rest_total_hits_as_int")]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<Vec<String>>,
                #[serde(rename = "scroll")]
                scroll: Option<String>,
                #[serde(rename = "search_type")]
                search_type: Option<SearchType>,
                #[serde(rename = "seq_no_primary_term")]
                seq_no_primary_term: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i64>,
                #[serde(rename = "sort")]
                sort: Option<Vec<String>>,
                #[serde(rename = "stats")]
                stats: Option<Vec<String>>,
                #[serde(rename = "stored_fields")]
                stored_fields: Option<Vec<String>>,
                #[serde(rename = "suggest_field")]
                suggest_field: Option<String>,
                #[serde(rename = "suggest_mode")]
                suggest_mode: Option<SuggestMode>,
                #[serde(rename = "suggest_size")]
                suggest_size: Option<i64>,
                #[serde(rename = "suggest_text")]
                suggest_text: Option<String>,
                #[serde(rename = "terminate_after")]
                terminate_after: Option<i64>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "track_scores")]
                track_scores: Option<bool>,
                #[serde(rename = "track_total_hits")]
                track_total_hits: Option<bool>,
                #[serde(rename = "typed_keys")]
                typed_keys: Option<bool>,
                #[serde(rename = "version")]
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
                expand_wildcards: self.expand_wildcards,
                explain: self.explain,
                from: self.from,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                max_concurrent_shard_requests: self.max_concurrent_shard_requests,
                pre_filter_shard_size: self.pre_filter_shard_size,
                preference: self.preference,
                q: self.q,
                request_cache: self.request_cache,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                routing: self.routing,
                scroll: self.scroll,
                search_type: self.search_type,
                seq_no_primary_term: self.seq_no_primary_term,
                size: self.size,
                sort: self.sort,
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
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SearchShards<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
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
    pub fn new(client: Elasticsearch) -> Self {
        SearchShards {
            client,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            index: None,
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
        let path = "/_search_shards";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<String>,
                #[serde(rename = "routing")]
                routing: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
                preference: self.preference,
                routing: self.routing,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SearchTemplate<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    explain: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    preference: Option<String>,
    pretty: Option<bool>,
    profile: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    routing: Option<Vec<String>>,
    scroll: Option<String>,
    search_type: Option<SearchType>,
    source: Option<String>,
    ty: Option<Vec<String>>,
    typed_keys: Option<bool>,
}
impl<B> SearchTemplate<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        SearchTemplate {
            client,
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
            index: None,
            preference: None,
            pretty: None,
            profile: None,
            rest_total_hits_as_int: None,
            routing: None,
            scroll: None,
            search_type: None,
            source: None,
            ty: None,
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
        let path = "/_search/template";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "ccs_minimize_roundtrips")]
                ccs_minimize_roundtrips: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "explain")]
                explain: Option<bool>,
                #[serde(rename = "ignore_throttled")]
                ignore_throttled: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<String>,
                #[serde(rename = "profile")]
                profile: Option<bool>,
                #[serde(rename = "rest_total_hits_as_int")]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<Vec<String>>,
                #[serde(rename = "scroll")]
                scroll: Option<String>,
                #[serde(rename = "search_type")]
                search_type: Option<SearchType>,
                #[serde(rename = "typed_keys")]
                typed_keys: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
                expand_wildcards: self.expand_wildcards,
                explain: self.explain,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                preference: self.preference,
                profile: self.profile,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                routing: self.routing,
                scroll: self.scroll,
                search_type: self.search_type,
                typed_keys: self.typed_keys,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Termvectors<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    field_statistics: Option<bool>,
    fields: Option<Vec<String>>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: Option<String>,
    index: String,
    offsets: Option<bool>,
    payloads: Option<bool>,
    positions: Option<bool>,
    preference: Option<String>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    term_statistics: Option<bool>,
    ty: Option<String>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<B> Termvectors<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String) -> Self {
        Termvectors {
            client,
            index: index,
            body: None,
            error_trace: None,
            field_statistics: None,
            fields: None,
            filter_path: None,
            human: None,
            id: None,
            offsets: None,
            payloads: None,
            positions: None,
            preference: None,
            pretty: None,
            realtime: None,
            routing: None,
            source: None,
            term_statistics: None,
            ty: None,
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
        let path = "/{index}/_termvectors/{id}";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "field_statistics")]
                field_statistics: Option<bool>,
                #[serde(rename = "fields")]
                fields: Option<Vec<String>>,
                #[serde(rename = "offsets")]
                offsets: Option<bool>,
                #[serde(rename = "payloads")]
                payloads: Option<bool>,
                #[serde(rename = "positions")]
                positions: Option<bool>,
                #[serde(rename = "preference")]
                preference: Option<String>,
                #[serde(rename = "realtime")]
                realtime: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "term_statistics")]
                term_statistics: Option<bool>,
                #[serde(rename = "version")]
                version: Option<i64>,
                #[serde(rename = "version_type")]
                version_type: Option<VersionType>,
            }
            let query_params = QueryParamsStruct {
                field_statistics: self.field_statistics,
                fields: self.fields,
                offsets: self.offsets,
                payloads: self.payloads,
                positions: self.positions,
                preference: self.preference,
                realtime: self.realtime,
                routing: self.routing,
                term_statistics: self.term_statistics,
                version: self.version,
                version_type: self.version_type,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct Update<B> {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    index: String,
    lang: Option<String>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    retry_on_conflict: Option<i64>,
    routing: Option<String>,
    source: Option<String>,
    timeout: Option<String>,
    ty: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl<B> Update<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String, id: String) -> Self {
        Update {
            client,
            index: index,
            id: id,
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
            ty: None,
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
        let path = "/{index}/_update/{id}";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "_source")]
                _source: Option<Vec<String>>,
                #[serde(rename = "_source_excludes")]
                _source_excludes: Option<Vec<String>>,
                #[serde(rename = "_source_includes")]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "if_primary_term")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no")]
                if_seq_no: Option<i64>,
                #[serde(rename = "lang")]
                lang: Option<String>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "retry_on_conflict")]
                retry_on_conflict: Option<i64>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                lang: self.lang,
                refresh: self.refresh,
                retry_on_conflict: self.retry_on_conflict,
                routing: self.routing,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct UpdateByQuery<B> {
    client: Elasticsearch,
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
    index: Vec<String>,
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
    ty: Option<Vec<String>>,
    version: Option<bool>,
    version_type: Option<bool>,
    wait_for_active_shards: Option<String>,
    wait_for_completion: Option<bool>,
}
impl<B> UpdateByQuery<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        UpdateByQuery {
            client,
            index: index,
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
            ty: None,
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
        let path = "/{index}/_update_by_query";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "_source")]
                _source: Option<Vec<String>>,
                #[serde(rename = "_source_excludes")]
                _source_excludes: Option<Vec<String>>,
                #[serde(rename = "_source_includes")]
                _source_includes: Option<Vec<String>>,
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer")]
                analyzer: Option<String>,
                #[serde(rename = "conflicts")]
                conflicts: Option<Conflicts>,
                #[serde(rename = "default_operator")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df")]
                df: Option<String>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "from")]
                from: Option<i64>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient")]
                lenient: Option<bool>,
                #[serde(rename = "max_docs")]
                max_docs: Option<i64>,
                #[serde(rename = "pipeline")]
                pipeline: Option<String>,
                #[serde(rename = "preference")]
                preference: Option<String>,
                #[serde(rename = "q")]
                q: Option<String>,
                #[serde(rename = "refresh")]
                refresh: Option<bool>,
                #[serde(rename = "request_cache")]
                request_cache: Option<bool>,
                #[serde(rename = "requests_per_second")]
                requests_per_second: Option<i64>,
                #[serde(rename = "routing")]
                routing: Option<Vec<String>>,
                #[serde(rename = "scroll")]
                scroll: Option<String>,
                #[serde(rename = "scroll_size")]
                scroll_size: Option<i64>,
                #[serde(rename = "search_timeout")]
                search_timeout: Option<String>,
                #[serde(rename = "search_type")]
                search_type: Option<SearchType>,
                #[serde(rename = "size")]
                size: Option<i64>,
                #[serde(rename = "slices")]
                slices: Option<i64>,
                #[serde(rename = "sort")]
                sort: Option<Vec<String>>,
                #[serde(rename = "stats")]
                stats: Option<Vec<String>>,
                #[serde(rename = "terminate_after")]
                terminate_after: Option<i64>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "version")]
                version: Option<bool>,
                #[serde(rename = "version_type")]
                version_type: Option<bool>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
                #[serde(rename = "wait_for_completion")]
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
                expand_wildcards: self.expand_wildcards,
                from: self.from,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                max_docs: self.max_docs,
                pipeline: self.pipeline,
                preference: self.preference,
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
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct UpdateByQueryRethrottle<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<String>,
    task_id: String,
}
impl<B> UpdateByQueryRethrottle<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, task_id: String) -> Self {
        UpdateByQueryRethrottle {
            client,
            task_id: task_id,
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
        let path = "/_update_by_query/{task_id}/_rethrottle";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "requests_per_second")]
                requests_per_second: Option<i64>,
            }
            let query_params = QueryParamsStruct {
                requests_per_second: self.requests_per_second,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
impl Elasticsearch {
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-bulk.html"]
    pub fn bulk<B>(&self) -> Bulk<B>
    where
        B: Serialize,
    {
        Bulk::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-request-body.html#request-body-search-scroll"]
    pub fn clear_scroll<B>(&self) -> ClearScroll<B>
    where
        B: Serialize,
    {
        ClearScroll::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-count.html"]
    pub fn count<B>(&self) -> Count<B>
    where
        B: Serialize,
    {
        Count::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-index_.html"]
    pub fn create<B>(&self, index: String, id: String) -> Create<B>
    where
        B: Serialize,
    {
        Create::new(self.clone(), index, id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-delete.html"]
    pub fn delete(&self, index: String, id: String) -> Delete {
        Delete::new(self.clone(), index, id)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-delete-by-query.html"]
    pub fn delete_by_query<B>(&self, index: Vec<String>) -> DeleteByQuery<B>
    where
        B: Serialize,
    {
        DeleteByQuery::new(self.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-delete-by-query.html"]
    pub fn delete_by_query_rethrottle<B>(&self, task_id: String) -> DeleteByQueryRethrottle<B>
    where
        B: Serialize,
    {
        DeleteByQueryRethrottle::new(self.clone(), task_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html"]
    pub fn delete_script(&self, id: String) -> DeleteScript {
        DeleteScript::new(self.clone(), id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn exists(&self, index: String, id: String) -> Exists {
        Exists::new(self.clone(), index, id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn exists_source(&self, index: String, id: String) -> ExistsSource {
        ExistsSource::new(self.clone(), index, id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-explain.html"]
    pub fn explain<B>(&self, index: String, id: String) -> Explain<B>
    where
        B: Serialize,
    {
        Explain::new(self.clone(), index, id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-field-caps.html"]
    pub fn field_caps<B>(&self) -> FieldCaps<B>
    where
        B: Serialize,
    {
        FieldCaps::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn get(&self, index: String, id: String) -> Get {
        Get::new(self.clone(), index, id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html"]
    pub fn get_script(&self, id: String) -> GetScript {
        GetScript::new(self.clone(), id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn get_source(&self, index: String, id: String) -> GetSource {
        GetSource::new(self.clone(), index, id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-index_.html"]
    pub fn index<B>(&self, index: String) -> Index<B>
    where
        B: Serialize,
    {
        Index::new(self.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/"]
    pub fn info(&self) -> Info {
        Info::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-multi-get.html"]
    pub fn mget<B>(&self) -> Mget<B>
    where
        B: Serialize,
    {
        Mget::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-multi-search.html"]
    pub fn msearch<B>(&self) -> Msearch<B>
    where
        B: Serialize,
    {
        Msearch::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/search-multi-search.html"]
    pub fn msearch_template<B>(&self) -> MsearchTemplate<B>
    where
        B: Serialize,
    {
        MsearchTemplate::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-multi-termvectors.html"]
    pub fn mtermvectors<B>(&self) -> Mtermvectors<B>
    where
        B: Serialize,
    {
        Mtermvectors::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/"]
    pub fn ping(&self) -> Ping {
        Ping::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html"]
    pub fn put_script<B>(&self, id: String) -> PutScript<B>
    where
        B: Serialize,
    {
        PutScript::new(self.clone(), id)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/search-rank-eval.html"]
    pub fn rank_eval<B>(&self) -> RankEval<B>
    where
        B: Serialize,
    {
        RankEval::new(self.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-reindex.html"]
    pub fn reindex<B>(&self) -> Reindex<B>
    where
        B: Serialize,
    {
        Reindex::new(self.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-reindex.html"]
    pub fn reindex_rethrottle<B>(&self, task_id: String) -> ReindexRethrottle<B>
    where
        B: Serialize,
    {
        ReindexRethrottle::new(self.clone(), task_id)
    }
    #[doc = "http://www.elasticsearch.org/guide/en/elasticsearch/reference/master/search-template.html"]
    pub fn render_search_template<B>(&self) -> RenderSearchTemplate<B>
    where
        B: Serialize,
    {
        RenderSearchTemplate::new(self.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/painless/master/painless-execute-api.html"]
    pub fn scripts_painless_execute<B>(&self) -> ScriptsPainlessExecute<B>
    where
        B: Serialize,
    {
        ScriptsPainlessExecute::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-request-body.html#request-body-search-scroll"]
    pub fn scroll<B>(&self) -> Scroll<B>
    where
        B: Serialize,
    {
        Scroll::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html"]
    pub fn search<B>(&self) -> Search<B>
    where
        B: Serialize,
    {
        Search::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-shards.html"]
    pub fn search_shards<B>(&self) -> SearchShards<B>
    where
        B: Serialize,
    {
        SearchShards::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/search-template.html"]
    pub fn search_template<B>(&self) -> SearchTemplate<B>
    where
        B: Serialize,
    {
        SearchTemplate::new(self.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-termvectors.html"]
    pub fn termvectors<B>(&self, index: String) -> Termvectors<B>
    where
        B: Serialize,
    {
        Termvectors::new(self.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-update.html"]
    pub fn update<B>(&self, index: String, id: String) -> Update<B>
    where
        B: Serialize,
    {
        Update::new(self.clone(), index, id)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-update-by-query.html"]
    pub fn update_by_query<B>(&self, index: Vec<String>) -> UpdateByQuery<B>
    where
        B: Serialize,
    {
        UpdateByQuery::new(self.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-update-by-query.html"]
    pub fn update_by_query_rethrottle<B>(&self, task_id: String) -> UpdateByQueryRethrottle<B>
    where
        B: Serialize,
    {
        UpdateByQueryRethrottle::new(self.clone(), task_id)
    }
}
