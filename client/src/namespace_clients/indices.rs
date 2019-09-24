

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct IndicesAnalyzeBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    index: Option<String>,
}
impl IndicesAnalyzeBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesAnalyzeBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesAnalyzeBuilder {
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
pub struct IndicesClearCacheBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    fielddata: Option<bool>,
    fields: Option<Vec<String>>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    query: Option<bool>,
    request: Option<bool>,
}
impl IndicesClearCacheBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesClearCacheBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesClearCacheBuilder {
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
pub struct IndicesCloseBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl IndicesCloseBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesCloseBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesCloseBuilder {
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
pub struct IndicesCreateBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    include_type_name: Option<bool>,
    master_timeout: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl IndicesCreateBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesCreateBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesCreateBuilder {
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
pub struct IndicesDeleteBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<String>,
    timeout: Option<String>,
}
impl IndicesDeleteBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesDeleteBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesDeleteBuilder {
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
pub struct IndicesDeleteAliasBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
    timeout: Option<String>,
}
impl IndicesDeleteAliasBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesDeleteAliasBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesDeleteAliasBuilder {
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
pub struct IndicesDeleteTemplateBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
    timeout: Option<String>,
}
impl IndicesDeleteTemplateBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesDeleteTemplateBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesDeleteTemplateBuilder {
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
pub struct IndicesExistsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    flat_settings: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    local: Option<bool>,
}
impl IndicesExistsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesExistsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesExistsBuilder {
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
pub struct IndicesExistsAliasBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
}
impl IndicesExistsAliasBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesExistsAliasBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesExistsAliasBuilder {
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
pub struct IndicesExistsTemplateBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    flat_settings: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
}
impl IndicesExistsTemplateBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesExistsTemplateBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesExistsTemplateBuilder {
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
pub struct IndicesExistsTypeBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
}
impl IndicesExistsTypeBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesExistsTypeBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesExistsTypeBuilder {
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
pub struct IndicesFlushBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    force: Option<bool>,
    ignore_unavailable: Option<bool>,
    wait_if_ongoing: Option<bool>,
}
impl IndicesFlushBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesFlushBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesFlushBuilder {
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
pub struct IndicesFlushSyncedBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
}
impl IndicesFlushSyncedBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesFlushSyncedBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesFlushSyncedBuilder {
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
pub struct IndicesForcemergeBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    flush: Option<bool>,
    ignore_unavailable: Option<bool>,
    max_num_segments: Option<i64>,
    only_expunge_deletes: Option<bool>,
}
impl IndicesForcemergeBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesForcemergeBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesForcemergeBuilder {
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
pub struct IndicesFreezeBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl IndicesFreezeBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesFreezeBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesFreezeBuilder {
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
pub struct IndicesGetBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    flat_settings: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
}
impl IndicesGetBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesGetBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesGetBuilder {
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
pub struct IndicesGetAliasBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
}
impl IndicesGetAliasBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesGetAliasBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesGetAliasBuilder {
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
pub struct IndicesGetFieldMappingBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
}
impl IndicesGetFieldMappingBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesGetFieldMappingBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesGetFieldMappingBuilder {
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
pub struct IndicesGetMappingBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
}
impl IndicesGetMappingBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesGetMappingBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesGetMappingBuilder {
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
pub struct IndicesGetSettingsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    flat_settings: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
}
impl IndicesGetSettingsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesGetSettingsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesGetSettingsBuilder {
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
pub struct IndicesGetTemplateBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    flat_settings: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
}
impl IndicesGetTemplateBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesGetTemplateBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesGetTemplateBuilder {
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
pub struct IndicesGetUpgradeBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
}
impl IndicesGetUpgradeBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesGetUpgradeBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesGetUpgradeBuilder {
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
pub struct IndicesOpenBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl IndicesOpenBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesOpenBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesOpenBuilder {
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
pub struct IndicesPutAliasBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
    timeout: Option<String>,
}
impl IndicesPutAliasBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesPutAliasBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesPutAliasBuilder {
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
pub struct IndicesPutMappingBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<String>,
    timeout: Option<String>,
}
impl IndicesPutMappingBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesPutMappingBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesPutMappingBuilder {
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
pub struct IndicesPutSettingsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    flat_settings: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<String>,
    preserve_existing: Option<bool>,
    timeout: Option<String>,
}
impl IndicesPutSettingsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesPutSettingsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesPutSettingsBuilder {
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
pub struct IndicesPutTemplateBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    create: Option<bool>,
    flat_settings: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<String>,
    order: Option<i64>,
    timeout: Option<String>,
}
impl IndicesPutTemplateBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesPutTemplateBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesPutTemplateBuilder {
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
pub struct IndicesRecoveryBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    active_only: Option<bool>,
    detailed: Option<bool>,
}
impl IndicesRecoveryBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesRecoveryBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesRecoveryBuilder {
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
pub struct IndicesRefreshBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
}
impl IndicesRefreshBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesRefreshBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesRefreshBuilder {
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
pub struct IndicesReloadSearchAnalyzersBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
}
impl IndicesReloadSearchAnalyzersBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesReloadSearchAnalyzersBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesReloadSearchAnalyzersBuilder {
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
pub struct IndicesRolloverBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    dry_run: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl IndicesRolloverBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesRolloverBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesRolloverBuilder {
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
pub struct IndicesSegmentsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    verbose: Option<bool>,
}
impl IndicesSegmentsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesSegmentsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesSegmentsBuilder {
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
pub struct IndicesShardStoresBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    status: Option<Vec<String>>,
}
impl IndicesShardStoresBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesShardStoresBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesShardStoresBuilder {
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
pub struct IndicesShrinkBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    copy_settings: Option<bool>,
    master_timeout: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl IndicesShrinkBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesShrinkBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesShrinkBuilder {
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
pub struct IndicesSplitBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    copy_settings: Option<bool>,
    master_timeout: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl IndicesSplitBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesSplitBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesSplitBuilder {
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
pub struct IndicesStatsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    completion_fields: Option<Vec<String>>,
    expand_wildcards: Option<i32>,
    fielddata_fields: Option<Vec<String>>,
    fields: Option<Vec<String>>,
    forbid_closed_indices: Option<bool>,
    groups: Option<Vec<String>>,
    include_segment_file_sizes: Option<bool>,
    include_unloaded_segments: Option<bool>,
    level: Option<i32>,
    types: Option<Vec<String>>,
}
impl IndicesStatsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesStatsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesStatsBuilder {
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
pub struct IndicesUnfreezeBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl IndicesUnfreezeBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesUnfreezeBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesUnfreezeBuilder {
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
pub struct IndicesUpdateAliasesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
    timeout: Option<String>,
}
impl IndicesUpdateAliasesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesUpdateAliasesBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesUpdateAliasesBuilder {
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
pub struct IndicesUpgradeBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_indices: Option<bool>,
    expand_wildcards: Option<i32>,
    ignore_unavailable: Option<bool>,
    only_ancient_segments: Option<bool>,
    wait_for_completion: Option<bool>,
}
impl IndicesUpgradeBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesUpgradeBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesUpgradeBuilder {
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
pub struct IndicesValidateQueryBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    all_shards: Option<bool>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<String>,
    default_operator: Option<i32>,
    df: Option<String>,
    expand_wildcards: Option<i32>,
    explain: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    q: Option<String>,
    rewrite: Option<bool>,
}
impl IndicesValidateQueryBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesValidateQueryBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IndicesValidateQueryBuilder {
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
#[doc = "Indices APIs"]
pub struct IndicesClient {
    client: ElasticsearchClient,
}
impl IndicesClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        IndicesClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-analyze.html"]
    pub fn analyze(&self) -> IndicesAnalyzeBuilder {
        IndicesAnalyzeBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-clearcache.html"]
    pub fn clear_cache(&self) -> IndicesClearCacheBuilder {
        IndicesClearCacheBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html"]
    pub fn close(&self) -> IndicesCloseBuilder {
        IndicesCloseBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-create-index.html"]
    pub fn create(&self) -> IndicesCreateBuilder {
        IndicesCreateBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-delete-index.html"]
    pub fn delete(&self) -> IndicesDeleteBuilder {
        IndicesDeleteBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn delete_alias(&self) -> IndicesDeleteAliasBuilder {
        IndicesDeleteAliasBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn delete_template(&self) -> IndicesDeleteTemplateBuilder {
        IndicesDeleteTemplateBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-exists.html"]
    pub fn exists(&self) -> IndicesExistsBuilder {
        IndicesExistsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn exists_alias(&self) -> IndicesExistsAliasBuilder {
        IndicesExistsAliasBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn exists_template(&self) -> IndicesExistsTemplateBuilder {
        IndicesExistsTemplateBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-types-exists.html"]
    pub fn exists_type(&self) -> IndicesExistsTypeBuilder {
        IndicesExistsTypeBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-flush.html"]
    pub fn flush(&self) -> IndicesFlushBuilder {
        IndicesFlushBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-flush.html#synced-flush-api"]
    pub fn flush_synced(&self) -> IndicesFlushSyncedBuilder {
        IndicesFlushSyncedBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-forcemerge.html"]
    pub fn forcemerge(&self) -> IndicesForcemergeBuilder {
        IndicesForcemergeBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html"]
    pub fn freeze(&self) -> IndicesFreezeBuilder {
        IndicesFreezeBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-index.html"]
    pub fn get(&self) -> IndicesGetBuilder {
        IndicesGetBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn get_alias(&self) -> IndicesGetAliasBuilder {
        IndicesGetAliasBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-field-mapping.html"]
    pub fn get_field_mapping(&self) -> IndicesGetFieldMappingBuilder {
        IndicesGetFieldMappingBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-mapping.html"]
    pub fn get_mapping(&self) -> IndicesGetMappingBuilder {
        IndicesGetMappingBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-settings.html"]
    pub fn get_settings(&self) -> IndicesGetSettingsBuilder {
        IndicesGetSettingsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn get_template(&self) -> IndicesGetTemplateBuilder {
        IndicesGetTemplateBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html"]
    pub fn get_upgrade(&self) -> IndicesGetUpgradeBuilder {
        IndicesGetUpgradeBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html"]
    pub fn open(&self) -> IndicesOpenBuilder {
        IndicesOpenBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn put_alias(&self) -> IndicesPutAliasBuilder {
        IndicesPutAliasBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-put-mapping.html"]
    pub fn put_mapping(&self) -> IndicesPutMappingBuilder {
        IndicesPutMappingBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-update-settings.html"]
    pub fn put_settings(&self) -> IndicesPutSettingsBuilder {
        IndicesPutSettingsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn put_template(&self) -> IndicesPutTemplateBuilder {
        IndicesPutTemplateBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-recovery.html"]
    pub fn recovery(&self) -> IndicesRecoveryBuilder {
        IndicesRecoveryBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-refresh.html"]
    pub fn refresh(&self) -> IndicesRefreshBuilder {
        IndicesRefreshBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-reload-analyzers.html"]
    pub fn reload_search_analyzers(&self) -> IndicesReloadSearchAnalyzersBuilder {
        IndicesReloadSearchAnalyzersBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-rollover-index.html"]
    pub fn rollover(&self) -> IndicesRolloverBuilder {
        IndicesRolloverBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-segments.html"]
    pub fn segments(&self) -> IndicesSegmentsBuilder {
        IndicesSegmentsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shards-stores.html"]
    pub fn shard_stores(&self) -> IndicesShardStoresBuilder {
        IndicesShardStoresBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shrink-index.html"]
    pub fn shrink(&self) -> IndicesShrinkBuilder {
        IndicesShrinkBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-split-index.html"]
    pub fn split(&self) -> IndicesSplitBuilder {
        IndicesSplitBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-stats.html"]
    pub fn stats(&self) -> IndicesStatsBuilder {
        IndicesStatsBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html"]
    pub fn unfreeze(&self) -> IndicesUnfreezeBuilder {
        IndicesUnfreezeBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn update_aliases(&self) -> IndicesUpdateAliasesBuilder {
        IndicesUpdateAliasesBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html"]
    pub fn upgrade(&self) -> IndicesUpgradeBuilder {
        IndicesUpgradeBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-validate.html"]
    pub fn validate_query(&self) -> IndicesValidateQueryBuilder {
        IndicesValidateQueryBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Indices APIs"]
    pub fn indices(&self) -> IndicesClient {
        IndicesClient::new(self.clone())
    }
}
