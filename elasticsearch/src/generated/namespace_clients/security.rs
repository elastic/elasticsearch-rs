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
#[allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
    params::*,
};
use serde::Serialize;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Authenticate API"]
pub enum SecurityAuthenticateParts {
    #[doc = "No parts"]
    None,
}
impl SecurityAuthenticateParts {
    #[doc = "Builds a relative URL path to the Security Authenticate API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityAuthenticateParts::None => "/_security/_authenticate".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Authenticate API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-authenticate.html)"]
pub struct SecurityAuthenticate<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SecurityAuthenticateParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityAuthenticate<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityAuthenticate]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SecurityAuthenticate {
            client,
            parts: SecurityAuthenticateParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Authenticate API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Change Password API"]
pub enum SecurityChangePasswordParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
    #[doc = "No parts"]
    None,
}
impl<'b> SecurityChangePasswordParts<'b> {
    #[doc = "Builds a relative URL path to the Security Change Password API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityChangePasswordParts::Username(ref username) => {
                let mut p = String::with_capacity(26usize + username.len());
                p.push_str("/_security/user/");
                p.push_str(username.as_ref());
                p.push_str("/_password");
                p.into()
            }
            SecurityChangePasswordParts::None => "/_security/user/_password".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Change Password API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-change-password.html)"]
pub struct SecurityChangePassword<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityChangePasswordParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityChangePassword<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityChangePassword] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityChangePasswordParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityChangePassword {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> SecurityChangePassword<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityChangePassword {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Change Password API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                refresh: self.refresh,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Clear Cached Realms API"]
pub enum SecurityClearCachedRealmsParts<'b> {
    #[doc = "Realms"]
    Realms(&'b [&'b str]),
}
impl<'b> SecurityClearCachedRealmsParts<'b> {
    #[doc = "Builds a relative URL path to the Security Clear Cached Realms API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityClearCachedRealmsParts::Realms(ref realms) => {
                let realms_str = realms.join(",");
                let mut p = String::with_capacity(30usize + realms_str.len());
                p.push_str("/_security/realm/");
                p.push_str(realms_str.as_ref());
                p.push_str("/_clear_cache");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Clear Cached Realms API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-clear-cache.html)"]
