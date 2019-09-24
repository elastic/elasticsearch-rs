

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct SecurityAuthenticateBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityAuthenticateBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityAuthenticateBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityAuthenticateBuilder {
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
pub struct SecurityChangePasswordBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    refresh: Option<i32>,
}
impl SecurityChangePasswordBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityChangePasswordBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityChangePasswordBuilder {
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
pub struct SecurityClearCachedRealmsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    usernames: Option<Vec<String>>,
}
impl SecurityClearCachedRealmsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityClearCachedRealmsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityClearCachedRealmsBuilder {
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
pub struct SecurityClearCachedRolesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityClearCachedRolesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityClearCachedRolesBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityClearCachedRolesBuilder {
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
pub struct SecurityCreateApiKeyBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    refresh: Option<i32>,
}
impl SecurityCreateApiKeyBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityCreateApiKeyBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityCreateApiKeyBuilder {
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
pub struct SecurityDeletePrivilegesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    refresh: Option<i32>,
}
impl SecurityDeletePrivilegesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityDeletePrivilegesBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityDeletePrivilegesBuilder {
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
pub struct SecurityDeleteRoleBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    refresh: Option<i32>,
}
impl SecurityDeleteRoleBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityDeleteRoleBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityDeleteRoleBuilder {
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
pub struct SecurityDeleteRoleMappingBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    refresh: Option<i32>,
}
impl SecurityDeleteRoleMappingBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityDeleteRoleMappingBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityDeleteRoleMappingBuilder {
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
pub struct SecurityDeleteUserBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    refresh: Option<i32>,
}
impl SecurityDeleteUserBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityDeleteUserBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityDeleteUserBuilder {
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
pub struct SecurityDisableUserBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    refresh: Option<i32>,
}
impl SecurityDisableUserBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityDisableUserBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityDisableUserBuilder {
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
pub struct SecurityEnableUserBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    refresh: Option<i32>,
}
impl SecurityEnableUserBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityEnableUserBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityEnableUserBuilder {
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
pub struct SecurityGetApiKeyBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    id: Option<String>,
    name: Option<String>,
    realm_name: Option<String>,
    username: Option<String>,
}
impl SecurityGetApiKeyBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityGetApiKeyBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityGetApiKeyBuilder {
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
pub struct SecurityGetBuiltinPrivilegesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityGetBuiltinPrivilegesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityGetBuiltinPrivilegesBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityGetBuiltinPrivilegesBuilder {
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
pub struct SecurityGetPrivilegesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityGetPrivilegesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityGetPrivilegesBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityGetPrivilegesBuilder {
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
pub struct SecurityGetRoleBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityGetRoleBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityGetRoleBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityGetRoleBuilder {
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
pub struct SecurityGetRoleMappingBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityGetRoleMappingBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityGetRoleMappingBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityGetRoleMappingBuilder {
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
pub struct SecurityGetTokenBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityGetTokenBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityGetTokenBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityGetTokenBuilder {
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
pub struct SecurityGetUserBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityGetUserBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityGetUserBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityGetUserBuilder {
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
pub struct SecurityGetUserPrivilegesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityGetUserPrivilegesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityGetUserPrivilegesBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityGetUserPrivilegesBuilder {
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
pub struct SecurityHasPrivilegesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityHasPrivilegesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityHasPrivilegesBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityHasPrivilegesBuilder {
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
pub struct SecurityInvalidateApiKeyBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityInvalidateApiKeyBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityInvalidateApiKeyBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityInvalidateApiKeyBuilder {
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
pub struct SecurityInvalidateTokenBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityInvalidateTokenBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityInvalidateTokenBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityInvalidateTokenBuilder {
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
pub struct SecurityPutPrivilegesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    refresh: Option<i32>,
}
impl SecurityPutPrivilegesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityPutPrivilegesBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityPutPrivilegesBuilder {
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
pub struct SecurityPutRoleBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    refresh: Option<i32>,
}
impl SecurityPutRoleBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityPutRoleBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityPutRoleBuilder {
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
pub struct SecurityPutRoleMappingBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    refresh: Option<i32>,
}
impl SecurityPutRoleMappingBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityPutRoleMappingBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityPutRoleMappingBuilder {
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
pub struct SecurityPutUserBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    refresh: Option<i32>,
}
impl SecurityPutUserBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityPutUserBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SecurityPutUserBuilder {
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
pub struct SecurityClient {
    client: ElasticsearchClient,
}
impl SecurityClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        SecurityClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-authenticate.html"]
    pub fn authenticate(&self) -> SecurityAuthenticateBuilder {
        SecurityAuthenticateBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-change-password.html"]
    pub fn change_password(&self) -> SecurityChangePasswordBuilder {
        SecurityChangePasswordBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-cache.html"]
    pub fn clear_cached_realms(&self) -> SecurityClearCachedRealmsBuilder {
        SecurityClearCachedRealmsBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-role-cache.html"]
    pub fn clear_cached_roles(&self) -> SecurityClearCachedRolesBuilder {
        SecurityClearCachedRolesBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-create-api-key.html"]
    pub fn create_api_key(&self) -> SecurityCreateApiKeyBuilder {
        SecurityCreateApiKeyBuilder::new(self.client.clone())
    }
    #[doc = "TODO"]
    pub fn delete_privileges(&self) -> SecurityDeletePrivilegesBuilder {
        SecurityDeletePrivilegesBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-role.html"]
    pub fn delete_role(&self) -> SecurityDeleteRoleBuilder {
        SecurityDeleteRoleBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-role-mapping.html"]
    pub fn delete_role_mapping(&self) -> SecurityDeleteRoleMappingBuilder {
        SecurityDeleteRoleMappingBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-user.html"]
    pub fn delete_user(&self) -> SecurityDeleteUserBuilder {
        SecurityDeleteUserBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-disable-user.html"]
    pub fn disable_user(&self) -> SecurityDisableUserBuilder {
        SecurityDisableUserBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-enable-user.html"]
    pub fn enable_user(&self) -> SecurityEnableUserBuilder {
        SecurityEnableUserBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-api-key.html"]
    pub fn get_api_key(&self) -> SecurityGetApiKeyBuilder {
        SecurityGetApiKeyBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-builtin-privileges.html"]
    pub fn get_builtin_privileges(&self) -> SecurityGetBuiltinPrivilegesBuilder {
        SecurityGetBuiltinPrivilegesBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-privileges.html"]
    pub fn get_privileges(&self) -> SecurityGetPrivilegesBuilder {
        SecurityGetPrivilegesBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-role.html"]
    pub fn get_role(&self) -> SecurityGetRoleBuilder {
        SecurityGetRoleBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-role-mapping.html"]
    pub fn get_role_mapping(&self) -> SecurityGetRoleMappingBuilder {
        SecurityGetRoleMappingBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-token.html"]
    pub fn get_token(&self) -> SecurityGetTokenBuilder {
        SecurityGetTokenBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-user.html"]
    pub fn get_user(&self) -> SecurityGetUserBuilder {
        SecurityGetUserBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-privileges.html"]
    pub fn get_user_privileges(&self) -> SecurityGetUserPrivilegesBuilder {
        SecurityGetUserPrivilegesBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-has-privileges.html"]
    pub fn has_privileges(&self) -> SecurityHasPrivilegesBuilder {
        SecurityHasPrivilegesBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-api-key.html"]
    pub fn invalidate_api_key(&self) -> SecurityInvalidateApiKeyBuilder {
        SecurityInvalidateApiKeyBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-token.html"]
    pub fn invalidate_token(&self) -> SecurityInvalidateTokenBuilder {
        SecurityInvalidateTokenBuilder::new(self.client.clone())
    }
    #[doc = "TODO"]
    pub fn put_privileges(&self) -> SecurityPutPrivilegesBuilder {
        SecurityPutPrivilegesBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role.html"]
    pub fn put_role(&self) -> SecurityPutRoleBuilder {
        SecurityPutRoleBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role-mapping.html"]
    pub fn put_role_mapping(&self) -> SecurityPutRoleMappingBuilder {
        SecurityPutRoleMappingBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-user.html"]
    pub fn put_user(&self) -> SecurityPutUserBuilder {
        SecurityPutUserBuilder::new(self.client.clone())
    }
}
impl ElasticsearchClient {
    #[doc = "Security APIs"]
    pub fn security(&self) -> SecurityClient {
        SecurityClient::new(self.clone())
    }
}
