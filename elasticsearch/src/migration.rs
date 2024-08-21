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

//! Migration APIs
//!
//! [Simplify upgrading X-Pack indices from one version to another](https://www.elastic.co/guide/en/elasticsearch/reference/master/migration-api.html).

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
#[doc = "API parts for the Migration Deprecations API"]
pub enum MigrationDeprecationsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> MigrationDeprecationsParts<'b> {
    #[doc = "Builds a relative URL path to the Migration Deprecations API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MigrationDeprecationsParts::None => "/_migration/deprecations".into(),
            MigrationDeprecationsParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(25usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_migration/deprecations");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Migration Deprecations API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/migration-api-deprecation.html)\n\nRetrieves information about different cluster, node, and index level settings that use deprecated features that will be removed or changed in the next major version."]
#[derive(Clone, Debug)]
pub struct MigrationDeprecations<'a, 'b> {
    transport: &'a Transport,
    parts: MigrationDeprecationsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MigrationDeprecations<'a, 'b> {
    #[doc = "Creates a new instance of [MigrationDeprecations] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MigrationDeprecationsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MigrationDeprecations {
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
    #[doc = "Creates an asynchronous call to the Migration Deprecations API that can be awaited"]
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
#[doc = "API parts for the Migration Get Feature Upgrade Status API"]
pub enum MigrationGetFeatureUpgradeStatusParts {
    #[doc = "No parts"]
    None,
}
impl MigrationGetFeatureUpgradeStatusParts {
    #[doc = "Builds a relative URL path to the Migration Get Feature Upgrade Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MigrationGetFeatureUpgradeStatusParts::None => "/_migration/system_features".into(),
        }
    }
}
#[doc = "Builder for the [Migration Get Feature Upgrade Status API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/migration-api-feature-upgrade.html)\n\nFind out whether system features need to be upgraded or not"]
#[derive(Clone, Debug)]
pub struct MigrationGetFeatureUpgradeStatus<'a, 'b> {
    transport: &'a Transport,
    parts: MigrationGetFeatureUpgradeStatusParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MigrationGetFeatureUpgradeStatus<'a, 'b> {
    #[doc = "Creates a new instance of [MigrationGetFeatureUpgradeStatus]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MigrationGetFeatureUpgradeStatus {
            transport,
            parts: MigrationGetFeatureUpgradeStatusParts::None,
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
    #[doc = "Creates an asynchronous call to the Migration Get Feature Upgrade Status API that can be awaited"]
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
#[doc = "API parts for the Migration Post Feature Upgrade API"]
pub enum MigrationPostFeatureUpgradeParts {
    #[doc = "No parts"]
    None,
}
impl MigrationPostFeatureUpgradeParts {
    #[doc = "Builds a relative URL path to the Migration Post Feature Upgrade API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MigrationPostFeatureUpgradeParts::None => "/_migration/system_features".into(),
        }
    }
}
#[doc = "Builder for the [Migration Post Feature Upgrade API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/migration-api-feature-upgrade.html)\n\nBegin upgrades for system features"]
#[derive(Clone, Debug)]
pub struct MigrationPostFeatureUpgrade<'a, 'b, B> {
    transport: &'a Transport,
    parts: MigrationPostFeatureUpgradeParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MigrationPostFeatureUpgrade<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MigrationPostFeatureUpgrade]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MigrationPostFeatureUpgrade {
            transport,
            parts: MigrationPostFeatureUpgradeParts::None,
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
    pub fn body<T>(self, body: T) -> MigrationPostFeatureUpgrade<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MigrationPostFeatureUpgrade {
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
    #[doc = "Creates an asynchronous call to the Migration Post Feature Upgrade API that can be awaited"]
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
#[doc = "Namespace client for Migration APIs"]
pub struct Migration<'a> {
    transport: &'a Transport,
}
impl<'a> Migration<'a> {
    #[doc = "Creates a new instance of [Migration]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Migration Deprecations API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/migration-api-deprecation.html)\n\nRetrieves information about different cluster, node, and index level settings that use deprecated features that will be removed or changed in the next major version."]
    pub fn deprecations<'b>(
        &'a self,
        parts: MigrationDeprecationsParts<'b>,
    ) -> MigrationDeprecations<'a, 'b> {
        MigrationDeprecations::new(self.transport(), parts)
    }
    #[doc = "[Migration Get Feature Upgrade Status API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/migration-api-feature-upgrade.html)\n\nFind out whether system features need to be upgraded or not"]
    pub fn get_feature_upgrade_status<'b>(&'a self) -> MigrationGetFeatureUpgradeStatus<'a, 'b> {
        MigrationGetFeatureUpgradeStatus::new(self.transport())
    }
    #[doc = "[Migration Post Feature Upgrade API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/migration-api-feature-upgrade.html)\n\nBegin upgrades for system features"]
    pub fn post_feature_upgrade<'b>(&'a self) -> MigrationPostFeatureUpgrade<'a, 'b, ()> {
        MigrationPostFeatureUpgrade::new(self.transport())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Migration APIs"]
    pub fn migration(&self) -> Migration {
        Migration::new(self.transport())
    }
}