pub struct SecurityClearCachedRealms<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityClearCachedRealmsParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    usernames: Option<&'b [&'b str]>,
}
impl<'a, 'b, B> SecurityClearCachedRealms<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityClearCachedRealms] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityClearCachedRealmsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityClearCachedRealms {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> SecurityClearCachedRealms<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityClearCachedRealms {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            usernames: self.usernames,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Comma-separated list of usernames to clear from the cache"]
    pub fn usernames(mut self, usernames: &'b [&'b str]) -> Self {
        self.usernames = Some(usernames);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Clear Cached Realms API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(
                    rename = "usernames",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                usernames: Option<&'b [&'b str]>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                usernames: self.usernames,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Clear Cached Roles API"]
pub enum SecurityClearCachedRolesParts<'b> {
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> SecurityClearCachedRolesParts<'b> {
    #[doc = "Builds a relative URL path to the Security Clear Cached Roles API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityClearCachedRolesParts::Name(ref name) => {
                let name_str = name.join(",");
                let mut p = String::with_capacity(29usize + name_str.len());
                p.push_str("/_security/role/");
                p.push_str(name_str.as_ref());
                p.push_str("/_clear_cache");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Clear Cached Roles API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-clear-role-cache.html)"]
pub struct SecurityClearCachedRoles<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityClearCachedRolesParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityClearCachedRoles<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityClearCachedRoles] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityClearCachedRolesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityClearCachedRoles {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityClearCachedRoles<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityClearCachedRoles {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Clear Cached Roles API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Create Api Key API"]
pub enum SecurityCreateApiKeyParts {
    #[doc = "No parts"]
    None,
}
impl SecurityCreateApiKeyParts {
    #[doc = "Builds a relative URL path to the Security Create Api Key API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityCreateApiKeyParts::None => "/_security/api_key".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Create Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-create-api-key.html)"]
pub struct SecurityCreateApiKey<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityCreateApiKeyParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityCreateApiKey<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityCreateApiKey]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SecurityCreateApiKey {
            client,
            parts: SecurityCreateApiKeyParts::None,
            headers,
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
    pub fn body<T>(self, body: T) -> SecurityCreateApiKey<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityCreateApiKey {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Create Api Key API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                refresh: self.refresh,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Delete Privileges API"]
pub enum SecurityDeletePrivilegesParts<'b> {
    #[doc = "Application and Name"]
    ApplicationName(&'b str, &'b str),
}
impl<'b> SecurityDeletePrivilegesParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete Privileges API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeletePrivilegesParts::ApplicationName(ref application, ref name) => {
                let mut p = String::with_capacity(22usize + application.len() + name.len());
                p.push_str("/_security/privilege/");
                p.push_str(application.as_ref());
                p.push_str("/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the Security Delete Privileges API"]
pub struct SecurityDeletePrivileges<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SecurityDeletePrivilegesParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeletePrivileges<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeletePrivileges] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityDeletePrivilegesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeletePrivileges {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Delete Privileges API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                refresh: self.refresh,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Delete Role API"]
pub enum SecurityDeleteRoleParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> SecurityDeleteRoleParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete Role API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteRoleParts::Name(ref name) => {
                let mut p = String::with_capacity(16usize + name.len());
                p.push_str("/_security/role/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Delete Role API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-delete-role.html)"]
pub struct SecurityDeleteRole<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SecurityDeleteRoleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteRole<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteRole] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityDeleteRoleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteRole {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Delete Role API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                refresh: self.refresh,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Delete Role Mapping API"]
pub enum SecurityDeleteRoleMappingParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> SecurityDeleteRoleMappingParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete Role Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteRoleMappingParts::Name(ref name) => {
                let mut p = String::with_capacity(24usize + name.len());
                p.push_str("/_security/role_mapping/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Delete Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-delete-role-mapping.html)"]
pub struct SecurityDeleteRoleMapping<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SecurityDeleteRoleMappingParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteRoleMapping<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteRoleMapping] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityDeleteRoleMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteRoleMapping {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Delete Role Mapping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                refresh: self.refresh,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Delete User API"]
pub enum SecurityDeleteUserParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityDeleteUserParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteUserParts::Username(ref username) => {
                let mut p = String::with_capacity(16usize + username.len());
                p.push_str("/_security/user/");
                p.push_str(username.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Delete User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-delete-user.html)"]
pub struct SecurityDeleteUser<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SecurityDeleteUserParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteUser<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteUser] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityDeleteUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteUser {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Delete User API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                refresh: self.refresh,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Disable User API"]
pub enum SecurityDisableUserParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityDisableUserParts<'b> {
    #[doc = "Builds a relative URL path to the Security Disable User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDisableUserParts::Username(ref username) => {
                let mut p = String::with_capacity(25usize + username.len());
                p.push_str("/_security/user/");
                p.push_str(username.as_ref());
                p.push_str("/_disable");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Disable User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-disable-user.html)"]
pub struct SecurityDisableUser<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityDisableUserParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityDisableUser<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityDisableUser] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityDisableUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDisableUser {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> SecurityDisableUser<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityDisableUser {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Disable User API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                refresh: self.refresh,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Enable User API"]
pub enum SecurityEnableUserParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityEnableUserParts<'b> {
    #[doc = "Builds a relative URL path to the Security Enable User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityEnableUserParts::Username(ref username) => {
                let mut p = String::with_capacity(24usize + username.len());
                p.push_str("/_security/user/");
                p.push_str(username.as_ref());
                p.push_str("/_enable");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Enable User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-enable-user.html)"]
pub struct SecurityEnableUser<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityEnableUserParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityEnableUser<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityEnableUser] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityEnableUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityEnableUser {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> SecurityEnableUser<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityEnableUser {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Enable User API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                refresh: self.refresh,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Api Key API"]
pub enum SecurityGetApiKeyParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetApiKeyParts {
    #[doc = "Builds a relative URL path to the Security Get Api Key API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetApiKeyParts::None => "/_security/api_key".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Get Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-api-key.html)"]
pub struct SecurityGetApiKey<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SecurityGetApiKeyParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    id: Option<&'b str>,
    name: Option<&'b str>,
    owner: Option<bool>,
    pretty: Option<bool>,
    realm_name: Option<&'b str>,
    source: Option<&'b str>,
    username: Option<&'b str>,
}
impl<'a, 'b> SecurityGetApiKey<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetApiKey]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SecurityGetApiKey {
            client,
            parts: SecurityGetApiKeyParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            id: None,
            name: None,
            owner: None,
            pretty: None,
            realm_name: None,
            source: None,
            username: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "API key id of the API key to be retrieved"]
    pub fn id(mut self, id: &'b str) -> Self {
        self.id = Some(id);
        self
    }
    #[doc = "API key name of the API key to be retrieved"]
    pub fn name(mut self, name: &'b str) -> Self {
        self.name = Some(name);
        self
    }
    #[doc = "flag to query API keys owned by the currently authenticated user"]
    pub fn owner(mut self, owner: bool) -> Self {
        self.owner = Some(owner);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "realm name of the user who created this API key to be retrieved"]
    pub fn realm_name(mut self, realm_name: &'b str) -> Self {
        self.realm_name = Some(realm_name);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "user name of the user who created this API key to be retrieved"]
    pub fn username(mut self, username: &'b str) -> Self {
        self.username = Some(username);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Api Key API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "id")]
                id: Option<&'b str>,
                #[serde(rename = "name")]
                name: Option<&'b str>,
                #[serde(rename = "owner")]
                owner: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "realm_name")]
                realm_name: Option<&'b str>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "username")]
                username: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                id: self.id,
                name: self.name,
                owner: self.owner,
                pretty: self.pretty,
                realm_name: self.realm_name,
                source: self.source,
                username: self.username,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Builtin Privileges API"]
pub enum SecurityGetBuiltinPrivilegesParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetBuiltinPrivilegesParts {
    #[doc = "Builds a relative URL path to the Security Get Builtin Privileges API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetBuiltinPrivilegesParts::None => "/_security/privilege/_builtin".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Get Builtin Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-builtin-privileges.html)"]
pub struct SecurityGetBuiltinPrivileges<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SecurityGetBuiltinPrivilegesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetBuiltinPrivileges<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetBuiltinPrivileges]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SecurityGetBuiltinPrivileges {
            client,
            parts: SecurityGetBuiltinPrivilegesParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Builtin Privileges API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Privileges API"]
pub enum SecurityGetPrivilegesParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Application"]
    Application(&'b str),
    #[doc = "Application and Name"]
    ApplicationName(&'b str, &'b str),
}
impl<'b> SecurityGetPrivilegesParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get Privileges API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetPrivilegesParts::None => "/_security/privilege".into(),
            SecurityGetPrivilegesParts::Application(ref application) => {
                let mut p = String::with_capacity(21usize + application.len());
                p.push_str("/_security/privilege/");
                p.push_str(application.as_ref());
                p.into()
            }
            SecurityGetPrivilegesParts::ApplicationName(ref application, ref name) => {
                let mut p = String::with_capacity(22usize + application.len() + name.len());
                p.push_str("/_security/privilege/");
                p.push_str(application.as_ref());
                p.push_str("/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Get Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-privileges.html)"]
pub struct SecurityGetPrivileges<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SecurityGetPrivilegesParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetPrivileges<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetPrivileges] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityGetPrivilegesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetPrivileges {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Privileges API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Role API"]
pub enum SecurityGetRoleParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
    #[doc = "No parts"]
    None,
}
impl<'b> SecurityGetRoleParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get Role API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetRoleParts::Name(ref name) => {
                let mut p = String::with_capacity(16usize + name.len());
                p.push_str("/_security/role/");
                p.push_str(name.as_ref());
                p.into()
            }
            SecurityGetRoleParts::None => "/_security/role".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Get Role API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-role.html)"]
pub struct SecurityGetRole<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SecurityGetRoleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetRole<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetRole] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityGetRoleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetRole {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Role API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Role Mapping API"]
pub enum SecurityGetRoleMappingParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
    #[doc = "No parts"]
    None,
}
impl<'b> SecurityGetRoleMappingParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get Role Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetRoleMappingParts::Name(ref name) => {
                let mut p = String::with_capacity(24usize + name.len());
                p.push_str("/_security/role_mapping/");
                p.push_str(name.as_ref());
                p.into()
            }
            SecurityGetRoleMappingParts::None => "/_security/role_mapping".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Get Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-role-mapping.html)"]
