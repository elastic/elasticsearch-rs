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
//! - Authenticate users against an OpenID Connect or SAML authentication realm when using a custom web application other than Kibana

#![allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        self,
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        transport::Transport,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::{borrow::Cow, time::Duration};
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Activate User Profile API"]
pub enum SecurityActivateUserProfileParts {
    #[doc = "No parts"]
    None,
}
impl SecurityActivateUserProfileParts {
    #[doc = "Builds a relative URL path to the Security Activate User Profile API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityActivateUserProfileParts::None => "/_security/profile/_activate".into(),
        }
    }
}
#[doc = "Builder for the [Security Activate User Profile API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-activate-user-profile.html)\n\nCreates or updates the user profile on behalf of another user."]
#[derive(Clone, Debug)]
pub struct SecurityActivateUserProfile<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityActivateUserProfileParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityActivateUserProfile<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityActivateUserProfile]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityActivateUserProfile {
            transport,
            parts: SecurityActivateUserProfileParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityActivateUserProfile<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityActivateUserProfile {
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
    #[doc = "Creates an asynchronous call to the Security Activate User Profile API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[doc = "Builder for the [Security Authenticate API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-authenticate.html)\n\nEnables authentication as a user and retrieve information about the authenticated user."]
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
        let method = http::Method::Get;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Bulk Delete Role API"]
pub enum SecurityBulkDeleteRoleParts {
    #[doc = "No parts"]
    None,
}
impl SecurityBulkDeleteRoleParts {
    #[doc = "Builds a relative URL path to the Security Bulk Delete Role API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityBulkDeleteRoleParts::None => "/_security/role".into(),
        }
    }
}
#[doc = "Builder for the [Security Bulk Delete Role API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-bulk-delete-role.html)\n\nBulk delete roles in the native realm."]
#[derive(Clone, Debug)]
pub struct SecurityBulkDeleteRole<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityBulkDeleteRoleParts,
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
impl<'a, 'b, B> SecurityBulkDeleteRole<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityBulkDeleteRole]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityBulkDeleteRole {
            transport,
            parts: SecurityBulkDeleteRoleParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityBulkDeleteRole<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityBulkDeleteRole {
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
    #[doc = "Creates an asynchronous call to the Security Bulk Delete Role API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Delete;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Bulk Put Role API"]
pub enum SecurityBulkPutRoleParts {
    #[doc = "No parts"]
    None,
}
impl SecurityBulkPutRoleParts {
    #[doc = "Builds a relative URL path to the Security Bulk Put Role API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityBulkPutRoleParts::None => "/_security/role".into(),
        }
    }
}
#[doc = "Builder for the [Security Bulk Put Role API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-bulk-put-role.html)\n\nBulk adds and updates roles in the native realm."]
#[derive(Clone, Debug)]
pub struct SecurityBulkPutRole<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityBulkPutRoleParts,
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
impl<'a, 'b, B> SecurityBulkPutRole<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityBulkPutRole]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityBulkPutRole {
            transport,
            parts: SecurityBulkPutRoleParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityBulkPutRole<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityBulkPutRole {
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
    #[doc = "Creates an asynchronous call to the Security Bulk Put Role API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Bulk Update Api Keys API"]
pub enum SecurityBulkUpdateApiKeysParts {
    #[doc = "No parts"]
    None,
}
impl SecurityBulkUpdateApiKeysParts {
    #[doc = "Builds a relative URL path to the Security Bulk Update Api Keys API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityBulkUpdateApiKeysParts::None => "/_security/api_key/_bulk_update".into(),
        }
    }
}
#[doc = "Builder for the [Security Bulk Update Api Keys API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-bulk-update-api-keys.html)\n\nUpdates the attributes of multiple existing API keys."]
#[derive(Clone, Debug)]
pub struct SecurityBulkUpdateApiKeys<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityBulkUpdateApiKeysParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityBulkUpdateApiKeys<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityBulkUpdateApiKeys]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityBulkUpdateApiKeys {
            transport,
            parts: SecurityBulkUpdateApiKeysParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityBulkUpdateApiKeys<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityBulkUpdateApiKeys {
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
    #[doc = "Creates an asynchronous call to the Security Bulk Update Api Keys API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
            SecurityChangePasswordParts::Username(username) => {
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
#[doc = "Builder for the [Security Change Password API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-change-password.html)\n\nChanges the passwords of users in the native realm and built-in users."]
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
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Clear Api Key Cache API"]
pub enum SecurityClearApiKeyCacheParts<'b> {
    #[doc = "Ids"]
    Ids(&'b [&'b str]),
}
impl<'b> SecurityClearApiKeyCacheParts<'b> {
    #[doc = "Builds a relative URL path to the Security Clear Api Key Cache API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityClearApiKeyCacheParts::Ids(ids) => {
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
#[doc = "Builder for the [Security Clear Api Key Cache API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-clear-api-key-cache.html)\n\nClear a subset or all entries from the API key cache."]
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
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Clear Cached Privileges API"]
pub enum SecurityClearCachedPrivilegesParts<'b> {
    #[doc = "Application"]
    Application(&'b [&'b str]),
}
impl<'b> SecurityClearCachedPrivilegesParts<'b> {
    #[doc = "Builds a relative URL path to the Security Clear Cached Privileges API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityClearCachedPrivilegesParts::Application(application) => {
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
#[doc = "Builder for the [Security Clear Cached Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-clear-privilege-cache.html)\n\nEvicts application privileges from the native application privileges cache."]
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
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Clear Cached Realms API"]
pub enum SecurityClearCachedRealmsParts<'b> {
    #[doc = "Realms"]
    Realms(&'b [&'b str]),
}
impl<'b> SecurityClearCachedRealmsParts<'b> {
    #[doc = "Builds a relative URL path to the Security Clear Cached Realms API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityClearCachedRealmsParts::Realms(realms) => {
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
#[doc = "Builder for the [Security Clear Cached Realms API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-clear-cache.html)\n\nEvicts users from the user cache. Can completely clear the cache or evict specific users."]
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
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Clear Cached Roles API"]
pub enum SecurityClearCachedRolesParts<'b> {
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> SecurityClearCachedRolesParts<'b> {
    #[doc = "Builds a relative URL path to the Security Clear Cached Roles API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityClearCachedRolesParts::Name(name) => {
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
#[doc = "Builder for the [Security Clear Cached Roles API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-clear-role-cache.html)\n\nEvicts roles from the native role cache."]
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
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Clear Cached Service Tokens API"]
pub enum SecurityClearCachedServiceTokensParts<'b> {
    #[doc = "Namespace, Service and Name"]
    NamespaceServiceName(&'b str, &'b str, &'b [&'b str]),
}
impl<'b> SecurityClearCachedServiceTokensParts<'b> {
    #[doc = "Builds a relative URL path to the Security Clear Cached Service Tokens API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityClearCachedServiceTokensParts::NamespaceServiceName(
                namespace,
                service,
                name,
            ) => {
                let name_str = name.join(",");
                let encoded_namespace: Cow<str> =
                    percent_encode(namespace.as_bytes(), PARTS_ENCODED).into();
                let encoded_service: Cow<str> =
                    percent_encode(service.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    51usize + encoded_namespace.len() + encoded_service.len() + encoded_name.len(),
                );
                p.push_str("/_security/service/");
                p.push_str(encoded_namespace.as_ref());
                p.push('/');
                p.push_str(encoded_service.as_ref());
                p.push_str("/credential/token/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/_clear_cache");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Clear Cached Service Tokens API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-clear-service-token-caches.html)\n\nEvicts tokens from the service account token caches."]
