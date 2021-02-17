/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

// -----------------------------------------------
// This file is generated, Please do not edit it manually.
// Run the following in the root of the repo to regenerate:
//
// cargo make generate-api
// -----------------------------------------------

//! Security APIs
//!
//! [Perform security related activities](https://www.elastic.co/guide/en/elasticsearch/reference/master/security-api.html), including
//!
//! - Manage users
//! - Manage application privileges
//! - Manage roles
//! - Manage role mappings
//! - Manage API keys
//! - Manage Bearer tokens
//! - Authenticate users against an OpenID Connect or SAML authentication realm when using a
//! custom web application other than Kibana

#![allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        transport::Transport,
        Method,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::{borrow::Cow, time::Duration};
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
#[doc = "Builder for the [Security Authenticate API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-authenticate.html)\n\nEnables authentication as a user and retrieve information about the authenticated user."]
#[derive(Clone, Debug)]
pub struct SecurityAuthenticate<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityAuthenticateParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityAuthenticate<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityAuthenticate]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityAuthenticate {
            transport,
            parts: SecurityAuthenticateParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_username: Cow<str> =
                    percent_encode(username.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(26usize + encoded_username.len());
                p.push_str("/_security/user/");
                p.push_str(encoded_username.as_ref());
                p.push_str("/_password");
                p.into()
            }
            SecurityChangePasswordParts::None => "/_security/user/_password".into(),
        }
    }
}
#[doc = "Builder for the [Security Change Password API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-change-password.html)\n\nChanges the passwords of users in the native realm and built-in users."]
#[derive(Clone, Debug)]
pub struct SecurityChangePassword<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityChangePasswordParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityChangePassword<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityChangePassword] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityChangePasswordParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityChangePassword {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityChangePassword<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityChangePassword {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Clear Api Key Cache API"]
pub enum SecurityClearApiKeyCacheParts<'b> {
    #[doc = "Ids"]
    Ids(&'b [&'b str]),
}
impl<'b> SecurityClearApiKeyCacheParts<'b> {
    #[doc = "Builds a relative URL path to the Security Clear Api Key Cache API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityClearApiKeyCacheParts::Ids(ref ids) => {
                let ids_str = ids.join(",");
                let encoded_ids: Cow<str> =
                    percent_encode(ids_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_ids.len());
                p.push_str("/_security/api_key/");
                p.push_str(encoded_ids.as_ref());
                p.push_str("/_clear_cache");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Clear Api Key Cache API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-clear-api-key-cache.html)\n\nClear a subset or all entries from the API key cache."]
#[derive(Clone, Debug)]
pub struct SecurityClearApiKeyCache<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityClearApiKeyCacheParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityClearApiKeyCache<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityClearApiKeyCache] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityClearApiKeyCacheParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityClearApiKeyCache {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityClearApiKeyCache<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityClearApiKeyCache {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Clear Api Key Cache API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Clear Cached Privileges API"]
pub enum SecurityClearCachedPrivilegesParts<'b> {
    #[doc = "Application"]
    Application(&'b [&'b str]),
}
impl<'b> SecurityClearCachedPrivilegesParts<'b> {
    #[doc = "Builds a relative URL path to the Security Clear Cached Privileges API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityClearCachedPrivilegesParts::Application(ref application) => {
                let application_str = application.join(",");
                let encoded_application: Cow<str> =
                    percent_encode(application_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(34usize + encoded_application.len());
                p.push_str("/_security/privilege/");
                p.push_str(encoded_application.as_ref());
                p.push_str("/_clear_cache");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Clear Cached Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-clear-privilege-cache.html)\n\nEvicts application privileges from the native application privileges cache."]
#[derive(Clone, Debug)]
pub struct SecurityClearCachedPrivileges<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityClearCachedPrivilegesParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityClearCachedPrivileges<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityClearCachedPrivileges] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityClearCachedPrivilegesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityClearCachedPrivileges {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityClearCachedPrivileges<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityClearCachedPrivileges {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Clear Cached Privileges API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_realms: Cow<str> =
                    percent_encode(realms_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_realms.len());
                p.push_str("/_security/realm/");
                p.push_str(encoded_realms.as_ref());
                p.push_str("/_clear_cache");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Clear Cached Realms API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-clear-cache.html)\n\nEvicts users from the user cache. Can completely clear the cache or evict specific users."]
#[derive(Clone, Debug)]
pub struct SecurityClearCachedRealms<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityClearCachedRealmsParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    usernames: Option<&'b [&'b str]>,
}
impl<'a, 'b, B> SecurityClearCachedRealms<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityClearCachedRealms] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityClearCachedRealmsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityClearCachedRealms {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
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
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_name.len());
                p.push_str("/_security/role/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/_clear_cache");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Clear Cached Roles API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-clear-role-cache.html)\n\nEvicts roles from the native role cache."]