pub struct SecurityGetRoleMapping<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SecurityGetRoleMappingParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetRoleMapping<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetRoleMapping] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityGetRoleMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetRoleMapping {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Role Mapping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Token API"]
pub enum SecurityGetTokenParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetTokenParts {
    #[doc = "Builds a relative URL path to the Security Get Token API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetTokenParts::None => "/_security/oauth2/token".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Get Token API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-token.html)"]
pub struct SecurityGetToken<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityGetTokenParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityGetToken<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityGetToken]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SecurityGetToken {
            client,
            parts: SecurityGetTokenParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityGetToken<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityGetToken {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Token API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get User API"]
pub enum SecurityGetUserParts<'b> {
    #[doc = "Username"]
    Username(&'b [&'b str]),
    #[doc = "No parts"]
    None,
}
impl<'b> SecurityGetUserParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetUserParts::Username(ref username) => {
                let username_str = username.join(",");
                let mut p = String::with_capacity(16usize + username_str.len());
                p.push_str("/_security/user/");
                p.push_str(username_str.as_ref());
                p.into()
            }
            SecurityGetUserParts::None => "/_security/user".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Get User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-user.html)"]
pub struct SecurityGetUser<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SecurityGetUserParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetUser<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetUser] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityGetUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetUser {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get User API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get User Privileges API"]
pub enum SecurityGetUserPrivilegesParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetUserPrivilegesParts {
    #[doc = "Builds a relative URL path to the Security Get User Privileges API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetUserPrivilegesParts::None => "/_security/user/_privileges".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Get User Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-privileges.html)"]
pub struct SecurityGetUserPrivileges<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SecurityGetUserPrivilegesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetUserPrivileges<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetUserPrivileges]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SecurityGetUserPrivileges {
            client,
            parts: SecurityGetUserPrivilegesParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get User Privileges API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Has Privileges API"]
pub enum SecurityHasPrivilegesParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "User"]
    User(&'b str),
}
impl<'b> SecurityHasPrivilegesParts<'b> {
    #[doc = "Builds a relative URL path to the Security Has Privileges API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityHasPrivilegesParts::None => "/_security/user/_has_privileges".into(),
            SecurityHasPrivilegesParts::User(ref user) => {
                let mut p = String::with_capacity(32usize + user.len());
                p.push_str("/_security/user/");
                p.push_str(user.as_ref());
                p.push_str("/_has_privileges");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Has Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-has-privileges.html)"]
pub struct SecurityHasPrivileges<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityHasPrivilegesParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityHasPrivileges<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityHasPrivileges] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityHasPrivilegesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityHasPrivileges {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityHasPrivileges<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityHasPrivileges {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Has Privileges API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Invalidate Api Key API"]
pub enum SecurityInvalidateApiKeyParts {
    #[doc = "No parts"]
    None,
}
impl SecurityInvalidateApiKeyParts {
    #[doc = "Builds a relative URL path to the Security Invalidate Api Key API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityInvalidateApiKeyParts::None => "/_security/api_key".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Invalidate Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-invalidate-api-key.html)"]
pub struct SecurityInvalidateApiKey<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityInvalidateApiKeyParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityInvalidateApiKey<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityInvalidateApiKey]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SecurityInvalidateApiKey {
            client,
            parts: SecurityInvalidateApiKeyParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityInvalidateApiKey<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityInvalidateApiKey {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Invalidate Api Key API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Invalidate Token API"]
pub enum SecurityInvalidateTokenParts {
    #[doc = "No parts"]
    None,
}
impl SecurityInvalidateTokenParts {
    #[doc = "Builds a relative URL path to the Security Invalidate Token API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityInvalidateTokenParts::None => "/_security/oauth2/token".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Invalidate Token API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-invalidate-token.html)"]
pub struct SecurityInvalidateToken<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityInvalidateTokenParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityInvalidateToken<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityInvalidateToken]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SecurityInvalidateToken {
            client,
            parts: SecurityInvalidateTokenParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityInvalidateToken<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityInvalidateToken {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Invalidate Token API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Put Privileges API"]
pub enum SecurityPutPrivilegesParts {
    #[doc = "No parts"]
    None,
}
impl SecurityPutPrivilegesParts {
    #[doc = "Builds a relative URL path to the Security Put Privileges API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPutPrivilegesParts::None => "/_security/privilege/".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the Security Put Privileges API"]
pub struct SecurityPutPrivileges<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityPutPrivilegesParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPutPrivileges<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPutPrivileges]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SecurityPutPrivileges {
            client,
            parts: SecurityPutPrivilegesParts::None,
            headers,
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
    pub fn body<T>(self, body: T) -> SecurityPutPrivileges<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPutPrivileges {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Put Privileges API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                refresh: self.refresh,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Put Role API"]
pub enum SecurityPutRoleParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> SecurityPutRoleParts<'b> {
    #[doc = "Builds a relative URL path to the Security Put Role API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPutRoleParts::Name(ref name) => {
                let mut p = String::with_capacity(16usize + name.len());
                p.push_str("/_security/role/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Put Role API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-put-role.html)"]
pub struct SecurityPutRole<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityPutRoleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPutRole<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPutRole] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityPutRoleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityPutRole {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> SecurityPutRole<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPutRole {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Put Role API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                refresh: self.refresh,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Put Role Mapping API"]
pub enum SecurityPutRoleMappingParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> SecurityPutRoleMappingParts<'b> {
    #[doc = "Builds a relative URL path to the Security Put Role Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPutRoleMappingParts::Name(ref name) => {
                let mut p = String::with_capacity(24usize + name.len());
                p.push_str("/_security/role_mapping/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Put Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-put-role-mapping.html)"]
pub struct SecurityPutRoleMapping<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityPutRoleMappingParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPutRoleMapping<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPutRoleMapping] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityPutRoleMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityPutRoleMapping {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> SecurityPutRoleMapping<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPutRoleMapping {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Put Role Mapping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                refresh: self.refresh,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Put User API"]
pub enum SecurityPutUserParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityPutUserParts<'b> {
    #[doc = "Builds a relative URL path to the Security Put User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPutUserParts::Username(ref username) => {
                let mut p = String::with_capacity(16usize + username.len());
                p.push_str("/_security/user/");
                p.push_str(username.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Security Put User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-put-user.html)"]
pub struct SecurityPutUser<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SecurityPutUserParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPutUser<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPutUser] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SecurityPutUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityPutUser {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> SecurityPutUser<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPutUser {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Put User API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                refresh: self.refresh,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Security APIs"]
pub struct Security<'a> {
    client: &'a Elasticsearch,
}
impl<'a> Security<'a> {
    #[doc = "Creates a new instance of [Security]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        Self { client }
    }
    #[doc = "[Security Authenticate API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-authenticate.html)"]
    pub fn authenticate<'b>(&'a self) -> SecurityAuthenticate<'a, 'b> {
        SecurityAuthenticate::new(&self.client)
    }
    #[doc = "[Security Change Password API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-change-password.html)"]
    pub fn change_password<'b>(
        &'a self,
        parts: SecurityChangePasswordParts<'b>,
    ) -> SecurityChangePassword<'a, 'b, ()> {
        SecurityChangePassword::new(&self.client, parts)
    }
    #[doc = "[Security Clear Cached Realms API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-clear-cache.html)"]
    pub fn clear_cached_realms<'b>(
        &'a self,
        parts: SecurityClearCachedRealmsParts<'b>,
    ) -> SecurityClearCachedRealms<'a, 'b, ()> {
        SecurityClearCachedRealms::new(&self.client, parts)
    }
    #[doc = "[Security Clear Cached Roles API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-clear-role-cache.html)"]
    pub fn clear_cached_roles<'b>(
        &'a self,
        parts: SecurityClearCachedRolesParts<'b>,
    ) -> SecurityClearCachedRoles<'a, 'b, ()> {
        SecurityClearCachedRoles::new(&self.client, parts)
    }
    #[doc = "[Security Create Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-create-api-key.html)"]
    pub fn create_api_key<'b>(&'a self) -> SecurityCreateApiKey<'a, 'b, ()> {
        SecurityCreateApiKey::new(&self.client)
    }
    #[doc = "Security Delete Privileges API"]
    pub fn delete_privileges<'b>(
        &'a self,
        parts: SecurityDeletePrivilegesParts<'b>,
    ) -> SecurityDeletePrivileges<'a, 'b> {
        SecurityDeletePrivileges::new(&self.client, parts)
    }
    #[doc = "[Security Delete Role API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-delete-role.html)"]
    pub fn delete_role<'b>(
        &'a self,
        parts: SecurityDeleteRoleParts<'b>,
    ) -> SecurityDeleteRole<'a, 'b> {
        SecurityDeleteRole::new(&self.client, parts)
    }
    #[doc = "[Security Delete Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-delete-role-mapping.html)"]
    pub fn delete_role_mapping<'b>(
        &'a self,
        parts: SecurityDeleteRoleMappingParts<'b>,
    ) -> SecurityDeleteRoleMapping<'a, 'b> {
        SecurityDeleteRoleMapping::new(&self.client, parts)
    }
    #[doc = "[Security Delete User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-delete-user.html)"]
    pub fn delete_user<'b>(
        &'a self,
        parts: SecurityDeleteUserParts<'b>,
    ) -> SecurityDeleteUser<'a, 'b> {
        SecurityDeleteUser::new(&self.client, parts)
    }
    #[doc = "[Security Disable User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-disable-user.html)"]
    pub fn disable_user<'b>(
        &'a self,
        parts: SecurityDisableUserParts<'b>,
    ) -> SecurityDisableUser<'a, 'b, ()> {
        SecurityDisableUser::new(&self.client, parts)
    }
    #[doc = "[Security Enable User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-enable-user.html)"]
    pub fn enable_user<'b>(
        &'a self,
        parts: SecurityEnableUserParts<'b>,
    ) -> SecurityEnableUser<'a, 'b, ()> {
        SecurityEnableUser::new(&self.client, parts)
    }
    #[doc = "[Security Get Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-api-key.html)"]
    pub fn get_api_key<'b>(&'a self) -> SecurityGetApiKey<'a, 'b> {
        SecurityGetApiKey::new(&self.client)
    }
    #[doc = "[Security Get Builtin Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-builtin-privileges.html)"]
    pub fn get_builtin_privileges<'b>(&'a self) -> SecurityGetBuiltinPrivileges<'a, 'b> {
        SecurityGetBuiltinPrivileges::new(&self.client)
    }
    #[doc = "[Security Get Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-privileges.html)"]
    pub fn get_privileges<'b>(
        &'a self,
        parts: SecurityGetPrivilegesParts<'b>,
    ) -> SecurityGetPrivileges<'a, 'b> {
        SecurityGetPrivileges::new(&self.client, parts)
    }
    #[doc = "[Security Get Role API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-role.html)"]
    pub fn get_role<'b>(&'a self, parts: SecurityGetRoleParts<'b>) -> SecurityGetRole<'a, 'b> {
        SecurityGetRole::new(&self.client, parts)
    }
    #[doc = "[Security Get Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-role-mapping.html)"]
    pub fn get_role_mapping<'b>(
        &'a self,
        parts: SecurityGetRoleMappingParts<'b>,
    ) -> SecurityGetRoleMapping<'a, 'b> {
        SecurityGetRoleMapping::new(&self.client, parts)
    }
    #[doc = "[Security Get Token API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-token.html)"]
    pub fn get_token<'b>(&'a self) -> SecurityGetToken<'a, 'b, ()> {
        SecurityGetToken::new(&self.client)
    }
    #[doc = "[Security Get User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-user.html)"]
    pub fn get_user<'b>(&'a self, parts: SecurityGetUserParts<'b>) -> SecurityGetUser<'a, 'b> {
        SecurityGetUser::new(&self.client, parts)
    }
    #[doc = "[Security Get User Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-get-privileges.html)"]
    pub fn get_user_privileges<'b>(&'a self) -> SecurityGetUserPrivileges<'a, 'b> {
        SecurityGetUserPrivileges::new(&self.client)
    }
    #[doc = "[Security Has Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-has-privileges.html)"]
    pub fn has_privileges<'b>(
        &'a self,
        parts: SecurityHasPrivilegesParts<'b>,
    ) -> SecurityHasPrivileges<'a, 'b, ()> {
        SecurityHasPrivileges::new(&self.client, parts)
    }
    #[doc = "[Security Invalidate Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-invalidate-api-key.html)"]
    pub fn invalidate_api_key<'b>(&'a self) -> SecurityInvalidateApiKey<'a, 'b, ()> {
        SecurityInvalidateApiKey::new(&self.client)
    }
    #[doc = "[Security Invalidate Token API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-invalidate-token.html)"]
    pub fn invalidate_token<'b>(&'a self) -> SecurityInvalidateToken<'a, 'b, ()> {
        SecurityInvalidateToken::new(&self.client)
    }
    #[doc = "Security Put Privileges API"]
    pub fn put_privileges<'b>(&'a self) -> SecurityPutPrivileges<'a, 'b, ()> {
        SecurityPutPrivileges::new(&self.client)
    }
    #[doc = "[Security Put Role API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-put-role.html)"]
    pub fn put_role<'b>(&'a self, parts: SecurityPutRoleParts<'b>) -> SecurityPutRole<'a, 'b, ()> {
        SecurityPutRole::new(&self.client, parts)
    }
    #[doc = "[Security Put Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-put-role-mapping.html)"]
    pub fn put_role_mapping<'b>(
        &'a self,
        parts: SecurityPutRoleMappingParts<'b>,
    ) -> SecurityPutRoleMapping<'a, 'b, ()> {
        SecurityPutRoleMapping::new(&self.client, parts)
    }
    #[doc = "[Security Put User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.6/security-api-put-user.html)"]
    pub fn put_user<'b>(&'a self, parts: SecurityPutUserParts<'b>) -> SecurityPutUser<'a, 'b, ()> {
        SecurityPutUser::new(&self.client, parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Security APIs"]
    pub fn security(&self) -> Security {
        Security::new(&self)
    }
}