#[derive(Clone, Debug)]
pub struct SecurityClearCachedServiceTokens<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityClearCachedServiceTokensParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityClearCachedServiceTokens<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityClearCachedServiceTokens] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityClearCachedServiceTokensParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityClearCachedServiceTokens {
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
    pub fn body<T>(self, body: T) -> SecurityClearCachedServiceTokens<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityClearCachedServiceTokens {
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
    #[doc = "Creates an asynchronous call to the Security Clear Cached Service Tokens API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[doc = "Builder for the [Security Create Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-create-api-key.html)\n\nCreates an API key for access without requiring basic authentication."]
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
        let method = http::Method::Post;
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
#[cfg(feature = "beta-apis")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Create Cross Cluster Api Key API"]
pub enum SecurityCreateCrossClusterApiKeyParts {
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "beta-apis")]
impl SecurityCreateCrossClusterApiKeyParts {
    #[doc = "Builds a relative URL path to the Security Create Cross Cluster Api Key API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityCreateCrossClusterApiKeyParts::None => {
                "/_security/cross_cluster/api_key".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Create Cross Cluster Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-create-cross-cluster-api-key.html)\n\nCreates a cross-cluster API key for API key based remote cluster access."]
#[doc = "&nbsp;\n# Optional, beta\nThis requires the `beta-apis` feature. On track to become stable but breaking changes can\nhappen in minor versions.\n        "]
#[cfg(feature = "beta-apis")]
#[derive(Clone, Debug)]
pub struct SecurityCreateCrossClusterApiKey<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityCreateCrossClusterApiKeyParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "beta-apis")]
impl<'a, 'b, B> SecurityCreateCrossClusterApiKey<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityCreateCrossClusterApiKey]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityCreateCrossClusterApiKey {
            transport,
            parts: SecurityCreateCrossClusterApiKeyParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityCreateCrossClusterApiKey<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityCreateCrossClusterApiKey {
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
    #[doc = "Creates an asynchronous call to the Security Create Cross Cluster Api Key API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Create Service Token API"]
pub enum SecurityCreateServiceTokenParts<'b> {
    #[doc = "Namespace, Service and Name"]
    NamespaceServiceName(&'b str, &'b str, &'b str),
    #[doc = "Namespace and Service"]
    NamespaceService(&'b str, &'b str),
}
impl<'b> SecurityCreateServiceTokenParts<'b> {
    #[doc = "Builds a relative URL path to the Security Create Service Token API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityCreateServiceTokenParts::NamespaceServiceName(namespace, service, name) => {
                let encoded_namespace: Cow<str> =
                    percent_encode(namespace.as_bytes(), PARTS_ENCODED).into();
                let encoded_service: Cow<str> =
                    percent_encode(service.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    38usize + encoded_namespace.len() + encoded_service.len() + encoded_name.len(),
                );
                p.push_str("/_security/service/");
                p.push_str(encoded_namespace.as_ref());
                p.push('/');
                p.push_str(encoded_service.as_ref());
                p.push_str("/credential/token/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
            SecurityCreateServiceTokenParts::NamespaceService(namespace, service) => {
                let encoded_namespace: Cow<str> =
                    percent_encode(namespace.as_bytes(), PARTS_ENCODED).into();
                let encoded_service: Cow<str> =
                    percent_encode(service.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    37usize + encoded_namespace.len() + encoded_service.len(),
                );
                p.push_str("/_security/service/");
                p.push_str(encoded_namespace.as_ref());
                p.push('/');
                p.push_str(encoded_service.as_ref());
                p.push_str("/credential/token");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Create Service Token API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-create-service-token.html)\n\nCreates a service account token for access without requiring basic authentication."]
#[derive(Clone, Debug)]
pub struct SecurityCreateServiceToken<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityCreateServiceTokenParts<'b>,
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
impl<'a, 'b, B> SecurityCreateServiceToken<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityCreateServiceToken] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityCreateServiceTokenParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityCreateServiceToken {
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
    pub fn body<T>(self, body: T) -> SecurityCreateServiceToken<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityCreateServiceToken {
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
    #[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` (the default) then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
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
    #[doc = "Creates an asynchronous call to the Security Create Service Token API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Delete Privileges API"]
pub enum SecurityDeletePrivilegesParts<'b> {
    #[doc = "Application and Name"]
    ApplicationName(&'b str, &'b str),
}
impl<'b> SecurityDeletePrivilegesParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete Privileges API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeletePrivilegesParts::ApplicationName(application, name) => {
                let encoded_application: Cow<str> =
                    percent_encode(application.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(22usize + encoded_application.len() + encoded_name.len());
                p.push_str("/_security/privilege/");
                p.push_str(encoded_application.as_ref());
                p.push('/');
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-delete-privilege.html)\n\nRemoves application privileges."]
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
        let method = http::Method::Delete;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Delete Role API"]
pub enum SecurityDeleteRoleParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> SecurityDeleteRoleParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete Role API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteRoleParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_name.len());
                p.push_str("/_security/role/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete Role API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-delete-role.html)\n\nRemoves roles in the native realm."]
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
        let method = http::Method::Delete;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Delete Role Mapping API"]
pub enum SecurityDeleteRoleMappingParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> SecurityDeleteRoleMappingParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete Role Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteRoleMappingParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_name.len());
                p.push_str("/_security/role_mapping/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-delete-role-mapping.html)\n\nRemoves role mappings."]
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
        let method = http::Method::Delete;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Delete Service Token API"]
pub enum SecurityDeleteServiceTokenParts<'b> {
    #[doc = "Namespace, Service and Name"]
    NamespaceServiceName(&'b str, &'b str, &'b str),
}
impl<'b> SecurityDeleteServiceTokenParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete Service Token API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteServiceTokenParts::NamespaceServiceName(namespace, service, name) => {
                let encoded_namespace: Cow<str> =
                    percent_encode(namespace.as_bytes(), PARTS_ENCODED).into();
                let encoded_service: Cow<str> =
                    percent_encode(service.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    38usize + encoded_namespace.len() + encoded_service.len() + encoded_name.len(),
                );
                p.push_str("/_security/service/");
                p.push_str(encoded_namespace.as_ref());
                p.push('/');
                p.push_str(encoded_service.as_ref());
                p.push_str("/credential/token/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete Service Token API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-delete-service-token.html)\n\nDeletes a service account token."]
#[derive(Clone, Debug)]
pub struct SecurityDeleteServiceToken<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityDeleteServiceTokenParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteServiceToken<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteServiceToken] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDeleteServiceTokenParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteServiceToken {
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
    #[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` (the default) then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
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
    #[doc = "Creates an asynchronous call to the Security Delete Service Token API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Delete;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Delete User API"]
pub enum SecurityDeleteUserParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityDeleteUserParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteUserParts::Username(username) => {
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
#[doc = "Builder for the [Security Delete User API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-delete-user.html)\n\nDeletes users from the native realm."]
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
        let method = http::Method::Delete;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Disable User API"]
pub enum SecurityDisableUserParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityDisableUserParts<'b> {
    #[doc = "Builds a relative URL path to the Security Disable User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDisableUserParts::Username(username) => {
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
#[doc = "Builder for the [Security Disable User API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-disable-user.html)\n\nDisables users in the native realm."]
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
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Disable User Profile API"]
pub enum SecurityDisableUserProfileParts<'b> {
    #[doc = "Uid"]
    Uid(&'b str),
}
impl<'b> SecurityDisableUserProfileParts<'b> {
    #[doc = "Builds a relative URL path to the Security Disable User Profile API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDisableUserProfileParts::Uid(uid) => {
                let encoded_uid: Cow<str> = percent_encode(uid.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(28usize + encoded_uid.len());
                p.push_str("/_security/profile/");
                p.push_str(encoded_uid.as_ref());
                p.push_str("/_disable");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Disable User Profile API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-disable-user-profile.html)\n\nDisables a user profile so it's not visible in user profile searches."]
#[derive(Clone, Debug)]
pub struct SecurityDisableUserProfile<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityDisableUserProfileParts<'b>,
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
impl<'a, 'b, B> SecurityDisableUserProfile<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityDisableUserProfile] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDisableUserProfileParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDisableUserProfile {
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
    pub fn body<T>(self, body: T) -> SecurityDisableUserProfile<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityDisableUserProfile {
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
    #[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` (the default) then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
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
    #[doc = "Creates an asynchronous call to the Security Disable User Profile API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Enable User API"]
pub enum SecurityEnableUserParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityEnableUserParts<'b> {
    #[doc = "Builds a relative URL path to the Security Enable User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityEnableUserParts::Username(username) => {
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
#[doc = "Builder for the [Security Enable User API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-enable-user.html)\n\nEnables users in the native realm."]
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
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Enable User Profile API"]
pub enum SecurityEnableUserProfileParts<'b> {
    #[doc = "Uid"]
    Uid(&'b str),
}
impl<'b> SecurityEnableUserProfileParts<'b> {
    #[doc = "Builds a relative URL path to the Security Enable User Profile API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityEnableUserProfileParts::Uid(uid) => {
                let encoded_uid: Cow<str> = percent_encode(uid.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_uid.len());
                p.push_str("/_security/profile/");
                p.push_str(encoded_uid.as_ref());
                p.push_str("/_enable");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Enable User Profile API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-enable-user-profile.html)\n\nEnables a user profile so it's visible in user profile searches."]
#[derive(Clone, Debug)]
pub struct SecurityEnableUserProfile<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityEnableUserProfileParts<'b>,
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
impl<'a, 'b, B> SecurityEnableUserProfile<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityEnableUserProfile] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityEnableUserProfileParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityEnableUserProfile {
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
    pub fn body<T>(self, body: T) -> SecurityEnableUserProfile<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityEnableUserProfile {
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
    #[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` (the default) then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
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
    #[doc = "Creates an asynchronous call to the Security Enable User Profile API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Enroll Kibana API"]
pub enum SecurityEnrollKibanaParts {
    #[doc = "No parts"]
    None,
}
impl SecurityEnrollKibanaParts {
    #[doc = "Builds a relative URL path to the Security Enroll Kibana API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityEnrollKibanaParts::None => "/_security/enroll/kibana".into(),
        }
    }
}
#[doc = "Builder for the [Security Enroll Kibana API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-kibana-enrollment.html)\n\nAllows a kibana instance to configure itself to communicate with a secured elasticsearch cluster."]
#[derive(Clone, Debug)]
pub struct SecurityEnrollKibana<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityEnrollKibanaParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityEnrollKibana<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityEnrollKibana]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityEnrollKibana {
            transport,
            parts: SecurityEnrollKibanaParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Enroll Kibana API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Enroll Node API"]
pub enum SecurityEnrollNodeParts {
    #[doc = "No parts"]
    None,
}
impl SecurityEnrollNodeParts {
    #[doc = "Builds a relative URL path to the Security Enroll Node API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityEnrollNodeParts::None => "/_security/enroll/node".into(),
        }
    }
}
#[doc = "Builder for the [Security Enroll Node API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-node-enrollment.html)\n\nAllows a new node to enroll to an existing cluster with security enabled."]
#[derive(Clone, Debug)]
pub struct SecurityEnrollNode<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityEnrollNodeParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityEnrollNode<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityEnrollNode]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityEnrollNode {
            transport,
            parts: SecurityEnrollNodeParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Enroll Node API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[doc = "Builder for the [Security Get Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-api-key.html)\n\nRetrieves information for one or more API keys."]
#[derive(Clone, Debug)]
pub struct SecurityGetApiKey<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetApiKeyParts,
    active_only: Option<bool>,
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
    with_limited_by: Option<bool>,
    with_profile_uid: Option<bool>,
}
impl<'a, 'b> SecurityGetApiKey<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetApiKey]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetApiKey {
            transport,
            parts: SecurityGetApiKeyParts::None,
            headers,
            active_only: None,
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
            with_limited_by: None,
            with_profile_uid: None,
        }
    }
    #[doc = "flag to limit response to only active (not invalidated or expired) API keys"]
    pub fn active_only(mut self, active_only: bool) -> Self {
        self.active_only = Some(active_only);
        self
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
    #[doc = "flag to show the limited-by role descriptors of API Keys"]
    pub fn with_limited_by(mut self, with_limited_by: bool) -> Self {
        self.with_limited_by = Some(with_limited_by);
        self
    }
    #[doc = "flag to also retrieve the API Key's owner profile uid, if it exists"]
    pub fn with_profile_uid(mut self, with_profile_uid: bool) -> Self {
        self.with_profile_uid = Some(with_profile_uid);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Api Key API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                active_only: Option<bool>,
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
                with_limited_by: Option<bool>,
                with_profile_uid: Option<bool>,
            }
            let query_params = QueryParams {
                active_only: self.active_only,
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
                with_limited_by: self.with_limited_by,
                with_profile_uid: self.with_profile_uid,
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[doc = "Builder for the [Security Get Builtin Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-builtin-privileges.html)\n\nRetrieves the list of cluster privileges and index privileges that are available in this version of Elasticsearch."]
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
        let method = http::Method::Get;
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
            SecurityGetPrivilegesParts::Application(application) => {
                let encoded_application: Cow<str> =
                    percent_encode(application.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_application.len());
                p.push_str("/_security/privilege/");
                p.push_str(encoded_application.as_ref());
                p.into()
            }
            SecurityGetPrivilegesParts::ApplicationName(application, name) => {
                let encoded_application: Cow<str> =
                    percent_encode(application.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(22usize + encoded_application.len() + encoded_name.len());
                p.push_str("/_security/privilege/");
                p.push_str(encoded_application.as_ref());
                p.push('/');
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Get Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-privileges.html)\n\nRetrieves application privileges."]
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
        let method = http::Method::Get;
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
            SecurityGetRoleParts::Name(name) => {
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
#[doc = "Builder for the [Security Get Role API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-role.html)\n\nRetrieves roles in the native realm."]
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
        let method = http::Method::Get;
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
            SecurityGetRoleMappingParts::Name(name) => {
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
#[doc = "Builder for the [Security Get Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-role-mapping.html)\n\nRetrieves role mappings."]
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
        let method = http::Method::Get;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Get Service Accounts API"]
pub enum SecurityGetServiceAccountsParts<'b> {
    #[doc = "Namespace and Service"]
    NamespaceService(&'b str, &'b str),
    #[doc = "Namespace"]
    Namespace(&'b str),
    #[doc = "No parts"]
    None,
}
impl<'b> SecurityGetServiceAccountsParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get Service Accounts API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetServiceAccountsParts::NamespaceService(namespace, service) => {
                let encoded_namespace: Cow<str> =
                    percent_encode(namespace.as_bytes(), PARTS_ENCODED).into();
                let encoded_service: Cow<str> =
                    percent_encode(service.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    20usize + encoded_namespace.len() + encoded_service.len(),
                );
                p.push_str("/_security/service/");
                p.push_str(encoded_namespace.as_ref());
                p.push('/');
                p.push_str(encoded_service.as_ref());
                p.into()
            }
            SecurityGetServiceAccountsParts::Namespace(namespace) => {
                let encoded_namespace: Cow<str> =
                    percent_encode(namespace.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_namespace.len());
                p.push_str("/_security/service/");
                p.push_str(encoded_namespace.as_ref());
                p.into()
            }
            SecurityGetServiceAccountsParts::None => "/_security/service".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Service Accounts API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-service-accounts.html)\n\nRetrieves information about service accounts."]
#[derive(Clone, Debug)]
pub struct SecurityGetServiceAccounts<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetServiceAccountsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetServiceAccounts<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetServiceAccounts] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetServiceAccountsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetServiceAccounts {
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
    #[doc = "Creates an asynchronous call to the Security Get Service Accounts API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Get Service Credentials API"]
pub enum SecurityGetServiceCredentialsParts<'b> {
    #[doc = "Namespace and Service"]
    NamespaceService(&'b str, &'b str),
}
impl<'b> SecurityGetServiceCredentialsParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get Service Credentials API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetServiceCredentialsParts::NamespaceService(namespace, service) => {
                let encoded_namespace: Cow<str> =
                    percent_encode(namespace.as_bytes(), PARTS_ENCODED).into();
                let encoded_service: Cow<str> =
                    percent_encode(service.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    31usize + encoded_namespace.len() + encoded_service.len(),
                );
                p.push_str("/_security/service/");
                p.push_str(encoded_namespace.as_ref());
                p.push('/');
                p.push_str(encoded_service.as_ref());
                p.push_str("/credential");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Get Service Credentials API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-service-credentials.html)\n\nRetrieves information of all service credentials for a service account."]
#[derive(Clone, Debug)]
pub struct SecurityGetServiceCredentials<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetServiceCredentialsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetServiceCredentials<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetServiceCredentials] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetServiceCredentialsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetServiceCredentials {
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
    #[doc = "Creates an asynchronous call to the Security Get Service Credentials API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Get Settings API"]
pub enum SecurityGetSettingsParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetSettingsParts {
    #[doc = "Builds a relative URL path to the Security Get Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetSettingsParts::None => "/_security/settings".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-settings.html)\n\nRetrieve settings for the security system indices"]
#[derive(Clone, Debug)]
pub struct SecurityGetSettings<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetSettingsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetSettings<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetSettings]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetSettings {
            transport,
            parts: SecurityGetSettingsParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
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
    #[doc = "Timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous call to the Security Get Settings API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
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
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[doc = "Builder for the [Security Get Token API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-token.html)\n\nCreates a bearer token for access without requiring basic authentication."]
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
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
            SecurityGetUserParts::Username(username) => {
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
#[doc = "Builder for the [Security Get User API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-user.html)\n\nRetrieves information about users in the native realm and built-in users."]
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
    with_profile_uid: Option<bool>,
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
            with_profile_uid: None,
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
    #[doc = "flag to retrieve profile uid (if exists) associated to the user"]
    pub fn with_profile_uid(mut self, with_profile_uid: bool) -> Self {
        self.with_profile_uid = Some(with_profile_uid);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get User API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
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
                with_profile_uid: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                with_profile_uid: self.with_profile_uid,
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[doc = "Builder for the [Security Get User Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-user-privileges.html)\n\nRetrieves security privileges for the logged in user."]
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
        let method = http::Method::Get;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Get User Profile API"]
pub enum SecurityGetUserProfileParts<'b> {
    #[doc = "Uid"]
    Uid(&'b [&'b str]),
}
impl<'b> SecurityGetUserProfileParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get User Profile API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetUserProfileParts::Uid(uid) => {
                let uid_str = uid.join(",");
                let encoded_uid: Cow<str> =
                    percent_encode(uid_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_uid.len());
                p.push_str("/_security/profile/");
                p.push_str(encoded_uid.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Get User Profile API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-user-profile.html)\n\nRetrieves user profiles for the given unique ID(s)."]
#[derive(Clone, Debug)]
pub struct SecurityGetUserProfile<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetUserProfileParts<'b>,
    data: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetUserProfile<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetUserProfile] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetUserProfileParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetUserProfile {
            transport,
            parts,
            headers,
            data: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "A comma-separated list of keys for which the corresponding application data are retrieved."]
    pub fn data(mut self, data: &'b [&'b str]) -> Self {
        self.data = Some(data);
        self
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
    #[doc = "Creates an asynchronous call to the Security Get User Profile API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                data: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                data: self.data,
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[doc = "Builder for the [Security Grant Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-grant-api-key.html)\n\nCreates an API key on behalf of another user."]
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
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
            SecurityHasPrivilegesParts::User(user) => {
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
#[doc = "Builder for the [Security Has Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-has-privileges.html)\n\nDetermines whether the specified user has a specified list of privileges."]
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
            Some(_) => http::Method::Post,
            None => http::Method::Get,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Has Privileges User Profile API"]
pub enum SecurityHasPrivilegesUserProfileParts {
    #[doc = "No parts"]
    None,
}
impl SecurityHasPrivilegesUserProfileParts {
    #[doc = "Builds a relative URL path to the Security Has Privileges User Profile API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityHasPrivilegesUserProfileParts::None => {
                "/_security/profile/_has_privileges".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Has Privileges User Profile API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-has-privileges-user-profile.html)\n\nDetermines whether the users associated with the specified profile IDs have all the requested privileges."]
#[derive(Clone, Debug)]
pub struct SecurityHasPrivilegesUserProfile<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityHasPrivilegesUserProfileParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityHasPrivilegesUserProfile<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityHasPrivilegesUserProfile]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityHasPrivilegesUserProfile {
            transport,
            parts: SecurityHasPrivilegesUserProfileParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityHasPrivilegesUserProfile<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityHasPrivilegesUserProfile {
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
    #[doc = "Creates an asynchronous call to the Security Has Privileges User Profile API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => http::Method::Post,
            None => http::Method::Get,
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[doc = "Builder for the [Security Invalidate Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-invalidate-api-key.html)\n\nInvalidates one or more API keys."]
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
        let method = http::Method::Delete;
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[doc = "Builder for the [Security Invalidate Token API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-invalidate-token.html)\n\nInvalidates one or more access tokens or refresh tokens."]
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
        let method = http::Method::Delete;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Oidc Authenticate API"]
pub enum SecurityOidcAuthenticateParts {
    #[doc = "No parts"]
    None,
}
impl SecurityOidcAuthenticateParts {
    #[doc = "Builds a relative URL path to the Security Oidc Authenticate API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityOidcAuthenticateParts::None => "/_security/oidc/authenticate".into(),
        }
    }
}
#[doc = "Builder for the [Security Oidc Authenticate API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-oidc-authenticate.html)\n\nExchanges an OpenID Connection authentication response message for an Elasticsearch access token and refresh token pair"]
#[derive(Clone, Debug)]
pub struct SecurityOidcAuthenticate<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityOidcAuthenticateParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityOidcAuthenticate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityOidcAuthenticate]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityOidcAuthenticate {
            transport,
            parts: SecurityOidcAuthenticateParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityOidcAuthenticate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityOidcAuthenticate {
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
    #[doc = "Creates an asynchronous call to the Security Oidc Authenticate API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Oidc Logout API"]
pub enum SecurityOidcLogoutParts {
    #[doc = "No parts"]
    None,
}
impl SecurityOidcLogoutParts {
    #[doc = "Builds a relative URL path to the Security Oidc Logout API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityOidcLogoutParts::None => "/_security/oidc/logout".into(),
        }
    }
}
#[doc = "Builder for the [Security Oidc Logout API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-oidc-logout.html)\n\nInvalidates a refresh token and access token that was generated from the OpenID Connect Authenticate API"]
#[derive(Clone, Debug)]
pub struct SecurityOidcLogout<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityOidcLogoutParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityOidcLogout<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityOidcLogout]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityOidcLogout {
            transport,
            parts: SecurityOidcLogoutParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityOidcLogout<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityOidcLogout {
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
    #[doc = "Creates an asynchronous call to the Security Oidc Logout API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Oidc Prepare Authentication API"]
pub enum SecurityOidcPrepareAuthenticationParts {
    #[doc = "No parts"]
    None,
}
impl SecurityOidcPrepareAuthenticationParts {
    #[doc = "Builds a relative URL path to the Security Oidc Prepare Authentication API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityOidcPrepareAuthenticationParts::None => "/_security/oidc/prepare".into(),
        }
    }
}
#[doc = "Builder for the [Security Oidc Prepare Authentication API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-oidc-prepare-authentication.html)\n\nCreates an OAuth 2.0 authentication request as a URL string"]
#[derive(Clone, Debug)]
pub struct SecurityOidcPrepareAuthentication<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityOidcPrepareAuthenticationParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityOidcPrepareAuthentication<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityOidcPrepareAuthentication]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityOidcPrepareAuthentication {
            transport,
            parts: SecurityOidcPrepareAuthenticationParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityOidcPrepareAuthentication<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityOidcPrepareAuthentication {
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
    #[doc = "Creates an asynchronous call to the Security Oidc Prepare Authentication API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Put Privileges API"]
pub enum SecurityPutPrivilegesParts {
    #[doc = "No parts"]
    None,
}
impl SecurityPutPrivilegesParts {
    #[doc = "Builds a relative URL path to the Security Put Privileges API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPutPrivilegesParts::None => "/_security/privilege".into(),
        }
    }
}
#[doc = "Builder for the [Security Put Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-put-privileges.html)\n\nAdds or updates application privileges."]
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
        let method = http::Method::Put;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Put Role API"]
