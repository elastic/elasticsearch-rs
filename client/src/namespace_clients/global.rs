

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct GlobalBulkRequest<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    pipeline: &'a String,
    refresh: Option<&'a i32>,
    routing: &'a String,
    timeout: &'a String,
    ty: &'a String,
    wait_for_active_shards: &'a String,
}
pub struct GlobalBulkRequestBuilder<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    pipeline: &'a String,
    refresh: Option<&'a i32>,
    routing: &'a String,
    timeout: &'a String,
    ty: &'a String,
    wait_for_active_shards: &'a String,
}
impl<'a> GlobalBulkRequestBuilder<'a> {
    pub fn build(&self) -> GlobalBulkRequest<'a> {
        GlobalBulkRequest {
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            pipeline: self.pipeline,
            refresh: self.refresh,
            routing: self.routing,
            timeout: self.timeout,
            ty: self.ty,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
}
pub struct GlobalClearScrollRequest<'a> {}
pub struct GlobalClearScrollRequestBuilder<'a> {}
impl<'a> GlobalClearScrollRequestBuilder<'a> {
    pub fn build(&self) -> GlobalClearScrollRequest<'a> {
        GlobalClearScrollRequest {}
    }
}
pub struct GlobalCountRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    analyze_wildcard: Option<&'a bool>,
    analyzer: &'a String,
    default_operator: Option<&'a i32>,
    df: &'a String,
    expand_wildcards: Option<&'a i32>,
    ignore_throttled: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    lenient: Option<&'a bool>,
    min_score: Option<&'a i64>,
    preference: &'a String,
    q: &'a String,
    routing: &'a Vec<String>,
    terminate_after: Option<&'a i64>,
}
pub struct GlobalCountRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    analyze_wildcard: Option<&'a bool>,
    analyzer: &'a String,
    default_operator: Option<&'a i32>,
    df: &'a String,
    expand_wildcards: Option<&'a i32>,
    ignore_throttled: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    lenient: Option<&'a bool>,
    min_score: Option<&'a i64>,
    preference: &'a String,
    q: &'a String,
    routing: &'a Vec<String>,
    terminate_after: Option<&'a i64>,
}
impl<'a> GlobalCountRequestBuilder<'a> {
    pub fn build(&self) -> GlobalCountRequest<'a> {
        GlobalCountRequest {
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
        }
    }
}
pub struct GlobalCreateRequest<'a> {
    pipeline: &'a String,
    refresh: Option<&'a i32>,
    routing: &'a String,
    timeout: &'a String,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
    wait_for_active_shards: &'a String,
}
pub struct GlobalCreateRequestBuilder<'a> {
    pipeline: &'a String,
    refresh: Option<&'a i32>,
    routing: &'a String,
    timeout: &'a String,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
    wait_for_active_shards: &'a String,
}
impl<'a> GlobalCreateRequestBuilder<'a> {
    pub fn build(&self) -> GlobalCreateRequest<'a> {
        GlobalCreateRequest {
            pipeline: self.pipeline,
            refresh: self.refresh,
            routing: self.routing,
            timeout: self.timeout,
            version: self.version,
            version_type: self.version_type,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
}
pub struct GlobalDeleteRequest<'a> {
    if_primary_term: Option<&'a i64>,
    if_seq_no: Option<&'a i64>,
    refresh: Option<&'a i32>,
    routing: &'a String,
    timeout: &'a String,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
    wait_for_active_shards: &'a String,
}
pub struct GlobalDeleteRequestBuilder<'a> {
    if_primary_term: Option<&'a i64>,
    if_seq_no: Option<&'a i64>,
    refresh: Option<&'a i32>,
    routing: &'a String,
    timeout: &'a String,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
    wait_for_active_shards: &'a String,
}
impl<'a> GlobalDeleteRequestBuilder<'a> {
    pub fn build(&self) -> GlobalDeleteRequest<'a> {
        GlobalDeleteRequest {
            if_primary_term: self.if_primary_term,
            if_seq_no: self.if_seq_no,
            refresh: self.refresh,
            routing: self.routing,
            timeout: self.timeout,
            version: self.version,
            version_type: self.version_type,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
}
pub struct GlobalDeleteByQueryRequest<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    allow_no_indices: Option<&'a bool>,
    analyze_wildcard: Option<&'a bool>,
    analyzer: &'a String,
    conflicts: Option<&'a i32>,
    default_operator: Option<&'a i32>,
    df: &'a String,
    expand_wildcards: Option<&'a i32>,
    from: Option<&'a i64>,
    ignore_unavailable: Option<&'a bool>,
    lenient: Option<&'a bool>,
    max_docs: Option<&'a i64>,
    preference: &'a String,
    q: &'a String,
    refresh: Option<&'a bool>,
    request_cache: Option<&'a bool>,
    requests_per_second: Option<&'a i64>,
    routing: &'a Vec<String>,
    scroll: &'a String,
    scroll_size: Option<&'a i64>,
    search_timeout: &'a String,
    search_type: Option<&'a i32>,
    size: Option<&'a i64>,
    slices: Option<&'a i64>,
    sort: &'a Vec<String>,
    stats: &'a Vec<String>,
    terminate_after: Option<&'a i64>,
    timeout: &'a String,
    version: Option<&'a bool>,
    wait_for_active_shards: &'a String,
    wait_for_completion: Option<&'a bool>,
}
pub struct GlobalDeleteByQueryRequestBuilder<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    allow_no_indices: Option<&'a bool>,
    analyze_wildcard: Option<&'a bool>,
    analyzer: &'a String,
    conflicts: Option<&'a i32>,
    default_operator: Option<&'a i32>,
    df: &'a String,
    expand_wildcards: Option<&'a i32>,
    from: Option<&'a i64>,
    ignore_unavailable: Option<&'a bool>,
    lenient: Option<&'a bool>,
    max_docs: Option<&'a i64>,
    preference: &'a String,
    q: &'a String,
    refresh: Option<&'a bool>,
    request_cache: Option<&'a bool>,
    requests_per_second: Option<&'a i64>,
    routing: &'a Vec<String>,
    scroll: &'a String,
    scroll_size: Option<&'a i64>,
    search_timeout: &'a String,
    search_type: Option<&'a i32>,
    size: Option<&'a i64>,
    slices: Option<&'a i64>,
    sort: &'a Vec<String>,
    stats: &'a Vec<String>,
    terminate_after: Option<&'a i64>,
    timeout: &'a String,
    version: Option<&'a bool>,
    wait_for_active_shards: &'a String,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> GlobalDeleteByQueryRequestBuilder<'a> {
    pub fn build(&self) -> GlobalDeleteByQueryRequest<'a> {
        GlobalDeleteByQueryRequest {
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
        }
    }
}
pub struct GlobalDeleteByQueryRethrottleRequest<'a> {
    requests_per_second: Option<&'a i64>,
}
pub struct GlobalDeleteByQueryRethrottleRequestBuilder<'a> {
    requests_per_second: Option<&'a i64>,
}
impl<'a> GlobalDeleteByQueryRethrottleRequestBuilder<'a> {
    pub fn build(&self) -> GlobalDeleteByQueryRethrottleRequest<'a> {
        GlobalDeleteByQueryRethrottleRequest {
            requests_per_second: self.requests_per_second,
        }
    }
}
pub struct GlobalDeleteScriptRequest<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct GlobalDeleteScriptRequestBuilder<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> GlobalDeleteScriptRequestBuilder<'a> {
    pub fn build(&self) -> GlobalDeleteScriptRequest<'a> {
        GlobalDeleteScriptRequest {
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
pub struct GlobalExistsRequest<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    refresh: Option<&'a bool>,
    routing: &'a String,
    stored_fields: &'a Vec<String>,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
}
pub struct GlobalExistsRequestBuilder<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    refresh: Option<&'a bool>,
    routing: &'a String,
    stored_fields: &'a Vec<String>,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
}
impl<'a> GlobalExistsRequestBuilder<'a> {
    pub fn build(&self) -> GlobalExistsRequest<'a> {
        GlobalExistsRequest {
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
        }
    }
}
pub struct GlobalExistsSourceRequest<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    refresh: Option<&'a bool>,
    routing: &'a String,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
}
pub struct GlobalExistsSourceRequestBuilder<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    refresh: Option<&'a bool>,
    routing: &'a String,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
}
impl<'a> GlobalExistsSourceRequestBuilder<'a> {
    pub fn build(&self) -> GlobalExistsSourceRequest<'a> {
        GlobalExistsSourceRequest {
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            preference: self.preference,
            realtime: self.realtime,
            refresh: self.refresh,
            routing: self.routing,
            version: self.version,
            version_type: self.version_type,
        }
    }
}
pub struct GlobalExplainRequest<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    analyze_wildcard: Option<&'a bool>,
    analyzer: &'a String,
    default_operator: Option<&'a i32>,
    df: &'a String,
    lenient: Option<&'a bool>,
    preference: &'a String,
    q: &'a String,
    routing: &'a String,
    stored_fields: &'a Vec<String>,
}
pub struct GlobalExplainRequestBuilder<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    analyze_wildcard: Option<&'a bool>,
    analyzer: &'a String,
    default_operator: Option<&'a i32>,
    df: &'a String,
    lenient: Option<&'a bool>,
    preference: &'a String,
    q: &'a String,
    routing: &'a String,
    stored_fields: &'a Vec<String>,
}
impl<'a> GlobalExplainRequestBuilder<'a> {
    pub fn build(&self) -> GlobalExplainRequest<'a> {
        GlobalExplainRequest {
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
        }
    }
}
pub struct GlobalFieldCapsRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    fields: &'a Vec<String>,
    ignore_unavailable: Option<&'a bool>,
    include_unmapped: Option<&'a bool>,
}
pub struct GlobalFieldCapsRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    fields: &'a Vec<String>,
    ignore_unavailable: Option<&'a bool>,
    include_unmapped: Option<&'a bool>,
}
impl<'a> GlobalFieldCapsRequestBuilder<'a> {
    pub fn build(&self) -> GlobalFieldCapsRequest<'a> {
        GlobalFieldCapsRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            fields: self.fields,
            ignore_unavailable: self.ignore_unavailable,
            include_unmapped: self.include_unmapped,
        }
    }
}
pub struct GlobalGetRequest<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    refresh: Option<&'a bool>,
    routing: &'a String,
    stored_fields: &'a Vec<String>,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
}
pub struct GlobalGetRequestBuilder<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    refresh: Option<&'a bool>,
    routing: &'a String,
    stored_fields: &'a Vec<String>,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
}
impl<'a> GlobalGetRequestBuilder<'a> {
    pub fn build(&self) -> GlobalGetRequest<'a> {
        GlobalGetRequest {
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
        }
    }
}
pub struct GlobalGetScriptRequest<'a> {
    master_timeout: &'a String,
}
pub struct GlobalGetScriptRequestBuilder<'a> {
    master_timeout: &'a String,
}
impl<'a> GlobalGetScriptRequestBuilder<'a> {
    pub fn build(&self) -> GlobalGetScriptRequest<'a> {
        GlobalGetScriptRequest {
            master_timeout: self.master_timeout,
        }
    }
}
pub struct GlobalGetSourceRequest<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    refresh: Option<&'a bool>,
    routing: &'a String,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
}
pub struct GlobalGetSourceRequestBuilder<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    refresh: Option<&'a bool>,
    routing: &'a String,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
}
impl<'a> GlobalGetSourceRequestBuilder<'a> {
    pub fn build(&self) -> GlobalGetSourceRequest<'a> {
        GlobalGetSourceRequest {
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            preference: self.preference,
            realtime: self.realtime,
            refresh: self.refresh,
            routing: self.routing,
            version: self.version,
            version_type: self.version_type,
        }
    }
}
pub struct GlobalIndexRequest<'a> {
    if_primary_term: Option<&'a i64>,
    if_seq_no: Option<&'a i64>,
    op_type: Option<&'a i32>,
    pipeline: &'a String,
    refresh: Option<&'a i32>,
    routing: &'a String,
    timeout: &'a String,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
    wait_for_active_shards: &'a String,
}
pub struct GlobalIndexRequestBuilder<'a> {
    if_primary_term: Option<&'a i64>,
    if_seq_no: Option<&'a i64>,
    op_type: Option<&'a i32>,
    pipeline: &'a String,
    refresh: Option<&'a i32>,
    routing: &'a String,
    timeout: &'a String,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
    wait_for_active_shards: &'a String,
}
impl<'a> GlobalIndexRequestBuilder<'a> {
    pub fn build(&self) -> GlobalIndexRequest<'a> {
        GlobalIndexRequest {
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
        }
    }
}
pub struct GlobalInfoRequest<'a> {}
pub struct GlobalInfoRequestBuilder<'a> {}
impl<'a> GlobalInfoRequestBuilder<'a> {
    pub fn build(&self) -> GlobalInfoRequest<'a> {
        GlobalInfoRequest {}
    }
}
pub struct GlobalMgetRequest<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    refresh: Option<&'a bool>,
    routing: &'a String,
    stored_fields: &'a Vec<String>,
}
pub struct GlobalMgetRequestBuilder<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    refresh: Option<&'a bool>,
    routing: &'a String,
    stored_fields: &'a Vec<String>,
}
impl<'a> GlobalMgetRequestBuilder<'a> {
    pub fn build(&self) -> GlobalMgetRequest<'a> {
        GlobalMgetRequest {
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            preference: self.preference,
            realtime: self.realtime,
            refresh: self.refresh,
            routing: self.routing,
            stored_fields: self.stored_fields,
        }
    }
}
pub struct GlobalMsearchRequest<'a> {
    ccs_minimize_roundtrips: Option<&'a bool>,
    max_concurrent_searches: Option<&'a i64>,
    max_concurrent_shard_requests: Option<&'a i64>,
    pre_filter_shard_size: Option<&'a i64>,
    rest_total_hits_as_int: Option<&'a bool>,
    search_type: Option<&'a i32>,
    typed_keys: Option<&'a bool>,
}
pub struct GlobalMsearchRequestBuilder<'a> {
    ccs_minimize_roundtrips: Option<&'a bool>,
    max_concurrent_searches: Option<&'a i64>,
    max_concurrent_shard_requests: Option<&'a i64>,
    pre_filter_shard_size: Option<&'a i64>,
    rest_total_hits_as_int: Option<&'a bool>,
    search_type: Option<&'a i32>,
    typed_keys: Option<&'a bool>,
}
impl<'a> GlobalMsearchRequestBuilder<'a> {
    pub fn build(&self) -> GlobalMsearchRequest<'a> {
        GlobalMsearchRequest {
            ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
            max_concurrent_searches: self.max_concurrent_searches,
            max_concurrent_shard_requests: self.max_concurrent_shard_requests,
            pre_filter_shard_size: self.pre_filter_shard_size,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            search_type: self.search_type,
            typed_keys: self.typed_keys,
        }
    }
}
pub struct GlobalMsearchTemplateRequest<'a> {
    ccs_minimize_roundtrips: Option<&'a bool>,
    max_concurrent_searches: Option<&'a i64>,
    rest_total_hits_as_int: Option<&'a bool>,
    search_type: Option<&'a i32>,
    typed_keys: Option<&'a bool>,
}
pub struct GlobalMsearchTemplateRequestBuilder<'a> {
    ccs_minimize_roundtrips: Option<&'a bool>,
    max_concurrent_searches: Option<&'a i64>,
    rest_total_hits_as_int: Option<&'a bool>,
    search_type: Option<&'a i32>,
    typed_keys: Option<&'a bool>,
}
impl<'a> GlobalMsearchTemplateRequestBuilder<'a> {
    pub fn build(&self) -> GlobalMsearchTemplateRequest<'a> {
        GlobalMsearchTemplateRequest {
            ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
            max_concurrent_searches: self.max_concurrent_searches,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            search_type: self.search_type,
            typed_keys: self.typed_keys,
        }
    }
}
pub struct GlobalMtermvectorsRequest<'a> {
    field_statistics: Option<&'a bool>,
    fields: &'a Vec<String>,
    ids: &'a Vec<String>,
    offsets: Option<&'a bool>,
    payloads: Option<&'a bool>,
    positions: Option<&'a bool>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    routing: &'a String,
    term_statistics: Option<&'a bool>,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
}
pub struct GlobalMtermvectorsRequestBuilder<'a> {
    field_statistics: Option<&'a bool>,
    fields: &'a Vec<String>,
    ids: &'a Vec<String>,
    offsets: Option<&'a bool>,
    payloads: Option<&'a bool>,
    positions: Option<&'a bool>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    routing: &'a String,
    term_statistics: Option<&'a bool>,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
}
impl<'a> GlobalMtermvectorsRequestBuilder<'a> {
    pub fn build(&self) -> GlobalMtermvectorsRequest<'a> {
        GlobalMtermvectorsRequest {
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
        }
    }
}
pub struct GlobalPingRequest<'a> {}
pub struct GlobalPingRequestBuilder<'a> {}
impl<'a> GlobalPingRequestBuilder<'a> {
    pub fn build(&self) -> GlobalPingRequest<'a> {
        GlobalPingRequest {}
    }
}
pub struct GlobalPutScriptRequest<'a> {
    context: &'a String,
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct GlobalPutScriptRequestBuilder<'a> {
    context: &'a String,
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> GlobalPutScriptRequestBuilder<'a> {
    pub fn build(&self) -> GlobalPutScriptRequest<'a> {
        GlobalPutScriptRequest {
            context: self.context,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
pub struct GlobalRankEvalRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
pub struct GlobalRankEvalRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
impl<'a> GlobalRankEvalRequestBuilder<'a> {
    pub fn build(&self) -> GlobalRankEvalRequest<'a> {
        GlobalRankEvalRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
        }
    }
}
pub struct GlobalReindexRequest<'a> {
    max_docs: Option<&'a i64>,
    refresh: Option<&'a bool>,
    requests_per_second: Option<&'a i64>,
    scroll: &'a String,
    slices: Option<&'a i64>,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
    wait_for_completion: Option<&'a bool>,
}
pub struct GlobalReindexRequestBuilder<'a> {
    max_docs: Option<&'a i64>,
    refresh: Option<&'a bool>,
    requests_per_second: Option<&'a i64>,
    scroll: &'a String,
    slices: Option<&'a i64>,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> GlobalReindexRequestBuilder<'a> {
    pub fn build(&self) -> GlobalReindexRequest<'a> {
        GlobalReindexRequest {
            max_docs: self.max_docs,
            refresh: self.refresh,
            requests_per_second: self.requests_per_second,
            scroll: self.scroll,
            slices: self.slices,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
            wait_for_completion: self.wait_for_completion,
        }
    }
}
pub struct GlobalReindexRethrottleRequest<'a> {
    requests_per_second: Option<&'a i64>,
}
pub struct GlobalReindexRethrottleRequestBuilder<'a> {
    requests_per_second: Option<&'a i64>,
}
impl<'a> GlobalReindexRethrottleRequestBuilder<'a> {
    pub fn build(&self) -> GlobalReindexRethrottleRequest<'a> {
        GlobalReindexRethrottleRequest {
            requests_per_second: self.requests_per_second,
        }
    }
}
pub struct GlobalRenderSearchTemplateRequest<'a> {}
pub struct GlobalRenderSearchTemplateRequestBuilder<'a> {}
impl<'a> GlobalRenderSearchTemplateRequestBuilder<'a> {
    pub fn build(&self) -> GlobalRenderSearchTemplateRequest<'a> {
        GlobalRenderSearchTemplateRequest {}
    }
}
pub struct GlobalScriptsPainlessExecuteRequest<'a> {}
pub struct GlobalScriptsPainlessExecuteRequestBuilder<'a> {}
impl<'a> GlobalScriptsPainlessExecuteRequestBuilder<'a> {
    pub fn build(&self) -> GlobalScriptsPainlessExecuteRequest<'a> {
        GlobalScriptsPainlessExecuteRequest {}
    }
}
pub struct GlobalScrollRequest<'a> {
    rest_total_hits_as_int: Option<&'a bool>,
    scroll: &'a String,
    scroll_id: &'a String,
}
pub struct GlobalScrollRequestBuilder<'a> {
    rest_total_hits_as_int: Option<&'a bool>,
    scroll: &'a String,
    scroll_id: &'a String,
}
impl<'a> GlobalScrollRequestBuilder<'a> {
    pub fn build(&self) -> GlobalScrollRequest<'a> {
        GlobalScrollRequest {
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            scroll: self.scroll,
            scroll_id: self.scroll_id,
        }
    }
}
pub struct GlobalSearchRequest<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    allow_no_indices: Option<&'a bool>,
    allow_partial_search_results: Option<&'a bool>,
    analyze_wildcard: Option<&'a bool>,
    analyzer: &'a String,
    batched_reduce_size: Option<&'a i64>,
    ccs_minimize_roundtrips: Option<&'a bool>,
    default_operator: Option<&'a i32>,
    df: &'a String,
    docvalue_fields: &'a Vec<String>,
    expand_wildcards: Option<&'a i32>,
    explain: Option<&'a bool>,
    from: Option<&'a i64>,
    ignore_throttled: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    lenient: Option<&'a bool>,
    max_concurrent_shard_requests: Option<&'a i64>,
    pre_filter_shard_size: Option<&'a i64>,
    preference: &'a String,
    q: &'a String,
    request_cache: Option<&'a bool>,
    rest_total_hits_as_int: Option<&'a bool>,
    routing: &'a Vec<String>,
    scroll: &'a String,
    search_type: Option<&'a i32>,
    seq_no_primary_term: Option<&'a bool>,
    size: Option<&'a i64>,
    sort: &'a Vec<String>,
    stats: &'a Vec<String>,
    stored_fields: &'a Vec<String>,
    suggest_field: &'a String,
    suggest_mode: Option<&'a i32>,
    suggest_size: Option<&'a i64>,
    suggest_text: &'a String,
    terminate_after: Option<&'a i64>,
    timeout: &'a String,
    track_scores: Option<&'a bool>,
    track_total_hits: Option<&'a bool>,
    typed_keys: Option<&'a bool>,
    version: Option<&'a bool>,
}
pub struct GlobalSearchRequestBuilder<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    allow_no_indices: Option<&'a bool>,
    allow_partial_search_results: Option<&'a bool>,
    analyze_wildcard: Option<&'a bool>,
    analyzer: &'a String,
    batched_reduce_size: Option<&'a i64>,
    ccs_minimize_roundtrips: Option<&'a bool>,
    default_operator: Option<&'a i32>,
    df: &'a String,
    docvalue_fields: &'a Vec<String>,
    expand_wildcards: Option<&'a i32>,
    explain: Option<&'a bool>,
    from: Option<&'a i64>,
    ignore_throttled: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    lenient: Option<&'a bool>,
    max_concurrent_shard_requests: Option<&'a i64>,
    pre_filter_shard_size: Option<&'a i64>,
    preference: &'a String,
    q: &'a String,
    request_cache: Option<&'a bool>,
    rest_total_hits_as_int: Option<&'a bool>,
    routing: &'a Vec<String>,
    scroll: &'a String,
    search_type: Option<&'a i32>,
    seq_no_primary_term: Option<&'a bool>,
    size: Option<&'a i64>,
    sort: &'a Vec<String>,
    stats: &'a Vec<String>,
    stored_fields: &'a Vec<String>,
    suggest_field: &'a String,
    suggest_mode: Option<&'a i32>,
    suggest_size: Option<&'a i64>,
    suggest_text: &'a String,
    terminate_after: Option<&'a i64>,
    timeout: &'a String,
    track_scores: Option<&'a bool>,
    track_total_hits: Option<&'a bool>,
    typed_keys: Option<&'a bool>,
    version: Option<&'a bool>,
}
impl<'a> GlobalSearchRequestBuilder<'a> {
    pub fn build(&self) -> GlobalSearchRequest<'a> {
        GlobalSearchRequest {
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
        }
    }
}
pub struct GlobalSearchShardsRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
    preference: &'a String,
    routing: &'a String,
}
pub struct GlobalSearchShardsRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
    preference: &'a String,
    routing: &'a String,
}
impl<'a> GlobalSearchShardsRequestBuilder<'a> {
    pub fn build(&self) -> GlobalSearchShardsRequest<'a> {
        GlobalSearchShardsRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            local: self.local,
            preference: self.preference,
            routing: self.routing,
        }
    }
}
pub struct GlobalSearchTemplateRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    ccs_minimize_roundtrips: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    explain: Option<&'a bool>,
    ignore_throttled: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    preference: &'a String,
    profile: Option<&'a bool>,
    rest_total_hits_as_int: Option<&'a bool>,
    routing: &'a Vec<String>,
    scroll: &'a String,
    search_type: Option<&'a i32>,
    typed_keys: Option<&'a bool>,
}
pub struct GlobalSearchTemplateRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    ccs_minimize_roundtrips: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    explain: Option<&'a bool>,
    ignore_throttled: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    preference: &'a String,
    profile: Option<&'a bool>,
    rest_total_hits_as_int: Option<&'a bool>,
    routing: &'a Vec<String>,
    scroll: &'a String,
    search_type: Option<&'a i32>,
    typed_keys: Option<&'a bool>,
}
impl<'a> GlobalSearchTemplateRequestBuilder<'a> {
    pub fn build(&self) -> GlobalSearchTemplateRequest<'a> {
        GlobalSearchTemplateRequest {
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
        }
    }
}
pub struct GlobalTermvectorsRequest<'a> {
    field_statistics: Option<&'a bool>,
    fields: &'a Vec<String>,
    offsets: Option<&'a bool>,
    payloads: Option<&'a bool>,
    positions: Option<&'a bool>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    routing: &'a String,
    term_statistics: Option<&'a bool>,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
}
pub struct GlobalTermvectorsRequestBuilder<'a> {
    field_statistics: Option<&'a bool>,
    fields: &'a Vec<String>,
    offsets: Option<&'a bool>,
    payloads: Option<&'a bool>,
    positions: Option<&'a bool>,
    preference: &'a String,
    realtime: Option<&'a bool>,
    routing: &'a String,
    term_statistics: Option<&'a bool>,
    version: Option<&'a i64>,
    version_type: Option<&'a i32>,
}
impl<'a> GlobalTermvectorsRequestBuilder<'a> {
    pub fn build(&self) -> GlobalTermvectorsRequest<'a> {
        GlobalTermvectorsRequest {
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
        }
    }
}
pub struct GlobalUpdateRequest<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    if_primary_term: Option<&'a i64>,
    if_seq_no: Option<&'a i64>,
    lang: &'a String,
    refresh: Option<&'a i32>,
    retry_on_conflict: Option<&'a i64>,
    routing: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
