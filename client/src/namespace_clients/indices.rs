use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct IndicesAnalyzeRequest<'a> {
    index: &'a String,
}
pub struct IndicesAnalyzeRequestBuilder<'a> {
    index: &'a String,
}
impl<'a> IndicesAnalyzeRequestBuilder<'a> {
    pub fn build(&self) -> IndicesAnalyzeRequest<'a> {
        IndicesAnalyzeRequest { index: self.index }
    }
}
pub struct IndicesClearCacheRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    fielddata: Option<&'a bool>,
    fields: &'a Vec<String>,
    ignore_unavailable: Option<&'a bool>,
    index: &'a Vec<String>,
    query: Option<&'a bool>,
    request: Option<&'a bool>,
}
pub struct IndicesClearCacheRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    fielddata: Option<&'a bool>,
    fields: &'a Vec<String>,
    ignore_unavailable: Option<&'a bool>,
    index: &'a Vec<String>,
    query: Option<&'a bool>,
    request: Option<&'a bool>,
}
impl<'a> IndicesClearCacheRequestBuilder<'a> {
    pub fn build(&self) -> IndicesClearCacheRequest<'a> {
        IndicesClearCacheRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            fielddata: self.fielddata,
            fields: self.fields,
            ignore_unavailable: self.ignore_unavailable,
            index: self.index,
            query: self.query,
            request: self.request,
        }
    }
}
pub struct IndicesCloseRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
pub struct IndicesCloseRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
impl<'a> IndicesCloseRequestBuilder<'a> {
    pub fn build(&self) -> IndicesCloseRequest<'a> {
        IndicesCloseRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
}
pub struct IndicesCreateRequest<'a> {
    include_type_name: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
pub struct IndicesCreateRequestBuilder<'a> {
    include_type_name: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
impl<'a> IndicesCreateRequestBuilder<'a> {
    pub fn build(&self) -> IndicesCreateRequest<'a> {
        IndicesCreateRequest {
            include_type_name: self.include_type_name,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
}
pub struct IndicesDeleteRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct IndicesDeleteRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> IndicesDeleteRequestBuilder<'a> {
    pub fn build(&self) -> IndicesDeleteRequest<'a> {
        IndicesDeleteRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
pub struct IndicesDeleteAliasRequest<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct IndicesDeleteAliasRequestBuilder<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> IndicesDeleteAliasRequestBuilder<'a> {
    pub fn build(&self) -> IndicesDeleteAliasRequest<'a> {
        IndicesDeleteAliasRequest {
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
pub struct IndicesDeleteTemplateRequest<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct IndicesDeleteTemplateRequestBuilder<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> IndicesDeleteTemplateRequestBuilder<'a> {
    pub fn build(&self) -> IndicesDeleteTemplateRequest<'a> {
        IndicesDeleteTemplateRequest {
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
pub struct IndicesExistsRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    local: Option<&'a bool>,
}
pub struct IndicesExistsRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    local: Option<&'a bool>,
}
impl<'a> IndicesExistsRequestBuilder<'a> {
    pub fn build(&self) -> IndicesExistsRequest<'a> {
        IndicesExistsRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            flat_settings: self.flat_settings,
            ignore_unavailable: self.ignore_unavailable,
            include_defaults: self.include_defaults,
            local: self.local,
        }
    }
}
pub struct IndicesExistsAliasRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
}
pub struct IndicesExistsAliasRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
}
impl<'a> IndicesExistsAliasRequestBuilder<'a> {
    pub fn build(&self) -> IndicesExistsAliasRequest<'a> {
        IndicesExistsAliasRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            local: self.local,
        }
    }
}
pub struct IndicesExistsTemplateRequest<'a> {
    flat_settings: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
pub struct IndicesExistsTemplateRequestBuilder<'a> {
    flat_settings: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