pub enum SecurityPutRoleParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> SecurityPutRoleParts<'b> {
    #[doc = "Builds a relative URL path to the Security Put Role API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPutRoleParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_name.len());
                p.push_str("/_security/role/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Put Role API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-put-role.html)\n\nAdds and updates roles in the native realm."]
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
        let method = http::Method::Put;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Put Role Mapping API"]
pub enum SecurityPutRoleMappingParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> SecurityPutRoleMappingParts<'b> {
    #[doc = "Builds a relative URL path to the Security Put Role Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPutRoleMappingParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_name.len());
                p.push_str("/_security/role_mapping/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Put Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-put-role-mapping.html)\n\nCreates and updates role mappings."]
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
        let method = http::Method::Put;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Put User API"]
pub enum SecurityPutUserParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityPutUserParts<'b> {
    #[doc = "Builds a relative URL path to the Security Put User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPutUserParts::Username(username) => {
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
#[doc = "Builder for the [Security Put User API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-put-user.html)\n\nAdds and updates users in the native realm. These users are commonly referred to as native users."]
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
        let method = http::Method::Put;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Query Api Keys API"]
pub enum SecurityQueryApiKeysParts {
    #[doc = "No parts"]
    None,
}
impl SecurityQueryApiKeysParts {
    #[doc = "Builds a relative URL path to the Security Query Api Keys API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityQueryApiKeysParts::None => "/_security/_query/api_key".into(),
        }
    }
}
#[doc = "Builder for the [Security Query Api Keys API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-query-api-key.html)\n\nRetrieves information for API keys using a subset of query DSL"]
#[derive(Clone, Debug)]
pub struct SecurityQueryApiKeys<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityQueryApiKeysParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    typed_keys: Option<bool>,
    with_limited_by: Option<bool>,
    with_profile_uid: Option<bool>,
}
impl<'a, 'b, B> SecurityQueryApiKeys<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityQueryApiKeys]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityQueryApiKeys {
            transport,
            parts: SecurityQueryApiKeysParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            typed_keys: None,
            with_limited_by: None,
            with_profile_uid: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityQueryApiKeys<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityQueryApiKeys {
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
            typed_keys: self.typed_keys,
            with_limited_by: self.with_limited_by,
            with_profile_uid: self.with_profile_uid,
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
    #[doc = "flag to prefix aggregation names by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "flag to show the limited-by role descriptors of API Keys"]
    pub fn with_limited_by(mut self, with_limited_by: bool) -> Self {
        self.with_limited_by = Some(with_limited_by);
        self
    }
    #[doc = "flag to also retrieve the API Key's owner profile uid, if it exists"]
    pub fn with_profile_uid(mut self, with_profile_uid: bool) -> Self {
        self.with_profile_uid = Some(with_profile_uid);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Query Api Keys API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => http::Method::Post,
            None => http::Method::Get,
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
                typed_keys: Option<bool>,
                with_limited_by: Option<bool>,
                with_profile_uid: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                typed_keys: self.typed_keys,
                with_limited_by: self.with_limited_by,
                with_profile_uid: self.with_profile_uid,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Query Role API"]
pub enum SecurityQueryRoleParts {
    #[doc = "No parts"]
    None,
}
impl SecurityQueryRoleParts {
    #[doc = "Builds a relative URL path to the Security Query Role API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityQueryRoleParts::None => "/_security/_query/role".into(),
        }
    }
}
#[doc = "Builder for the [Security Query Role API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-query-role.html)\n\nRetrieves information for Roles using a subset of query DSL"]
#[derive(Clone, Debug)]
pub struct SecurityQueryRole<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityQueryRoleParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityQueryRole<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityQueryRole]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityQueryRole {
            transport,
            parts: SecurityQueryRoleParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityQueryRole<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityQueryRole {
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
    #[doc = "Creates an asynchronous call to the Security Query Role API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => http::Method::Post,
            None => http::Method::Get,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Query User API"]
pub enum SecurityQueryUserParts {
    #[doc = "No parts"]
    None,
}
impl SecurityQueryUserParts {
    #[doc = "Builds a relative URL path to the Security Query User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityQueryUserParts::None => "/_security/_query/user".into(),
        }
    }
}
#[doc = "Builder for the [Security Query User API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-query-user.html)\n\nRetrieves information for Users using a subset of query DSL"]
#[derive(Clone, Debug)]
pub struct SecurityQueryUser<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityQueryUserParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    with_profile_uid: Option<bool>,
}
impl<'a, 'b, B> SecurityQueryUser<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityQueryUser]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityQueryUser {
            transport,
            parts: SecurityQueryUserParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            with_profile_uid: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityQueryUser<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityQueryUser {
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
            with_profile_uid: self.with_profile_uid,
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
    #[doc = "flag to retrieve profile uid (if exists) associated with the user"]
    pub fn with_profile_uid(mut self, with_profile_uid: bool) -> Self {
        self.with_profile_uid = Some(with_profile_uid);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Query User API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => http::Method::Post,
            None => http::Method::Get,
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
                with_profile_uid: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                with_profile_uid: self.with_profile_uid,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Saml Authenticate API"]
pub enum SecuritySamlAuthenticateParts {
    #[doc = "No parts"]
    None,
}
impl SecuritySamlAuthenticateParts {
    #[doc = "Builds a relative URL path to the Security Saml Authenticate API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecuritySamlAuthenticateParts::None => "/_security/saml/authenticate".into(),
        }
    }
}
#[doc = "Builder for the [Security Saml Authenticate API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-saml-authenticate.html)\n\nExchanges a SAML Response message for an Elasticsearch access token and refresh token pair"]
#[derive(Clone, Debug)]
pub struct SecuritySamlAuthenticate<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecuritySamlAuthenticateParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecuritySamlAuthenticate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecuritySamlAuthenticate]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecuritySamlAuthenticate {
            transport,
            parts: SecuritySamlAuthenticateParts::None,
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
    pub fn body<T>(self, body: T) -> SecuritySamlAuthenticate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecuritySamlAuthenticate {
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
    #[doc = "Creates an asynchronous call to the Security Saml Authenticate API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Saml Complete Logout API"]
pub enum SecuritySamlCompleteLogoutParts {
    #[doc = "No parts"]
    None,
}
impl SecuritySamlCompleteLogoutParts {
    #[doc = "Builds a relative URL path to the Security Saml Complete Logout API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecuritySamlCompleteLogoutParts::None => "/_security/saml/complete_logout".into(),
        }
    }
}
#[doc = "Builder for the [Security Saml Complete Logout API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-saml-complete-logout.html)\n\nVerifies the logout response sent from the SAML IdP"]
#[derive(Clone, Debug)]
pub struct SecuritySamlCompleteLogout<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecuritySamlCompleteLogoutParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecuritySamlCompleteLogout<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecuritySamlCompleteLogout]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecuritySamlCompleteLogout {
            transport,
            parts: SecuritySamlCompleteLogoutParts::None,
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
    pub fn body<T>(self, body: T) -> SecuritySamlCompleteLogout<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecuritySamlCompleteLogout {
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
    #[doc = "Creates an asynchronous call to the Security Saml Complete Logout API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Saml Invalidate API"]
pub enum SecuritySamlInvalidateParts {
    #[doc = "No parts"]
    None,
}
impl SecuritySamlInvalidateParts {
    #[doc = "Builds a relative URL path to the Security Saml Invalidate API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecuritySamlInvalidateParts::None => "/_security/saml/invalidate".into(),
        }
    }
}
#[doc = "Builder for the [Security Saml Invalidate API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-saml-invalidate.html)\n\nConsumes a SAML LogoutRequest"]
#[derive(Clone, Debug)]
pub struct SecuritySamlInvalidate<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecuritySamlInvalidateParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecuritySamlInvalidate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecuritySamlInvalidate]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecuritySamlInvalidate {
            transport,
            parts: SecuritySamlInvalidateParts::None,
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
    pub fn body<T>(self, body: T) -> SecuritySamlInvalidate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecuritySamlInvalidate {
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
    #[doc = "Creates an asynchronous call to the Security Saml Invalidate API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Saml Logout API"]
pub enum SecuritySamlLogoutParts {
    #[doc = "No parts"]
    None,
}
impl SecuritySamlLogoutParts {
    #[doc = "Builds a relative URL path to the Security Saml Logout API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecuritySamlLogoutParts::None => "/_security/saml/logout".into(),
        }
    }
}
#[doc = "Builder for the [Security Saml Logout API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-saml-logout.html)\n\nInvalidates an access token and a refresh token that were generated via the SAML Authenticate API"]
#[derive(Clone, Debug)]
pub struct SecuritySamlLogout<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecuritySamlLogoutParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecuritySamlLogout<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecuritySamlLogout]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecuritySamlLogout {
            transport,
            parts: SecuritySamlLogoutParts::None,
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
    pub fn body<T>(self, body: T) -> SecuritySamlLogout<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecuritySamlLogout {
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
    #[doc = "Creates an asynchronous call to the Security Saml Logout API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Saml Prepare Authentication API"]
pub enum SecuritySamlPrepareAuthenticationParts {
    #[doc = "No parts"]
    None,
}
impl SecuritySamlPrepareAuthenticationParts {
    #[doc = "Builds a relative URL path to the Security Saml Prepare Authentication API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecuritySamlPrepareAuthenticationParts::None => "/_security/saml/prepare".into(),
        }
    }
}
#[doc = "Builder for the [Security Saml Prepare Authentication API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-saml-prepare-authentication.html)\n\nCreates a SAML authentication request"]
#[derive(Clone, Debug)]
pub struct SecuritySamlPrepareAuthentication<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecuritySamlPrepareAuthenticationParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecuritySamlPrepareAuthentication<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecuritySamlPrepareAuthentication]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecuritySamlPrepareAuthentication {
            transport,
            parts: SecuritySamlPrepareAuthenticationParts::None,
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
    pub fn body<T>(self, body: T) -> SecuritySamlPrepareAuthentication<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecuritySamlPrepareAuthentication {
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
    #[doc = "Creates an asynchronous call to the Security Saml Prepare Authentication API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Saml Service Provider Metadata API"]
pub enum SecuritySamlServiceProviderMetadataParts<'b> {
    #[doc = "RealmName"]
    RealmName(&'b str),
}
impl<'b> SecuritySamlServiceProviderMetadataParts<'b> {
    #[doc = "Builds a relative URL path to the Security Saml Service Provider Metadata API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecuritySamlServiceProviderMetadataParts::RealmName(realm_name) => {
                let encoded_realm_name: Cow<str> =
                    percent_encode(realm_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(25usize + encoded_realm_name.len());
                p.push_str("/_security/saml/metadata/");
                p.push_str(encoded_realm_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Saml Service Provider Metadata API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-saml-sp-metadata.html)\n\nGenerates SAML metadata for the Elastic stack SAML 2.0 Service Provider"]
#[derive(Clone, Debug)]
pub struct SecuritySamlServiceProviderMetadata<'a, 'b> {
    transport: &'a Transport,
    parts: SecuritySamlServiceProviderMetadataParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecuritySamlServiceProviderMetadata<'a, 'b> {
    #[doc = "Creates a new instance of [SecuritySamlServiceProviderMetadata] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: SecuritySamlServiceProviderMetadataParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        SecuritySamlServiceProviderMetadata {
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
    #[doc = "Creates an asynchronous call to the Security Saml Service Provider Metadata API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Suggest User Profiles API"]
pub enum SecuritySuggestUserProfilesParts {
    #[doc = "No parts"]
    None,
}
impl SecuritySuggestUserProfilesParts {
    #[doc = "Builds a relative URL path to the Security Suggest User Profiles API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecuritySuggestUserProfilesParts::None => "/_security/profile/_suggest".into(),
        }
    }
}
#[doc = "Builder for the [Security Suggest User Profiles API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-suggest-user-profile.html)\n\nGet suggestions for user profiles that match specified search criteria."]
#[derive(Clone, Debug)]
pub struct SecuritySuggestUserProfiles<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecuritySuggestUserProfilesParts,
    body: Option<B>,
    data: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecuritySuggestUserProfiles<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecuritySuggestUserProfiles]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecuritySuggestUserProfiles {
            transport,
            parts: SecuritySuggestUserProfilesParts::None,
            headers,
            body: None,
            data: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecuritySuggestUserProfiles<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecuritySuggestUserProfiles {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            data: self.data,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "A comma-separated list of keys for which the corresponding application data are retrieved."]
    pub fn data(mut self, data: &'b [&'b str]) -> Self {
        self.data = Some(data);
        self
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
    #[doc = "Creates an asynchronous call to the Security Suggest User Profiles API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => http::Method::Post,
            None => http::Method::Get,
        };
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                data: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                data: self.data,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Update Api Key API"]
pub enum SecurityUpdateApiKeyParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> SecurityUpdateApiKeyParts<'b> {
    #[doc = "Builds a relative URL path to the Security Update Api Key API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityUpdateApiKeyParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_id.len());
                p.push_str("/_security/api_key/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Update Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-update-api-key.html)\n\nUpdates attributes of an existing API key."]
#[derive(Clone, Debug)]
pub struct SecurityUpdateApiKey<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityUpdateApiKeyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityUpdateApiKey<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityUpdateApiKey] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityUpdateApiKeyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityUpdateApiKey {
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
    pub fn body<T>(self, body: T) -> SecurityUpdateApiKey<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityUpdateApiKey {
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
    #[doc = "Creates an asynchronous call to the Security Update Api Key API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
#[cfg(feature = "beta-apis")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Update Cross Cluster Api Key API"]
pub enum SecurityUpdateCrossClusterApiKeyParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
#[cfg(feature = "beta-apis")]
impl<'b> SecurityUpdateCrossClusterApiKeyParts<'b> {
    #[doc = "Builds a relative URL path to the Security Update Cross Cluster Api Key API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityUpdateCrossClusterApiKeyParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_id.len());
                p.push_str("/_security/cross_cluster/api_key/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Update Cross Cluster Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-update-cross-cluster-api-key.html)\n\nUpdates attributes of an existing cross-cluster API key."]
#[doc = "&nbsp;\n# Optional, beta\nThis requires the `beta-apis` feature. On track to become stable but breaking changes can\nhappen in minor versions.\n        "]
#[cfg(feature = "beta-apis")]
#[derive(Clone, Debug)]
pub struct SecurityUpdateCrossClusterApiKey<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityUpdateCrossClusterApiKeyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[cfg(feature = "beta-apis")]
impl<'a, 'b, B> SecurityUpdateCrossClusterApiKey<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityUpdateCrossClusterApiKey] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityUpdateCrossClusterApiKeyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityUpdateCrossClusterApiKey {
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
    pub fn body<T>(self, body: T) -> SecurityUpdateCrossClusterApiKey<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityUpdateCrossClusterApiKey {
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
    #[doc = "Creates an asynchronous call to the Security Update Cross Cluster Api Key API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Update Settings API"]
pub enum SecurityUpdateSettingsParts {
    #[doc = "No parts"]
    None,
}
impl SecurityUpdateSettingsParts {
    #[doc = "Builds a relative URL path to the Security Update Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityUpdateSettingsParts::None => "/_security/settings".into(),
        }
    }
}
#[doc = "Builder for the [Security Update Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-update-settings.html)\n\nUpdate settings for the security system index"]
#[derive(Clone, Debug)]
pub struct SecurityUpdateSettings<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityUpdateSettingsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> SecurityUpdateSettings<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityUpdateSettings]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityUpdateSettings {
            transport,
            parts: SecurityUpdateSettingsParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityUpdateSettings<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityUpdateSettings {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
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
    #[doc = "Timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Timeout for acknowledgements from all nodes"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Update Settings API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Put;
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
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Security Update User Profile Data API"]
pub enum SecurityUpdateUserProfileDataParts<'b> {
    #[doc = "Uid"]
    Uid(&'b str),
}
impl<'b> SecurityUpdateUserProfileDataParts<'b> {
    #[doc = "Builds a relative URL path to the Security Update User Profile Data API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityUpdateUserProfileDataParts::Uid(uid) => {
                let encoded_uid: Cow<str> = percent_encode(uid.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(25usize + encoded_uid.len());
                p.push_str("/_security/profile/");
                p.push_str(encoded_uid.as_ref());
                p.push_str("/_data");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Update User Profile Data API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-update-user-profile-data.html)\n\nUpdate application specific data for the user profile of the given unique ID."]
#[derive(Clone, Debug)]
pub struct SecurityUpdateUserProfileData<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityUpdateUserProfileDataParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityUpdateUserProfileData<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityUpdateUserProfileData] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityUpdateUserProfileDataParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityUpdateUserProfileData {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityUpdateUserProfileData<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityUpdateUserProfileData {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            if_primary_term: self.if_primary_term,
            if_seq_no: self.if_seq_no,
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
    #[doc = "only perform the update operation if the last operation that has changed the document has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "only perform the update operation if the last operation that has changed the document has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
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
    #[doc = "Creates an asynchronous call to the Security Update User Profile Data API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
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
                if_primary_term: Option<i64>,
                if_seq_no: Option<i64>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
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
    #[doc = "[Security Activate User Profile API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-activate-user-profile.html)\n\nCreates or updates the user profile on behalf of another user."]
    pub fn activate_user_profile<'b>(&'a self) -> SecurityActivateUserProfile<'a, 'b, ()> {
        SecurityActivateUserProfile::new(self.transport())
    }
    #[doc = "[Security Authenticate API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-authenticate.html)\n\nEnables authentication as a user and retrieve information about the authenticated user."]
    pub fn authenticate<'b>(&'a self) -> SecurityAuthenticate<'a, 'b> {
        SecurityAuthenticate::new(self.transport())
    }
    #[doc = "[Security Bulk Delete Role API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-bulk-delete-role.html)\n\nBulk delete roles in the native realm."]
    pub fn bulk_delete_role<'b>(&'a self) -> SecurityBulkDeleteRole<'a, 'b, ()> {
        SecurityBulkDeleteRole::new(self.transport())
    }
    #[doc = "[Security Bulk Put Role API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-bulk-put-role.html)\n\nBulk adds and updates roles in the native realm."]
    pub fn bulk_put_role<'b>(&'a self) -> SecurityBulkPutRole<'a, 'b, ()> {
        SecurityBulkPutRole::new(self.transport())
    }
    #[doc = "[Security Bulk Update Api Keys API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-bulk-update-api-keys.html)\n\nUpdates the attributes of multiple existing API keys."]
    pub fn bulk_update_api_keys<'b>(&'a self) -> SecurityBulkUpdateApiKeys<'a, 'b, ()> {
        SecurityBulkUpdateApiKeys::new(self.transport())
    }
    #[doc = "[Security Change Password API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-change-password.html)\n\nChanges the passwords of users in the native realm and built-in users."]
    pub fn change_password<'b>(
        &'a self,
        parts: SecurityChangePasswordParts<'b>,
    ) -> SecurityChangePassword<'a, 'b, ()> {
        SecurityChangePassword::new(self.transport(), parts)
    }
    #[doc = "[Security Clear Api Key Cache API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-clear-api-key-cache.html)\n\nClear a subset or all entries from the API key cache."]
    pub fn clear_api_key_cache<'b>(
        &'a self,
        parts: SecurityClearApiKeyCacheParts<'b>,
    ) -> SecurityClearApiKeyCache<'a, 'b, ()> {
        SecurityClearApiKeyCache::new(self.transport(), parts)
    }
    #[doc = "[Security Clear Cached Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-clear-privilege-cache.html)\n\nEvicts application privileges from the native application privileges cache."]
    pub fn clear_cached_privileges<'b>(
        &'a self,
        parts: SecurityClearCachedPrivilegesParts<'b>,
    ) -> SecurityClearCachedPrivileges<'a, 'b, ()> {
        SecurityClearCachedPrivileges::new(self.transport(), parts)
    }
    #[doc = "[Security Clear Cached Realms API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-clear-cache.html)\n\nEvicts users from the user cache. Can completely clear the cache or evict specific users."]
    pub fn clear_cached_realms<'b>(
        &'a self,
        parts: SecurityClearCachedRealmsParts<'b>,
    ) -> SecurityClearCachedRealms<'a, 'b, ()> {
        SecurityClearCachedRealms::new(self.transport(), parts)
    }
    #[doc = "[Security Clear Cached Roles API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-clear-role-cache.html)\n\nEvicts roles from the native role cache."]
    pub fn clear_cached_roles<'b>(
        &'a self,
        parts: SecurityClearCachedRolesParts<'b>,
    ) -> SecurityClearCachedRoles<'a, 'b, ()> {
        SecurityClearCachedRoles::new(self.transport(), parts)
    }
    #[doc = "[Security Clear Cached Service Tokens API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-clear-service-token-caches.html)\n\nEvicts tokens from the service account token caches."]
    pub fn clear_cached_service_tokens<'b>(
        &'a self,
        parts: SecurityClearCachedServiceTokensParts<'b>,
    ) -> SecurityClearCachedServiceTokens<'a, 'b, ()> {
        SecurityClearCachedServiceTokens::new(self.transport(), parts)
    }
    #[doc = "[Security Create Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-create-api-key.html)\n\nCreates an API key for access without requiring basic authentication."]
    pub fn create_api_key<'b>(&'a self) -> SecurityCreateApiKey<'a, 'b, ()> {
        SecurityCreateApiKey::new(self.transport())
    }
    #[doc = "[Security Create Cross Cluster Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-create-cross-cluster-api-key.html)\n\nCreates a cross-cluster API key for API key based remote cluster access."]
    #[doc = "&nbsp;\n# Optional, beta\nThis requires the `beta-apis` feature. On track to become stable but breaking changes can\nhappen in minor versions.\n        "]
    #[cfg(feature = "beta-apis")]
    pub fn create_cross_cluster_api_key<'b>(
        &'a self,
    ) -> SecurityCreateCrossClusterApiKey<'a, 'b, ()> {
        SecurityCreateCrossClusterApiKey::new(self.transport())
    }
    #[doc = "[Security Create Service Token API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-create-service-token.html)\n\nCreates a service account token for access without requiring basic authentication."]
    pub fn create_service_token<'b>(
        &'a self,
        parts: SecurityCreateServiceTokenParts<'b>,
    ) -> SecurityCreateServiceToken<'a, 'b, ()> {
        SecurityCreateServiceToken::new(self.transport(), parts)
    }
    #[doc = "[Security Delete Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-delete-privilege.html)\n\nRemoves application privileges."]
    pub fn delete_privileges<'b>(
        &'a self,
        parts: SecurityDeletePrivilegesParts<'b>,
    ) -> SecurityDeletePrivileges<'a, 'b> {
        SecurityDeletePrivileges::new(self.transport(), parts)
    }
    #[doc = "[Security Delete Role API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-delete-role.html)\n\nRemoves roles in the native realm."]
    pub fn delete_role<'b>(
        &'a self,
        parts: SecurityDeleteRoleParts<'b>,
    ) -> SecurityDeleteRole<'a, 'b> {
        SecurityDeleteRole::new(self.transport(), parts)
    }
    #[doc = "[Security Delete Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-delete-role-mapping.html)\n\nRemoves role mappings."]
    pub fn delete_role_mapping<'b>(
        &'a self,
        parts: SecurityDeleteRoleMappingParts<'b>,
    ) -> SecurityDeleteRoleMapping<'a, 'b> {
        SecurityDeleteRoleMapping::new(self.transport(), parts)
    }
    #[doc = "[Security Delete Service Token API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-delete-service-token.html)\n\nDeletes a service account token."]
    pub fn delete_service_token<'b>(
        &'a self,
        parts: SecurityDeleteServiceTokenParts<'b>,
    ) -> SecurityDeleteServiceToken<'a, 'b> {
        SecurityDeleteServiceToken::new(self.transport(), parts)
    }
    #[doc = "[Security Delete User API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-delete-user.html)\n\nDeletes users from the native realm."]
    pub fn delete_user<'b>(
        &'a self,
        parts: SecurityDeleteUserParts<'b>,
    ) -> SecurityDeleteUser<'a, 'b> {
        SecurityDeleteUser::new(self.transport(), parts)
    }
    #[doc = "[Security Disable User API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-disable-user.html)\n\nDisables users in the native realm."]
    pub fn disable_user<'b>(
        &'a self,
        parts: SecurityDisableUserParts<'b>,
    ) -> SecurityDisableUser<'a, 'b, ()> {
        SecurityDisableUser::new(self.transport(), parts)
    }
    #[doc = "[Security Disable User Profile API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-disable-user-profile.html)\n\nDisables a user profile so it's not visible in user profile searches."]
    pub fn disable_user_profile<'b>(
        &'a self,
        parts: SecurityDisableUserProfileParts<'b>,
    ) -> SecurityDisableUserProfile<'a, 'b, ()> {
        SecurityDisableUserProfile::new(self.transport(), parts)
    }
    #[doc = "[Security Enable User API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-enable-user.html)\n\nEnables users in the native realm."]
    pub fn enable_user<'b>(
        &'a self,
        parts: SecurityEnableUserParts<'b>,
    ) -> SecurityEnableUser<'a, 'b, ()> {
        SecurityEnableUser::new(self.transport(), parts)
    }
    #[doc = "[Security Enable User Profile API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-enable-user-profile.html)\n\nEnables a user profile so it's visible in user profile searches."]
    pub fn enable_user_profile<'b>(
        &'a self,
        parts: SecurityEnableUserProfileParts<'b>,
    ) -> SecurityEnableUserProfile<'a, 'b, ()> {
        SecurityEnableUserProfile::new(self.transport(), parts)
    }
    #[doc = "[Security Enroll Kibana API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-kibana-enrollment.html)\n\nAllows a kibana instance to configure itself to communicate with a secured elasticsearch cluster."]
    pub fn enroll_kibana<'b>(&'a self) -> SecurityEnrollKibana<'a, 'b> {
        SecurityEnrollKibana::new(self.transport())
    }
    #[doc = "[Security Enroll Node API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-node-enrollment.html)\n\nAllows a new node to enroll to an existing cluster with security enabled."]
    pub fn enroll_node<'b>(&'a self) -> SecurityEnrollNode<'a, 'b> {
        SecurityEnrollNode::new(self.transport())
    }
    #[doc = "[Security Get Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-api-key.html)\n\nRetrieves information for one or more API keys."]
    pub fn get_api_key<'b>(&'a self) -> SecurityGetApiKey<'a, 'b> {
        SecurityGetApiKey::new(self.transport())
    }
    #[doc = "[Security Get Builtin Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-builtin-privileges.html)\n\nRetrieves the list of cluster privileges and index privileges that are available in this version of Elasticsearch."]
    pub fn get_builtin_privileges<'b>(&'a self) -> SecurityGetBuiltinPrivileges<'a, 'b> {
        SecurityGetBuiltinPrivileges::new(self.transport())
    }
    #[doc = "[Security Get Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-privileges.html)\n\nRetrieves application privileges."]
    pub fn get_privileges<'b>(
        &'a self,
        parts: SecurityGetPrivilegesParts<'b>,
    ) -> SecurityGetPrivileges<'a, 'b> {
        SecurityGetPrivileges::new(self.transport(), parts)
    }
    #[doc = "[Security Get Role API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-role.html)\n\nRetrieves roles in the native realm."]
    pub fn get_role<'b>(&'a self, parts: SecurityGetRoleParts<'b>) -> SecurityGetRole<'a, 'b> {
        SecurityGetRole::new(self.transport(), parts)
    }
    #[doc = "[Security Get Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-role-mapping.html)\n\nRetrieves role mappings."]
    pub fn get_role_mapping<'b>(
        &'a self,
        parts: SecurityGetRoleMappingParts<'b>,
    ) -> SecurityGetRoleMapping<'a, 'b> {
        SecurityGetRoleMapping::new(self.transport(), parts)
    }
    #[doc = "[Security Get Service Accounts API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-service-accounts.html)\n\nRetrieves information about service accounts."]
    pub fn get_service_accounts<'b>(
        &'a self,
        parts: SecurityGetServiceAccountsParts<'b>,
    ) -> SecurityGetServiceAccounts<'a, 'b> {
        SecurityGetServiceAccounts::new(self.transport(), parts)
    }
    #[doc = "[Security Get Service Credentials API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-service-credentials.html)\n\nRetrieves information of all service credentials for a service account."]
    pub fn get_service_credentials<'b>(
        &'a self,
        parts: SecurityGetServiceCredentialsParts<'b>,
    ) -> SecurityGetServiceCredentials<'a, 'b> {
        SecurityGetServiceCredentials::new(self.transport(), parts)
    }
    #[doc = "[Security Get Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-settings.html)\n\nRetrieve settings for the security system indices"]
    pub fn get_settings<'b>(&'a self) -> SecurityGetSettings<'a, 'b> {
        SecurityGetSettings::new(self.transport())
    }
    #[doc = "[Security Get Token API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-token.html)\n\nCreates a bearer token for access without requiring basic authentication."]
    pub fn get_token<'b>(&'a self) -> SecurityGetToken<'a, 'b, ()> {
        SecurityGetToken::new(self.transport())
    }
    #[doc = "[Security Get User API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-user.html)\n\nRetrieves information about users in the native realm and built-in users."]
    pub fn get_user<'b>(&'a self, parts: SecurityGetUserParts<'b>) -> SecurityGetUser<'a, 'b> {
        SecurityGetUser::new(self.transport(), parts)
    }
    #[doc = "[Security Get User Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-user-privileges.html)\n\nRetrieves security privileges for the logged in user."]
    pub fn get_user_privileges<'b>(&'a self) -> SecurityGetUserPrivileges<'a, 'b> {
        SecurityGetUserPrivileges::new(self.transport())
    }
    #[doc = "[Security Get User Profile API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-get-user-profile.html)\n\nRetrieves user profiles for the given unique ID(s)."]
    pub fn get_user_profile<'b>(
        &'a self,
        parts: SecurityGetUserProfileParts<'b>,
    ) -> SecurityGetUserProfile<'a, 'b> {
        SecurityGetUserProfile::new(self.transport(), parts)
    }
    #[doc = "[Security Grant Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-grant-api-key.html)\n\nCreates an API key on behalf of another user."]
    pub fn grant_api_key<'b>(&'a self) -> SecurityGrantApiKey<'a, 'b, ()> {
        SecurityGrantApiKey::new(self.transport())
    }
    #[doc = "[Security Has Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-has-privileges.html)\n\nDetermines whether the specified user has a specified list of privileges."]
    pub fn has_privileges<'b>(
        &'a self,
        parts: SecurityHasPrivilegesParts<'b>,
    ) -> SecurityHasPrivileges<'a, 'b, ()> {
        SecurityHasPrivileges::new(self.transport(), parts)
    }
    #[doc = "[Security Has Privileges User Profile API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-has-privileges-user-profile.html)\n\nDetermines whether the users associated with the specified profile IDs have all the requested privileges."]
    pub fn has_privileges_user_profile<'b>(
        &'a self,
    ) -> SecurityHasPrivilegesUserProfile<'a, 'b, ()> {
        SecurityHasPrivilegesUserProfile::new(self.transport())
    }
    #[doc = "[Security Invalidate Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-invalidate-api-key.html)\n\nInvalidates one or more API keys."]
    pub fn invalidate_api_key<'b>(&'a self) -> SecurityInvalidateApiKey<'a, 'b, ()> {
        SecurityInvalidateApiKey::new(self.transport())
    }
    #[doc = "[Security Invalidate Token API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-invalidate-token.html)\n\nInvalidates one or more access tokens or refresh tokens."]
    pub fn invalidate_token<'b>(&'a self) -> SecurityInvalidateToken<'a, 'b, ()> {
        SecurityInvalidateToken::new(self.transport())
    }
    #[doc = "[Security Oidc Authenticate API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-oidc-authenticate.html)\n\nExchanges an OpenID Connection authentication response message for an Elasticsearch access token and refresh token pair"]
    pub fn oidc_authenticate<'b>(&'a self) -> SecurityOidcAuthenticate<'a, 'b, ()> {
        SecurityOidcAuthenticate::new(self.transport())
    }
    #[doc = "[Security Oidc Logout API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-oidc-logout.html)\n\nInvalidates a refresh token and access token that was generated from the OpenID Connect Authenticate API"]
    pub fn oidc_logout<'b>(&'a self) -> SecurityOidcLogout<'a, 'b, ()> {
        SecurityOidcLogout::new(self.transport())
    }
    #[doc = "[Security Oidc Prepare Authentication API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-oidc-prepare-authentication.html)\n\nCreates an OAuth 2.0 authentication request as a URL string"]
    pub fn oidc_prepare_authentication<'b>(
        &'a self,
    ) -> SecurityOidcPrepareAuthentication<'a, 'b, ()> {
        SecurityOidcPrepareAuthentication::new(self.transport())
    }
    #[doc = "[Security Put Privileges API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-put-privileges.html)\n\nAdds or updates application privileges."]
    pub fn put_privileges<'b>(&'a self) -> SecurityPutPrivileges<'a, 'b, ()> {
        SecurityPutPrivileges::new(self.transport())
    }
    #[doc = "[Security Put Role API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-put-role.html)\n\nAdds and updates roles in the native realm."]
    pub fn put_role<'b>(&'a self, parts: SecurityPutRoleParts<'b>) -> SecurityPutRole<'a, 'b, ()> {
        SecurityPutRole::new(self.transport(), parts)
    }
    #[doc = "[Security Put Role Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-put-role-mapping.html)\n\nCreates and updates role mappings."]
    pub fn put_role_mapping<'b>(
        &'a self,
        parts: SecurityPutRoleMappingParts<'b>,
    ) -> SecurityPutRoleMapping<'a, 'b, ()> {
        SecurityPutRoleMapping::new(self.transport(), parts)
    }
    #[doc = "[Security Put User API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-put-user.html)\n\nAdds and updates users in the native realm. These users are commonly referred to as native users."]
    pub fn put_user<'b>(&'a self, parts: SecurityPutUserParts<'b>) -> SecurityPutUser<'a, 'b, ()> {
        SecurityPutUser::new(self.transport(), parts)
    }
    #[doc = "[Security Query Api Keys API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-query-api-key.html)\n\nRetrieves information for API keys using a subset of query DSL"]
    pub fn query_api_keys<'b>(&'a self) -> SecurityQueryApiKeys<'a, 'b, ()> {
        SecurityQueryApiKeys::new(self.transport())
    }
    #[doc = "[Security Query Role API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-query-role.html)\n\nRetrieves information for Roles using a subset of query DSL"]
    pub fn query_role<'b>(&'a self) -> SecurityQueryRole<'a, 'b, ()> {
        SecurityQueryRole::new(self.transport())
    }
    #[doc = "[Security Query User API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-query-user.html)\n\nRetrieves information for Users using a subset of query DSL"]
    pub fn query_user<'b>(&'a self) -> SecurityQueryUser<'a, 'b, ()> {
        SecurityQueryUser::new(self.transport())
    }
    #[doc = "[Security Saml Authenticate API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-saml-authenticate.html)\n\nExchanges a SAML Response message for an Elasticsearch access token and refresh token pair"]
    pub fn saml_authenticate<'b>(&'a self) -> SecuritySamlAuthenticate<'a, 'b, ()> {
        SecuritySamlAuthenticate::new(self.transport())
    }
    #[doc = "[Security Saml Complete Logout API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-saml-complete-logout.html)\n\nVerifies the logout response sent from the SAML IdP"]
    pub fn saml_complete_logout<'b>(&'a self) -> SecuritySamlCompleteLogout<'a, 'b, ()> {
        SecuritySamlCompleteLogout::new(self.transport())
    }
    #[doc = "[Security Saml Invalidate API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-saml-invalidate.html)\n\nConsumes a SAML LogoutRequest"]
    pub fn saml_invalidate<'b>(&'a self) -> SecuritySamlInvalidate<'a, 'b, ()> {
        SecuritySamlInvalidate::new(self.transport())
    }
    #[doc = "[Security Saml Logout API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-saml-logout.html)\n\nInvalidates an access token and a refresh token that were generated via the SAML Authenticate API"]
    pub fn saml_logout<'b>(&'a self) -> SecuritySamlLogout<'a, 'b, ()> {
        SecuritySamlLogout::new(self.transport())
    }
    #[doc = "[Security Saml Prepare Authentication API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-saml-prepare-authentication.html)\n\nCreates a SAML authentication request"]
    pub fn saml_prepare_authentication<'b>(
        &'a self,
    ) -> SecuritySamlPrepareAuthentication<'a, 'b, ()> {
        SecuritySamlPrepareAuthentication::new(self.transport())
    }
    #[doc = "[Security Saml Service Provider Metadata API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-saml-sp-metadata.html)\n\nGenerates SAML metadata for the Elastic stack SAML 2.0 Service Provider"]
    pub fn saml_service_provider_metadata<'b>(
        &'a self,
        parts: SecuritySamlServiceProviderMetadataParts<'b>,
    ) -> SecuritySamlServiceProviderMetadata<'a, 'b> {
        SecuritySamlServiceProviderMetadata::new(self.transport(), parts)
    }
    #[doc = "[Security Suggest User Profiles API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-suggest-user-profile.html)\n\nGet suggestions for user profiles that match specified search criteria."]
    pub fn suggest_user_profiles<'b>(&'a self) -> SecuritySuggestUserProfiles<'a, 'b, ()> {
        SecuritySuggestUserProfiles::new(self.transport())
    }
    #[doc = "[Security Update Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-update-api-key.html)\n\nUpdates attributes of an existing API key."]
    pub fn update_api_key<'b>(
        &'a self,
        parts: SecurityUpdateApiKeyParts<'b>,
    ) -> SecurityUpdateApiKey<'a, 'b, ()> {
        SecurityUpdateApiKey::new(self.transport(), parts)
    }
    #[doc = "[Security Update Cross Cluster Api Key API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-update-cross-cluster-api-key.html)\n\nUpdates attributes of an existing cross-cluster API key."]
    #[doc = "&nbsp;\n# Optional, beta\nThis requires the `beta-apis` feature. On track to become stable but breaking changes can\nhappen in minor versions.\n        "]
    #[cfg(feature = "beta-apis")]
    pub fn update_cross_cluster_api_key<'b>(
        &'a self,
        parts: SecurityUpdateCrossClusterApiKeyParts<'b>,
    ) -> SecurityUpdateCrossClusterApiKey<'a, 'b, ()> {
        SecurityUpdateCrossClusterApiKey::new(self.transport(), parts)
    }
    #[doc = "[Security Update Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-update-settings.html)\n\nUpdate settings for the security system index"]
    pub fn update_settings<'b>(&'a self) -> SecurityUpdateSettings<'a, 'b, ()> {
        SecurityUpdateSettings::new(self.transport())
    }
    #[doc = "[Security Update User Profile Data API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/security-api-update-user-profile-data.html)\n\nUpdate application specific data for the user profile of the given unique ID."]
    pub fn update_user_profile_data<'b>(
        &'a self,
        parts: SecurityUpdateUserProfileDataParts<'b>,
    ) -> SecurityUpdateUserProfileData<'a, 'b, ()> {
        SecurityUpdateUserProfileData::new(self.transport(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Security APIs"]
    pub fn security(&self) -> Security {
        Security::new(self.transport())
    }
}
