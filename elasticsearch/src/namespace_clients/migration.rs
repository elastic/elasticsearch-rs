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
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Migration Deprecations API"]
pub enum MigrationDeprecationsUrlParts {
    None,
    Index(String),
}
impl MigrationDeprecationsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MigrationDeprecationsUrlParts::None => "/_migration/deprecations".into(),
            MigrationDeprecationsUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(25usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_migration/deprecations");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Migration Deprecations API"]
pub struct MigrationDeprecations {
    client: Elasticsearch,
    parts: MigrationDeprecationsUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MigrationDeprecations {
    pub fn new(client: Elasticsearch, parts: MigrationDeprecationsUrlParts) -> Self {
        MigrationDeprecations {
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
impl Sender for MigrationDeprecations {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[doc = "Migration APIs"]
pub struct Migration {
    client: Elasticsearch,
}
impl Migration {
    pub fn new(client: Elasticsearch) -> Self {
        Migration { client }
    }
    pub fn deprecations(&self, parts: MigrationDeprecationsUrlParts) -> MigrationDeprecations {
        MigrationDeprecations::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Migration APIs"]
    pub fn migration(&self) -> Migration {
        Migration::new(self.clone())
    }
}