pub struct GlobalUpdateRequestBuilder<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    if_primary_term: Option<&'a i64>,
    if_seq_no: Option<&'a i64>,
    lang: &'a String,
    refresh: Option<&'a i32>,
    retry_on_conflict: Option<&'a i64>,
    routing: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
impl<'a> GlobalUpdateRequestBuilder<'a> {
    pub fn build(&self) -> GlobalUpdateRequest<'a> {
        GlobalUpdateRequest {
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
        }
    }
}
pub struct GlobalUpdateByQueryRequest<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    allow_no_indices: Option<&'a bool>,
    analyze_wildcard: Option<&'a bool>,
    analyzer: &'a String,
    conflicts: Option<&'a i32>,
    default_operator: Option<&'a i32>,
    df: &'a String,
    expand_wildcards: Option<&'a i32>,
    from: Option<&'a i64>,
    ignore_unavailable: Option<&'a bool>,
    lenient: Option<&'a bool>,
    max_docs: Option<&'a i64>,
    pipeline: &'a String,
    preference: &'a String,
    q: &'a String,
    refresh: Option<&'a bool>,
    request_cache: Option<&'a bool>,
    requests_per_second: Option<&'a i64>,
    routing: &'a Vec<String>,
    scroll: &'a String,
    scroll_size: Option<&'a i64>,
    search_timeout: &'a String,
    search_type: Option<&'a i32>,
    size: Option<&'a i64>,
    slices: Option<&'a i64>,
    sort: &'a Vec<String>,
    stats: &'a Vec<String>,
    terminate_after: Option<&'a i64>,
    timeout: &'a String,
    version: Option<&'a bool>,
    version_type: Option<&'a bool>,
    wait_for_active_shards: &'a String,
    wait_for_completion: Option<&'a bool>,
}
pub struct GlobalUpdateByQueryRequestBuilder<'a> {
    _source: &'a Vec<String>,
    _source_excludes: &'a Vec<String>,
    _source_includes: &'a Vec<String>,
    allow_no_indices: Option<&'a bool>,
    analyze_wildcard: Option<&'a bool>,
    analyzer: &'a String,
    conflicts: Option<&'a i32>,
    default_operator: Option<&'a i32>,
    df: &'a String,
    expand_wildcards: Option<&'a i32>,
    from: Option<&'a i64>,
    ignore_unavailable: Option<&'a bool>,
    lenient: Option<&'a bool>,
    max_docs: Option<&'a i64>,
    pipeline: &'a String,
    preference: &'a String,
    q: &'a String,
    refresh: Option<&'a bool>,
    request_cache: Option<&'a bool>,
    requests_per_second: Option<&'a i64>,
    routing: &'a Vec<String>,
    scroll: &'a String,
    scroll_size: Option<&'a i64>,
    search_timeout: &'a String,
    search_type: Option<&'a i32>,
    size: Option<&'a i64>,
    slices: Option<&'a i64>,
    sort: &'a Vec<String>,
    stats: &'a Vec<String>,
    terminate_after: Option<&'a i64>,
    timeout: &'a String,
    version: Option<&'a bool>,
    version_type: Option<&'a bool>,
    wait_for_active_shards: &'a String,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> GlobalUpdateByQueryRequestBuilder<'a> {
    pub fn build(&self) -> GlobalUpdateByQueryRequest<'a> {
        GlobalUpdateByQueryRequest {
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
        }
    }
}
pub struct GlobalUpdateByQueryRethrottleRequest<'a> {
    requests_per_second: Option<&'a i64>,
}
pub struct GlobalUpdateByQueryRethrottleRequestBuilder<'a> {
    requests_per_second: Option<&'a i64>,
}
impl<'a> GlobalUpdateByQueryRethrottleRequestBuilder<'a> {
    pub fn build(&self) -> GlobalUpdateByQueryRethrottleRequest<'a> {
        GlobalUpdateByQueryRethrottleRequest {
            requests_per_second: self.requests_per_second,
        }
    }
}
#[doc = "Global APIs"]
pub struct GlobalNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> GlobalNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        GlobalNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-bulk.html"]
    pub fn bulk(&self, request: &GlobalBulkRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_bulk")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-request-body.html#request-body-search-scroll"]
    pub fn clear_scroll(&self, request: &GlobalClearScrollRequest) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_search/scroll")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-count.html"]
    pub fn count(&self, request: &GlobalCountRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_count")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-index_.html"]
    pub fn create(&self, request: &GlobalCreateRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/{index}/_create/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-delete.html"]
    pub fn delete(&self, request: &GlobalDeleteRequest) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/{index}/_doc/{id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-delete-by-query.html"]
    pub fn delete_by_query(&self, request: &GlobalDeleteByQueryRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/{index}/_delete_by_query")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-delete-by-query.html"]
    pub fn delete_by_query_rethrottle(
        &self,
        request: &GlobalDeleteByQueryRethrottleRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_delete_by_query/{task_id}/_rethrottle")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html"]
    pub fn delete_script(&self, request: &GlobalDeleteScriptRequest) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_scripts/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn exists(&self, request: &GlobalExistsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Head, "/{index}/_doc/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn exists_source(&self, request: &GlobalExistsSourceRequest) -> Result<Response> {
        self.client.send(HttpMethod::Head, "/{index}/_source/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-explain.html"]
    pub fn explain(&self, request: &GlobalExplainRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_explain/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-field-caps.html"]
    pub fn field_caps(&self, request: &GlobalFieldCapsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_field_caps")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn get(&self, request: &GlobalGetRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_doc/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html"]
    pub fn get_script(&self, request: &GlobalGetScriptRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_scripts/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn get_source(&self, request: &GlobalGetSourceRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_source/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-index_.html"]
    pub fn index(&self, request: &GlobalIndexRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_doc/{id}")
    }
    #[doc = "http://www.elastic.co/guide/"]
    pub fn info(&self, request: &GlobalInfoRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-multi-get.html"]
    pub fn mget(&self, request: &GlobalMgetRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_mget")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-multi-search.html"]
    pub fn msearch(&self, request: &GlobalMsearchRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_msearch")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/search-multi-search.html"]
    pub fn msearch_template(&self, request: &GlobalMsearchTemplateRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_msearch/template")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-multi-termvectors.html"]
    pub fn mtermvectors(&self, request: &GlobalMtermvectorsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_mtermvectors")
    }
    #[doc = "http://www.elastic.co/guide/"]
    pub fn ping(&self, request: &GlobalPingRequest) -> Result<Response> {
        self.client.send(HttpMethod::Head, "/")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html"]
    pub fn put_script(&self, request: &GlobalPutScriptRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_scripts/{id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/search-rank-eval.html"]
    pub fn rank_eval(&self, request: &GlobalRankEvalRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_rank_eval")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-reindex.html"]
    pub fn reindex(&self, request: &GlobalReindexRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_reindex")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-reindex.html"]
    pub fn reindex_rethrottle(&self, request: &GlobalReindexRethrottleRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_reindex/{task_id}/_rethrottle")
    }
    #[doc = "http://www.elasticsearch.org/guide/en/elasticsearch/reference/master/search-template.html"]
    pub fn render_search_template(
        &self,
        request: &GlobalRenderSearchTemplateRequest,
    ) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_render/template")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/painless/master/painless-execute-api.html"]
    pub fn scripts_painless_execute(
        &self,
        request: &GlobalScriptsPainlessExecuteRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_scripts/painless/_execute")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-request-body.html#request-body-search-scroll"]
    pub fn scroll(&self, request: &GlobalScrollRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_search/scroll")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html"]
    pub fn search(&self, request: &GlobalSearchRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_search")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-shards.html"]
    pub fn search_shards(&self, request: &GlobalSearchShardsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_search_shards")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/search-template.html"]
    pub fn search_template(&self, request: &GlobalSearchTemplateRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_search/template")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-termvectors.html"]
    pub fn termvectors(&self, request: &GlobalTermvectorsRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/{index}/_termvectors/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-update.html"]
    pub fn update(&self, request: &GlobalUpdateRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_update/{id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-update-by-query.html"]
    pub fn update_by_query(&self, request: &GlobalUpdateByQueryRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/{index}/_update_by_query")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-update-by-query.html"]
    pub fn update_by_query_rethrottle(
        &self,
        request: &GlobalUpdateByQueryRethrottleRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_update_by_query/{task_id}/_rethrottle")
    }
}
impl ElasticsearchClient {
    #[doc = "Global APIs"]
    pub fn global(&self) -> GlobalNamespaceClient {
        GlobalNamespaceClient::new(self)
    }
}
