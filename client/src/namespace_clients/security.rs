

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct SecurityAuthenticateRequest<'a> {}
pub struct SecurityAuthenticateRequestBuilder<'a> {}
impl<'a> SecurityAuthenticateRequestBuilder<'a> {
    pub fn build(&self) -> SecurityAuthenticateRequest<'a> {
        SecurityAuthenticateRequest {}
    }
}
pub struct SecurityChangePasswordRequest<'a> {
    refresh: Option<&'a i32>,
}
pub struct SecurityChangePasswordRequestBuilder<'a> {
    refresh: Option<&'a i32>,
}
impl<'a> SecurityChangePasswordRequestBuilder<'a> {
    pub fn build(&self) -> SecurityChangePasswordRequest<'a> {
        SecurityChangePasswordRequest {
            refresh: self.refresh,
        }
    }
}
pub struct SecurityClearCachedRealmsRequest<'a> {
    usernames: &'a Vec<String>,
}
pub struct SecurityClearCachedRealmsRequestBuilder<'a> {
    usernames: &'a Vec<String>,
}
impl<'a> SecurityClearCachedRealmsRequestBuilder<'a> {
    pub fn build(&self) -> SecurityClearCachedRealmsRequest<'a> {
        SecurityClearCachedRealmsRequest {
            usernames: self.usernames,
        }
    }
}
pub struct SecurityClearCachedRolesRequest<'a> {}
pub struct SecurityClearCachedRolesRequestBuilder<'a> {}
impl<'a> SecurityClearCachedRolesRequestBuilder<'a> {
    pub fn build(&self) -> SecurityClearCachedRolesRequest<'a> {
        SecurityClearCachedRolesRequest {}
    }
}
pub struct SecurityCreateApiKeyRequest<'a> {
    refresh: Option<&'a i32>,
}
pub struct SecurityCreateApiKeyRequestBuilder<'a> {
    refresh: Option<&'a i32>,
}
impl<'a> SecurityCreateApiKeyRequestBuilder<'a> {
    pub fn build(&self) -> SecurityCreateApiKeyRequest<'a> {
        SecurityCreateApiKeyRequest {
            refresh: self.refresh,
        }
    }
}
pub struct SecurityDeletePrivilegesRequest<'a> {
    refresh: Option<&'a i32>,
}
pub struct SecurityDeletePrivilegesRequestBuilder<'a> {
    refresh: Option<&'a i32>,
}
impl<'a> SecurityDeletePrivilegesRequestBuilder<'a> {
    pub fn build(&self) -> SecurityDeletePrivilegesRequest<'a> {
        SecurityDeletePrivilegesRequest {
            refresh: self.refresh,
        }
    }
}
pub struct SecurityDeleteRoleRequest<'a> {
    refresh: Option<&'a i32>,
}
pub struct SecurityDeleteRoleRequestBuilder<'a> {
    refresh: Option<&'a i32>,
}
impl<'a> SecurityDeleteRoleRequestBuilder<'a> {
    pub fn build(&self) -> SecurityDeleteRoleRequest<'a> {
        SecurityDeleteRoleRequest {
            refresh: self.refresh,
        }
    }
}
pub struct SecurityDeleteRoleMappingRequest<'a> {
    refresh: Option<&'a i32>,
}
pub struct SecurityDeleteRoleMappingRequestBuilder<'a> {
    refresh: Option<&'a i32>,
}
impl<'a> SecurityDeleteRoleMappingRequestBuilder<'a> {
    pub fn build(&self) -> SecurityDeleteRoleMappingRequest<'a> {
        SecurityDeleteRoleMappingRequest {
            refresh: self.refresh,
        }
    }
}
pub struct SecurityDeleteUserRequest<'a> {
    refresh: Option<&'a i32>,
}
pub struct SecurityDeleteUserRequestBuilder<'a> {
    refresh: Option<&'a i32>,
}
impl<'a> SecurityDeleteUserRequestBuilder<'a> {
    pub fn build(&self) -> SecurityDeleteUserRequest<'a> {
        SecurityDeleteUserRequest {
            refresh: self.refresh,
        }
    }
}
pub struct SecurityDisableUserRequest<'a> {
    refresh: Option<&'a i32>,
}
pub struct SecurityDisableUserRequestBuilder<'a> {
    refresh: Option<&'a i32>,
}
impl<'a> SecurityDisableUserRequestBuilder<'a> {
    pub fn build(&self) -> SecurityDisableUserRequest<'a> {
        SecurityDisableUserRequest {
            refresh: self.refresh,
        }
    }
}
pub struct SecurityEnableUserRequest<'a> {
    refresh: Option<&'a i32>,
}
pub struct SecurityEnableUserRequestBuilder<'a> {
    refresh: Option<&'a i32>,
}
impl<'a> SecurityEnableUserRequestBuilder<'a> {
    pub fn build(&self) -> SecurityEnableUserRequest<'a> {
        SecurityEnableUserRequest {
            refresh: self.refresh,
        }
    }
}
pub struct SecurityGetApiKeyRequest<'a> {
    id: &'a String,
    name: &'a String,
    realm_name: &'a String,
    username: &'a String,
}
pub struct SecurityGetApiKeyRequestBuilder<'a> {
    id: &'a String,
    name: &'a String,
    realm_name: &'a String,
    username: &'a String,
}
impl<'a> SecurityGetApiKeyRequestBuilder<'a> {
    pub fn build(&self) -> SecurityGetApiKeyRequest<'a> {
        SecurityGetApiKeyRequest {
            id: self.id,
            name: self.name,
            realm_name: self.realm_name,
            username: self.username,
        }
    }
}
pub struct SecurityGetBuiltinPrivilegesRequest<'a> {}
pub struct SecurityGetBuiltinPrivilegesRequestBuilder<'a> {}
impl<'a> SecurityGetBuiltinPrivilegesRequestBuilder<'a> {
    pub fn build(&self) -> SecurityGetBuiltinPrivilegesRequest<'a> {
        SecurityGetBuiltinPrivilegesRequest {}
    }
}
pub struct SecurityGetPrivilegesRequest<'a> {}
pub struct SecurityGetPrivilegesRequestBuilder<'a> {}
impl<'a> SecurityGetPrivilegesRequestBuilder<'a> {
    pub fn build(&self) -> SecurityGetPrivilegesRequest<'a> {
        SecurityGetPrivilegesRequest {}
    }
}
pub struct SecurityGetRoleRequest<'a> {}
pub struct SecurityGetRoleRequestBuilder<'a> {}
impl<'a> SecurityGetRoleRequestBuilder<'a> {
    pub fn build(&self) -> SecurityGetRoleRequest<'a> {
        SecurityGetRoleRequest {}
    }
}
pub struct SecurityGetRoleMappingRequest<'a> {}
pub struct SecurityGetRoleMappingRequestBuilder<'a> {}
impl<'a> SecurityGetRoleMappingRequestBuilder<'a> {
    pub fn build(&self) -> SecurityGetRoleMappingRequest<'a> {
        SecurityGetRoleMappingRequest {}
    }
}
pub struct SecurityGetTokenRequest<'a> {}
pub struct SecurityGetTokenRequestBuilder<'a> {}
impl<'a> SecurityGetTokenRequestBuilder<'a> {
    pub fn build(&self) -> SecurityGetTokenRequest<'a> {
        SecurityGetTokenRequest {}
    }
}
pub struct SecurityGetUserRequest<'a> {}
pub struct SecurityGetUserRequestBuilder<'a> {}
impl<'a> SecurityGetUserRequestBuilder<'a> {
    pub fn build(&self) -> SecurityGetUserRequest<'a> {
        SecurityGetUserRequest {}
    }
}
pub struct SecurityGetUserPrivilegesRequest<'a> {}
pub struct SecurityGetUserPrivilegesRequestBuilder<'a> {}
impl<'a> SecurityGetUserPrivilegesRequestBuilder<'a> {
    pub fn build(&self) -> SecurityGetUserPrivilegesRequest<'a> {
        SecurityGetUserPrivilegesRequest {}
    }
}
pub struct SecurityHasPrivilegesRequest<'a> {}
pub struct SecurityHasPrivilegesRequestBuilder<'a> {}
impl<'a> SecurityHasPrivilegesRequestBuilder<'a> {
    pub fn build(&self) -> SecurityHasPrivilegesRequest<'a> {
        SecurityHasPrivilegesRequest {}
    }
}
pub struct SecurityInvalidateApiKeyRequest<'a> {}
pub struct SecurityInvalidateApiKeyRequestBuilder<'a> {}
impl<'a> SecurityInvalidateApiKeyRequestBuilder<'a> {
    pub fn build(&self) -> SecurityInvalidateApiKeyRequest<'a> {
        SecurityInvalidateApiKeyRequest {}
    }
}
pub struct SecurityInvalidateTokenRequest<'a> {}
pub struct SecurityInvalidateTokenRequestBuilder<'a> {}
impl<'a> SecurityInvalidateTokenRequestBuilder<'a> {
    pub fn build(&self) -> SecurityInvalidateTokenRequest<'a> {
        SecurityInvalidateTokenRequest {}
    }
}
pub struct SecurityPutPrivilegesRequest<'a> {
    refresh: Option<&'a i32>,
}
pub struct SecurityPutPrivilegesRequestBuilder<'a> {
    refresh: Option<&'a i32>,
}
impl<'a> SecurityPutPrivilegesRequestBuilder<'a> {
    pub fn build(&self) -> SecurityPutPrivilegesRequest<'a> {
        SecurityPutPrivilegesRequest {
            refresh: self.refresh,
        }
    }
}
pub struct SecurityPutRoleRequest<'a> {
    refresh: Option<&'a i32>,
}
pub struct SecurityPutRoleRequestBuilder<'a> {
    refresh: Option<&'a i32>,
}
impl<'a> SecurityPutRoleRequestBuilder<'a> {
    pub fn build(&self) -> SecurityPutRoleRequest<'a> {
        SecurityPutRoleRequest {
            refresh: self.refresh,
        }
    }
}
pub struct SecurityPutRoleMappingRequest<'a> {
    refresh: Option<&'a i32>,
}
pub struct SecurityPutRoleMappingRequestBuilder<'a> {
    refresh: Option<&'a i32>,
}
impl<'a> SecurityPutRoleMappingRequestBuilder<'a> {
    pub fn build(&self) -> SecurityPutRoleMappingRequest<'a> {
        SecurityPutRoleMappingRequest {
            refresh: self.refresh,
        }
    }
}
pub struct SecurityPutUserRequest<'a> {
    refresh: Option<&'a i32>,
}
pub struct SecurityPutUserRequestBuilder<'a> {
    refresh: Option<&'a i32>,
}
impl<'a> SecurityPutUserRequestBuilder<'a> {
    pub fn build(&self) -> SecurityPutUserRequest<'a> {
        SecurityPutUserRequest {
            refresh: self.refresh,
        }
    }
}
#[doc = "Security APIs"]
pub struct SecurityNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> SecurityNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SecurityNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-authenticate.html"]
    pub fn authenticate(&self, request: &SecurityAuthenticateRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_security/_authenticate")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-change-password.html"]
    pub fn change_password(&self, request: &SecurityChangePasswordRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_security/user/{username}/_password")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-cache.html"]
    pub fn clear_cached_realms(
        &self,
        request: &SecurityClearCachedRealmsRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_security/realm/{realms}/_clear_cache")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-role-cache.html"]
    pub fn clear_cached_roles(
        &self,
        request: &SecurityClearCachedRolesRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_security/role/{name}/_clear_cache")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-create-api-key.html"]
    pub fn create_api_key(&self, request: &SecurityCreateApiKeyRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_security/api_key")
    }
    #[doc = "TODO"]
    pub fn delete_privileges(&self, request: &SecurityDeletePrivilegesRequest) -> Result<Response> {
        self.client.send(
            HttpMethod::Delete,
            "/_security/privilege/{application}/{name}",
        )
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-role.html"]
    pub fn delete_role(&self, request: &SecurityDeleteRoleRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_security/role/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-role-mapping.html"]
    pub fn delete_role_mapping(
        &self,
        request: &SecurityDeleteRoleMappingRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_security/role_mapping/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-user.html"]
    pub fn delete_user(&self, request: &SecurityDeleteUserRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_security/user/{username}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-disable-user.html"]
    pub fn disable_user(&self, request: &SecurityDisableUserRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_security/user/{username}/_disable")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-enable-user.html"]
    pub fn enable_user(&self, request: &SecurityEnableUserRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_security/user/{username}/_enable")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-api-key.html"]
    pub fn get_api_key(&self, request: &SecurityGetApiKeyRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_security/api_key")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-builtin-privileges.html"]
    pub fn get_builtin_privileges(
        &self,
        request: &SecurityGetBuiltinPrivilegesRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_security/privilege/_builtin")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-privileges.html"]
    pub fn get_privileges(&self, request: &SecurityGetPrivilegesRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_security/privilege")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-role.html"]
    pub fn get_role(&self, request: &SecurityGetRoleRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_security/role/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-role-mapping.html"]
    pub fn get_role_mapping(&self, request: &SecurityGetRoleMappingRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_security/role_mapping/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-token.html"]
    pub fn get_token(&self, request: &SecurityGetTokenRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_security/oauth2/token")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-user.html"]
    pub fn get_user(&self, request: &SecurityGetUserRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_security/user/{username}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-privileges.html"]
    pub fn get_user_privileges(
        &self,
        request: &SecurityGetUserPrivilegesRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_security/user/_privileges")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-has-privileges.html"]
    pub fn has_privileges(&self, request: &SecurityHasPrivilegesRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_security/user/_has_privileges")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-api-key.html"]
    pub fn invalidate_api_key(
        &self,
        request: &SecurityInvalidateApiKeyRequest,
    ) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_security/api_key")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-token.html"]
    pub fn invalidate_token(&self, request: &SecurityInvalidateTokenRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_security/oauth2/token")
    }
    #[doc = "TODO"]
    pub fn put_privileges(&self, request: &SecurityPutPrivilegesRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_security/privilege/")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role.html"]
    pub fn put_role(&self, request: &SecurityPutRoleRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_security/role/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role-mapping.html"]
    pub fn put_role_mapping(&self, request: &SecurityPutRoleMappingRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_security/role_mapping/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-user.html"]
    pub fn put_user(&self, request: &SecurityPutUserRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_security/user/{username}")
    }
}
impl ElasticsearchClient {
    #[doc = "Security APIs"]
    pub fn security(&self) -> SecurityNamespaceClient {
        SecurityNamespaceClient::new(self)
    }
}