#[derive(Clone, Debug)]
pub struct SecurityClearCachedRoles<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityClearCachedRolesParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityClearCachedRoles<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityClearCachedRoles] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityClearCachedRolesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityClearCachedRoles {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityClearCachedRoles<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityClearCachedRoles {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
#[doc = "Builder for the [Security Create Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-create-api-key.html)\n\nCreates an API key for access without requiring basic authentication."]
#[derive(Clone, Debug)]
pub struct SecurityCreateApiKey<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityCreateApiKeyParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityCreateApiKey<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityCreateApiKey]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityCreateApiKey {
            transport,
            parts: SecurityCreateApiKeyParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityCreateApiKey<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityCreateApiKey {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_application: Cow<str> =
                    percent_encode(application.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(22usize + encoded_application.len() + encoded_name.len());
                p.push_str("/_security/privilege/");
                p.push_str(encoded_application.as_ref());
                p.push_str("/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-delete-privilege.html)\n\nRemoves application privileges."]
#[derive(Clone, Debug)]
pub struct SecurityDeletePrivileges<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityDeletePrivilegesParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeletePrivileges<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeletePrivileges] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDeletePrivilegesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeletePrivileges {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_name.len());
                p.push_str("/_security/role/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete Role API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-delete-role.html)\n\nRemoves roles in the native realm."]
#[derive(Clone, Debug)]
pub struct SecurityDeleteRole<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityDeleteRoleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteRole<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteRole] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDeleteRoleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteRole {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_name.len());
                p.push_str("/_security/role_mapping/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-delete-role-mapping.html)\n\nRemoves role mappings."]
#[derive(Clone, Debug)]
pub struct SecurityDeleteRoleMapping<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityDeleteRoleMappingParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteRoleMapping<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteRoleMapping] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDeleteRoleMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteRoleMapping {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_username: Cow<str> =
                    percent_encode(username.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_username.len());
                p.push_str("/_security/user/");
                p.push_str(encoded_username.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-delete-user.html)\n\nDeletes users from the native realm."]
#[derive(Clone, Debug)]
pub struct SecurityDeleteUser<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityDeleteUserParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteUser<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteUser] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDeleteUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteUser {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_username: Cow<str> =
                    percent_encode(username.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(25usize + encoded_username.len());
                p.push_str("/_security/user/");
                p.push_str(encoded_username.as_ref());
                p.push_str("/_disable");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Disable User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-disable-user.html)\n\nDisables users in the native realm."]
#[derive(Clone, Debug)]
pub struct SecurityDisableUser<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityDisableUserParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityDisableUser<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityDisableUser] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDisableUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDisableUser {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityDisableUser<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityDisableUser {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_username: Cow<str> =
                    percent_encode(username.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_username.len());
                p.push_str("/_security/user/");
                p.push_str(encoded_username.as_ref());
                p.push_str("/_enable");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Enable User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-enable-user.html)\n\nEnables users in the native realm."]
#[derive(Clone, Debug)]
pub struct SecurityEnableUser<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityEnableUserParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityEnableUser<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityEnableUser] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityEnableUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityEnableUser {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityEnableUser<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityEnableUser {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
#[doc = "Builder for the [Security Get Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-api-key.html)\n\nRetrieves information for one or more API keys."]
#[derive(Clone, Debug)]
pub struct SecurityGetApiKey<'a, 'b> {
    transport: &'a Transport,
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
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    username: Option<&'b str>,
}
impl<'a, 'b> SecurityGetApiKey<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetApiKey]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetApiKey {
            transport,
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
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                id: Option<&'b str>,
                name: Option<&'b str>,
                owner: Option<bool>,
                pretty: Option<bool>,
                realm_name: Option<&'b str>,
                source: Option<&'b str>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
#[doc = "Builder for the [Security Get Builtin Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-builtin-privileges.html)\n\nRetrieves the list of cluster privileges and index privileges that are available in this version of Elasticsearch."]
#[derive(Clone, Debug)]
pub struct SecurityGetBuiltinPrivileges<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetBuiltinPrivilegesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetBuiltinPrivileges<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetBuiltinPrivileges]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetBuiltinPrivileges {
            transport,
            parts: SecurityGetBuiltinPrivilegesParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_application: Cow<str> =
                    percent_encode(application.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_application.len());
                p.push_str("/_security/privilege/");
                p.push_str(encoded_application.as_ref());
                p.into()
            }
            SecurityGetPrivilegesParts::ApplicationName(ref application, ref name) => {
                let encoded_application: Cow<str> =
                    percent_encode(application.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(22usize + encoded_application.len() + encoded_name.len());
                p.push_str("/_security/privilege/");
                p.push_str(encoded_application.as_ref());
                p.push_str("/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Get Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-privileges.html)\n\nRetrieves application privileges."]
#[derive(Clone, Debug)]
pub struct SecurityGetPrivileges<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetPrivilegesParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetPrivileges<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetPrivileges] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetPrivilegesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetPrivileges {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Role API"]
pub enum SecurityGetRoleParts<'b> {
    #[doc = "Name"]
    Name(&'b [&'b str]),
    #[doc = "No parts"]
    None,
}
impl<'b> SecurityGetRoleParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get Role API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetRoleParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_name.len());
                p.push_str("/_security/role/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
            SecurityGetRoleParts::None => "/_security/role".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Role API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-role.html)\n\nRetrieves roles in the native realm."]
#[derive(Clone, Debug)]
pub struct SecurityGetRole<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetRoleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetRole<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetRole] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetRoleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetRole {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Get Role Mapping API"]
pub enum SecurityGetRoleMappingParts<'b> {
    #[doc = "Name"]
    Name(&'b [&'b str]),
    #[doc = "No parts"]
    None,
}
impl<'b> SecurityGetRoleMappingParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get Role Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetRoleMappingParts::Name(ref name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_name.len());
                p.push_str("/_security/role_mapping/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
            SecurityGetRoleMappingParts::None => "/_security/role_mapping".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-role-mapping.html)\n\nRetrieves role mappings."]
