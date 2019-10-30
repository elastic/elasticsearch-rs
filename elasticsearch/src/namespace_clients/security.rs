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
use crate::{
    client::{Elasticsearch, Sender},
    enums::*,
    error::ElasticsearchError,
    http_method::HttpMethod,
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
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
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
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
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityChangePassword<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
    username: Option<String>,
}
impl<B> SecurityChangePassword<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityChangePassword {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
            username: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityChangePassword<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/user/{username}/_password";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
            }
            let query_params = QueryParamsStruct {
                refresh: self.refresh,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityClearCachedRealms<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    realms: Vec<String>,
    source: Option<String>,
    usernames: Option<Vec<String>>,
}
impl<B> SecurityClearCachedRealms<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, realms: Vec<String>) -> Self {
        SecurityClearCachedRealms {
            client,
            realms: realms,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            usernames: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityClearCachedRealms<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/realm/{realms}/_clear_cache";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "usernames")]
                usernames: Option<Vec<String>>,
            }
            let query_params = QueryParamsStruct {
                usernames: self.usernames,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityClearCachedRoles<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: Vec<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> SecurityClearCachedRoles<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, name: Vec<String>) -> Self {
        SecurityClearCachedRoles {
            client,
            name: name,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityClearCachedRoles<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/role/{name}/_clear_cache";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityCreateApiKey<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
}
impl<B> SecurityCreateApiKey<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityCreateApiKey {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityCreateApiKey<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/api_key";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
            }
            let query_params = QueryParamsStruct {
                refresh: self.refresh,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
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
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
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
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
            }
            let query_params = QueryParamsStruct {
                refresh: self.refresh,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
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
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
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
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
            }
            let query_params = QueryParamsStruct {
                refresh: self.refresh,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
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
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
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
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
            }
            let query_params = QueryParamsStruct {
                refresh: self.refresh,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
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
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
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
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
            }
            let query_params = QueryParamsStruct {
                refresh: self.refresh,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityDisableUser<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
    username: String,
}
impl<B> SecurityDisableUser<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, username: String) -> Self {
        SecurityDisableUser {
            client,
            username: username,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityDisableUser<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/user/{username}/_disable";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
            }
            let query_params = QueryParamsStruct {
                refresh: self.refresh,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityEnableUser<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
    username: String,
}
impl<B> SecurityEnableUser<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, username: String) -> Self {
        SecurityEnableUser {
            client,
            username: username,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityEnableUser<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/user/{username}/_enable";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
            }
            let query_params = QueryParamsStruct {
                refresh: self.refresh,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
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
            error_trace: None,
            filter_path: None,
            human: None,
            id: None,
            name: None,
            pretty: None,
            realm_name: None,
            source: None,
            username: None,
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
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "id")]
                id: Option<String>,
                #[serde(rename = "name")]
                name: Option<String>,
                #[serde(rename = "realm_name")]
                realm_name: Option<String>,
                #[serde(rename = "username")]
                username: Option<String>,
            }
            let query_params = QueryParamsStruct {
                id: self.id,
                name: self.name,
                realm_name: self.realm_name,
                username: self.username,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
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
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
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
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
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
            application: None,
            error_trace: None,
            filter_path: None,
            human: None,
            name: None,
            pretty: None,
            source: None,
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
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
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
            error_trace: None,
            filter_path: None,
            human: None,
            name: None,
            pretty: None,
            source: None,
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
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
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
            error_trace: None,
            filter_path: None,
            human: None,
            name: None,
            pretty: None,
            source: None,
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
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityGetToken<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> SecurityGetToken<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetToken {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityGetToken<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/oauth2/token";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
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
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            username: None,
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
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
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
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
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
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityHasPrivileges<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    user: Option<String>,
}
impl<B> SecurityHasPrivileges<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityHasPrivileges {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            user: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityHasPrivileges<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/user/_has_privileges";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityInvalidateApiKey<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> SecurityInvalidateApiKey<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityInvalidateApiKey {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityInvalidateApiKey<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/api_key";
        let method = HttpMethod::Delete;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityInvalidateToken<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> SecurityInvalidateToken<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityInvalidateToken {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityInvalidateToken<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/oauth2/token";
        let method = HttpMethod::Delete;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityPutPrivileges<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
}
impl<B> SecurityPutPrivileges<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityPutPrivileges {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityPutPrivileges<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/privilege/";
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
            }
            let query_params = QueryParamsStruct {
                refresh: self.refresh,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityPutRole<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: String,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
}
impl<B> SecurityPutRole<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, name: String) -> Self {
        SecurityPutRole {
            client,
            name: name,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityPutRole<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/role/{name}";
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
            }
            let query_params = QueryParamsStruct {
                refresh: self.refresh,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityPutRoleMapping<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: String,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
}
impl<B> SecurityPutRoleMapping<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, name: String) -> Self {
        SecurityPutRoleMapping {
            client,
            name: name,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityPutRoleMapping<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/role_mapping/{name}";
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
            }
            let query_params = QueryParamsStruct {
                refresh: self.refresh,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SecurityPutUser<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<String>,
    username: String,
}
impl<B> SecurityPutUser<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, username: String) -> Self {
        SecurityPutUser {
            client,
            username: username,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for SecurityPutUser<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_security/user/{username}";
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
            }
            let query_params = QueryParamsStruct {
                refresh: self.refresh,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
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
    pub fn change_password<B>(&self) -> SecurityChangePassword<B>
    where
        B: Serialize,
    {
        SecurityChangePassword::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-cache.html"]
    pub fn clear_cached_realms<B>(&self, realms: Vec<String>) -> SecurityClearCachedRealms<B>
    where
        B: Serialize,
    {
        SecurityClearCachedRealms::new(self.client.clone(), realms)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-role-cache.html"]
    pub fn clear_cached_roles<B>(&self, name: Vec<String>) -> SecurityClearCachedRoles<B>
    where
        B: Serialize,
    {
        SecurityClearCachedRoles::new(self.client.clone(), name)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-create-api-key.html"]
    pub fn create_api_key<B>(&self) -> SecurityCreateApiKey<B>
    where
        B: Serialize,
    {
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
    pub fn disable_user<B>(&self, username: String) -> SecurityDisableUser<B>
    where
        B: Serialize,
    {
        SecurityDisableUser::new(self.client.clone(), username)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-enable-user.html"]
    pub fn enable_user<B>(&self, username: String) -> SecurityEnableUser<B>
    where
        B: Serialize,
    {
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
    pub fn get_token<B>(&self) -> SecurityGetToken<B>
    where
        B: Serialize,
    {
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
    pub fn has_privileges<B>(&self) -> SecurityHasPrivileges<B>
    where
        B: Serialize,
    {
        SecurityHasPrivileges::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-api-key.html"]
    pub fn invalidate_api_key<B>(&self) -> SecurityInvalidateApiKey<B>
    where
        B: Serialize,
    {
        SecurityInvalidateApiKey::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-token.html"]
    pub fn invalidate_token<B>(&self) -> SecurityInvalidateToken<B>
    where
        B: Serialize,
    {
        SecurityInvalidateToken::new(self.client.clone())
    }
    #[doc = "TODO"]
    pub fn put_privileges<B>(&self) -> SecurityPutPrivileges<B>
    where
        B: Serialize,
    {
        SecurityPutPrivileges::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role.html"]
    pub fn put_role<B>(&self, name: String) -> SecurityPutRole<B>
    where
        B: Serialize,
    {
        SecurityPutRole::new(self.client.clone(), name)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role-mapping.html"]
    pub fn put_role_mapping<B>(&self, name: String) -> SecurityPutRoleMapping<B>
    where
        B: Serialize,
    {
        SecurityPutRoleMapping::new(self.client.clone(), name)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-user.html"]
    pub fn put_user<B>(&self, username: String) -> SecurityPutUser<B>
    where
        B: Serialize,
    {
        SecurityPutUser::new(self.client.clone(), username)
    }
}
impl Elasticsearch {
    #[doc = "Security APIs"]
    pub fn security(&self) -> Security {
        Security::new(self.clone())
    }
}
