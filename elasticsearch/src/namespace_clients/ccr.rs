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
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ccr Delete Auto Follow Pattern API"]
pub enum CcrDeleteAutoFollowPatternUrlParts<'a> {
    Name(&'a str),
}
impl<'a> CcrDeleteAutoFollowPatternUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CcrDeleteAutoFollowPatternUrlParts::Name(ref name) => {
                let mut p = String::with_capacity(18usize + name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ccr Delete Auto Follow Pattern API"]
pub struct CcrDeleteAutoFollowPattern<'a> {
    client: Elasticsearch,
    parts: CcrDeleteAutoFollowPatternUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> CcrDeleteAutoFollowPattern<'a> {
    pub fn new(client: Elasticsearch, parts: CcrDeleteAutoFollowPatternUrlParts<'a>) -> Self {
        CcrDeleteAutoFollowPattern {
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
    #[doc = "Creates an asynchronous request to the Ccr Delete Auto Follow Pattern API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ccr Follow API"]
pub enum CcrFollowUrlParts<'a> {
    Index(&'a str),
}
impl<'a> CcrFollowUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CcrFollowUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(13usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_ccr/follow");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ccr Follow API"]
pub struct CcrFollow<'a, B> {
    client: Elasticsearch,
    parts: CcrFollowUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> CcrFollow<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: CcrFollowUrlParts<'a>) -> Self {
        CcrFollow {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrFollow<'a, T>
    where
        T: Serialize,
    {
        CcrFollow {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            wait_for_active_shards: self.wait_for_active_shards,
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
    #[doc = "Sets the number of shard copies that must be active before returning. Defaults to 0. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous request to the Ccr Follow API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(
                    rename = "wait_for_active_shards",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                wait_for_active_shards: self.wait_for_active_shards,
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
#[doc = "Url parts for the Ccr Follow Info API"]
pub enum CcrFollowInfoUrlParts<'a> {
    Index(&'a [&'a str]),
}
impl<'a> CcrFollowInfoUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CcrFollowInfoUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(11usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_ccr/info");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ccr Follow Info API"]
pub struct CcrFollowInfo<'a> {
    client: Elasticsearch,
    parts: CcrFollowInfoUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> CcrFollowInfo<'a> {
    pub fn new(client: Elasticsearch, parts: CcrFollowInfoUrlParts<'a>) -> Self {
        CcrFollowInfo {
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
    #[doc = "Creates an asynchronous request to the Ccr Follow Info API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ccr Follow Stats API"]
pub enum CcrFollowStatsUrlParts<'a> {
    Index(&'a [&'a str]),
}
impl<'a> CcrFollowStatsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CcrFollowStatsUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(12usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_ccr/stats");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ccr Follow Stats API"]
pub struct CcrFollowStats<'a> {
    client: Elasticsearch,
    parts: CcrFollowStatsUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> CcrFollowStats<'a> {
    pub fn new(client: Elasticsearch, parts: CcrFollowStatsUrlParts<'a>) -> Self {
        CcrFollowStats {
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
    #[doc = "Creates an asynchronous request to the Ccr Follow Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ccr Forget Follower API"]
pub enum CcrForgetFollowerUrlParts<'a> {
    Index(&'a str),
}
impl<'a> CcrForgetFollowerUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CcrForgetFollowerUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(22usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_ccr/forget_follower");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ccr Forget Follower API"]
pub struct CcrForgetFollower<'a, B> {
    client: Elasticsearch,
    parts: CcrForgetFollowerUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> CcrForgetFollower<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: CcrForgetFollowerUrlParts<'a>) -> Self {
        CcrForgetFollower {
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
    pub fn body<T>(self, body: T) -> CcrForgetFollower<'a, T>
    where
        T: Serialize,
    {
        CcrForgetFollower {
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
    #[doc = "Creates an asynchronous request to the Ccr Forget Follower API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ccr Get Auto Follow Pattern API"]
pub enum CcrGetAutoFollowPatternUrlParts<'a> {
    None,
    Name(&'a str),
}
impl<'a> CcrGetAutoFollowPatternUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CcrGetAutoFollowPatternUrlParts::None => "/_ccr/auto_follow".into(),
            CcrGetAutoFollowPatternUrlParts::Name(ref name) => {
                let mut p = String::with_capacity(18usize + name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ccr Get Auto Follow Pattern API"]
pub struct CcrGetAutoFollowPattern<'a> {
    client: Elasticsearch,
    parts: CcrGetAutoFollowPatternUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> CcrGetAutoFollowPattern<'a> {
    pub fn new(client: Elasticsearch, parts: CcrGetAutoFollowPatternUrlParts<'a>) -> Self {
        CcrGetAutoFollowPattern {
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
    #[doc = "Creates an asynchronous request to the Ccr Get Auto Follow Pattern API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ccr Pause Follow API"]
pub enum CcrPauseFollowUrlParts<'a> {
    Index(&'a str),
}
impl<'a> CcrPauseFollowUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CcrPauseFollowUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(19usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_ccr/pause_follow");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ccr Pause Follow API"]
pub struct CcrPauseFollow<'a, B> {
    client: Elasticsearch,
    parts: CcrPauseFollowUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> CcrPauseFollow<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: CcrPauseFollowUrlParts<'a>) -> Self {
        CcrPauseFollow {
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
    pub fn body<T>(self, body: T) -> CcrPauseFollow<'a, T>
    where
        T: Serialize,
    {
        CcrPauseFollow {
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
    #[doc = "Creates an asynchronous request to the Ccr Pause Follow API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ccr Put Auto Follow Pattern API"]
pub enum CcrPutAutoFollowPatternUrlParts<'a> {
    Name(&'a str),
}
impl<'a> CcrPutAutoFollowPatternUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CcrPutAutoFollowPatternUrlParts::Name(ref name) => {
                let mut p = String::with_capacity(18usize + name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ccr Put Auto Follow Pattern API"]
pub struct CcrPutAutoFollowPattern<'a, B> {
    client: Elasticsearch,
    parts: CcrPutAutoFollowPatternUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> CcrPutAutoFollowPattern<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: CcrPutAutoFollowPatternUrlParts<'a>) -> Self {
        CcrPutAutoFollowPattern {
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
    pub fn body<T>(self, body: T) -> CcrPutAutoFollowPattern<'a, T>
    where
        T: Serialize,
    {
        CcrPutAutoFollowPattern {
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
    #[doc = "Creates an asynchronous request to the Ccr Put Auto Follow Pattern API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ccr Resume Follow API"]
pub enum CcrResumeFollowUrlParts<'a> {
    Index(&'a str),
}
impl<'a> CcrResumeFollowUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CcrResumeFollowUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(20usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_ccr/resume_follow");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ccr Resume Follow API"]
pub struct CcrResumeFollow<'a, B> {
    client: Elasticsearch,
    parts: CcrResumeFollowUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> CcrResumeFollow<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: CcrResumeFollowUrlParts<'a>) -> Self {
        CcrResumeFollow {
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
    pub fn body<T>(self, body: T) -> CcrResumeFollow<'a, T>
    where
        T: Serialize,
    {
        CcrResumeFollow {
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
    #[doc = "Creates an asynchronous request to the Ccr Resume Follow API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ccr Stats API"]
pub enum CcrStatsUrlParts {
    None,
}
impl CcrStatsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CcrStatsUrlParts::None => "/_ccr/stats".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ccr Stats API"]
pub struct CcrStats<'a> {
    client: Elasticsearch,
    parts: CcrStatsUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> CcrStats<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        CcrStats {
            client,
            parts: CcrStatsUrlParts::None,
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
    #[doc = "Creates an asynchronous request to the Ccr Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ccr Unfollow API"]
pub enum CcrUnfollowUrlParts<'a> {
    Index(&'a str),
}
impl<'a> CcrUnfollowUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CcrUnfollowUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(15usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_ccr/unfollow");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ccr Unfollow API"]
pub struct CcrUnfollow<'a, B> {
    client: Elasticsearch,
    parts: CcrUnfollowUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> CcrUnfollow<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: CcrUnfollowUrlParts<'a>) -> Self {
        CcrUnfollow {
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
    pub fn body<T>(self, body: T) -> CcrUnfollow<'a, T>
    where
        T: Serialize,
    {
        CcrUnfollow {
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
    #[doc = "Creates an asynchronous request to the Ccr Unfollow API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Ccr APIs"]
pub struct Ccr {
    client: Elasticsearch,
}
impl Ccr {
    pub fn new(client: Elasticsearch) -> Self {
        Ccr { client }
    }
    pub fn delete_auto_follow_pattern<'a>(
        &self,
        parts: CcrDeleteAutoFollowPatternUrlParts<'a>,
    ) -> CcrDeleteAutoFollowPattern<'a> {
        CcrDeleteAutoFollowPattern::new(self.client.clone(), parts)
    }
    pub fn follow<'a>(&self, parts: CcrFollowUrlParts<'a>) -> CcrFollow<'a, ()> {
        CcrFollow::new(self.client.clone(), parts)
    }
    pub fn follow_info<'a>(&self, parts: CcrFollowInfoUrlParts<'a>) -> CcrFollowInfo<'a> {
        CcrFollowInfo::new(self.client.clone(), parts)
    }
    pub fn follow_stats<'a>(&self, parts: CcrFollowStatsUrlParts<'a>) -> CcrFollowStats<'a> {
        CcrFollowStats::new(self.client.clone(), parts)
    }
    pub fn forget_follower<'a>(
        &self,
        parts: CcrForgetFollowerUrlParts<'a>,
    ) -> CcrForgetFollower<'a, ()> {
        CcrForgetFollower::new(self.client.clone(), parts)
    }
    pub fn get_auto_follow_pattern<'a>(
        &self,
        parts: CcrGetAutoFollowPatternUrlParts<'a>,
    ) -> CcrGetAutoFollowPattern<'a> {
        CcrGetAutoFollowPattern::new(self.client.clone(), parts)
    }
    pub fn pause_follow<'a>(&self, parts: CcrPauseFollowUrlParts<'a>) -> CcrPauseFollow<'a, ()> {
        CcrPauseFollow::new(self.client.clone(), parts)
    }
    pub fn put_auto_follow_pattern<'a>(
        &self,
        parts: CcrPutAutoFollowPatternUrlParts<'a>,
    ) -> CcrPutAutoFollowPattern<'a, ()> {
        CcrPutAutoFollowPattern::new(self.client.clone(), parts)
    }
    pub fn resume_follow<'a>(&self, parts: CcrResumeFollowUrlParts<'a>) -> CcrResumeFollow<'a, ()> {
        CcrResumeFollow::new(self.client.clone(), parts)
    }
    pub fn stats<'a>(&self) -> CcrStats<'a> {
        CcrStats::new(self.client.clone())
    }
    pub fn unfollow<'a>(&self, parts: CcrUnfollowUrlParts<'a>) -> CcrUnfollow<'a, ()> {
        CcrUnfollow::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Ccr APIs"]
    pub fn ccr(&self) -> Ccr {
        Ccr::new(self.clone())
    }
}