impl<'a> IndicesExistsTemplateRequestBuilder<'a> {
    pub fn build(&self) -> IndicesExistsTemplateRequest<'a> {
        IndicesExistsTemplateRequest {
            flat_settings: self.flat_settings,
            local: self.local,
            master_timeout: self.master_timeout,
        }
    }
}
pub struct IndicesExistsTypeRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
}
pub struct IndicesExistsTypeRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
}
impl<'a> IndicesExistsTypeRequestBuilder<'a> {
    pub fn build(&self) -> IndicesExistsTypeRequest<'a> {
        IndicesExistsTypeRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            local: self.local,
        }
    }
}
pub struct IndicesFlushRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    force: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    wait_if_ongoing: Option<&'a bool>,
}
pub struct IndicesFlushRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    force: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    wait_if_ongoing: Option<&'a bool>,
}
impl<'a> IndicesFlushRequestBuilder<'a> {
    pub fn build(&self) -> IndicesFlushRequest<'a> {
        IndicesFlushRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            force: self.force,
            ignore_unavailable: self.ignore_unavailable,
            wait_if_ongoing: self.wait_if_ongoing,
        }
    }
}
pub struct IndicesFlushSyncedRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
pub struct IndicesFlushSyncedRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
impl<'a> IndicesFlushSyncedRequestBuilder<'a> {
    pub fn build(&self) -> IndicesFlushSyncedRequest<'a> {
        IndicesFlushSyncedRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
        }
    }
}
pub struct IndicesForcemergeRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flush: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    max_num_segments: Option<&'a i64>,
    only_expunge_deletes: Option<&'a bool>,
}
pub struct IndicesForcemergeRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flush: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    max_num_segments: Option<&'a i64>,
    only_expunge_deletes: Option<&'a bool>,
}
impl<'a> IndicesForcemergeRequestBuilder<'a> {
    pub fn build(&self) -> IndicesForcemergeRequest<'a> {
        IndicesForcemergeRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            flush: self.flush,
            ignore_unavailable: self.ignore_unavailable,
            max_num_segments: self.max_num_segments,
            only_expunge_deletes: self.only_expunge_deletes,
        }
    }
}
pub struct IndicesFreezeRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
pub struct IndicesFreezeRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
impl<'a> IndicesFreezeRequestBuilder<'a> {
    pub fn build(&self) -> IndicesFreezeRequest<'a> {
        IndicesFreezeRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
}
pub struct IndicesGetRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
pub struct IndicesGetRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
impl<'a> IndicesGetRequestBuilder<'a> {
    pub fn build(&self) -> IndicesGetRequest<'a> {
        IndicesGetRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            flat_settings: self.flat_settings,
            ignore_unavailable: self.ignore_unavailable,
            include_defaults: self.include_defaults,
            include_type_name: self.include_type_name,
            local: self.local,
            master_timeout: self.master_timeout,
        }
    }
}
pub struct IndicesGetAliasRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
}
pub struct IndicesGetAliasRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
}
impl<'a> IndicesGetAliasRequestBuilder<'a> {
    pub fn build(&self) -> IndicesGetAliasRequest<'a> {
        IndicesGetAliasRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            local: self.local,
        }
    }
}
pub struct IndicesGetFieldMappingRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    local: Option<&'a bool>,
}
pub struct IndicesGetFieldMappingRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    local: Option<&'a bool>,
}
impl<'a> IndicesGetFieldMappingRequestBuilder<'a> {
    pub fn build(&self) -> IndicesGetFieldMappingRequest<'a> {
        IndicesGetFieldMappingRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            include_defaults: self.include_defaults,
            include_type_name: self.include_type_name,
            local: self.local,
        }
    }
}
pub struct IndicesGetMappingRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
pub struct IndicesGetMappingRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
impl<'a> IndicesGetMappingRequestBuilder<'a> {
    pub fn build(&self) -> IndicesGetMappingRequest<'a> {
        IndicesGetMappingRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            include_type_name: self.include_type_name,
            local: self.local,
            master_timeout: self.master_timeout,
        }
    }
}
pub struct IndicesGetSettingsRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
pub struct IndicesGetSettingsRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
impl<'a> IndicesGetSettingsRequestBuilder<'a> {
    pub fn build(&self) -> IndicesGetSettingsRequest<'a> {
        IndicesGetSettingsRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            flat_settings: self.flat_settings,
            ignore_unavailable: self.ignore_unavailable,
            include_defaults: self.include_defaults,
            local: self.local,
            master_timeout: self.master_timeout,
        }
    }
}
pub struct IndicesGetTemplateRequest<'a> {
    flat_settings: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
