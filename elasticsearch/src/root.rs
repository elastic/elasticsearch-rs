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
use super::super::client::Elasticsearch;
use super::super::enums::*;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct Bulk {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
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
impl Bulk {
    pub fn new(client: Elasticsearch) -> Self {
        Bulk {
            client,
            ..Default::default()
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
impl Sender for Bulk {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct ClearScroll {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    scroll_id: Option<Vec<String>>,
    source: Option<String>,
}
impl ClearScroll {
    pub fn new(client: Elasticsearch) -> Self {
        ClearScroll {
            client,
            ..Default::default()
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
impl Sender for ClearScroll {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct Count {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<String>,
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
impl Count {
    pub fn new(client: Elasticsearch) -> Self {
        Count {
            client,
            ..Default::default()
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
impl Sender for Count {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct Create {
    client: Elasticsearch,
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
impl Create {
    pub fn new(client: Elasticsearch, index: String, id: String) -> Self {
        Create {
            client,
            index: index,
            id: id,
            ..Default::default()
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
impl Sender for Create {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
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
            ..Default::default()
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
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct DeleteByQuery {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<String>,
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
impl DeleteByQuery {
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        DeleteByQuery {
            client,
            index: index,
            ..Default::default()
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
impl Sender for DeleteByQuery {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct DeleteByQueryRethrottle {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<String>,
    task_id: String,
}
impl DeleteByQueryRethrottle {
    pub fn new(client: Elasticsearch, task_id: String) -> Self {
        DeleteByQueryRethrottle {
            client,
            task_id: task_id,
            ..Default::default()
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
impl Sender for DeleteByQueryRethrottle {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
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
            ..Default::default()
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
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
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
            ..Default::default()
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
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
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
            ..Default::default()
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
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct Explain {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<String>,
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
impl Explain {
    pub fn new(client: Elasticsearch, index: String, id: String) -> Self {
        Explain {
            client,
            index: index,
            id: id,
            ..Default::default()
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
impl Sender for Explain {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct FieldCaps {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
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
impl FieldCaps {
    pub fn new(client: Elasticsearch) -> Self {
        FieldCaps {
            client,
            ..Default::default()
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
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
impl Sender for FieldCaps {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
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
            ..Default::default()
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
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
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
            ..Default::default()
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
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
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
            ..Default::default()
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
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct Index {
    client: Elasticsearch,
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
impl Index {
    pub fn new(client: Elasticsearch, index: String) -> Self {
        Index {
            client,
            index: index,
            ..Default::default()
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
impl Sender for Index {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
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
            ..Default::default()
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
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct Mget {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
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
impl Mget {
    pub fn new(client: Elasticsearch) -> Self {
        Mget {
            client,
            ..Default::default()
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
}
impl Sender for Mget {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct Msearch {
    client: Elasticsearch,
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
impl Msearch {
    pub fn new(client: Elasticsearch) -> Self {
        Msearch {
            client,
            ..Default::default()
        }
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
impl Sender for Msearch {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct MsearchTemplate {
    client: Elasticsearch,
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
impl MsearchTemplate {
    pub fn new(client: Elasticsearch) -> Self {
        MsearchTemplate {
            client,
            ..Default::default()
        }
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
impl Sender for MsearchTemplate {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct Mtermvectors {
    client: Elasticsearch,
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
impl Mtermvectors {
    pub fn new(client: Elasticsearch) -> Self {
        Mtermvectors {
            client,
            ..Default::default()
        }
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
impl Sender for Mtermvectors {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
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
            ..Default::default()
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
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct PutScript {
    client: Elasticsearch,
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
impl PutScript {
    pub fn new(client: Elasticsearch, id: String) -> Self {
        PutScript {
            client,
            id: id,
            ..Default::default()
        }
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
impl Sender for PutScript {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct RankEval {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl RankEval {
    pub fn new(client: Elasticsearch) -> Self {
        RankEval {
            client,
            ..Default::default()
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
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
impl Sender for RankEval {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct Reindex {
    client: Elasticsearch,
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
impl Reindex {
    pub fn new(client: Elasticsearch) -> Self {
        Reindex {
            client,
            ..Default::default()
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
impl Sender for Reindex {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct ReindexRethrottle {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<String>,
    task_id: String,
}
impl ReindexRethrottle {
    pub fn new(client: Elasticsearch, task_id: String) -> Self {
        ReindexRethrottle {
            client,
            task_id: task_id,
            ..Default::default()
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
impl Sender for ReindexRethrottle {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct RenderSearchTemplate {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl RenderSearchTemplate {
    pub fn new(client: Elasticsearch) -> Self {
        RenderSearchTemplate {
            client,
            ..Default::default()
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
impl Sender for RenderSearchTemplate {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct ScriptsPainlessExecute {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl ScriptsPainlessExecute {
    pub fn new(client: Elasticsearch) -> Self {
        ScriptsPainlessExecute {
            client,
            ..Default::default()
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
impl Sender for ScriptsPainlessExecute {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct Scroll {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    scroll: Option<String>,
    scroll_id: Option<String>,
    source: Option<String>,
}
impl Scroll {
    pub fn new(client: Elasticsearch) -> Self {
        Scroll {
            client,
            ..Default::default()
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
impl Sender for Scroll {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct Search {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    allow_no_indices: Option<bool>,
    allow_partial_search_results: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<String>,
    batched_reduce_size: Option<i64>,
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
impl Search {
    pub fn new(client: Elasticsearch) -> Self {
        Search {
            client,
            ..Default::default()
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
impl Sender for Search {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct SearchShards {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
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
impl SearchShards {
    pub fn new(client: Elasticsearch) -> Self {
        SearchShards {
            client,
            ..Default::default()
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
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
impl Sender for SearchShards {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct SearchTemplate {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
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
impl SearchTemplate {
    pub fn new(client: Elasticsearch) -> Self {
        SearchTemplate {
            client,
            ..Default::default()
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
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
impl Sender for SearchTemplate {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct Termvectors {
    client: Elasticsearch,
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
impl Termvectors {
    pub fn new(client: Elasticsearch, index: String) -> Self {
        Termvectors {
            client,
            index: index,
            ..Default::default()
        }
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
impl Sender for Termvectors {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct Update {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
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
impl Update {
    pub fn new(client: Elasticsearch, index: String, id: String) -> Self {
        Update {
            client,
            index: index,
            id: id,
            ..Default::default()
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
impl Sender for Update {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct UpdateByQuery {
    client: Elasticsearch,
    _source: Option<Vec<String>>,
    _source_excludes: Option<Vec<String>>,
    _source_includes: Option<Vec<String>>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<String>,
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
impl UpdateByQuery {
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        UpdateByQuery {
            client,
            index: index,
            ..Default::default()
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
impl Sender for UpdateByQuery {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct UpdateByQueryRethrottle {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    requests_per_second: Option<i64>,
    source: Option<String>,
    task_id: String,
}
impl UpdateByQueryRethrottle {
    pub fn new(client: Elasticsearch, task_id: String) -> Self {
        UpdateByQueryRethrottle {
            client,
            task_id: task_id,
            ..Default::default()
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
impl Sender for UpdateByQueryRethrottle {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
impl Elasticsearch {
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-bulk.html"]
    pub fn bulk(&self) -> Bulk {
        Bulk::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-request-body.html#request-body-search-scroll"]
    pub fn clear_scroll(&self) -> ClearScroll {
        ClearScroll::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-count.html"]
    pub fn count(&self) -> Count {
        Count::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-index_.html"]
    pub fn create(&self, index: String, id: String) -> Create {
        Create::new(self.client.clone(), index, id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-delete.html"]
    pub fn delete(&self, index: String, id: String) -> Delete {
        Delete::new(self.client.clone(), index, id)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-delete-by-query.html"]
    pub fn delete_by_query(&self, index: Vec<String>) -> DeleteByQuery {
        DeleteByQuery::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-delete-by-query.html"]
    pub fn delete_by_query_rethrottle(&self, task_id: String) -> DeleteByQueryRethrottle {
        DeleteByQueryRethrottle::new(self.client.clone(), task_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html"]
    pub fn delete_script(&self, id: String) -> DeleteScript {
        DeleteScript::new(self.client.clone(), id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn exists(&self, index: String, id: String) -> Exists {
        Exists::new(self.client.clone(), index, id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn exists_source(&self, index: String, id: String) -> ExistsSource {
        ExistsSource::new(self.client.clone(), index, id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-explain.html"]
    pub fn explain(&self, index: String, id: String) -> Explain {
        Explain::new(self.client.clone(), index, id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-field-caps.html"]
    pub fn field_caps(&self) -> FieldCaps {
        FieldCaps::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn get(&self, index: String, id: String) -> Get {
        Get::new(self.client.clone(), index, id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html"]
    pub fn get_script(&self, id: String) -> GetScript {
        GetScript::new(self.client.clone(), id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn get_source(&self, index: String, id: String) -> GetSource {
        GetSource::new(self.client.clone(), index, id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-index_.html"]
    pub fn index(&self, index: String) -> Index {
        Index::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/"]
    pub fn info(&self) -> Info {
        Info::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-multi-get.html"]
    pub fn mget(&self) -> Mget {
        Mget::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-multi-search.html"]
    pub fn msearch(&self) -> Msearch {
        Msearch::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/search-multi-search.html"]
    pub fn msearch_template(&self) -> MsearchTemplate {
        MsearchTemplate::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-multi-termvectors.html"]
    pub fn mtermvectors(&self) -> Mtermvectors {
        Mtermvectors::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/"]
    pub fn ping(&self) -> Ping {
        Ping::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html"]
    pub fn put_script(&self, id: String) -> PutScript {
        PutScript::new(self.client.clone(), id)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/search-rank-eval.html"]
    pub fn rank_eval(&self) -> RankEval {
        RankEval::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-reindex.html"]
    pub fn reindex(&self) -> Reindex {
        Reindex::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-reindex.html"]
    pub fn reindex_rethrottle(&self, task_id: String) -> ReindexRethrottle {
        ReindexRethrottle::new(self.client.clone(), task_id)
    }
    #[doc = "http://www.elasticsearch.org/guide/en/elasticsearch/reference/master/search-template.html"]
    pub fn render_search_template(&self) -> RenderSearchTemplate {
        RenderSearchTemplate::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/painless/master/painless-execute-api.html"]
    pub fn scripts_painless_execute(&self) -> ScriptsPainlessExecute {
        ScriptsPainlessExecute::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-request-body.html#request-body-search-scroll"]
    pub fn scroll(&self) -> Scroll {
        Scroll::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html"]
    pub fn search(&self) -> Search {
        Search::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-shards.html"]
    pub fn search_shards(&self) -> SearchShards {
        SearchShards::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/search-template.html"]
    pub fn search_template(&self) -> SearchTemplate {
        SearchTemplate::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-termvectors.html"]
    pub fn termvectors(&self, index: String) -> Termvectors {
        Termvectors::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-update.html"]
    pub fn update(&self, index: String, id: String) -> Update {
        Update::new(self.client.clone(), index, id)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-update-by-query.html"]
    pub fn update_by_query(&self, index: Vec<String>) -> UpdateByQuery {
        UpdateByQuery::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-update-by-query.html"]
    pub fn update_by_query_rethrottle(&self, task_id: String) -> UpdateByQueryRethrottle {
        UpdateByQueryRethrottle::new(self.client.clone(), task_id)
    }
}