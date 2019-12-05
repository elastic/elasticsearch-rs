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
    client::Elasticsearch, enums::*, error::ElasticsearchError, http_method::HttpMethod,
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Security Authenticate API"]
pub enum SecurityAuthenticateUrlParts {
    None,
}
impl SecurityAuthenticateUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityAuthenticateUrlParts::None => "/_security/_authenticate".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Authenticate API"]
pub struct SecurityAuthenticate<'a> {
    client: Elasticsearch,
    parts: SecurityAuthenticateUrlParts,
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
            parts: SecurityAuthenticateUrlParts::None,
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
    #[doc = "Creates an asynchronous request to the Security Authenticate API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
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
#[doc = "Url parts for the Security Change Password API"]
pub enum SecurityChangePasswordUrlParts<'a> {
    Username(&'a str),
    None,
}
impl<'a> SecurityChangePasswordUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityChangePasswordUrlParts::Username(ref username) => {
                let mut p = String::with_capacity(26usize + username.len());
                p.push_str("/_security/user/");
                p.push_str(username.as_ref());
                p.push_str("/_password");
                p.into()
            }
            SecurityChangePasswordUrlParts::None => "/_security/user/_password".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Change Password API"]
pub struct SecurityChangePassword<'a, B> {
    client: Elasticsearch,
    parts: SecurityChangePasswordUrlParts<'a>,
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
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SecurityChangePasswordUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> SecurityChangePassword<'a, T>
    where
        T: Serialize,
    {
        SecurityChangePassword {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Change Password API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
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
#[doc = "Url parts for the Security Clear Cached Realms API"]
pub enum SecurityClearCachedRealmsUrlParts<'a> {
    Realms(&'a [&'a str]),
}
impl<'a> SecurityClearCachedRealmsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityClearCachedRealmsUrlParts::Realms(ref realms) => {
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
#[doc = "Request builder for the Security Clear Cached Realms API"]
pub struct SecurityClearCachedRealms<'a, B> {
    client: Elasticsearch,
    parts: SecurityClearCachedRealmsUrlParts<'a>,
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
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SecurityClearCachedRealmsUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> SecurityClearCachedRealms<'a, T>
    where
        T: Serialize,
    {
        SecurityClearCachedRealms {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Clear Cached Realms API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
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
#[doc = "Url parts for the Security Clear Cached Roles API"]
pub enum SecurityClearCachedRolesUrlParts<'a> {
    Name(&'a [&'a str]),
}
impl<'a> SecurityClearCachedRolesUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityClearCachedRolesUrlParts::Name(ref name) => {
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
#[doc = "Request builder for the Security Clear Cached Roles API"]
pub struct SecurityClearCachedRoles<'a, B> {
    client: Elasticsearch,
    parts: SecurityClearCachedRolesUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityClearCachedRoles<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SecurityClearCachedRolesUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> SecurityClearCachedRoles<'a, T>
    where
        T: Serialize,
    {
        SecurityClearCachedRoles {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Clear Cached Roles API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
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
#[doc = "Url parts for the Security Create Api Key API"]
pub enum SecurityCreateApiKeyUrlParts {
    None,
}
impl SecurityCreateApiKeyUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityCreateApiKeyUrlParts::None => "/_security/api_key".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Create Api Key API"]
pub struct SecurityCreateApiKey<'a, B> {
    client: Elasticsearch,
    parts: SecurityCreateApiKeyUrlParts,
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
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityCreateApiKey {
            client,
            parts: SecurityCreateApiKeyUrlParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityCreateApiKey<'a, T>
    where
        T: Serialize,
    {
        SecurityCreateApiKey {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Create Api Key API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
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
#[doc = "Url parts for the Security Delete Privileges API"]
pub enum SecurityDeletePrivilegesUrlParts<'a> {
    ApplicationName(&'a str, &'a str),
}
impl<'a> SecurityDeletePrivilegesUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityDeletePrivilegesUrlParts::ApplicationName(ref application, ref name) => {
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
#[doc = "Request builder for the Security Delete Privileges API"]
pub struct SecurityDeletePrivileges<'a> {
    client: Elasticsearch,
    parts: SecurityDeletePrivilegesUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a> SecurityDeletePrivileges<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityDeletePrivilegesUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Security Delete Privileges API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
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
#[doc = "Url parts for the Security Delete Role API"]
pub enum SecurityDeleteRoleUrlParts<'a> {
    Name(&'a str),
}
impl<'a> SecurityDeleteRoleUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteRoleUrlParts::Name(ref name) => {
                let mut p = String::with_capacity(16usize + name.len());
                p.push_str("/_security/role/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Delete Role API"]
pub struct SecurityDeleteRole<'a> {
    client: Elasticsearch,
    parts: SecurityDeleteRoleUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a> SecurityDeleteRole<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityDeleteRoleUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Security Delete Role API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
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
#[doc = "Url parts for the Security Delete Role Mapping API"]
pub enum SecurityDeleteRoleMappingUrlParts<'a> {
    Name(&'a str),
}
impl<'a> SecurityDeleteRoleMappingUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteRoleMappingUrlParts::Name(ref name) => {
                let mut p = String::with_capacity(24usize + name.len());
                p.push_str("/_security/role_mapping/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Delete Role Mapping API"]
pub struct SecurityDeleteRoleMapping<'a> {
    client: Elasticsearch,
    parts: SecurityDeleteRoleMappingUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a> SecurityDeleteRoleMapping<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityDeleteRoleMappingUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Security Delete Role Mapping API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
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
#[doc = "Url parts for the Security Delete User API"]
pub enum SecurityDeleteUserUrlParts<'a> {
    Username(&'a str),
}
impl<'a> SecurityDeleteUserUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteUserUrlParts::Username(ref username) => {
                let mut p = String::with_capacity(16usize + username.len());
                p.push_str("/_security/user/");
                p.push_str(username.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Delete User API"]
pub struct SecurityDeleteUser<'a> {
    client: Elasticsearch,
    parts: SecurityDeleteUserUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    source: Option<&'a str>,
}
impl<'a> SecurityDeleteUser<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityDeleteUserUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Security Delete User API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
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
#[doc = "Url parts for the Security Disable User API"]
pub enum SecurityDisableUserUrlParts<'a> {
    Username(&'a str),
}
impl<'a> SecurityDisableUserUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityDisableUserUrlParts::Username(ref username) => {
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
#[doc = "Request builder for the Security Disable User API"]
pub struct SecurityDisableUser<'a, B> {
    client: Elasticsearch,
    parts: SecurityDisableUserUrlParts<'a>,
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
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SecurityDisableUserUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> SecurityDisableUser<'a, T>
    where
        T: Serialize,
    {
        SecurityDisableUser {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Disable User API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
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
#[doc = "Url parts for the Security Enable User API"]
pub enum SecurityEnableUserUrlParts<'a> {
    Username(&'a str),
}
impl<'a> SecurityEnableUserUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityEnableUserUrlParts::Username(ref username) => {
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
#[doc = "Request builder for the Security Enable User API"]
pub struct SecurityEnableUser<'a, B> {
    client: Elasticsearch,
    parts: SecurityEnableUserUrlParts<'a>,
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
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SecurityEnableUserUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> SecurityEnableUser<'a, T>
    where
        T: Serialize,
    {
        SecurityEnableUser {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Enable User API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
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
#[doc = "Url parts for the Security Get Api Key API"]
pub enum SecurityGetApiKeyUrlParts {
    None,
}
impl SecurityGetApiKeyUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityGetApiKeyUrlParts::None => "/_security/api_key".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Get Api Key API"]
pub struct SecurityGetApiKey<'a> {
    client: Elasticsearch,
    parts: SecurityGetApiKeyUrlParts,
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
            parts: SecurityGetApiKeyUrlParts::None,
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
    #[doc = "Creates an asynchronous request to the Security Get Api Key API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
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
#[doc = "Url parts for the Security Get Builtin Privileges API"]
pub enum SecurityGetBuiltinPrivilegesUrlParts {
    None,
}
impl SecurityGetBuiltinPrivilegesUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityGetBuiltinPrivilegesUrlParts::None => "/_security/privilege/_builtin".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Get Builtin Privileges API"]
pub struct SecurityGetBuiltinPrivileges<'a> {
    client: Elasticsearch,
    parts: SecurityGetBuiltinPrivilegesUrlParts,
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
            parts: SecurityGetBuiltinPrivilegesUrlParts::None,
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
    #[doc = "Creates an asynchronous request to the Security Get Builtin Privileges API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
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
#[doc = "Url parts for the Security Get Privileges API"]
pub enum SecurityGetPrivilegesUrlParts<'a> {
    None,
    Application(&'a str),
    ApplicationName(&'a str, &'a str),
}
impl<'a> SecurityGetPrivilegesUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityGetPrivilegesUrlParts::None => "/_security/privilege".into(),
            SecurityGetPrivilegesUrlParts::Application(ref application) => {
                let mut p = String::with_capacity(21usize + application.len());
                p.push_str("/_security/privilege/");
                p.push_str(application.as_ref());
                p.into()
            }
            SecurityGetPrivilegesUrlParts::ApplicationName(ref application, ref name) => {
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
#[doc = "Request builder for the Security Get Privileges API"]
pub struct SecurityGetPrivileges<'a> {
    client: Elasticsearch,
    parts: SecurityGetPrivilegesUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SecurityGetPrivileges<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityGetPrivilegesUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Security Get Privileges API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
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
#[doc = "Url parts for the Security Get Role API"]
pub enum SecurityGetRoleUrlParts<'a> {
    Name(&'a str),
    None,
}
impl<'a> SecurityGetRoleUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityGetRoleUrlParts::Name(ref name) => {
                let mut p = String::with_capacity(16usize + name.len());
                p.push_str("/_security/role/");
                p.push_str(name.as_ref());
                p.into()
            }
            SecurityGetRoleUrlParts::None => "/_security/role".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Get Role API"]
pub struct SecurityGetRole<'a> {
    client: Elasticsearch,
    parts: SecurityGetRoleUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SecurityGetRole<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityGetRoleUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Security Get Role API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
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
#[doc = "Url parts for the Security Get Role Mapping API"]
pub enum SecurityGetRoleMappingUrlParts<'a> {
    Name(&'a str),
    None,
}
impl<'a> SecurityGetRoleMappingUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityGetRoleMappingUrlParts::Name(ref name) => {
                let mut p = String::with_capacity(24usize + name.len());
                p.push_str("/_security/role_mapping/");
                p.push_str(name.as_ref());
                p.into()
            }
            SecurityGetRoleMappingUrlParts::None => "/_security/role_mapping".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Get Role Mapping API"]
pub struct SecurityGetRoleMapping<'a> {
    client: Elasticsearch,
    parts: SecurityGetRoleMappingUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SecurityGetRoleMapping<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityGetRoleMappingUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Security Get Role Mapping API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
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
#[doc = "Url parts for the Security Get Token API"]
pub enum SecurityGetTokenUrlParts {
    None,
}
impl SecurityGetTokenUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityGetTokenUrlParts::None => "/_security/oauth2/token".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Get Token API"]
pub struct SecurityGetToken<'a, B> {
    client: Elasticsearch,
    parts: SecurityGetTokenUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityGetToken<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityGetToken {
            client,
            parts: SecurityGetTokenUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityGetToken<'a, T>
    where
        T: Serialize,
    {
        SecurityGetToken {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Get Token API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
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
#[doc = "Url parts for the Security Get User API"]
pub enum SecurityGetUserUrlParts<'a> {
    Username(&'a [&'a str]),
    None,
}
impl<'a> SecurityGetUserUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityGetUserUrlParts::Username(ref username) => {
                let username_str = username.join(",");
                let mut p = String::with_capacity(16usize + username_str.len());
                p.push_str("/_security/user/");
                p.push_str(username_str.as_ref());
                p.into()
            }
            SecurityGetUserUrlParts::None => "/_security/user".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Get User API"]
pub struct SecurityGetUser<'a> {
    client: Elasticsearch,
    parts: SecurityGetUserUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SecurityGetUser<'a> {
    pub fn new(client: Elasticsearch, parts: SecurityGetUserUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Security Get User API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
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
#[doc = "Url parts for the Security Get User Privileges API"]
pub enum SecurityGetUserPrivilegesUrlParts {
    None,
}
impl SecurityGetUserPrivilegesUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityGetUserPrivilegesUrlParts::None => "/_security/user/_privileges".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Get User Privileges API"]
pub struct SecurityGetUserPrivileges<'a> {
    client: Elasticsearch,
    parts: SecurityGetUserPrivilegesUrlParts,
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
            parts: SecurityGetUserPrivilegesUrlParts::None,
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
    #[doc = "Creates an asynchronous request to the Security Get User Privileges API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
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
#[doc = "Url parts for the Security Has Privileges API"]
pub enum SecurityHasPrivilegesUrlParts<'a> {
    None,
    User(&'a str),
}
impl<'a> SecurityHasPrivilegesUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityHasPrivilegesUrlParts::None => "/_security/user/_has_privileges".into(),
            SecurityHasPrivilegesUrlParts::User(ref user) => {
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
#[doc = "Request builder for the Security Has Privileges API"]
pub struct SecurityHasPrivileges<'a, B> {
    client: Elasticsearch,
    parts: SecurityHasPrivilegesUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityHasPrivileges<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SecurityHasPrivilegesUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> SecurityHasPrivileges<'a, T>
    where
        T: Serialize,
    {
        SecurityHasPrivileges {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Has Privileges API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
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
#[doc = "Url parts for the Security Invalidate Api Key API"]
pub enum SecurityInvalidateApiKeyUrlParts {
    None,
}
impl SecurityInvalidateApiKeyUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityInvalidateApiKeyUrlParts::None => "/_security/api_key".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Invalidate Api Key API"]
pub struct SecurityInvalidateApiKey<'a, B> {
    client: Elasticsearch,
    parts: SecurityInvalidateApiKeyUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityInvalidateApiKey<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityInvalidateApiKey {
            client,
            parts: SecurityInvalidateApiKeyUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityInvalidateApiKey<'a, T>
    where
        T: Serialize,
    {
        SecurityInvalidateApiKey {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Invalidate Api Key API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
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
#[doc = "Url parts for the Security Invalidate Token API"]
pub enum SecurityInvalidateTokenUrlParts {
    None,
}
impl SecurityInvalidateTokenUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityInvalidateTokenUrlParts::None => "/_security/oauth2/token".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Invalidate Token API"]
pub struct SecurityInvalidateToken<'a, B> {
    client: Elasticsearch,
    parts: SecurityInvalidateTokenUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> SecurityInvalidateToken<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityInvalidateToken {
            client,
            parts: SecurityInvalidateTokenUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityInvalidateToken<'a, T>
    where
        T: Serialize,
    {
        SecurityInvalidateToken {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Invalidate Token API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
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
#[doc = "Url parts for the Security Put Privileges API"]
pub enum SecurityPutPrivilegesUrlParts {
    None,
}
impl SecurityPutPrivilegesUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityPutPrivilegesUrlParts::None => "/_security/privilege/".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Put Privileges API"]
pub struct SecurityPutPrivileges<'a, B> {
    client: Elasticsearch,
    parts: SecurityPutPrivilegesUrlParts,
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
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        SecurityPutPrivileges {
            client,
            parts: SecurityPutPrivilegesUrlParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityPutPrivileges<'a, T>
    where
        T: Serialize,
    {
        SecurityPutPrivileges {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Put Privileges API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
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
#[doc = "Url parts for the Security Put Role API"]
pub enum SecurityPutRoleUrlParts<'a> {
    Name(&'a str),
}
impl<'a> SecurityPutRoleUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityPutRoleUrlParts::Name(ref name) => {
                let mut p = String::with_capacity(16usize + name.len());
                p.push_str("/_security/role/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Put Role API"]
pub struct SecurityPutRole<'a, B> {
    client: Elasticsearch,
    parts: SecurityPutRoleUrlParts<'a>,
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
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SecurityPutRoleUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> SecurityPutRole<'a, T>
    where
        T: Serialize,
    {
        SecurityPutRole {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Put Role API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
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
#[doc = "Url parts for the Security Put Role Mapping API"]
pub enum SecurityPutRoleMappingUrlParts<'a> {
    Name(&'a str),
}
impl<'a> SecurityPutRoleMappingUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityPutRoleMappingUrlParts::Name(ref name) => {
                let mut p = String::with_capacity(24usize + name.len());
                p.push_str("/_security/role_mapping/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Put Role Mapping API"]
pub struct SecurityPutRoleMapping<'a, B> {
    client: Elasticsearch,
    parts: SecurityPutRoleMappingUrlParts<'a>,
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
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SecurityPutRoleMappingUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> SecurityPutRoleMapping<'a, T>
    where
        T: Serialize,
    {
        SecurityPutRoleMapping {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Put Role Mapping API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
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
#[doc = "Url parts for the Security Put User API"]
pub enum SecurityPutUserUrlParts<'a> {
    Username(&'a str),
}
impl<'a> SecurityPutUserUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            SecurityPutUserUrlParts::Username(ref username) => {
                let mut p = String::with_capacity(16usize + username.len());
                p.push_str("/_security/user/");
                p.push_str(username.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Security Put User API"]
pub struct SecurityPutUser<'a, B> {
    client: Elasticsearch,
    parts: SecurityPutUserUrlParts<'a>,
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
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: SecurityPutUserUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> SecurityPutUser<'a, T>
    where
        T: Serialize,
    {
        SecurityPutUser {
            client: self.client,
            parts: self.parts,
            body: Some(body),
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
    #[doc = "Creates an asynchronous request to the Security Put User API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
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
#[doc = "Security APIs"]
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
        parts: SecurityChangePasswordUrlParts<'a>,
    ) -> SecurityChangePassword<'a, ()> {
        SecurityChangePassword::new(self.client.clone(), parts)
    }
    pub fn clear_cached_realms<'a>(
        &self,
        parts: SecurityClearCachedRealmsUrlParts<'a>,
    ) -> SecurityClearCachedRealms<'a, ()> {
        SecurityClearCachedRealms::new(self.client.clone(), parts)
    }
    pub fn clear_cached_roles<'a>(
        &self,
        parts: SecurityClearCachedRolesUrlParts<'a>,
    ) -> SecurityClearCachedRoles<'a, ()> {
        SecurityClearCachedRoles::new(self.client.clone(), parts)
    }
    pub fn create_api_key<'a>(&self) -> SecurityCreateApiKey<'a, ()> {
        SecurityCreateApiKey::new(self.client.clone())
    }
    pub fn delete_privileges<'a>(
        &self,
        parts: SecurityDeletePrivilegesUrlParts<'a>,
    ) -> SecurityDeletePrivileges<'a> {
        SecurityDeletePrivileges::new(self.client.clone(), parts)
    }
    pub fn delete_role<'a>(&self, parts: SecurityDeleteRoleUrlParts<'a>) -> SecurityDeleteRole<'a> {
        SecurityDeleteRole::new(self.client.clone(), parts)
    }
    pub fn delete_role_mapping<'a>(
        &self,
        parts: SecurityDeleteRoleMappingUrlParts<'a>,
    ) -> SecurityDeleteRoleMapping<'a> {
        SecurityDeleteRoleMapping::new(self.client.clone(), parts)
    }
    pub fn delete_user<'a>(&self, parts: SecurityDeleteUserUrlParts<'a>) -> SecurityDeleteUser<'a> {
        SecurityDeleteUser::new(self.client.clone(), parts)
    }
    pub fn disable_user<'a>(
        &self,
        parts: SecurityDisableUserUrlParts<'a>,
    ) -> SecurityDisableUser<'a, ()> {
        SecurityDisableUser::new(self.client.clone(), parts)
    }
    pub fn enable_user<'a>(
        &self,
        parts: SecurityEnableUserUrlParts<'a>,
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
        parts: SecurityGetPrivilegesUrlParts<'a>,
    ) -> SecurityGetPrivileges<'a> {
        SecurityGetPrivileges::new(self.client.clone(), parts)
    }
    pub fn get_role<'a>(&self, parts: SecurityGetRoleUrlParts<'a>) -> SecurityGetRole<'a> {
        SecurityGetRole::new(self.client.clone(), parts)
    }
    pub fn get_role_mapping<'a>(
        &self,
        parts: SecurityGetRoleMappingUrlParts<'a>,
    ) -> SecurityGetRoleMapping<'a> {
        SecurityGetRoleMapping::new(self.client.clone(), parts)
    }
    pub fn get_token<'a>(&self) -> SecurityGetToken<'a, ()> {
        SecurityGetToken::new(self.client.clone())
    }
    pub fn get_user<'a>(&self, parts: SecurityGetUserUrlParts<'a>) -> SecurityGetUser<'a> {
        SecurityGetUser::new(self.client.clone(), parts)
    }
    pub fn get_user_privileges<'a>(&self) -> SecurityGetUserPrivileges<'a> {
        SecurityGetUserPrivileges::new(self.client.clone())
    }
    pub fn has_privileges<'a>(
        &self,
        parts: SecurityHasPrivilegesUrlParts<'a>,
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
    pub fn put_role<'a>(&self, parts: SecurityPutRoleUrlParts<'a>) -> SecurityPutRole<'a, ()> {
        SecurityPutRole::new(self.client.clone(), parts)
    }
    pub fn put_role_mapping<'a>(
        &self,
        parts: SecurityPutRoleMappingUrlParts<'a>,
    ) -> SecurityPutRoleMapping<'a, ()> {
        SecurityPutRoleMapping::new(self.client.clone(), parts)
    }
    pub fn put_user<'a>(&self, parts: SecurityPutUserUrlParts<'a>) -> SecurityPutUser<'a, ()> {
        SecurityPutUser::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Security APIs"]
    pub fn security(&self) -> Security {
        Security::new(self.clone())
    }
}