

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct SecurityAuthenticateRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SecurityAuthenticateRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityAuthenticateRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityAuthenticateRequestBuilder<'a> {
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
pub struct SecurityChangePasswordRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    refresh: Option<&'a i32>,
}
impl<'a> SecurityChangePasswordRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityChangePasswordRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityChangePasswordRequestBuilder<'a> {
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
pub struct SecurityClearCachedRealmsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    usernames: Option<&'a Vec<String>>,
}
impl<'a> SecurityClearCachedRealmsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityClearCachedRealmsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityClearCachedRealmsRequestBuilder<'a> {
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
pub struct SecurityClearCachedRolesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SecurityClearCachedRolesRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityClearCachedRolesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityClearCachedRolesRequestBuilder<'a> {
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
pub struct SecurityCreateApiKeyRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    refresh: Option<&'a i32>,
}
impl<'a> SecurityCreateApiKeyRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityCreateApiKeyRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityCreateApiKeyRequestBuilder<'a> {
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
pub struct SecurityDeletePrivilegesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    refresh: Option<&'a i32>,
}
impl<'a> SecurityDeletePrivilegesRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityDeletePrivilegesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityDeletePrivilegesRequestBuilder<'a> {
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
pub struct SecurityDeleteRoleRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    refresh: Option<&'a i32>,
}
impl<'a> SecurityDeleteRoleRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityDeleteRoleRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityDeleteRoleRequestBuilder<'a> {
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
pub struct SecurityDeleteRoleMappingRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    refresh: Option<&'a i32>,
}
impl<'a> SecurityDeleteRoleMappingRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityDeleteRoleMappingRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityDeleteRoleMappingRequestBuilder<'a> {
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
pub struct SecurityDeleteUserRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    refresh: Option<&'a i32>,
}
impl<'a> SecurityDeleteUserRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityDeleteUserRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityDeleteUserRequestBuilder<'a> {
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
pub struct SecurityDisableUserRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    refresh: Option<&'a i32>,
}
impl<'a> SecurityDisableUserRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityDisableUserRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityDisableUserRequestBuilder<'a> {
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
pub struct SecurityEnableUserRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    refresh: Option<&'a i32>,
}
impl<'a> SecurityEnableUserRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityEnableUserRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityEnableUserRequestBuilder<'a> {
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
pub struct SecurityGetApiKeyRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    id: &'a str,
    name: &'a str,
    realm_name: &'a str,
    username: &'a str,
}
impl<'a> SecurityGetApiKeyRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityGetApiKeyRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityGetApiKeyRequestBuilder<'a> {
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
pub struct SecurityGetBuiltinPrivilegesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SecurityGetBuiltinPrivilegesRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityGetBuiltinPrivilegesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityGetBuiltinPrivilegesRequestBuilder<'a> {
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
pub struct SecurityGetPrivilegesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SecurityGetPrivilegesRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityGetPrivilegesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityGetPrivilegesRequestBuilder<'a> {
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
pub struct SecurityGetRoleRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SecurityGetRoleRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityGetRoleRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityGetRoleRequestBuilder<'a> {
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
pub struct SecurityGetRoleMappingRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SecurityGetRoleMappingRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityGetRoleMappingRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityGetRoleMappingRequestBuilder<'a> {
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
pub struct SecurityGetTokenRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SecurityGetTokenRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityGetTokenRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityGetTokenRequestBuilder<'a> {
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
pub struct SecurityGetUserRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SecurityGetUserRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityGetUserRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityGetUserRequestBuilder<'a> {
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
pub struct SecurityGetUserPrivilegesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SecurityGetUserPrivilegesRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityGetUserPrivilegesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityGetUserPrivilegesRequestBuilder<'a> {
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
pub struct SecurityHasPrivilegesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SecurityHasPrivilegesRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityHasPrivilegesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityHasPrivilegesRequestBuilder<'a> {
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
pub struct SecurityInvalidateApiKeyRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SecurityInvalidateApiKeyRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityInvalidateApiKeyRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityInvalidateApiKeyRequestBuilder<'a> {
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
pub struct SecurityInvalidateTokenRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SecurityInvalidateTokenRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityInvalidateTokenRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityInvalidateTokenRequestBuilder<'a> {
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
pub struct SecurityPutPrivilegesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    refresh: Option<&'a i32>,
}
impl<'a> SecurityPutPrivilegesRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityPutPrivilegesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityPutPrivilegesRequestBuilder<'a> {
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
pub struct SecurityPutRoleRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    refresh: Option<&'a i32>,
}
impl<'a> SecurityPutRoleRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityPutRoleRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityPutRoleRequestBuilder<'a> {
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
pub struct SecurityPutRoleMappingRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    refresh: Option<&'a i32>,
}
impl<'a> SecurityPutRoleMappingRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityPutRoleMappingRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityPutRoleMappingRequestBuilder<'a> {
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
pub struct SecurityPutUserRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    refresh: Option<&'a i32>,
}
impl<'a> SecurityPutUserRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityPutUserRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SecurityPutUserRequestBuilder<'a> {
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
#[doc = "Security APIs"]
pub struct SecurityNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> SecurityNamespaceClient<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SecurityNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-authenticate.html"]
    pub fn authenticate(&self) -> SecurityAuthenticateRequestBuilder {
        SecurityAuthenticateRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-change-password.html"]
    pub fn change_password(&self) -> SecurityChangePasswordRequestBuilder {
        SecurityChangePasswordRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-cache.html"]
    pub fn clear_cached_realms(&self) -> SecurityClearCachedRealmsRequestBuilder {
        SecurityClearCachedRealmsRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-role-cache.html"]
    pub fn clear_cached_roles(&self) -> SecurityClearCachedRolesRequestBuilder {
        SecurityClearCachedRolesRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-create-api-key.html"]
    pub fn create_api_key(&self) -> SecurityCreateApiKeyRequestBuilder {
        SecurityCreateApiKeyRequestBuilder::default()
    }
    #[doc = "TODO"]
    pub fn delete_privileges(&self) -> SecurityDeletePrivilegesRequestBuilder {
        SecurityDeletePrivilegesRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-role.html"]
    pub fn delete_role(&self) -> SecurityDeleteRoleRequestBuilder {
        SecurityDeleteRoleRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-role-mapping.html"]
    pub fn delete_role_mapping(&self) -> SecurityDeleteRoleMappingRequestBuilder {
        SecurityDeleteRoleMappingRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-user.html"]
    pub fn delete_user(&self) -> SecurityDeleteUserRequestBuilder {
        SecurityDeleteUserRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-disable-user.html"]
    pub fn disable_user(&self) -> SecurityDisableUserRequestBuilder {
        SecurityDisableUserRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-enable-user.html"]
    pub fn enable_user(&self) -> SecurityEnableUserRequestBuilder {
        SecurityEnableUserRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-api-key.html"]
    pub fn get_api_key(&self) -> SecurityGetApiKeyRequestBuilder {
        SecurityGetApiKeyRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-builtin-privileges.html"]
    pub fn get_builtin_privileges(&self) -> SecurityGetBuiltinPrivilegesRequestBuilder {
        SecurityGetBuiltinPrivilegesRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-privileges.html"]
    pub fn get_privileges(&self) -> SecurityGetPrivilegesRequestBuilder {
        SecurityGetPrivilegesRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-role.html"]
    pub fn get_role(&self) -> SecurityGetRoleRequestBuilder {
        SecurityGetRoleRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-role-mapping.html"]
    pub fn get_role_mapping(&self) -> SecurityGetRoleMappingRequestBuilder {
        SecurityGetRoleMappingRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-token.html"]
    pub fn get_token(&self) -> SecurityGetTokenRequestBuilder {
        SecurityGetTokenRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-user.html"]
    pub fn get_user(&self) -> SecurityGetUserRequestBuilder {
        SecurityGetUserRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-privileges.html"]
    pub fn get_user_privileges(&self) -> SecurityGetUserPrivilegesRequestBuilder {
        SecurityGetUserPrivilegesRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-has-privileges.html"]
    pub fn has_privileges(&self) -> SecurityHasPrivilegesRequestBuilder {
        SecurityHasPrivilegesRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-api-key.html"]
    pub fn invalidate_api_key(&self) -> SecurityInvalidateApiKeyRequestBuilder {
        SecurityInvalidateApiKeyRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-token.html"]
    pub fn invalidate_token(&self) -> SecurityInvalidateTokenRequestBuilder {
        SecurityInvalidateTokenRequestBuilder::default()
    }
    #[doc = "TODO"]
    pub fn put_privileges(&self) -> SecurityPutPrivilegesRequestBuilder {
        SecurityPutPrivilegesRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role.html"]
    pub fn put_role(&self) -> SecurityPutRoleRequestBuilder {
        SecurityPutRoleRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role-mapping.html"]
    pub fn put_role_mapping(&self) -> SecurityPutRoleMappingRequestBuilder {
        SecurityPutRoleMappingRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-user.html"]
    pub fn put_user(&self) -> SecurityPutUserRequestBuilder {
        SecurityPutUserRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Security APIs"]
    pub fn security(&self) -> SecurityNamespaceClient {
        SecurityNamespaceClient::new(self)
    }
}
