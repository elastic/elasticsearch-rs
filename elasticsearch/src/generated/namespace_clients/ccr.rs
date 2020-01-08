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
        headers::{HeaderMap, HeaderName, HeaderValue},
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
pub enum CcrDeleteAutoFollowPatternParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> CcrDeleteAutoFollowPatternParts<'b> {
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
pub struct CcrDeleteAutoFollowPattern<'a, 'b> {
    client: &'a Elasticsearch,
    parts: CcrDeleteAutoFollowPatternParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrDeleteAutoFollowPattern<'a, 'b> {
    #[doc = "Creates a new instance of [CcrDeleteAutoFollowPattern] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CcrDeleteAutoFollowPatternParts<'b>) -> Self {
        CcrDeleteAutoFollowPattern {
            client,
            parts,
            headers: HeaderMap::new(),
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
    #[doc = "Creates an asynchronous call to the Ccr Delete Auto Follow Pattern API that can be awaited"]
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
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Follow API"]
pub enum CcrFollowParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> CcrFollowParts<'b> {
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
pub struct CcrFollow<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: CcrFollowParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> CcrFollow<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrFollow] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CcrFollowParts<'b>) -> Self {
        CcrFollow {
            client,
            parts,
            headers: HeaderMap::new(),
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
    pub fn body<T>(self, body: T) -> CcrFollow<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrFollow {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    #[doc = "Sets the number of shard copies that must be active before returning. Defaults to 0. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Ccr Follow API that can be awaited"]
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
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ccr Follow Info API"]
pub enum CcrFollowInfoParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CcrFollowInfoParts<'b> {
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
pub struct CcrFollowInfo<'a, 'b> {
    client: &'a Elasticsearch,
    parts: CcrFollowInfoParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrFollowInfo<'a, 'b> {
    #[doc = "Creates a new instance of [CcrFollowInfo] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CcrFollowInfoParts<'b>) -> Self {
        CcrFollowInfo {
            client,
            parts,
            headers: HeaderMap::new(),
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
    #[doc = "Creates an asynchronous call to the Ccr Follow Info API that can be awaited"]
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
#[doc = "API parts for the Ccr Follow Stats API"]
pub enum CcrFollowStatsParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CcrFollowStatsParts<'b> {
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
pub struct CcrFollowStats<'a, 'b> {
    client: &'a Elasticsearch,
    parts: CcrFollowStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrFollowStats<'a, 'b> {
    #[doc = "Creates a new instance of [CcrFollowStats] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CcrFollowStatsParts<'b>) -> Self {
        CcrFollowStats {
            client,
            parts,
            headers: HeaderMap::new(),
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
    #[doc = "Creates an asynchronous call to the Ccr Follow Stats API that can be awaited"]
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
#[doc = "API parts for the Ccr Forget Follower API"]
pub enum CcrForgetFollowerParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> CcrForgetFollowerParts<'b> {
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
pub struct CcrForgetFollower<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: CcrForgetFollowerParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrForgetFollower<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrForgetFollower] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CcrForgetFollowerParts<'b>) -> Self {
        CcrForgetFollower {
            client,
            parts,
            headers: HeaderMap::new(),
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrForgetFollower<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrForgetFollower {
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
    #[doc = "Creates an asynchronous call to the Ccr Forget Follower API that can be awaited"]
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
#[doc = "API parts for the Ccr Get Auto Follow Pattern API"]
pub enum CcrGetAutoFollowPatternParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> CcrGetAutoFollowPatternParts<'b> {
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
pub struct CcrGetAutoFollowPattern<'a, 'b> {
    client: &'a Elasticsearch,
    parts: CcrGetAutoFollowPatternParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrGetAutoFollowPattern<'a, 'b> {
    #[doc = "Creates a new instance of [CcrGetAutoFollowPattern] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CcrGetAutoFollowPatternParts<'b>) -> Self {
        CcrGetAutoFollowPattern {
            client,
            parts,
            headers: HeaderMap::new(),
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
    #[doc = "Creates an asynchronous call to the Ccr Get Auto Follow Pattern API that can be awaited"]
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
#[doc = "API parts for the Ccr Pause Auto Follow Pattern API"]
pub enum CcrPauseAutoFollowPatternParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> CcrPauseAutoFollowPatternParts<'b> {
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
pub struct CcrPauseAutoFollowPattern<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: CcrPauseAutoFollowPatternParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrPauseAutoFollowPattern<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrPauseAutoFollowPattern] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CcrPauseAutoFollowPatternParts<'b>) -> Self {
        CcrPauseAutoFollowPattern {
            client,
            parts,
            headers: HeaderMap::new(),
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrPauseAutoFollowPattern<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrPauseAutoFollowPattern {
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
    #[doc = "Creates an asynchronous call to the Ccr Pause Auto Follow Pattern API that can be awaited"]
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
#[doc = "API parts for the Ccr Pause Follow API"]
pub enum CcrPauseFollowParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> CcrPauseFollowParts<'b> {
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
pub struct CcrPauseFollow<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: CcrPauseFollowParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrPauseFollow<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrPauseFollow] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CcrPauseFollowParts<'b>) -> Self {
        CcrPauseFollow {
            client,
            parts,
            headers: HeaderMap::new(),
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrPauseFollow<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrPauseFollow {
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
    #[doc = "Creates an asynchronous call to the Ccr Pause Follow API that can be awaited"]
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
#[doc = "API parts for the Ccr Put Auto Follow Pattern API"]
pub enum CcrPutAutoFollowPatternParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> CcrPutAutoFollowPatternParts<'b> {
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
pub struct CcrPutAutoFollowPattern<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: CcrPutAutoFollowPatternParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrPutAutoFollowPattern<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrPutAutoFollowPattern] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CcrPutAutoFollowPatternParts<'b>) -> Self {
        CcrPutAutoFollowPattern {
            client,
            parts,
            headers: HeaderMap::new(),
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrPutAutoFollowPattern<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrPutAutoFollowPattern {
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
    #[doc = "Creates an asynchronous call to the Ccr Put Auto Follow Pattern API that can be awaited"]
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
#[doc = "API parts for the Ccr Resume Auto Follow Pattern API"]
pub enum CcrResumeAutoFollowPatternParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> CcrResumeAutoFollowPatternParts<'b> {
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
pub struct CcrResumeAutoFollowPattern<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: CcrResumeAutoFollowPatternParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrResumeAutoFollowPattern<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrResumeAutoFollowPattern] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CcrResumeAutoFollowPatternParts<'b>) -> Self {
        CcrResumeAutoFollowPattern {
            client,
            parts,
            headers: HeaderMap::new(),
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrResumeAutoFollowPattern<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrResumeAutoFollowPattern {
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
    #[doc = "Creates an asynchronous call to the Ccr Resume Auto Follow Pattern API that can be awaited"]
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
#[doc = "API parts for the Ccr Resume Follow API"]
pub enum CcrResumeFollowParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> CcrResumeFollowParts<'b> {
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
pub struct CcrResumeFollow<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: CcrResumeFollowParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrResumeFollow<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrResumeFollow] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CcrResumeFollowParts<'b>) -> Self {
        CcrResumeFollow {
            client,
            parts,
            headers: HeaderMap::new(),
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrResumeFollow<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrResumeFollow {
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
    #[doc = "Creates an asynchronous call to the Ccr Resume Follow API that can be awaited"]
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
pub struct CcrStats<'a, 'b> {
    client: &'a Elasticsearch,
    parts: CcrStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> CcrStats<'a, 'b> {
    #[doc = "Creates a new instance of [CcrStats]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        CcrStats {
            client,
            parts: CcrStatsParts::None,
            headers: HeaderMap::new(),
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
    #[doc = "Creates an asynchronous call to the Ccr Stats API that can be awaited"]
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
#[doc = "API parts for the Ccr Unfollow API"]
pub enum CcrUnfollowParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> CcrUnfollowParts<'b> {
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
pub struct CcrUnfollow<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: CcrUnfollowParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CcrUnfollow<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CcrUnfollow] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: CcrUnfollowParts<'b>) -> Self {
        CcrUnfollow {
            client,
            parts,
            headers: HeaderMap::new(),
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CcrUnfollow<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CcrUnfollow {
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
    #[doc = "Creates an asynchronous call to the Ccr Unfollow API that can be awaited"]
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
#[doc = "Namespace client for Cross Cluster Replication APIs"]
pub struct Ccr<'a> {
    client: &'a Elasticsearch,
}
impl<'a> Ccr<'a> {
    #[doc = "Creates a new instance of [Ccr]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        Self { client }
    }
    pub fn delete_auto_follow_pattern<'b>(
        &'a self,
        parts: CcrDeleteAutoFollowPatternParts<'b>,
    ) -> CcrDeleteAutoFollowPattern<'a, 'b> {
        CcrDeleteAutoFollowPattern::new(&self.client, parts)
    }
    pub fn follow<'b>(&'a self, parts: CcrFollowParts<'b>) -> CcrFollow<'a, 'b, ()> {
        CcrFollow::new(&self.client, parts)
    }
    pub fn follow_info<'b>(&'a self, parts: CcrFollowInfoParts<'b>) -> CcrFollowInfo<'a, 'b> {
        CcrFollowInfo::new(&self.client, parts)
    }
    pub fn follow_stats<'b>(&'a self, parts: CcrFollowStatsParts<'b>) -> CcrFollowStats<'a, 'b> {
        CcrFollowStats::new(&self.client, parts)
    }
    pub fn forget_follower<'b>(
        &'a self,
        parts: CcrForgetFollowerParts<'b>,
    ) -> CcrForgetFollower<'a, 'b, ()> {
        CcrForgetFollower::new(&self.client, parts)
    }
    pub fn get_auto_follow_pattern<'b>(
        &'a self,
        parts: CcrGetAutoFollowPatternParts<'b>,
    ) -> CcrGetAutoFollowPattern<'a, 'b> {
        CcrGetAutoFollowPattern::new(&self.client, parts)
    }
    pub fn pause_auto_follow_pattern<'b>(
        &'a self,
        parts: CcrPauseAutoFollowPatternParts<'b>,
    ) -> CcrPauseAutoFollowPattern<'a, 'b, ()> {
        CcrPauseAutoFollowPattern::new(&self.client, parts)
    }
    pub fn pause_follow<'b>(
        &'a self,
        parts: CcrPauseFollowParts<'b>,
    ) -> CcrPauseFollow<'a, 'b, ()> {
        CcrPauseFollow::new(&self.client, parts)
    }
    pub fn put_auto_follow_pattern<'b>(
        &'a self,
        parts: CcrPutAutoFollowPatternParts<'b>,
    ) -> CcrPutAutoFollowPattern<'a, 'b, ()> {
        CcrPutAutoFollowPattern::new(&self.client, parts)
    }
    pub fn resume_auto_follow_pattern<'b>(
        &'a self,
        parts: CcrResumeAutoFollowPatternParts<'b>,
    ) -> CcrResumeAutoFollowPattern<'a, 'b, ()> {
        CcrResumeAutoFollowPattern::new(&self.client, parts)
    }
    pub fn resume_follow<'b>(
        &'a self,
        parts: CcrResumeFollowParts<'b>,
    ) -> CcrResumeFollow<'a, 'b, ()> {
        CcrResumeFollow::new(&self.client, parts)
    }
    pub fn stats<'b>(&'a self) -> CcrStats<'a, 'b> {
        CcrStats::new(&self.client)
    }
    pub fn unfollow<'b>(&'a self, parts: CcrUnfollowParts<'b>) -> CcrUnfollow<'a, 'b, ()> {
        CcrUnfollow::new(&self.client, parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Cross Cluster Replication APIs"]
    pub fn ccr(&self) -> Ccr {
        Ccr::new(&self)
    }
}