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
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
    params::*,
};
use serde::Serialize;
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Delete Auto Follow Pattern API"]
pub enum CcrDeleteAutoFollowPatternParts<'a> {
    #[doc = "Name"]
    Name(&'a str),
}
impl<'a> CcrDeleteAutoFollowPatternParts<'a> {
    #[doc = "Builds a relative URL path to the Ccr Delete Auto Follow Pattern API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrDeleteAutoFollowPatternParts::Name(ref name) => {
                let mut p = String::with_capacity(18usize + name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Delete Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-delete-auto-follow-pattern.html)."]
pub struct CcrDeleteAutoFollowPattern<'a> {
    client: Elasticsearch,
    parts: CcrDeleteAutoFollowPatternParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> CcrDeleteAutoFollowPattern<'a> {
    #[doc = "Creates a new instance of [CcrDeleteAutoFollowPattern] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: CcrDeleteAutoFollowPatternParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ccr Delete Auto Follow Pattern API that can be awaited"]
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
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Follow API"]
pub enum CcrFollowParts<'a> {
    #[doc = "Index"]
    Index(&'a str),
}
impl<'a> CcrFollowParts<'a> {
    #[doc = "Builds a relative URL path to the Ccr Follow API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrFollowParts::Index(ref index) => {
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
#[doc = "Builder for the [Ccr Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-put-follow.html)."]
pub struct CcrFollow<'a, B> {
    client: Elasticsearch,
    parts: CcrFollowParts<'a>,
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
    B: Body,
{
    #[doc = "Creates a new instance of [CcrFollow] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: CcrFollowParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> CcrFollow<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrFollow {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous call to the Ccr Follow API that can be awaited"]
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
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParams {
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
#[doc = "API parts for the Ccr Follow Info API"]
pub enum CcrFollowInfoParts<'a> {
    #[doc = "Index"]
    Index(&'a [&'a str]),
}
impl<'a> CcrFollowInfoParts<'a> {
    #[doc = "Builds a relative URL path to the Ccr Follow Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrFollowInfoParts::Index(ref index) => {
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
#[doc = "Builder for the [Ccr Follow Info API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-follow-info.html)."]
pub struct CcrFollowInfo<'a> {
    client: Elasticsearch,
    parts: CcrFollowInfoParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> CcrFollowInfo<'a> {
    #[doc = "Creates a new instance of [CcrFollowInfo] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: CcrFollowInfoParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ccr Follow Info API that can be awaited"]
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
#[doc = "API parts for the Ccr Follow Stats API"]
pub enum CcrFollowStatsParts<'a> {
    #[doc = "Index"]
    Index(&'a [&'a str]),
}
impl<'a> CcrFollowStatsParts<'a> {
    #[doc = "Builds a relative URL path to the Ccr Follow Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrFollowStatsParts::Index(ref index) => {
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
#[doc = "Builder for the [Ccr Follow Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-follow-stats.html)."]
pub struct CcrFollowStats<'a> {
    client: Elasticsearch,
    parts: CcrFollowStatsParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> CcrFollowStats<'a> {
    #[doc = "Creates a new instance of [CcrFollowStats] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: CcrFollowStatsParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ccr Follow Stats API that can be awaited"]
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
#[doc = "API parts for the Ccr Forget Follower API"]
pub enum CcrForgetFollowerParts<'a> {
    #[doc = "Index"]
    Index(&'a str),
}
impl<'a> CcrForgetFollowerParts<'a> {
    #[doc = "Builds a relative URL path to the Ccr Forget Follower API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrForgetFollowerParts::Index(ref index) => {
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
#[doc = "Builder for the [Ccr Forget Follower API](http://www.elastic.co/guide/en/elasticsearch/reference/current)."]
pub struct CcrForgetFollower<'a, B> {
    client: Elasticsearch,
    parts: CcrForgetFollowerParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> CcrForgetFollower<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrForgetFollower] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: CcrForgetFollowerParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> CcrForgetFollower<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrForgetFollower {
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
    #[doc = "Creates an asynchronous call to the Ccr Forget Follower API that can be awaited"]
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
#[doc = "API parts for the Ccr Get Auto Follow Pattern API"]
pub enum CcrGetAutoFollowPatternParts<'a> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'a str),
}
impl<'a> CcrGetAutoFollowPatternParts<'a> {
    #[doc = "Builds a relative URL path to the Ccr Get Auto Follow Pattern API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrGetAutoFollowPatternParts::None => "/_ccr/auto_follow".into(),
            CcrGetAutoFollowPatternParts::Name(ref name) => {
                let mut p = String::with_capacity(18usize + name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Get Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-auto-follow-pattern.html)."]
pub struct CcrGetAutoFollowPattern<'a> {
    client: Elasticsearch,
    parts: CcrGetAutoFollowPatternParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> CcrGetAutoFollowPattern<'a> {
    #[doc = "Creates a new instance of [CcrGetAutoFollowPattern] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: CcrGetAutoFollowPatternParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ccr Get Auto Follow Pattern API that can be awaited"]
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
#[doc = "API parts for the Ccr Pause Auto Follow Pattern API"]
pub enum CcrPauseAutoFollowPatternParts<'a> {
    #[doc = "Name"]
    Name(&'a str),
}
impl<'a> CcrPauseAutoFollowPatternParts<'a> {
    #[doc = "Builds a relative URL path to the Ccr Pause Auto Follow Pattern API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrPauseAutoFollowPatternParts::Name(ref name) => {
                let mut p = String::with_capacity(24usize + name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(name.as_ref());
                p.push_str("/pause");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Pause Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-pause-auto-follow-pattern.html)."]
pub struct CcrPauseAutoFollowPattern<'a, B> {
    client: Elasticsearch,
    parts: CcrPauseAutoFollowPatternParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> CcrPauseAutoFollowPattern<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrPauseAutoFollowPattern] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: CcrPauseAutoFollowPatternParts<'a>) -> Self {
        CcrPauseAutoFollowPattern {
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
    pub fn body<T>(self, body: T) -> CcrPauseAutoFollowPattern<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrPauseAutoFollowPattern {
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
    #[doc = "Creates an asynchronous call to the Ccr Pause Auto Follow Pattern API that can be awaited"]
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
#[doc = "API parts for the Ccr Pause Follow API"]
pub enum CcrPauseFollowParts<'a> {
    #[doc = "Index"]
    Index(&'a str),
}
impl<'a> CcrPauseFollowParts<'a> {
    #[doc = "Builds a relative URL path to the Ccr Pause Follow API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrPauseFollowParts::Index(ref index) => {
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
#[doc = "Builder for the [Ccr Pause Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-post-pause-follow.html)."]
pub struct CcrPauseFollow<'a, B> {
    client: Elasticsearch,
    parts: CcrPauseFollowParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> CcrPauseFollow<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrPauseFollow] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: CcrPauseFollowParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> CcrPauseFollow<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrPauseFollow {
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
    #[doc = "Creates an asynchronous call to the Ccr Pause Follow API that can be awaited"]
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
#[doc = "API parts for the Ccr Put Auto Follow Pattern API"]
pub enum CcrPutAutoFollowPatternParts<'a> {
    #[doc = "Name"]
    Name(&'a str),
}
impl<'a> CcrPutAutoFollowPatternParts<'a> {
    #[doc = "Builds a relative URL path to the Ccr Put Auto Follow Pattern API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrPutAutoFollowPatternParts::Name(ref name) => {
                let mut p = String::with_capacity(18usize + name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Put Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-put-auto-follow-pattern.html)."]
pub struct CcrPutAutoFollowPattern<'a, B> {
    client: Elasticsearch,
    parts: CcrPutAutoFollowPatternParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> CcrPutAutoFollowPattern<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrPutAutoFollowPattern] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: CcrPutAutoFollowPatternParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> CcrPutAutoFollowPattern<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrPutAutoFollowPattern {
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
    #[doc = "Creates an asynchronous call to the Ccr Put Auto Follow Pattern API that can be awaited"]
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
#[doc = "API parts for the Ccr Resume Auto Follow Pattern API"]
pub enum CcrResumeAutoFollowPatternParts<'a> {
    #[doc = "Name"]
    Name(&'a str),
}
impl<'a> CcrResumeAutoFollowPatternParts<'a> {
    #[doc = "Builds a relative URL path to the Ccr Resume Auto Follow Pattern API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrResumeAutoFollowPatternParts::Name(ref name) => {
                let mut p = String::with_capacity(25usize + name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(name.as_ref());
                p.push_str("/resume");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Resume Auto Follow Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-resume-auto-follow-pattern.html)."]
pub struct CcrResumeAutoFollowPattern<'a, B> {
    client: Elasticsearch,
    parts: CcrResumeAutoFollowPatternParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> CcrResumeAutoFollowPattern<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrResumeAutoFollowPattern] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: CcrResumeAutoFollowPatternParts<'a>) -> Self {
        CcrResumeAutoFollowPattern {
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
    pub fn body<T>(self, body: T) -> CcrResumeAutoFollowPattern<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrResumeAutoFollowPattern {
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
    #[doc = "Creates an asynchronous call to the Ccr Resume Auto Follow Pattern API that can be awaited"]
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
#[doc = "API parts for the Ccr Resume Follow API"]
pub enum CcrResumeFollowParts<'a> {
    #[doc = "Index"]
    Index(&'a str),
}
impl<'a> CcrResumeFollowParts<'a> {
    #[doc = "Builds a relative URL path to the Ccr Resume Follow API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrResumeFollowParts::Index(ref index) => {
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
#[doc = "Builder for the [Ccr Resume Follow API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-post-resume-follow.html)."]
pub struct CcrResumeFollow<'a, B> {
    client: Elasticsearch,
    parts: CcrResumeFollowParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> CcrResumeFollow<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrResumeFollow] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: CcrResumeFollowParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> CcrResumeFollow<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrResumeFollow {
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
    #[doc = "Creates an asynchronous call to the Ccr Resume Follow API that can be awaited"]
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
#[doc = "API parts for the Ccr Stats API"]
pub enum CcrStatsParts {
    #[doc = "No parts"]
    None,
}
impl CcrStatsParts {
    #[doc = "Builds a relative URL path to the Ccr Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrStatsParts::None => "/_ccr/stats".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ccr Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-stats.html)."]
pub struct CcrStats<'a> {
    client: Elasticsearch,
    parts: CcrStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> CcrStats<'a> {
    #[doc = "Creates a new instance of [CcrStats]"]
    pub fn new(client: Elasticsearch) -> Self {
        CcrStats {
            client,
            parts: CcrStatsParts::None,
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
    #[doc = "Creates an asynchronous call to the Ccr Stats API that can be awaited"]
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
#[doc = "API parts for the Ccr Unfollow API"]
pub enum CcrUnfollowParts<'a> {
    #[doc = "Index"]
    Index(&'a str),
}
impl<'a> CcrUnfollowParts<'a> {
    #[doc = "Builds a relative URL path to the Ccr Unfollow API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CcrUnfollowParts::Index(ref index) => {
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
#[doc = "Builder for the [Ccr Unfollow API](http://www.elastic.co/guide/en/elasticsearch/reference/current)."]
pub struct CcrUnfollow<'a, B> {
    client: Elasticsearch,
    parts: CcrUnfollowParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> CcrUnfollow<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrUnfollow] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: CcrUnfollowParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> CcrUnfollow<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrUnfollow {
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
    #[doc = "Creates an asynchronous call to the Ccr Unfollow API that can be awaited"]
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
#[doc = "Namespace client for Cross Cluster Replication APIs"]
pub struct Ccr {
    client: Elasticsearch,
}
impl Ccr {
    #[doc = "Creates a new instance of [Ccr]"]
    pub fn new(client: Elasticsearch) -> Self {
        Self { client }
    }
    pub fn delete_auto_follow_pattern<'a>(
        &self,
        parts: CcrDeleteAutoFollowPatternParts<'a>,
    ) -> CcrDeleteAutoFollowPattern<'a> {
        CcrDeleteAutoFollowPattern::new(self.client.clone(), parts)
    }
    pub fn follow<'a>(&self, parts: CcrFollowParts<'a>) -> CcrFollow<'a, ()> {
        CcrFollow::new(self.client.clone(), parts)
    }
    pub fn follow_info<'a>(&self, parts: CcrFollowInfoParts<'a>) -> CcrFollowInfo<'a> {
        CcrFollowInfo::new(self.client.clone(), parts)
    }
    pub fn follow_stats<'a>(&self, parts: CcrFollowStatsParts<'a>) -> CcrFollowStats<'a> {
        CcrFollowStats::new(self.client.clone(), parts)
    }
    pub fn forget_follower<'a>(
        &self,
        parts: CcrForgetFollowerParts<'a>,
    ) -> CcrForgetFollower<'a, ()> {
        CcrForgetFollower::new(self.client.clone(), parts)
    }
    pub fn get_auto_follow_pattern<'a>(
        &self,
        parts: CcrGetAutoFollowPatternParts<'a>,
    ) -> CcrGetAutoFollowPattern<'a> {
        CcrGetAutoFollowPattern::new(self.client.clone(), parts)
    }
    pub fn pause_auto_follow_pattern<'a>(
        &self,
        parts: CcrPauseAutoFollowPatternParts<'a>,
    ) -> CcrPauseAutoFollowPattern<'a, ()> {
        CcrPauseAutoFollowPattern::new(self.client.clone(), parts)
    }
    pub fn pause_follow<'a>(&self, parts: CcrPauseFollowParts<'a>) -> CcrPauseFollow<'a, ()> {
        CcrPauseFollow::new(self.client.clone(), parts)
    }
    pub fn put_auto_follow_pattern<'a>(
        &self,
        parts: CcrPutAutoFollowPatternParts<'a>,
    ) -> CcrPutAutoFollowPattern<'a, ()> {
        CcrPutAutoFollowPattern::new(self.client.clone(), parts)
    }
    pub fn resume_auto_follow_pattern<'a>(
        &self,
        parts: CcrResumeAutoFollowPatternParts<'a>,
    ) -> CcrResumeAutoFollowPattern<'a, ()> {
        CcrResumeAutoFollowPattern::new(self.client.clone(), parts)
    }
    pub fn resume_follow<'a>(&self, parts: CcrResumeFollowParts<'a>) -> CcrResumeFollow<'a, ()> {
        CcrResumeFollow::new(self.client.clone(), parts)
    }
    pub fn stats<'a>(&self) -> CcrStats<'a> {
        CcrStats::new(self.client.clone())
    }
    pub fn unfollow<'a>(&self, parts: CcrUnfollowParts<'a>) -> CcrUnfollow<'a, ()> {
        CcrUnfollow::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Cross Cluster Replication APIs"]
    pub fn ccr(&self) -> Ccr {
        Ccr::new(self.clone())
    }
}