pub struct IndicesGetTemplateRequestBuilder<'a> {
    flat_settings: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
impl<'a> IndicesGetTemplateRequestBuilder<'a> {
    pub fn build(&self) -> IndicesGetTemplateRequest<'a> {
        IndicesGetTemplateRequest {
            flat_settings: self.flat_settings,
            include_type_name: self.include_type_name,
            local: self.local,
            master_timeout: self.master_timeout,
        }
    }
}
pub struct IndicesGetUpgradeRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
pub struct IndicesGetUpgradeRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
impl<'a> IndicesGetUpgradeRequestBuilder<'a> {
    pub fn build(&self) -> IndicesGetUpgradeRequest<'a> {
        IndicesGetUpgradeRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
        }
    }
}
pub struct IndicesOpenRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
pub struct IndicesOpenRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
impl<'a> IndicesOpenRequestBuilder<'a> {
    pub fn build(&self) -> IndicesOpenRequest<'a> {
        IndicesOpenRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
}
pub struct IndicesPutAliasRequest<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct IndicesPutAliasRequestBuilder<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> IndicesPutAliasRequestBuilder<'a> {
    pub fn build(&self) -> IndicesPutAliasRequest<'a> {
        IndicesPutAliasRequest {
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
pub struct IndicesPutMappingRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct IndicesPutMappingRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> IndicesPutMappingRequestBuilder<'a> {
    pub fn build(&self) -> IndicesPutMappingRequest<'a> {
        IndicesPutMappingRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            include_type_name: self.include_type_name,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
pub struct IndicesPutSettingsRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    preserve_existing: Option<&'a bool>,
    timeout: &'a String,
}
pub struct IndicesPutSettingsRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    preserve_existing: Option<&'a bool>,
    timeout: &'a String,
}
impl<'a> IndicesPutSettingsRequestBuilder<'a> {
    pub fn build(&self) -> IndicesPutSettingsRequest<'a> {
        IndicesPutSettingsRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            flat_settings: self.flat_settings,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            preserve_existing: self.preserve_existing,
            timeout: self.timeout,
        }
    }
}
pub struct IndicesPutTemplateRequest<'a> {
    create: Option<&'a bool>,
    flat_settings: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    master_timeout: &'a String,
    order: Option<&'a i64>,
    timeout: &'a String,
}
pub struct IndicesPutTemplateRequestBuilder<'a> {
    create: Option<&'a bool>,
    flat_settings: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    master_timeout: &'a String,
    order: Option<&'a i64>,
    timeout: &'a String,
}
impl<'a> IndicesPutTemplateRequestBuilder<'a> {
    pub fn build(&self) -> IndicesPutTemplateRequest<'a> {
        IndicesPutTemplateRequest {
            create: self.create,
            flat_settings: self.flat_settings,
            include_type_name: self.include_type_name,
            master_timeout: self.master_timeout,
            order: self.order,
            timeout: self.timeout,
        }
    }
}
pub struct IndicesRecoveryRequest<'a> {
    active_only: Option<&'a bool>,
    detailed: Option<&'a bool>,
}
pub struct IndicesRecoveryRequestBuilder<'a> {
    active_only: Option<&'a bool>,
    detailed: Option<&'a bool>,
}
impl<'a> IndicesRecoveryRequestBuilder<'a> {
    pub fn build(&self) -> IndicesRecoveryRequest<'a> {
        IndicesRecoveryRequest {
            active_only: self.active_only,
            detailed: self.detailed,
        }
    }
}
pub struct IndicesRefreshRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
pub struct IndicesRefreshRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
impl<'a> IndicesRefreshRequestBuilder<'a> {
    pub fn build(&self) -> IndicesRefreshRequest<'a> {
        IndicesRefreshRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
        }
    }
}
pub struct IndicesReloadSearchAnalyzersRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
pub struct IndicesReloadSearchAnalyzersRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
impl<'a> IndicesReloadSearchAnalyzersRequestBuilder<'a> {
    pub fn build(&self) -> IndicesReloadSearchAnalyzersRequest<'a> {
        IndicesReloadSearchAnalyzersRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
        }
    }
}
pub struct IndicesRolloverRequest<'a> {
    dry_run: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
