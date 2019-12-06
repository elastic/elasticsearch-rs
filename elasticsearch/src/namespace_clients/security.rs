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
    enums::*,
    error::Error,
    http::{
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
};
use serde::Serialize;
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Authenticate API"]
pub enum SecurityAuthenticateParts {
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
#[doc = "Builder for the [Security Authenticate API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-authenticate.html)."]
pub struct SecurityAuthenticate<'a> {
    client: Elasticsearch,
    parts: SecurityAuthenticateParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SecurityAuthenticate<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityAuthenticate {
            client,
            parts: SecurityAuthenticateParts::None,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Authenticate API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Change Password API"]
pub enum SecurityChangePasswordParts<'a> {
    Username(&'a str),
    None,
}
impl<'a> SecurityChangePasswordParts<'a> {
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
#[doc = "Builder for the [Security Change Password API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-change-password.html)."]
pub struct SecurityChangePassword<'a, B> {
    client: Elasticsearch,
    parts: SecurityChangePasswordParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityChangePassword<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: SecurityChangePasswordParts<'a>) -> Self {
        SecurityChangePassword {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> SecurityChangePassword<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityChangePassword {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Change Password API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Clear Cached Realms API"]
pub enum SecurityClearCachedRealmsParts<'a> {
    Realms(&'a [&'a str]),
}
impl<'a> SecurityClearCachedRealmsParts<'a> {
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
#[doc = "Builder for the [Security Clear Cached Realms API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-cache.html)."]
pub struct SecurityClearCachedRealms<'a, B> {
    client: Elasticsearch,
    parts: SecurityClearCachedRealmsParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    usernames: Option<&'a [&'a str]>,
}
impl<'a, B> SecurityClearCachedRealms<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: SecurityClearCachedRealmsParts<'a>) -> Self {
        SecurityClearCachedRealms {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> SecurityClearCachedRealms<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityClearCachedRealms {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Comma-separated list of usernames to clear from the cache"]
    pub fn usernames(mut self, usernames: &'a [&'a str]) -> Self {
        self.usernames = Some(usernames);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Clear Cached Realms API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(
                    rename = "usernames",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                usernames: Option<&'a [&'a str]>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Clear Cached Roles API"]
pub enum SecurityClearCachedRolesParts<'a> {
    Name(&'a [&'a str]),
}
impl<'a> SecurityClearCachedRolesParts<'a> {
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
#[doc = "Builder for the [Security Clear Cached Roles API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-clear-role-cache.html)."]
pub struct SecurityClearCachedRoles<'a, B> {
    client: Elasticsearch,
    parts: SecurityClearCachedRolesParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityClearCachedRoles<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: SecurityClearCachedRolesParts<'a>) -> Self {
        SecurityClearCachedRoles {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityClearCachedRoles<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityClearCachedRoles {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Clear Cached Roles API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Create Api Key API"]
pub enum SecurityCreateApiKeyParts {
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
#[doc = "Builder for the [Security Create Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-create-api-key.html)."]
pub struct SecurityCreateApiKey<'a, B> {
    client: Elasticsearch,
    parts: SecurityCreateApiKeyParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityCreateApiKey<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityCreateApiKey {
            client,
            parts: SecurityCreateApiKeyParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityCreateApiKey<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityCreateApiKey {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Create Api Key API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Delete Privileges API"]
pub enum SecurityDeletePrivilegesParts<'a> {
    ApplicationName(&'a str, &'a str),
}
impl<'a> SecurityDeletePrivilegesParts<'a> {
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
pub struct SecurityDeletePrivileges<'a> {
    client: Elasticsearch,
    parts: SecurityDeletePrivilegesParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a> SecurityDeletePrivileges<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityDeletePrivilegesParts<'a>) -> Self {
        SecurityDeletePrivileges {
            client,
            parts,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Delete Privileges API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Delete Role API"]
pub enum SecurityDeleteRoleParts<'a> {
    Name(&'a str),
}
impl<'a> SecurityDeleteRoleParts<'a> {
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
#[doc = "Builder for the [Security Delete Role API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-role.html)."]
pub struct SecurityDeleteRole<'a> {
    client: Elasticsearch,
    parts: SecurityDeleteRoleParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a> SecurityDeleteRole<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityDeleteRoleParts<'a>) -> Self {
        SecurityDeleteRole {
            client,
            parts,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Delete Role API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Delete Role Mapping API"]
pub enum SecurityDeleteRoleMappingParts<'a> {
    Name(&'a str),
}
impl<'a> SecurityDeleteRoleMappingParts<'a> {
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
#[doc = "Builder for the [Security Delete Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-role-mapping.html)."]
pub struct SecurityDeleteRoleMapping<'a> {
    client: Elasticsearch,
    parts: SecurityDeleteRoleMappingParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a> SecurityDeleteRoleMapping<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityDeleteRoleMappingParts<'a>) -> Self {
        SecurityDeleteRoleMapping {
            client,
            parts,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Delete Role Mapping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Delete User API"]
pub enum SecurityDeleteUserParts<'a> {
    Username(&'a str),
}
impl<'a> SecurityDeleteUserParts<'a> {
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
#[doc = "Builder for the [Security Delete User API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-delete-user.html)."]
pub struct SecurityDeleteUser<'a> {
    client: Elasticsearch,
    parts: SecurityDeleteUserParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a> SecurityDeleteUser<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityDeleteUserParts<'a>) -> Self {
        SecurityDeleteUser {
            client,
            parts,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Delete User API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Disable User API"]
pub enum SecurityDisableUserParts<'a> {
    Username(&'a str),
}
impl<'a> SecurityDisableUserParts<'a> {
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
#[doc = "Builder for the [Security Disable User API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-disable-user.html)."]
pub struct SecurityDisableUser<'a, B> {
    client: Elasticsearch,
    parts: SecurityDisableUserParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityDisableUser<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: SecurityDisableUserParts<'a>) -> Self {
        SecurityDisableUser {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> SecurityDisableUser<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityDisableUser {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Disable User API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Enable User API"]
pub enum SecurityEnableUserParts<'a> {
    Username(&'a str),
}
impl<'a> SecurityEnableUserParts<'a> {
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
#[doc = "Builder for the [Security Enable User API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-enable-user.html)."]
pub struct SecurityEnableUser<'a, B> {
    client: Elasticsearch,
    parts: SecurityEnableUserParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityEnableUser<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: SecurityEnableUserParts<'a>) -> Self {
        SecurityEnableUser {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> SecurityEnableUser<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityEnableUser {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Enable User API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Api Key API"]
pub enum SecurityGetApiKeyParts {
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
#[doc = "Builder for the [Security Get Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-api-key.html)."]
pub struct SecurityGetApiKey<'a> {
    client: Elasticsearch,
    parts: SecurityGetApiKeyParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    id: Option<&'a str>,
    name: Option<&'a str>,
    owner: Option<bool>,
    pretty: Option<bool>,
    realm_name: Option<&'a str>,
    source: Option<&'a str>,
    username: Option<&'a str>,
}
impl<'a> SecurityGetApiKey<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetApiKey {
            client,
            parts: SecurityGetApiKeyParts::None,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "API key id of the API key to be retrieved"]
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }
    #[doc = "API key name of the API key to be retrieved"]
    pub fn name(mut self, name: &'a str) -> Self {
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
    pub fn realm_name(mut self, realm_name: &'a str) -> Self {
        self.realm_name = Some(realm_name);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "user name of the user who created this API key to be retrieved"]
    pub fn username(mut self, username: &'a str) -> Self {
        self.username = Some(username);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Api Key API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "id")]
                id: Option<&'a str>,
                #[serde(rename = "name")]
                name: Option<&'a str>,
                #[serde(rename = "owner")]
                owner: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "realm_name")]
                realm_name: Option<&'a str>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "username")]
                username: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Builtin Privileges API"]
pub enum SecurityGetBuiltinPrivilegesParts {
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
#[doc = "Builder for the [Security Get Builtin Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-builtin-privileges.html)."]
pub struct SecurityGetBuiltinPrivileges<'a> {
    client: Elasticsearch,
    parts: SecurityGetBuiltinPrivilegesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SecurityGetBuiltinPrivileges<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetBuiltinPrivileges {
            client,
            parts: SecurityGetBuiltinPrivilegesParts::None,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Builtin Privileges API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Privileges API"]
pub enum SecurityGetPrivilegesParts<'a> {
    None,
    Application(&'a str),
    ApplicationName(&'a str, &'a str),
}
impl<'a> SecurityGetPrivilegesParts<'a> {
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
#[doc = "Builder for the [Security Get Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-privileges.html)."]
pub struct SecurityGetPrivileges<'a> {
    client: Elasticsearch,
    parts: SecurityGetPrivilegesParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SecurityGetPrivileges<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityGetPrivilegesParts<'a>) -> Self {
        SecurityGetPrivileges {
            client,
            parts,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Privileges API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Role API"]
pub enum SecurityGetRoleParts<'a> {
    Name(&'a str),
    None,
}
impl<'a> SecurityGetRoleParts<'a> {
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
#[doc = "Builder for the [Security Get Role API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-role.html)."]
pub struct SecurityGetRole<'a> {
    client: Elasticsearch,
    parts: SecurityGetRoleParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SecurityGetRole<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityGetRoleParts<'a>) -> Self {
        SecurityGetRole {
            client,
            parts,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Role API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Role Mapping API"]
pub enum SecurityGetRoleMappingParts<'a> {
    Name(&'a str),
    None,
}
impl<'a> SecurityGetRoleMappingParts<'a> {
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
#[doc = "Builder for the [Security Get Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-role-mapping.html)."]
pub struct SecurityGetRoleMapping<'a> {
    client: Elasticsearch,
    parts: SecurityGetRoleMappingParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SecurityGetRoleMapping<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityGetRoleMappingParts<'a>) -> Self {
        SecurityGetRoleMapping {
            client,
            parts,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Role Mapping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Token API"]
pub enum SecurityGetTokenParts {
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
#[doc = "Builder for the [Security Get Token API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-token.html)."]
pub struct SecurityGetToken<'a, B> {
    client: Elasticsearch,
    parts: SecurityGetTokenParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityGetToken<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetToken {
            client,
            parts: SecurityGetTokenParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityGetToken<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityGetToken {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Token API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get User API"]
pub enum SecurityGetUserParts<'a> {
    Username(&'a [&'a str]),
    None,
}
impl<'a> SecurityGetUserParts<'a> {
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
#[doc = "Builder for the [Security Get User API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-user.html)."]
pub struct SecurityGetUser<'a> {
    client: Elasticsearch,
    parts: SecurityGetUserParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SecurityGetUser<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityGetUserParts<'a>) -> Self {
        SecurityGetUser {
            client,
            parts,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get User API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get User Privileges API"]
pub enum SecurityGetUserPrivilegesParts {
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
#[doc = "Builder for the [Security Get User Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-get-privileges.html)."]
pub struct SecurityGetUserPrivileges<'a> {
    client: Elasticsearch,
    parts: SecurityGetUserPrivilegesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SecurityGetUserPrivileges<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetUserPrivileges {
            client,
            parts: SecurityGetUserPrivilegesParts::None,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get User Privileges API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Has Privileges API"]
pub enum SecurityHasPrivilegesParts<'a> {
    None,
    User(&'a str),
}
impl<'a> SecurityHasPrivilegesParts<'a> {
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
#[doc = "Builder for the [Security Has Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-has-privileges.html)."]
pub struct SecurityHasPrivileges<'a, B> {
    client: Elasticsearch,
    parts: SecurityHasPrivilegesParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityHasPrivileges<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: SecurityHasPrivilegesParts<'a>) -> Self {
        SecurityHasPrivileges {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityHasPrivileges<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityHasPrivileges {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
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
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Invalidate Api Key API"]
pub enum SecurityInvalidateApiKeyParts {
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
#[doc = "Builder for the [Security Invalidate Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-api-key.html)."]
pub struct SecurityInvalidateApiKey<'a, B> {
    client: Elasticsearch,
    parts: SecurityInvalidateApiKeyParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityInvalidateApiKey<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityInvalidateApiKey {
            client,
            parts: SecurityInvalidateApiKeyParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityInvalidateApiKey<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityInvalidateApiKey {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Invalidate Api Key API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Invalidate Token API"]
pub enum SecurityInvalidateTokenParts {
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
#[doc = "Builder for the [Security Invalidate Token API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-invalidate-token.html)."]
pub struct SecurityInvalidateToken<'a, B> {
    client: Elasticsearch,
    parts: SecurityInvalidateTokenParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityInvalidateToken<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityInvalidateToken {
            client,
            parts: SecurityInvalidateTokenParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityInvalidateToken<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityInvalidateToken {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Invalidate Token API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Put Privileges API"]
pub enum SecurityPutPrivilegesParts {
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
pub struct SecurityPutPrivileges<'a, B> {
    client: Elasticsearch,
    parts: SecurityPutPrivilegesParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityPutPrivileges<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityPutPrivileges {
            client,
            parts: SecurityPutPrivilegesParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityPutPrivileges<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPutPrivileges {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Put Privileges API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Put Role API"]
pub enum SecurityPutRoleParts<'a> {
    Name(&'a str),
}
impl<'a> SecurityPutRoleParts<'a> {
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
#[doc = "Builder for the [Security Put Role API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role.html)."]
pub struct SecurityPutRole<'a, B> {
    client: Elasticsearch,
    parts: SecurityPutRoleParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityPutRole<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: SecurityPutRoleParts<'a>) -> Self {
        SecurityPutRole {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> SecurityPutRole<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPutRole {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Put Role API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Put Role Mapping API"]
pub enum SecurityPutRoleMappingParts<'a> {
    Name(&'a str),
}
impl<'a> SecurityPutRoleMappingParts<'a> {
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
#[doc = "Builder for the [Security Put Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-role-mapping.html)."]
pub struct SecurityPutRoleMapping<'a, B> {
    client: Elasticsearch,
    parts: SecurityPutRoleMappingParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityPutRoleMapping<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: SecurityPutRoleMappingParts<'a>) -> Self {
        SecurityPutRoleMapping {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> SecurityPutRoleMapping<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPutRoleMapping {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Put Role Mapping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Put User API"]
pub enum SecurityPutUserParts<'a> {
    Username(&'a str),
}
impl<'a> SecurityPutUserParts<'a> {
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
#[doc = "Builder for the [Security Put User API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-put-user.html)."]
pub struct SecurityPutUser<'a, B> {
    client: Elasticsearch,
    parts: SecurityPutUserParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityPutUser<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: SecurityPutUserParts<'a>) -> Self {
        SecurityPutUser {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> SecurityPutUser<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPutUser {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Put User API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "refresh")]
                refresh: Option<Refresh>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Security APIs"]
pub struct Security {
    client: Elasticsearch,
}
impl Security {
    pub fn new(client: Elasticsearch) -> Self {
        Security { client }
    }
    pub fn authenticate<'a>(&self) -> SecurityAuthenticate<'a> {
        SecurityAuthenticate::new(self.client.clone())
    }
    pub fn change_password<'a>(
        &self,
        parts: SecurityChangePasswordParts<'a>,
    ) -> SecurityChangePassword<'a, ()> {
        SecurityChangePassword::new(self.client.clone(), parts)
    }
    pub fn clear_cached_realms<'a>(
        &self,
        parts: SecurityClearCachedRealmsParts<'a>,
    ) -> SecurityClearCachedRealms<'a, ()> {
        SecurityClearCachedRealms::new(self.client.clone(), parts)
    }
    pub fn clear_cached_roles<'a>(
        &self,
        parts: SecurityClearCachedRolesParts<'a>,
    ) -> SecurityClearCachedRoles<'a, ()> {
        SecurityClearCachedRoles::new(self.client.clone(), parts)
    }
    pub fn create_api_key<'a>(&self) -> SecurityCreateApiKey<'a, ()> {
        SecurityCreateApiKey::new(self.client.clone())
    }
    pub fn delete_privileges<'a>(
        &self,
        parts: SecurityDeletePrivilegesParts<'a>,
    ) -> SecurityDeletePrivileges<'a> {
        SecurityDeletePrivileges::new(self.client.clone(), parts)
    }
    pub fn delete_role<'a>(&self, parts: SecurityDeleteRoleParts<'a>) -> SecurityDeleteRole<'a> {
        SecurityDeleteRole::new(self.client.clone(), parts)
    }
    pub fn delete_role_mapping<'a>(
        &self,
        parts: SecurityDeleteRoleMappingParts<'a>,
    ) -> SecurityDeleteRoleMapping<'a> {
        SecurityDeleteRoleMapping::new(self.client.clone(), parts)
    }
    pub fn delete_user<'a>(&self, parts: SecurityDeleteUserParts<'a>) -> SecurityDeleteUser<'a> {
        SecurityDeleteUser::new(self.client.clone(), parts)
    }
    pub fn disable_user<'a>(
        &self,
        parts: SecurityDisableUserParts<'a>,
    ) -> SecurityDisableUser<'a, ()> {
        SecurityDisableUser::new(self.client.clone(), parts)
    }
    pub fn enable_user<'a>(
        &self,
        parts: SecurityEnableUserParts<'a>,
    ) -> SecurityEnableUser<'a, ()> {
        SecurityEnableUser::new(self.client.clone(), parts)
    }
    pub fn get_api_key<'a>(&self) -> SecurityGetApiKey<'a> {
        SecurityGetApiKey::new(self.client.clone())
    }
    pub fn get_builtin_privileges<'a>(&self) -> SecurityGetBuiltinPrivileges<'a> {
        SecurityGetBuiltinPrivileges::new(self.client.clone())
    }
    pub fn get_privileges<'a>(
        &self,
        parts: SecurityGetPrivilegesParts<'a>,
    ) -> SecurityGetPrivileges<'a> {
        SecurityGetPrivileges::new(self.client.clone(), parts)
    }
    pub fn get_role<'a>(&self, parts: SecurityGetRoleParts<'a>) -> SecurityGetRole<'a> {
        SecurityGetRole::new(self.client.clone(), parts)
    }
    pub fn get_role_mapping<'a>(
        &self,
        parts: SecurityGetRoleMappingParts<'a>,
    ) -> SecurityGetRoleMapping<'a> {
        SecurityGetRoleMapping::new(self.client.clone(), parts)
    }
    pub fn get_token<'a>(&self) -> SecurityGetToken<'a, ()> {
        SecurityGetToken::new(self.client.clone())
    }
    pub fn get_user<'a>(&self, parts: SecurityGetUserParts<'a>) -> SecurityGetUser<'a> {
        SecurityGetUser::new(self.client.clone(), parts)
    }
    pub fn get_user_privileges<'a>(&self) -> SecurityGetUserPrivileges<'a> {
        SecurityGetUserPrivileges::new(self.client.clone())
    }
    pub fn has_privileges<'a>(
        &self,
        parts: SecurityHasPrivilegesParts<'a>,
    ) -> SecurityHasPrivileges<'a, ()> {
        SecurityHasPrivileges::new(self.client.clone(), parts)
    }
    pub fn invalidate_api_key<'a>(&self) -> SecurityInvalidateApiKey<'a, ()> {
        SecurityInvalidateApiKey::new(self.client.clone())
    }
    pub fn invalidate_token<'a>(&self) -> SecurityInvalidateToken<'a, ()> {
        SecurityInvalidateToken::new(self.client.clone())
    }
    pub fn put_privileges<'a>(&self) -> SecurityPutPrivileges<'a, ()> {
        SecurityPutPrivileges::new(self.client.clone())
    }
    pub fn put_role<'a>(&self, parts: SecurityPutRoleParts<'a>) -> SecurityPutRole<'a, ()> {
        SecurityPutRole::new(self.client.clone(), parts)
    }
    pub fn put_role_mapping<'a>(
        &self,
        parts: SecurityPutRoleMappingParts<'a>,
    ) -> SecurityPutRoleMapping<'a, ()> {
        SecurityPutRoleMapping::new(self.client.clone(), parts)
    }
    pub fn put_user<'a>(&self, parts: SecurityPutUserParts<'a>) -> SecurityPutUser<'a, ()> {
        SecurityPutUser::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Security APIs"]
    pub fn security(&self) -> Security {
        Security::new(self.clone())
    }
}