

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[Default]
pub struct IndicesAnalyzeRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    index: &'a str,
}
impl<'a> IndicesAnalyzeRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesAnalyzeRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesAnalyzeRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesClearCacheRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
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
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesClearCacheRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesClearCacheRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesCloseRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a str,
    timeout: &'a str,
    wait_for_active_shards: &'a str,
}
impl<'a> IndicesCloseRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesCloseRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesCloseRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesCreateRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    include_type_name: Option<&'a bool>,
    master_timeout: &'a str,
    timeout: &'a str,
    wait_for_active_shards: &'a str,
}
impl<'a> IndicesCreateRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesCreateRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesCreateRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesDeleteRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a str,
    timeout: &'a str,
}
impl<'a> IndicesDeleteRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesDeleteRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesDeleteRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesDeleteAliasRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    master_timeout: &'a str,
    timeout: &'a str,
}
impl<'a> IndicesDeleteAliasRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesDeleteAliasRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesDeleteAliasRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesDeleteTemplateRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    master_timeout: &'a str,
    timeout: &'a str,
}
impl<'a> IndicesDeleteTemplateRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesDeleteTemplateRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesDeleteTemplateRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesExistsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    local: Option<&'a bool>,
}
impl<'a> IndicesExistsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesExistsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesExistsRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesExistsAliasRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
}
impl<'a> IndicesExistsAliasRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesExistsAliasRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesExistsAliasRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesExistsTemplateRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    flat_settings: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
}
impl<'a> IndicesExistsTemplateRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesExistsTemplateRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesExistsTemplateRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesExistsTypeRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
}
impl<'a> IndicesExistsTypeRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesExistsTypeRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesExistsTypeRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesFlushRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    force: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    wait_if_ongoing: Option<&'a bool>,
}
impl<'a> IndicesFlushRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesFlushRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesFlushRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesFlushSyncedRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
impl<'a> IndicesFlushSyncedRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesFlushSyncedRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesFlushSyncedRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesForcemergeRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flush: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    max_num_segments: Option<&'a i64>,
    only_expunge_deletes: Option<&'a bool>,
}
impl<'a> IndicesForcemergeRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesForcemergeRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesForcemergeRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesFreezeRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a str,
    timeout: &'a str,
    wait_for_active_shards: &'a str,
}
impl<'a> IndicesFreezeRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesFreezeRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesFreezeRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesGetRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
}
impl<'a> IndicesGetRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesGetRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesGetRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesGetAliasRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    local: Option<&'a bool>,
}
impl<'a> IndicesGetAliasRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesGetAliasRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesGetAliasRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesGetFieldMappingRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    local: Option<&'a bool>,
}
impl<'a> IndicesGetFieldMappingRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesGetFieldMappingRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesGetFieldMappingRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesGetMappingRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
}
impl<'a> IndicesGetMappingRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesGetMappingRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesGetMappingRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesGetSettingsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    include_defaults: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
}
impl<'a> IndicesGetSettingsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesGetSettingsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesGetSettingsRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesGetTemplateRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    flat_settings: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
}
impl<'a> IndicesGetTemplateRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesGetTemplateRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesGetTemplateRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesGetUpgradeRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
impl<'a> IndicesGetUpgradeRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesGetUpgradeRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesGetUpgradeRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesOpenRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a str,
    timeout: &'a str,
    wait_for_active_shards: &'a str,
}
impl<'a> IndicesOpenRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesOpenRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesOpenRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesPutAliasRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    master_timeout: &'a str,
    timeout: &'a str,
}
impl<'a> IndicesPutAliasRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesPutAliasRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesPutAliasRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesPutMappingRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    master_timeout: &'a str,
    timeout: &'a str,
}
impl<'a> IndicesPutMappingRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesPutMappingRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesPutMappingRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesPutSettingsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    flat_settings: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a str,
    preserve_existing: Option<&'a bool>,
    timeout: &'a str,
}
impl<'a> IndicesPutSettingsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesPutSettingsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesPutSettingsRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesPutTemplateRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    create: Option<&'a bool>,
    flat_settings: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    master_timeout: &'a str,
    order: Option<&'a i64>,
    timeout: &'a str,
}
impl<'a> IndicesPutTemplateRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesPutTemplateRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesPutTemplateRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesRecoveryRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    active_only: Option<&'a bool>,
    detailed: Option<&'a bool>,
}
impl<'a> IndicesRecoveryRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesRecoveryRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesRecoveryRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesRefreshRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
impl<'a> IndicesRefreshRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesRefreshRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesRefreshRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesReloadSearchAnalyzersRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
}
impl<'a> IndicesReloadSearchAnalyzersRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesReloadSearchAnalyzersRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesReloadSearchAnalyzersRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesRolloverRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    dry_run: Option<&'a bool>,
    include_type_name: Option<&'a bool>,
    master_timeout: &'a str,
    timeout: &'a str,
    wait_for_active_shards: &'a str,
}
impl<'a> IndicesRolloverRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesRolloverRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesRolloverRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesSegmentsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    verbose: Option<&'a bool>,
}
impl<'a> IndicesSegmentsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesSegmentsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesSegmentsRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesShardStoresRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    status: &'a Vec<String>,
}
impl<'a> IndicesShardStoresRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesShardStoresRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesShardStoresRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesShrinkRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    copy_settings: Option<&'a bool>,
    master_timeout: &'a str,
    timeout: &'a str,
    wait_for_active_shards: &'a str,
}
impl<'a> IndicesShrinkRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesShrinkRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesShrinkRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesSplitRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    copy_settings: Option<&'a bool>,
    master_timeout: &'a str,
    timeout: &'a str,
    wait_for_active_shards: &'a str,
}
impl<'a> IndicesSplitRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesSplitRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesSplitRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesStatsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
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
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesStatsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesStatsRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesUnfreezeRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a str,
    timeout: &'a str,
    wait_for_active_shards: &'a str,
}
impl<'a> IndicesUnfreezeRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesUnfreezeRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesUnfreezeRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesUpdateAliasesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    master_timeout: &'a str,
    timeout: &'a str,
}
impl<'a> IndicesUpdateAliasesRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesUpdateAliasesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesUpdateAliasesRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesUpgradeRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    allow_no_indices: Option<&'a bool>,
    expand_wildcards: Option<&'a i32>,
    ignore_unavailable: Option<&'a bool>,
    only_ancient_segments: Option<&'a bool>,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> IndicesUpgradeRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesUpgradeRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesUpgradeRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct IndicesValidateQueryRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    all_shards: Option<&'a bool>,
    allow_no_indices: Option<&'a bool>,
    analyze_wildcard: Option<&'a bool>,
    analyzer: &'a str,
    default_operator: Option<&'a i32>,
    df: &'a str,
    expand_wildcards: Option<&'a i32>,
    explain: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    lenient: Option<&'a bool>,
    q: &'a str,
    rewrite: Option<&'a bool>,
}
impl<'a> IndicesValidateQueryRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesValidateQueryRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IndicesValidateQueryRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
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
    pub fn analyze(&self) -> IndicesAnalyzeRequestBuilder {
        IndicesAnalyzeRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-clearcache.html"]
    pub fn clear_cache(&self) -> IndicesClearCacheRequestBuilder {
        IndicesClearCacheRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html"]
    pub fn close(&self) -> IndicesCloseRequestBuilder {
        IndicesCloseRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-create-index.html"]
    pub fn create(&self) -> IndicesCreateRequestBuilder {
        IndicesCreateRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-delete-index.html"]
    pub fn delete(&self) -> IndicesDeleteRequestBuilder {
        IndicesDeleteRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn delete_alias(&self) -> IndicesDeleteAliasRequestBuilder {
        IndicesDeleteAliasRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn delete_template(&self) -> IndicesDeleteTemplateRequestBuilder {
        IndicesDeleteTemplateRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-exists.html"]
    pub fn exists(&self) -> IndicesExistsRequestBuilder {
        IndicesExistsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn exists_alias(&self) -> IndicesExistsAliasRequestBuilder {
        IndicesExistsAliasRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn exists_template(&self) -> IndicesExistsTemplateRequestBuilder {
        IndicesExistsTemplateRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-types-exists.html"]
    pub fn exists_type(&self) -> IndicesExistsTypeRequestBuilder {
        IndicesExistsTypeRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-flush.html"]
    pub fn flush(&self) -> IndicesFlushRequestBuilder {
        IndicesFlushRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-flush.html#synced-flush-api"]
    pub fn flush_synced(&self) -> IndicesFlushSyncedRequestBuilder {
        IndicesFlushSyncedRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-forcemerge.html"]
    pub fn forcemerge(&self) -> IndicesForcemergeRequestBuilder {
        IndicesForcemergeRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html"]
    pub fn freeze(&self) -> IndicesFreezeRequestBuilder {
        IndicesFreezeRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-index.html"]
    pub fn get(&self) -> IndicesGetRequestBuilder {
        IndicesGetRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn get_alias(&self) -> IndicesGetAliasRequestBuilder {
        IndicesGetAliasRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-field-mapping.html"]
    pub fn get_field_mapping(&self) -> IndicesGetFieldMappingRequestBuilder {
        IndicesGetFieldMappingRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-mapping.html"]
    pub fn get_mapping(&self) -> IndicesGetMappingRequestBuilder {
        IndicesGetMappingRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-settings.html"]
    pub fn get_settings(&self) -> IndicesGetSettingsRequestBuilder {
        IndicesGetSettingsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn get_template(&self) -> IndicesGetTemplateRequestBuilder {
        IndicesGetTemplateRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html"]
    pub fn get_upgrade(&self) -> IndicesGetUpgradeRequestBuilder {
        IndicesGetUpgradeRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html"]
    pub fn open(&self) -> IndicesOpenRequestBuilder {
        IndicesOpenRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn put_alias(&self) -> IndicesPutAliasRequestBuilder {
        IndicesPutAliasRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-put-mapping.html"]
    pub fn put_mapping(&self) -> IndicesPutMappingRequestBuilder {
        IndicesPutMappingRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-update-settings.html"]
    pub fn put_settings(&self) -> IndicesPutSettingsRequestBuilder {
        IndicesPutSettingsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn put_template(&self) -> IndicesPutTemplateRequestBuilder {
        IndicesPutTemplateRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-recovery.html"]
    pub fn recovery(&self) -> IndicesRecoveryRequestBuilder {
        IndicesRecoveryRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-refresh.html"]
    pub fn refresh(&self) -> IndicesRefreshRequestBuilder {
        IndicesRefreshRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-reload-analyzers.html"]
    pub fn reload_search_analyzers(&self) -> IndicesReloadSearchAnalyzersRequestBuilder {
        IndicesReloadSearchAnalyzersRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-rollover-index.html"]
    pub fn rollover(&self) -> IndicesRolloverRequestBuilder {
        IndicesRolloverRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-segments.html"]
    pub fn segments(&self) -> IndicesSegmentsRequestBuilder {
        IndicesSegmentsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shards-stores.html"]
    pub fn shard_stores(&self) -> IndicesShardStoresRequestBuilder {
        IndicesShardStoresRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shrink-index.html"]
    pub fn shrink(&self) -> IndicesShrinkRequestBuilder {
        IndicesShrinkRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-split-index.html"]
    pub fn split(&self) -> IndicesSplitRequestBuilder {
        IndicesSplitRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-stats.html"]
    pub fn stats(&self) -> IndicesStatsRequestBuilder {
        IndicesStatsRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html"]
    pub fn unfreeze(&self) -> IndicesUnfreezeRequestBuilder {
        IndicesUnfreezeRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn update_aliases(&self) -> IndicesUpdateAliasesRequestBuilder {
        IndicesUpdateAliasesRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html"]
    pub fn upgrade(&self) -> IndicesUpgradeRequestBuilder {
        IndicesUpgradeRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-validate.html"]
    pub fn validate_query(&self) -> IndicesValidateQueryRequestBuilder {
        IndicesValidateQueryRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Indices APIs"]
    pub fn indices(&self) -> IndicesNamespaceClient {
        IndicesNamespaceClient::new(self)
    }
}