pub struct IndicesRolloverRequestBuilder<'a> {
    dry_run: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
impl<'a> IndicesRolloverRequestBuilder<'a> {
    pub fn build(&self) -> IndicesRolloverRequest<'a> {
        IndicesRolloverRequest {
            dry_run: self.dry_run,
            include_type_name: self.include_type_name,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
}
pub struct IndicesSegmentsRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    verbose: Option<&'a bool>,
}
pub struct IndicesSegmentsRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    verbose: Option<&'a bool>,
}
impl<'a> IndicesSegmentsRequestBuilder<'a> {
    pub fn build(&self) -> IndicesSegmentsRequest<'a> {
        IndicesSegmentsRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            verbose: self.verbose,
        }
    }
}
pub struct IndicesShardStoresRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    status: &'a Vec<String>,
}
pub struct IndicesShardStoresRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    status: &'a Vec<String>,
}
impl<'a> IndicesShardStoresRequestBuilder<'a> {
    pub fn build(&self) -> IndicesShardStoresRequest<'a> {
        IndicesShardStoresRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            status: self.status,
        }
    }
}
pub struct IndicesShrinkRequest<'a> {
    copy_settings: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
pub struct IndicesShrinkRequestBuilder<'a> {
    copy_settings: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
impl<'a> IndicesShrinkRequestBuilder<'a> {
    pub fn build(&self) -> IndicesShrinkRequest<'a> {
        IndicesShrinkRequest {
            copy_settings: self.copy_settings,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
}
pub struct IndicesSplitRequest<'a> {
    copy_settings: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
pub struct IndicesSplitRequestBuilder<'a> {
    copy_settings: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
impl<'a> IndicesSplitRequestBuilder<'a> {
    pub fn build(&self) -> IndicesSplitRequest<'a> {
        IndicesSplitRequest {
            copy_settings: self.copy_settings,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
}
pub struct IndicesStatsRequest<'a> {
    completion_fields: &'a Vec<String>,
    expand_wildcards: Option<&'a i32>,
    fielddata_fields: &'a Vec<String>,
    fields: &'a Vec<String>,
    forbid_closed_indices: Option<&'a bool>,
    groups: &'a Vec<String>,
    include_segment_file_sizes: Option<&'a bool>,
    include_unloaded_segments: Option<&'a bool>,
    level: Option<&'a i32>,
    types: &'a Vec<String>,
}
pub struct IndicesStatsRequestBuilder<'a> {
    completion_fields: &'a Vec<String>,
    expand_wildcards: Option<&'a i32>,
    fielddata_fields: &'a Vec<String>,
    fields: &'a Vec<String>,
    forbid_closed_indices: Option<&'a bool>,
    groups: &'a Vec<String>,
    include_segment_file_sizes: Option<&'a bool>,
    include_unloaded_segments: Option<&'a bool>,
    level: Option<&'a i32>,
    types: &'a Vec<String>,
}
impl<'a> IndicesStatsRequestBuilder<'a> {
    pub fn build(&self) -> IndicesStatsRequest<'a> {
        IndicesStatsRequest {
            completion_fields: self.completion_fields,
            expand_wildcards: self.expand_wildcards,
            fielddata_fields: self.fielddata_fields,
            fields: self.fields,
            forbid_closed_indices: self.forbid_closed_indices,
            groups: self.groups,
            include_segment_file_sizes: self.include_segment_file_sizes,
            include_unloaded_segments: self.include_unloaded_segments,
            level: self.level,
            types: self.types,
        }
    }
}
pub struct IndicesUnfreezeRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
pub struct IndicesUnfreezeRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    timeout: &'a String,
    wait_for_active_shards: &'a String,
}
impl<'a> IndicesUnfreezeRequestBuilder<'a> {
    pub fn build(&self) -> IndicesUnfreezeRequest<'a> {
        IndicesUnfreezeRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
}
pub struct IndicesUpdateAliasesRequest<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct IndicesUpdateAliasesRequestBuilder<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> IndicesUpdateAliasesRequestBuilder<'a> {
    pub fn build(&self) -> IndicesUpdateAliasesRequest<'a> {
        IndicesUpdateAliasesRequest {
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
pub struct IndicesUpgradeRequest<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    only_ancient_segments: Option<&'a bool>,
    wait_for_completion: Option<&'a bool>,
}
pub struct IndicesUpgradeRequestBuilder<'a> {
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    only_ancient_segments: Option<&'a bool>,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> IndicesUpgradeRequestBuilder<'a> {
    pub fn build(&self) -> IndicesUpgradeRequest<'a> {
        IndicesUpgradeRequest {
            allow_no_indices: self.allow_no_indices,
            expand_wildcards: self.expand_wildcards,
            ignore_unavailable: self.ignore_unavailable,
            only_ancient_segments: self.only_ancient_segments,
            wait_for_completion: self.wait_for_completion,
        }
    }
}
pub struct IndicesValidateQueryRequest<'a> {
    all_shards: Option<&'a bool>,
    allow_no_indices: Option<&'a bool>,
    analyze_wildcard: Option<&'a bool>,
    analyzer: &'a String,
    default_operator: Option<&'a i32>,
    df: &'a String,
    expand_wildcards: Option<&'a i32>,
    explain: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    lenient: Option<&'a bool>,
    q: &'a String,
    rewrite: Option<&'a bool>,
}
pub struct IndicesValidateQueryRequestBuilder<'a> {
    all_shards: Option<&'a bool>,
    allow_no_indices: Option<&'a bool>,
    analyze_wildcard: Option<&'a bool>,
    analyzer: &'a String,
    default_operator: Option<&'a i32>,
    df: &'a String,
    expand_wildcards: Option<&'a i32>,
    explain: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    lenient: Option<&'a bool>,
    q: &'a String,
    rewrite: Option<&'a bool>,
}
impl<'a> IndicesValidateQueryRequestBuilder<'a> {
    pub fn build(&self) -> IndicesValidateQueryRequest<'a> {
        IndicesValidateQueryRequest {
            all_shards: self.all_shards,
            allow_no_indices: self.allow_no_indices,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            default_operator: self.default_operator,
            df: self.df,
            expand_wildcards: self.expand_wildcards,
            explain: self.explain,
            ignore_unavailable: self.ignore_unavailable,
            lenient: self.lenient,
            q: self.q,
            rewrite: self.rewrite,
        }
    }
}
#[doc = "Indices APIs"]
pub struct IndicesNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IndicesNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-analyze.html"]
    pub fn analyze(&self, request: &IndicesAnalyzeRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_analyze")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-clearcache.html"]
    pub fn clear_cache(&self, request: &IndicesClearCacheRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_cache/clear")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html"]
    pub fn close(&self, request: &IndicesCloseRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_close")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-create-index.html"]
    pub fn create(&self, request: &IndicesCreateRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/{index}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-delete-index.html"]
    pub fn delete(&self, request: &IndicesDeleteRequest) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/{index}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn delete_alias(&self, request: &IndicesDeleteAliasRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/{index}/_alias/{name}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn delete_template(&self, request: &IndicesDeleteTemplateRequest) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_template/{name}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-exists.html"]
    pub fn exists(&self, request: &IndicesExistsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Head, "/{index}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn exists_alias(&self, request: &IndicesExistsAliasRequest) -> Result<Response> {
        self.client.send(HttpMethod::Head, "/_alias/{name}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn exists_template(&self, request: &IndicesExistsTemplateRequest) -> Result<Response> {
        self.client.send(HttpMethod::Head, "/_template/{name}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-types-exists.html"]
    pub fn exists_type(&self, request: &IndicesExistsTypeRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Head, "/{index}/_mapping/{type}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-flush.html"]
    pub fn flush(&self, request: &IndicesFlushRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_flush")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-flush.html#synced-flush-api"]
    pub fn flush_synced(&self, request: &IndicesFlushSyncedRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_flush/synced")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-forcemerge.html"]
    pub fn forcemerge(&self, request: &IndicesForcemergeRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_forcemerge")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html"]
    pub fn freeze(&self, request: &IndicesFreezeRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_freeze")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-index.html"]
    pub fn get(&self, request: &IndicesGetRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn get_alias(&self, request: &IndicesGetAliasRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_alias")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-field-mapping.html"]
    pub fn get_field_mapping(&self, request: &IndicesGetFieldMappingRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_mapping/field/{fields}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-mapping.html"]
    pub fn get_mapping(&self, request: &IndicesGetMappingRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_mapping")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-settings.html"]
    pub fn get_settings(&self, request: &IndicesGetSettingsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_settings")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn get_template(&self, request: &IndicesGetTemplateRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_template")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html"]
    pub fn get_upgrade(&self, request: &IndicesGetUpgradeRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_upgrade")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html"]
    pub fn open(&self, request: &IndicesOpenRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_open")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn put_alias(&self, request: &IndicesPutAliasRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/{index}/_alias/{name}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-put-mapping.html"]
    pub fn put_mapping(&self, request: &IndicesPutMappingRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "{index}/_mapping")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-update-settings.html"]
    pub fn put_settings(&self, request: &IndicesPutSettingsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_settings")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn put_template(&self, request: &IndicesPutTemplateRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_template/{name}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-recovery.html"]
    pub fn recovery(&self, request: &IndicesRecoveryRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_recovery")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-refresh.html"]
    pub fn refresh(&self, request: &IndicesRefreshRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_refresh")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-reload-analyzers.html"]
    pub fn reload_search_analyzers(
        &self,
        request: &IndicesReloadSearchAnalyzersRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/{index}/_reload_search_analyzers")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-rollover-index.html"]
    pub fn rollover(&self, request: &IndicesRolloverRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{alias}/_rollover")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-segments.html"]
    pub fn segments(&self, request: &IndicesSegmentsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_segments")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shards-stores.html"]
    pub fn shard_stores(&self, request: &IndicesShardStoresRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_shard_stores")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shrink-index.html"]
    pub fn shrink(&self, request: &IndicesShrinkRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/{index}/_shrink/{target}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-split-index.html"]
    pub fn split(&self, request: &IndicesSplitRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/{index}/_split/{target}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-stats.html"]
    pub fn stats(&self, request: &IndicesStatsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_stats")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html"]
    pub fn unfreeze(&self, request: &IndicesUnfreezeRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_unfreeze")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn update_aliases(&self, request: &IndicesUpdateAliasesRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_aliases")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html"]
    pub fn upgrade(&self, request: &IndicesUpgradeRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_upgrade")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-validate.html"]
    pub fn validate_query(&self, request: &IndicesValidateQueryRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_validate/query")
    }
}
impl ElasticsearchClient {
    #[doc = "Indices APIs"]
    pub fn indices(&self) -> IndicesNamespaceClient {
        IndicesNamespaceClient::new(self)
    }
}
