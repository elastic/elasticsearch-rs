use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
#[doc = "Security APIs"]
pub struct SecurityNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl SecurityNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SecurityNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-authenticate.html"]
    pub fn authenticate(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_security/_authenticate")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-change-password.html"]
    pub fn change_password(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_security/user/{username}/_password")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-cache.html"]
    pub fn clear_cached_realms(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_security/realm/{realms}/_clear_cache")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-role-cache.html"]
    pub fn clear_cached_roles(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_security/role/{name}/_clear_cache")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-create-api-key.html"]
    pub fn create_api_key(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_security/api_key")
    }
    #[doc = "TODO"]
    pub fn delete_privileges(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Delete,
            "/_security/privilege/{application}/{name}",
        )
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-role.html"]
    pub fn delete_role(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_security/role/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-role-mapping.html"]
    pub fn delete_role_mapping(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_security/role_mapping/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-user.html"]
    pub fn delete_user(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_security/user/{username}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-disable-user.html"]
    pub fn disable_user(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_security/user/{username}/_disable")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-enable-user.html"]
    pub fn enable_user(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_security/user/{username}/_enable")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-api-key.html"]
    pub fn get_api_key(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_security/api_key")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-builtin-privileges.html"]
    pub fn get_builtin_privileges(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_security/privilege/_builtin")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-privileges.html"]
    pub fn get_privileges(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_security/privilege")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-role.html"]
    pub fn get_role(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_security/role/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-role-mapping.html"]
    pub fn get_role_mapping(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_security/role_mapping/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-token.html"]
    pub fn get_token(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_security/oauth2/token")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-user.html"]
    pub fn get_user(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_security/user/{username}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-privileges.html"]
    pub fn get_user_privileges(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_security/user/_privileges")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-has-privileges.html"]
    pub fn has_privileges(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_security/user/_has_privileges")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-api-key.html"]
    pub fn invalidate_api_key(&self) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_security/api_key")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-token.html"]
    pub fn invalidate_token(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_security/oauth2/token")
    }
    #[doc = "TODO"]
    pub fn put_privileges(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_security/privilege/")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role.html"]
    pub fn put_role(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_security/role/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role-mapping.html"]
    pub fn put_role_mapping(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_security/role_mapping/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-user.html"]
    pub fn put_user(&self) -> Result<Response> {
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