#[derive(Clone, Debug)]
pub struct SecurityGetRoleMapping<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetRoleMappingParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetRoleMapping<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetRoleMapping] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetRoleMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetRoleMapping {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
#[doc = "Builder for the [Security Get Token API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-token.html)\n\nCreates a bearer token for access without requiring basic authentication."]
#[derive(Clone, Debug)]
pub struct SecurityGetToken<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityGetTokenParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityGetToken<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityGetToken]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetToken {
            transport,
            parts: SecurityGetTokenParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityGetToken<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityGetToken {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_username: Cow<str> =
                    percent_encode(username_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_username.len());
                p.push_str("/_security/user/");
                p.push_str(encoded_username.as_ref());
                p.into()
            }
            SecurityGetUserParts::None => "/_security/user".into(),
        }
    }
}
#[doc = "Builder for the [Security Get User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-user.html)\n\nRetrieves information about users in the native realm and built-in users."]
#[derive(Clone, Debug)]
pub struct SecurityGetUser<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetUserParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetUser<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetUser] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetUser {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
#[doc = "Builder for the [Security Get User Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-privileges.html)\n\nRetrieves application privileges."]
#[derive(Clone, Debug)]
pub struct SecurityGetUserPrivileges<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetUserPrivilegesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetUserPrivileges<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetUserPrivileges]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetUserPrivileges {
            transport,
            parts: SecurityGetUserPrivilegesParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Security Grant Api Key API"]
pub enum SecurityGrantApiKeyParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGrantApiKeyParts {
    #[doc = "Builds a relative URL path to the Security Grant Api Key API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGrantApiKeyParts::None => "/_security/api_key/grant".into(),
        }
    }
}
#[doc = "Builder for the [Security Grant Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-grant-api-key.html)\n\nCreates an API key on behalf of another user."]
#[derive(Clone, Debug)]
pub struct SecurityGrantApiKey<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityGrantApiKeyParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityGrantApiKey<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityGrantApiKey]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGrantApiKey {
            transport,
            parts: SecurityGrantApiKeyParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityGrantApiKey<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityGrantApiKey {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Grant Api Key API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_user: Cow<str> = percent_encode(user.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_user.len());
                p.push_str("/_security/user/");
                p.push_str(encoded_user.as_ref());
                p.push_str("/_has_privileges");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Has Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-has-privileges.html)\n\nDetermines whether the specified user has a specified list of privileges."]
#[derive(Clone, Debug)]
pub struct SecurityHasPrivileges<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityHasPrivilegesParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityHasPrivileges<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityHasPrivileges] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityHasPrivilegesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityHasPrivileges {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityHasPrivileges<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityHasPrivileges {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
#[doc = "Builder for the [Security Invalidate Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-invalidate-api-key.html)\n\nInvalidates one or more API keys."]
#[derive(Clone, Debug)]
pub struct SecurityInvalidateApiKey<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityInvalidateApiKeyParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityInvalidateApiKey<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityInvalidateApiKey]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityInvalidateApiKey {
            transport,
            parts: SecurityInvalidateApiKeyParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityInvalidateApiKey<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityInvalidateApiKey {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
#[doc = "Builder for the [Security Invalidate Token API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-invalidate-token.html)\n\nInvalidates one or more access tokens or refresh tokens."]
#[derive(Clone, Debug)]
pub struct SecurityInvalidateToken<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityInvalidateTokenParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityInvalidateToken<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityInvalidateToken]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityInvalidateToken {
            transport,
            parts: SecurityInvalidateTokenParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityInvalidateToken<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityInvalidateToken {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
#[doc = "Builder for the [Security Put Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-put-privileges.html)\n\nAdds or updates application privileges."]
#[derive(Clone, Debug)]
pub struct SecurityPutPrivileges<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPutPrivilegesParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPutPrivileges<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPutPrivileges]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityPutPrivileges {
            transport,
            parts: SecurityPutPrivilegesParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityPutPrivileges<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPutPrivileges {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_name.len());
                p.push_str("/_security/role/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Put Role API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-put-role.html)\n\nAdds and updates roles in the native realm."]
#[derive(Clone, Debug)]
pub struct SecurityPutRole<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPutRoleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPutRole<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPutRole] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityPutRoleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityPutRole {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityPutRole<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPutRole {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_name.len());
                p.push_str("/_security/role_mapping/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Put Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-put-role-mapping.html)\n\nCreates and updates role mappings."]
#[derive(Clone, Debug)]
pub struct SecurityPutRoleMapping<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPutRoleMappingParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPutRoleMapping<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPutRoleMapping] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityPutRoleMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityPutRoleMapping {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityPutRoleMapping<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPutRoleMapping {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
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
                let encoded_username: Cow<str> =
                    percent_encode(username.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_username.len());
                p.push_str("/_security/user/");
                p.push_str(encoded_username.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Put User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-put-user.html)\n\nAdds and updates users in the native realm. These users are commonly referred to as native users."]
#[derive(Clone, Debug)]
pub struct SecurityPutUser<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPutUserParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPutUser<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPutUser] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityPutUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityPutUser {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityPutUser<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPutUser {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
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
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
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
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Security APIs"]
pub struct Security<'a> {
    transport: &'a Transport,
}
impl<'a> Security<'a> {
    #[doc = "Creates a new instance of [Security]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Security Authenticate API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-authenticate.html)\n\nEnables authentication as a user and retrieve information about the authenticated user."]
    pub fn authenticate<'b>(&'a self) -> SecurityAuthenticate<'a, 'b> {
        SecurityAuthenticate::new(self.transport())
    }
    #[doc = "[Security Change Password API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-change-password.html)\n\nChanges the passwords of users in the native realm and built-in users."]
    pub fn change_password<'b>(
        &'a self,
        parts: SecurityChangePasswordParts<'b>,
    ) -> SecurityChangePassword<'a, 'b, ()> {
        SecurityChangePassword::new(self.transport(), parts)
    }
    #[doc = "[Security Clear Api Key Cache API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-clear-api-key-cache.html)\n\nClear a subset or all entries from the API key cache."]
    pub fn clear_api_key_cache<'b>(
        &'a self,
        parts: SecurityClearApiKeyCacheParts<'b>,
    ) -> SecurityClearApiKeyCache<'a, 'b, ()> {
        SecurityClearApiKeyCache::new(self.transport(), parts)
    }
    #[doc = "[Security Clear Cached Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-clear-privilege-cache.html)\n\nEvicts application privileges from the native application privileges cache."]
    pub fn clear_cached_privileges<'b>(
        &'a self,
        parts: SecurityClearCachedPrivilegesParts<'b>,
    ) -> SecurityClearCachedPrivileges<'a, 'b, ()> {
        SecurityClearCachedPrivileges::new(self.transport(), parts)
    }
    #[doc = "[Security Clear Cached Realms API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-clear-cache.html)\n\nEvicts users from the user cache. Can completely clear the cache or evict specific users."]
    pub fn clear_cached_realms<'b>(
        &'a self,
        parts: SecurityClearCachedRealmsParts<'b>,
    ) -> SecurityClearCachedRealms<'a, 'b, ()> {
        SecurityClearCachedRealms::new(self.transport(), parts)
    }
    #[doc = "[Security Clear Cached Roles API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-clear-role-cache.html)\n\nEvicts roles from the native role cache."]
    pub fn clear_cached_roles<'b>(
        &'a self,
        parts: SecurityClearCachedRolesParts<'b>,
    ) -> SecurityClearCachedRoles<'a, 'b, ()> {
        SecurityClearCachedRoles::new(self.transport(), parts)
    }
    #[doc = "[Security Create Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-create-api-key.html)\n\nCreates an API key for access without requiring basic authentication."]
    pub fn create_api_key<'b>(&'a self) -> SecurityCreateApiKey<'a, 'b, ()> {
        SecurityCreateApiKey::new(self.transport())
    }
    #[doc = "[Security Delete Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-delete-privilege.html)\n\nRemoves application privileges."]
    pub fn delete_privileges<'b>(
        &'a self,
        parts: SecurityDeletePrivilegesParts<'b>,
    ) -> SecurityDeletePrivileges<'a, 'b> {
        SecurityDeletePrivileges::new(self.transport(), parts)
    }
    #[doc = "[Security Delete Role API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-delete-role.html)\n\nRemoves roles in the native realm."]
    pub fn delete_role<'b>(
        &'a self,
        parts: SecurityDeleteRoleParts<'b>,
    ) -> SecurityDeleteRole<'a, 'b> {
        SecurityDeleteRole::new(self.transport(), parts)
    }
    #[doc = "[Security Delete Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-delete-role-mapping.html)\n\nRemoves role mappings."]
    pub fn delete_role_mapping<'b>(
        &'a self,
        parts: SecurityDeleteRoleMappingParts<'b>,
    ) -> SecurityDeleteRoleMapping<'a, 'b> {
        SecurityDeleteRoleMapping::new(self.transport(), parts)
    }
    #[doc = "[Security Delete User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-delete-user.html)\n\nDeletes users from the native realm."]
    pub fn delete_user<'b>(
        &'a self,
        parts: SecurityDeleteUserParts<'b>,
    ) -> SecurityDeleteUser<'a, 'b> {
        SecurityDeleteUser::new(self.transport(), parts)
    }
    #[doc = "[Security Disable User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-disable-user.html)\n\nDisables users in the native realm."]
    pub fn disable_user<'b>(
        &'a self,
        parts: SecurityDisableUserParts<'b>,
    ) -> SecurityDisableUser<'a, 'b, ()> {
        SecurityDisableUser::new(self.transport(), parts)
    }
    #[doc = "[Security Enable User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-enable-user.html)\n\nEnables users in the native realm."]
    pub fn enable_user<'b>(
        &'a self,
        parts: SecurityEnableUserParts<'b>,
    ) -> SecurityEnableUser<'a, 'b, ()> {
        SecurityEnableUser::new(self.transport(), parts)
    }
    #[doc = "[Security Get Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-api-key.html)\n\nRetrieves information for one or more API keys."]
    pub fn get_api_key<'b>(&'a self) -> SecurityGetApiKey<'a, 'b> {
        SecurityGetApiKey::new(self.transport())
    }
    #[doc = "[Security Get Builtin Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-builtin-privileges.html)\n\nRetrieves the list of cluster privileges and index privileges that are available in this version of Elasticsearch."]
    pub fn get_builtin_privileges<'b>(&'a self) -> SecurityGetBuiltinPrivileges<'a, 'b> {
        SecurityGetBuiltinPrivileges::new(self.transport())
    }
    #[doc = "[Security Get Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-privileges.html)\n\nRetrieves application privileges."]
    pub fn get_privileges<'b>(
        &'a self,
        parts: SecurityGetPrivilegesParts<'b>,
    ) -> SecurityGetPrivileges<'a, 'b> {
        SecurityGetPrivileges::new(self.transport(), parts)
    }
    #[doc = "[Security Get Role API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-role.html)\n\nRetrieves roles in the native realm."]
    pub fn get_role<'b>(&'a self, parts: SecurityGetRoleParts<'b>) -> SecurityGetRole<'a, 'b> {
        SecurityGetRole::new(self.transport(), parts)
    }
    #[doc = "[Security Get Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-role-mapping.html)\n\nRetrieves role mappings."]
    pub fn get_role_mapping<'b>(
        &'a self,
        parts: SecurityGetRoleMappingParts<'b>,
    ) -> SecurityGetRoleMapping<'a, 'b> {
        SecurityGetRoleMapping::new(self.transport(), parts)
    }
    #[doc = "[Security Get Token API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-token.html)\n\nCreates a bearer token for access without requiring basic authentication."]
    pub fn get_token<'b>(&'a self) -> SecurityGetToken<'a, 'b, ()> {
        SecurityGetToken::new(self.transport())
    }
    #[doc = "[Security Get User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-user.html)\n\nRetrieves information about users in the native realm and built-in users."]
    pub fn get_user<'b>(&'a self, parts: SecurityGetUserParts<'b>) -> SecurityGetUser<'a, 'b> {
        SecurityGetUser::new(self.transport(), parts)
    }
    #[doc = "[Security Get User Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-get-privileges.html)\n\nRetrieves application privileges."]
    pub fn get_user_privileges<'b>(&'a self) -> SecurityGetUserPrivileges<'a, 'b> {
        SecurityGetUserPrivileges::new(self.transport())
    }
    #[doc = "[Security Grant Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-grant-api-key.html)\n\nCreates an API key on behalf of another user."]
    pub fn grant_api_key<'b>(&'a self) -> SecurityGrantApiKey<'a, 'b, ()> {
        SecurityGrantApiKey::new(self.transport())
    }
    #[doc = "[Security Has Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-has-privileges.html)\n\nDetermines whether the specified user has a specified list of privileges."]
    pub fn has_privileges<'b>(
        &'a self,
        parts: SecurityHasPrivilegesParts<'b>,
    ) -> SecurityHasPrivileges<'a, 'b, ()> {
        SecurityHasPrivileges::new(self.transport(), parts)
    }
    #[doc = "[Security Invalidate Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-invalidate-api-key.html)\n\nInvalidates one or more API keys."]
    pub fn invalidate_api_key<'b>(&'a self) -> SecurityInvalidateApiKey<'a, 'b, ()> {
        SecurityInvalidateApiKey::new(self.transport())
    }
    #[doc = "[Security Invalidate Token API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-invalidate-token.html)\n\nInvalidates one or more access tokens or refresh tokens."]
    pub fn invalidate_token<'b>(&'a self) -> SecurityInvalidateToken<'a, 'b, ()> {
        SecurityInvalidateToken::new(self.transport())
    }
    #[doc = "[Security Put Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-put-privileges.html)\n\nAdds or updates application privileges."]
    pub fn put_privileges<'b>(&'a self) -> SecurityPutPrivileges<'a, 'b, ()> {
        SecurityPutPrivileges::new(self.transport())
    }
    #[doc = "[Security Put Role API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-put-role.html)\n\nAdds and updates roles in the native realm."]
    pub fn put_role<'b>(&'a self, parts: SecurityPutRoleParts<'b>) -> SecurityPutRole<'a, 'b, ()> {
        SecurityPutRole::new(self.transport(), parts)
    }
    #[doc = "[Security Put Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-put-role-mapping.html)\n\nCreates and updates role mappings."]
    pub fn put_role_mapping<'b>(
        &'a self,
        parts: SecurityPutRoleMappingParts<'b>,
    ) -> SecurityPutRoleMapping<'a, 'b, ()> {
        SecurityPutRoleMapping::new(self.transport(), parts)
    }
    #[doc = "[Security Put User API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/security-api-put-user.html)\n\nAdds and updates users in the native realm. These users are commonly referred to as native users."]
    pub fn put_user<'b>(&'a self, parts: SecurityPutUserParts<'b>) -> SecurityPutUser<'a, 'b, ()> {
        SecurityPutUser::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Security APIs"]
    pub fn security(&self) -> Security {
        Security::new(self.transport())
    }
}
