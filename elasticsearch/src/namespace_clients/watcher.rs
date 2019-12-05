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
    client::Elasticsearch,
    enums::*,
    error::ElasticsearchError,
    http_method::HttpMethod,
    request::{Body, JsonBody, NdBody},
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Watcher Ack Watch API"]
pub enum WatcherAckWatchUrlParts<'a> {
    WatchId(&'a str),
    WatchIdActionId(&'a str, &'a [&'a str]),
}
impl<'a> WatcherAckWatchUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherAckWatchUrlParts::WatchId(ref watch_id) => {
                let mut p = String::with_capacity(21usize + watch_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(watch_id.as_ref());
                p.push_str("/_ack");
                p.into()
            }
            WatcherAckWatchUrlParts::WatchIdActionId(ref watch_id, ref action_id) => {
                let action_id_str = action_id.join(",");
                let mut p = String::with_capacity(22usize + watch_id.len() + action_id_str.len());
                p.push_str("/_watcher/watch/");
                p.push_str(watch_id.as_ref());
                p.push_str("/_ack/");
                p.push_str(action_id_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Ack Watch API](http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-ack-watch.html)."]
pub struct WatcherAckWatch<'a, B> {
    client: Elasticsearch,
    parts: WatcherAckWatchUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> WatcherAckWatch<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: WatcherAckWatchUrlParts<'a>) -> Self {
        WatcherAckWatch {
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
    pub fn body<T>(self, body: T) -> WatcherAckWatch<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherAckWatch {
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
    #[doc = "Creates an asynchronous call to the Watcher Ack Watch API that can be awaited"]
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
#[doc = "Url parts for the Watcher Activate Watch API"]
pub enum WatcherActivateWatchUrlParts<'a> {
    WatchId(&'a str),
}
impl<'a> WatcherActivateWatchUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherActivateWatchUrlParts::WatchId(ref watch_id) => {
                let mut p = String::with_capacity(26usize + watch_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(watch_id.as_ref());
                p.push_str("/_activate");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Activate Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-activate-watch.html)."]
pub struct WatcherActivateWatch<'a, B> {
    client: Elasticsearch,
    parts: WatcherActivateWatchUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> WatcherActivateWatch<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: WatcherActivateWatchUrlParts<'a>) -> Self {
        WatcherActivateWatch {
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
    pub fn body<T>(self, body: T) -> WatcherActivateWatch<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherActivateWatch {
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
    #[doc = "Creates an asynchronous call to the Watcher Activate Watch API that can be awaited"]
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
#[doc = "Url parts for the Watcher Deactivate Watch API"]
pub enum WatcherDeactivateWatchUrlParts<'a> {
    WatchId(&'a str),
}
impl<'a> WatcherDeactivateWatchUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherDeactivateWatchUrlParts::WatchId(ref watch_id) => {
                let mut p = String::with_capacity(28usize + watch_id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(watch_id.as_ref());
                p.push_str("/_deactivate");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Deactivate Watch API](https://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-deactivate-watch.html)."]
pub struct WatcherDeactivateWatch<'a, B> {
    client: Elasticsearch,
    parts: WatcherDeactivateWatchUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> WatcherDeactivateWatch<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: WatcherDeactivateWatchUrlParts<'a>) -> Self {
        WatcherDeactivateWatch {
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
    pub fn body<T>(self, body: T) -> WatcherDeactivateWatch<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherDeactivateWatch {
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
    #[doc = "Creates an asynchronous call to the Watcher Deactivate Watch API that can be awaited"]
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
#[doc = "Url parts for the Watcher Delete Watch API"]
pub enum WatcherDeleteWatchUrlParts<'a> {
    Id(&'a str),
}
impl<'a> WatcherDeleteWatchUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherDeleteWatchUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(16usize + id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Delete Watch API](http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-delete-watch.html)."]
pub struct WatcherDeleteWatch<'a> {
    client: Elasticsearch,
    parts: WatcherDeleteWatchUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> WatcherDeleteWatch<'a> {
    pub fn new(client: Elasticsearch, parts: WatcherDeleteWatchUrlParts<'a>) -> Self {
        WatcherDeleteWatch {
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
    #[doc = "Creates an asynchronous call to the Watcher Delete Watch API that can be awaited"]
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
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Watcher Execute Watch API"]
pub enum WatcherExecuteWatchUrlParts<'a> {
    Id(&'a str),
    None,
}
impl<'a> WatcherExecuteWatchUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherExecuteWatchUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(25usize + id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(id.as_ref());
                p.push_str("/_execute");
                p.into()
            }
            WatcherExecuteWatchUrlParts::None => "/_watcher/watch/_execute".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Execute Watch API](http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-execute-watch.html)."]
pub struct WatcherExecuteWatch<'a, B> {
    client: Elasticsearch,
    parts: WatcherExecuteWatchUrlParts<'a>,
    body: Option<B>,
    debug: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> WatcherExecuteWatch<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: WatcherExecuteWatchUrlParts<'a>) -> Self {
        WatcherExecuteWatch {
            client,
            parts,
            body: None,
            debug: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherExecuteWatch<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherExecuteWatch {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            debug: self.debug,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "indicates whether the watch should execute in debug mode"]
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = Some(debug);
        self
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
    #[doc = "Creates an asynchronous call to the Watcher Execute Watch API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "debug")]
                debug: Option<bool>,
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
                debug: self.debug,
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
#[doc = "Url parts for the Watcher Get Watch API"]
pub enum WatcherGetWatchUrlParts<'a> {
    Id(&'a str),
}
impl<'a> WatcherGetWatchUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherGetWatchUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(16usize + id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Get Watch API](http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-get-watch.html)."]
pub struct WatcherGetWatch<'a> {
    client: Elasticsearch,
    parts: WatcherGetWatchUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> WatcherGetWatch<'a> {
    pub fn new(client: Elasticsearch, parts: WatcherGetWatchUrlParts<'a>) -> Self {
        WatcherGetWatch {
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
    #[doc = "Creates an asynchronous call to the Watcher Get Watch API that can be awaited"]
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
#[doc = "Url parts for the Watcher Put Watch API"]
pub enum WatcherPutWatchUrlParts<'a> {
    Id(&'a str),
}
impl<'a> WatcherPutWatchUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherPutWatchUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(16usize + id.len());
                p.push_str("/_watcher/watch/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Put Watch API](http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-put-watch.html)."]
pub struct WatcherPutWatch<'a, B> {
    client: Elasticsearch,
    parts: WatcherPutWatchUrlParts<'a>,
    active: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    version: Option<i64>,
}
impl<'a, B> WatcherPutWatch<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: WatcherPutWatchUrlParts<'a>) -> Self {
        WatcherPutWatch {
            client,
            parts,
            active: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            pretty: None,
            source: None,
            version: None,
        }
    }
    #[doc = "Specify whether the watch is in/active by default"]
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherPutWatch<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherPutWatch {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            active: self.active,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            if_primary_term: self.if_primary_term,
            if_seq_no: self.if_seq_no,
            pretty: self.pretty,
            source: self.source,
            version: self.version,
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
    #[doc = "only update the watch if the last operation that has changed the watch has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "only update the watch if the last operation that has changed the watch has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
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
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Creates an asynchronous call to the Watcher Put Watch API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "active")]
                active: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "if_primary_term")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no")]
                if_seq_no: Option<i64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "version")]
                version: Option<i64>,
            }
            let query_params = QueryParams {
                active: self.active,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                pretty: self.pretty,
                source: self.source,
                version: self.version,
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
#[doc = "Url parts for the Watcher Start API"]
pub enum WatcherStartUrlParts {
    None,
}
impl WatcherStartUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherStartUrlParts::None => "/_watcher/_start".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Start API](http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-start.html)."]
pub struct WatcherStart<'a, B> {
    client: Elasticsearch,
    parts: WatcherStartUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> WatcherStart<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch) -> Self {
        WatcherStart {
            client,
            parts: WatcherStartUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherStart<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherStart {
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
    #[doc = "Creates an asynchronous call to the Watcher Start API that can be awaited"]
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
#[doc = "Url parts for the Watcher Stats API"]
pub enum WatcherStatsUrlParts<'a> {
    None,
    Metric(&'a [&'a str]),
}
impl<'a> WatcherStatsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherStatsUrlParts::None => "/_watcher/stats".into(),
            WatcherStatsUrlParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(16usize + metric_str.len());
                p.push_str("/_watcher/stats/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Stats API](http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-stats.html)."]
pub struct WatcherStats<'a> {
    client: Elasticsearch,
    parts: WatcherStatsUrlParts<'a>,
    emit_stacktraces: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    metric: Option<&'a [&'a str]>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> WatcherStats<'a> {
    pub fn new(client: Elasticsearch, parts: WatcherStatsUrlParts<'a>) -> Self {
        WatcherStats {
            client,
            parts,
            emit_stacktraces: None,
            error_trace: None,
            filter_path: None,
            human: None,
            metric: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Emits stack traces of currently running watches"]
    pub fn emit_stacktraces(mut self, emit_stacktraces: bool) -> Self {
        self.emit_stacktraces = Some(emit_stacktraces);
        self
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
    #[doc = "Controls what additional stat metrics should be include in the response"]
    pub fn metric(mut self, metric: &'a [&'a str]) -> Self {
        self.metric = Some(metric);
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
    #[doc = "Creates an asynchronous call to the Watcher Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "emit_stacktraces")]
                emit_stacktraces: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "metric", serialize_with = "crate::client::serialize_coll_qs")]
                metric: Option<&'a [&'a str]>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                emit_stacktraces: self.emit_stacktraces,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                metric: self.metric,
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
#[doc = "Url parts for the Watcher Stop API"]
pub enum WatcherStopUrlParts {
    None,
}
impl WatcherStopUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            WatcherStopUrlParts::None => "/_watcher/_stop".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Watcher Stop API](http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-stop.html)."]
pub struct WatcherStop<'a, B> {
    client: Elasticsearch,
    parts: WatcherStopUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> WatcherStop<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch) -> Self {
        WatcherStop {
            client,
            parts: WatcherStopUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> WatcherStop<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        WatcherStop {
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
    #[doc = "Creates an asynchronous call to the Watcher Stop API that can be awaited"]
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
#[doc = "Namespace client for Watcher APIs"]
pub struct Watcher {
    client: Elasticsearch,
}
impl Watcher {
    pub fn new(client: Elasticsearch) -> Self {
        Watcher { client }
    }
    pub fn ack_watch<'a>(&self, parts: WatcherAckWatchUrlParts<'a>) -> WatcherAckWatch<'a, ()> {
        WatcherAckWatch::new(self.client.clone(), parts)
    }
    pub fn activate_watch<'a>(
        &self,
        parts: WatcherActivateWatchUrlParts<'a>,
    ) -> WatcherActivateWatch<'a, ()> {
        WatcherActivateWatch::new(self.client.clone(), parts)
    }
    pub fn deactivate_watch<'a>(
        &self,
        parts: WatcherDeactivateWatchUrlParts<'a>,
    ) -> WatcherDeactivateWatch<'a, ()> {
        WatcherDeactivateWatch::new(self.client.clone(), parts)
    }
    pub fn delete_watch<'a>(
        &self,
        parts: WatcherDeleteWatchUrlParts<'a>,
    ) -> WatcherDeleteWatch<'a> {
        WatcherDeleteWatch::new(self.client.clone(), parts)
    }
    pub fn execute_watch<'a>(
        &self,
        parts: WatcherExecuteWatchUrlParts<'a>,
    ) -> WatcherExecuteWatch<'a, ()> {
        WatcherExecuteWatch::new(self.client.clone(), parts)
    }
    pub fn get_watch<'a>(&self, parts: WatcherGetWatchUrlParts<'a>) -> WatcherGetWatch<'a> {
        WatcherGetWatch::new(self.client.clone(), parts)
    }
    pub fn put_watch<'a>(&self, parts: WatcherPutWatchUrlParts<'a>) -> WatcherPutWatch<'a, ()> {
        WatcherPutWatch::new(self.client.clone(), parts)
    }
    pub fn start<'a>(&self) -> WatcherStart<'a, ()> {
        WatcherStart::new(self.client.clone())
    }
    pub fn stats<'a>(&self, parts: WatcherStatsUrlParts<'a>) -> WatcherStats<'a> {
        WatcherStats::new(self.client.clone(), parts)
    }
    pub fn stop<'a>(&self) -> WatcherStop<'a, ()> {
        WatcherStop::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Watcher APIs"]
    pub fn watcher(&self) -> Watcher {
        Watcher::new(self.clone())
    }
}