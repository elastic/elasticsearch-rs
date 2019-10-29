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
use crate::error::ElasticsearchError;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct SecurityAuthenticate {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityAuthenticate {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityAuthenticate {
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
impl Sender for SecurityAuthenticate {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/_authenticate";
        let method = HttpMethod::Get;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityChangePassword {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
    username: Option<String>,
}
impl SecurityChangePassword {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityChangePassword {
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
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SecurityChangePassword {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/user/{username}/_password";
        let method = HttpMethod::Post;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityClearCachedRealms {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    realms: Vec<String>,
    source: Option<String>,
    usernames: Option<Vec<String>>,
}
impl SecurityClearCachedRealms {
    pub fn new(client: Elasticsearch, realms: Vec<String>) -> Self {
        SecurityClearCachedRealms {
            client,
            realms: realms,
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
    #[doc = "Comma-separated list of usernames to clear from the cache"]
    pub fn usernames(mut self, usernames: Option<Vec<String>>) -> Self {
        self.usernames = usernames;
        self
    }
}
impl Sender for SecurityClearCachedRealms {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/realm/{realms}/_clear_cache";
        let method = HttpMethod::Post;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityClearCachedRoles {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: Vec<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityClearCachedRoles {
    pub fn new(client: Elasticsearch, name: Vec<String>) -> Self {
        SecurityClearCachedRoles {
            client,
            name: name,
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
impl Sender for SecurityClearCachedRoles {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/role/{name}/_clear_cache";
        let method = HttpMethod::Post;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityCreateApiKey {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
}
impl SecurityCreateApiKey {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityCreateApiKey {
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
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SecurityCreateApiKey {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/api_key";
        let method = HttpMethod::Post;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityDeletePrivileges {
    client: Elasticsearch,
    application: String,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: String,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
}
impl SecurityDeletePrivileges {
    pub fn new(client: Elasticsearch, application: String, name: String) -> Self {
        SecurityDeletePrivileges {
            client,
            application: application,
            name: name,
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
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SecurityDeletePrivileges {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/privilege/{application}/{name}";
        let method = HttpMethod::Delete;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityDeleteRole {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: String,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
}
impl SecurityDeleteRole {
    pub fn new(client: Elasticsearch, name: String) -> Self {
        SecurityDeleteRole {
            client,
            name: name,
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
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SecurityDeleteRole {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/role/{name}";
        let method = HttpMethod::Delete;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityDeleteRoleMapping {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: String,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
}
impl SecurityDeleteRoleMapping {
    pub fn new(client: Elasticsearch, name: String) -> Self {
        SecurityDeleteRoleMapping {
            client,
            name: name,
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
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SecurityDeleteRoleMapping {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/role_mapping/{name}";
        let method = HttpMethod::Delete;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityDeleteUser {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
    username: String,
}
impl SecurityDeleteUser {
    pub fn new(client: Elasticsearch, username: String) -> Self {
        SecurityDeleteUser {
            client,
            username: username,
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
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SecurityDeleteUser {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/user/{username}";
        let method = HttpMethod::Delete;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityDisableUser {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
    username: String,
}
impl SecurityDisableUser {
    pub fn new(client: Elasticsearch, username: String) -> Self {
        SecurityDisableUser {
            client,
            username: username,
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
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SecurityDisableUser {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/user/{username}/_disable";
        let method = HttpMethod::Post;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityEnableUser {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
    username: String,
}
impl SecurityEnableUser {
    pub fn new(client: Elasticsearch, username: String) -> Self {
        SecurityEnableUser {
            client,
            username: username,
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
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SecurityEnableUser {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/user/{username}/_enable";
        let method = HttpMethod::Post;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityGetApiKey {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: Option<String>,
    name: Option<String>,
    pretty: Option<bool>,
    realm_name: Option<String>,
    source: Option<String>,
    username: Option<String>,
}
impl SecurityGetApiKey {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetApiKey {
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
    #[doc = "API key id of the API key to be retrieved"]
    pub fn id(mut self, id: Option<String>) -> Self {
        self.id = id;
        self
    }
    #[doc = "API key name of the API key to be retrieved"]
    pub fn name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "realm name of the user who created this API key to be retrieved"]
    pub fn realm_name(mut self, realm_name: Option<String>) -> Self {
        self.realm_name = realm_name;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "user name of the user who created this API key to be retrieved"]
    pub fn username(mut self, username: Option<String>) -> Self {
        self.username = username;
        self
    }
}
impl Sender for SecurityGetApiKey {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/api_key";
        let method = HttpMethod::Get;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityGetBuiltinPrivileges {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityGetBuiltinPrivileges {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetBuiltinPrivileges {
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
impl Sender for SecurityGetBuiltinPrivileges {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/privilege/_builtin";
        let method = HttpMethod::Get;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityGetPrivileges {
    client: Elasticsearch,
    application: Option<String>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityGetPrivileges {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetPrivileges {
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
impl Sender for SecurityGetPrivileges {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/privilege";
        let method = HttpMethod::Get;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityGetRole {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityGetRole {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetRole {
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
impl Sender for SecurityGetRole {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/role/{name}";
        let method = HttpMethod::Get;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityGetRoleMapping {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityGetRoleMapping {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetRoleMapping {
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
impl Sender for SecurityGetRoleMapping {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/role_mapping/{name}";
        let method = HttpMethod::Get;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityGetToken {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityGetToken {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetToken {
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
impl Sender for SecurityGetToken {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/oauth2/token";
        let method = HttpMethod::Post;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityGetUser {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    username: Option<Vec<String>>,
}
impl SecurityGetUser {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetUser {
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
impl Sender for SecurityGetUser {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/user/{username}";
        let method = HttpMethod::Get;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityGetUserPrivileges {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityGetUserPrivileges {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetUserPrivileges {
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
impl Sender for SecurityGetUserPrivileges {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/user/_privileges";
        let method = HttpMethod::Get;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityHasPrivileges {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    user: Option<String>,
}
impl SecurityHasPrivileges {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityHasPrivileges {
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
impl Sender for SecurityHasPrivileges {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/user/_has_privileges";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityInvalidateApiKey {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityInvalidateApiKey {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityInvalidateApiKey {
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
impl Sender for SecurityInvalidateApiKey {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/api_key";
        let method = HttpMethod::Delete;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityInvalidateToken {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SecurityInvalidateToken {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityInvalidateToken {
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
impl Sender for SecurityInvalidateToken {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/oauth2/token";
        let method = HttpMethod::Delete;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityPutPrivileges {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
}
impl SecurityPutPrivileges {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityPutPrivileges {
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
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SecurityPutPrivileges {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/privilege/";
        let method = HttpMethod::Put;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityPutRole {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: String,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
}
impl SecurityPutRole {
    pub fn new(client: Elasticsearch, name: String) -> Self {
        SecurityPutRole {
            client,
            name: name,
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
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SecurityPutRole {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/role/{name}";
        let method = HttpMethod::Put;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityPutRoleMapping {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: String,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
}
impl SecurityPutRoleMapping {
    pub fn new(client: Elasticsearch, name: String) -> Self {
        SecurityPutRoleMapping {
            client,
            name: name,
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
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SecurityPutRoleMapping {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/role_mapping/{name}";
        let method = HttpMethod::Put;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct SecurityPutUser {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
    username: String,
}
impl SecurityPutUser {
    pub fn new(client: Elasticsearch, username: String) -> Self {
        SecurityPutUser {
            client,
            username: username,
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
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Option<Refresh>) -> Self {
        self.refresh = refresh;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SecurityPutUser {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/user/{username}";
        let method = HttpMethod::Put;
        let response = self.client.send::<()>(method, path, None, None)?;
        Ok(response)
    }
}
#[doc = "Security APIs"]
pub struct Security {
    client: Elasticsearch,
}
impl Security {
    pub fn new(client: Elasticsearch) -> Self {
        Security { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-authenticate.html"]
    pub fn authenticate(&self) -> SecurityAuthenticate {
        SecurityAuthenticate::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-change-password.html"]
    pub fn change_password(&self) -> SecurityChangePassword {
        SecurityChangePassword::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-cache.html"]
    pub fn clear_cached_realms(&self, realms: Vec<String>) -> SecurityClearCachedRealms {
        SecurityClearCachedRealms::new(self.client.clone(), realms)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-role-cache.html"]
    pub fn clear_cached_roles(&self, name: Vec<String>) -> SecurityClearCachedRoles {
        SecurityClearCachedRoles::new(self.client.clone(), name)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-create-api-key.html"]
    pub fn create_api_key(&self) -> SecurityCreateApiKey {
        SecurityCreateApiKey::new(self.client.clone())
    }
    #[doc = "TODO"]
    pub fn delete_privileges(&self, application: String, name: String) -> SecurityDeletePrivileges {
        SecurityDeletePrivileges::new(self.client.clone(), application, name)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-role.html"]
    pub fn delete_role(&self, name: String) -> SecurityDeleteRole {
        SecurityDeleteRole::new(self.client.clone(), name)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-role-mapping.html"]
    pub fn delete_role_mapping(&self, name: String) -> SecurityDeleteRoleMapping {
        SecurityDeleteRoleMapping::new(self.client.clone(), name)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-user.html"]
    pub fn delete_user(&self, username: String) -> SecurityDeleteUser {
        SecurityDeleteUser::new(self.client.clone(), username)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-disable-user.html"]
    pub fn disable_user(&self, username: String) -> SecurityDisableUser {
        SecurityDisableUser::new(self.client.clone(), username)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-enable-user.html"]
    pub fn enable_user(&self, username: String) -> SecurityEnableUser {
        SecurityEnableUser::new(self.client.clone(), username)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-api-key.html"]
    pub fn get_api_key(&self) -> SecurityGetApiKey {
        SecurityGetApiKey::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-builtin-privileges.html"]
    pub fn get_builtin_privileges(&self) -> SecurityGetBuiltinPrivileges {
        SecurityGetBuiltinPrivileges::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-privileges.html"]
    pub fn get_privileges(&self) -> SecurityGetPrivileges {
        SecurityGetPrivileges::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-role.html"]
    pub fn get_role(&self) -> SecurityGetRole {
        SecurityGetRole::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-role-mapping.html"]
    pub fn get_role_mapping(&self) -> SecurityGetRoleMapping {
        SecurityGetRoleMapping::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-token.html"]
    pub fn get_token(&self) -> SecurityGetToken {
        SecurityGetToken::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-user.html"]
    pub fn get_user(&self) -> SecurityGetUser {
        SecurityGetUser::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-privileges.html"]
    pub fn get_user_privileges(&self) -> SecurityGetUserPrivileges {
        SecurityGetUserPrivileges::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-has-privileges.html"]
    pub fn has_privileges(&self) -> SecurityHasPrivileges {
        SecurityHasPrivileges::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-api-key.html"]
    pub fn invalidate_api_key(&self) -> SecurityInvalidateApiKey {
        SecurityInvalidateApiKey::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-token.html"]
    pub fn invalidate_token(&self) -> SecurityInvalidateToken {
        SecurityInvalidateToken::new(self.client.clone())
    }
    #[doc = "TODO"]
    pub fn put_privileges(&self) -> SecurityPutPrivileges {
        SecurityPutPrivileges::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role.html"]
    pub fn put_role(&self, name: String) -> SecurityPutRole {
        SecurityPutRole::new(self.client.clone(), name)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role-mapping.html"]
    pub fn put_role_mapping(&self, name: String) -> SecurityPutRoleMapping {
        SecurityPutRoleMapping::new(self.client.clone(), name)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-user.html"]
    pub fn put_user(&self, username: String) -> SecurityPutUser {
        SecurityPutUser::new(self.client.clone(), username)
    }
}
impl Elasticsearch {
    #[doc = "Security APIs"]
    pub fn security(&self) -> Security {
        Security::new(self.clone())
    }
}