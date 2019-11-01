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
pub struct CcrDeleteAutoFollowPattern {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrDeleteAutoFollowPattern {
    pub fn new(client: Elasticsearch, name: String) -> Self {
        CcrDeleteAutoFollowPattern {
            client,
            name: name,
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
    #[doc = "The name of the auto follow pattern."]
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
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
impl Sender for CcrDeleteAutoFollowPattern {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let name = self.name;
            let mut p = String::with_capacity(18usize + name.len());
            p.push_str("/_ccr/auto_follow/");
            p.push_str(name.as_ref());
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
pub struct CcrFollow<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl<B> CcrFollow<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String) -> Self {
        CcrFollow {
            client,
            index: index,
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
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
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
    #[doc = "The name of the follower index"]
    pub fn index(mut self, index: String) -> Self {
        self.index = index;
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
    #[doc = "Sets the number of shard copies that must be active before returning. Defaults to 0. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1)"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl<B> Sender for CcrFollow<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index = self.index;
            let mut p = String::with_capacity(13usize + index.len());
            p.push_str("/");
            p.push_str(index.as_ref());
            p.push_str("/_ccr/follow");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct CcrFollowInfo {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Vec<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrFollowInfo {
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        CcrFollowInfo {
            client,
            index: index,
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
    #[doc = "A comma-separated list of index patterns; use `_all` to perform the operation on all indices"]
    pub fn index(mut self, index: Vec<String>) -> Self {
        self.index = index;
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
impl Sender for CcrFollowInfo {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index_str = self.index.join(",");
            let mut p = String::with_capacity(11usize + index_str.len());
            p.push_str("/");
            p.push_str(index_str.as_ref());
            p.push_str("/_ccr/info");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
pub struct CcrFollowStats {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Vec<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrFollowStats {
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        CcrFollowStats {
            client,
            index: index,
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
    #[doc = "A comma-separated list of index patterns; use `_all` to perform the operation on all indices"]
    pub fn index(mut self, index: Vec<String>) -> Self {
        self.index = index;
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
impl Sender for CcrFollowStats {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index_str = self.index.join(",");
            let mut p = String::with_capacity(12usize + index_str.len());
            p.push_str("/");
            p.push_str(index_str.as_ref());
            p.push_str("/_ccr/stats");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
pub struct CcrForgetFollower<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> CcrForgetFollower<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String) -> Self {
        CcrForgetFollower {
            client,
            index: index,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
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
    #[doc = "the name of the leader index for which specified follower retention leases should be removed"]
    pub fn index(mut self, index: String) -> Self {
        self.index = index;
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
impl<B> Sender for CcrForgetFollower<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index = self.index;
            let mut p = String::with_capacity(22usize + index.len());
            p.push_str("/");
            p.push_str(index.as_ref());
            p.push_str("/_ccr/forget_follower");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct CcrGetAutoFollowPattern {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrGetAutoFollowPattern {
    pub fn new(client: Elasticsearch) -> Self {
        CcrGetAutoFollowPattern {
            client,
            error_trace: None,
            filter_path: None,
            human: None,
            name: None,
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
    #[doc = "The name of the auto follow pattern."]
    pub fn name(mut self, name: Option<String>) -> Self {
        self.name = name;
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
impl Sender for CcrGetAutoFollowPattern {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.name {
            Some(name) => {
                let name = name;
                let mut p = String::with_capacity(18usize + name.len());
                p.push_str("/_ccr/auto_follow/");
                p.push_str(name.as_ref());
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_ccr/auto_follow"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
pub struct CcrPauseFollow<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> CcrPauseFollow<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String) -> Self {
        CcrPauseFollow {
            client,
            index: index,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
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
    #[doc = "The name of the follower index that should pause following its leader index."]
    pub fn index(mut self, index: String) -> Self {
        self.index = index;
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
impl<B> Sender for CcrPauseFollow<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index = self.index;
            let mut p = String::with_capacity(19usize + index.len());
            p.push_str("/");
            p.push_str(index.as_ref());
            p.push_str("/_ccr/pause_follow");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct CcrPutAutoFollowPattern<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    name: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> CcrPutAutoFollowPattern<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, name: String) -> Self {
        CcrPutAutoFollowPattern {
            client,
            name: name,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
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
    #[doc = "The name of the auto follow pattern."]
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
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
impl<B> Sender for CcrPutAutoFollowPattern<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let name = self.name;
            let mut p = String::with_capacity(18usize + name.len());
            p.push_str("/_ccr/auto_follow/");
            p.push_str(name.as_ref());
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct CcrResumeFollow<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> CcrResumeFollow<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String) -> Self {
        CcrResumeFollow {
            client,
            index: index,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
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
    #[doc = "The name of the follow index to resume following."]
    pub fn index(mut self, index: String) -> Self {
        self.index = index;
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
impl<B> Sender for CcrResumeFollow<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index = self.index;
            let mut p = String::with_capacity(20usize + index.len());
            p.push_str("/");
            p.push_str(index.as_ref());
            p.push_str("/_ccr/resume_follow");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct CcrStats {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrStats {
    pub fn new(client: Elasticsearch) -> Self {
        CcrStats {
            client,
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
impl Sender for CcrStats {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = std::borrow::Cow::Borrowed("/_ccr/stats");
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
pub struct CcrUnfollow<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> CcrUnfollow<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String) -> Self {
        CcrUnfollow {
            client,
            index: index,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
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
    #[doc = "The name of the follower index that should be turned into a regular index."]
    pub fn index(mut self, index: String) -> Self {
        self.index = index;
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
impl<B> Sender for CcrUnfollow<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index = self.index;
            let mut p = String::with_capacity(15usize + index.len());
            p.push_str("/");
            p.push_str(index.as_ref());
            p.push_str("/_ccr/unfollow");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
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
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-delete-auto-follow-pattern.html"]
    pub fn delete_auto_follow_pattern(&self, name: String) -> CcrDeleteAutoFollowPattern {
        CcrDeleteAutoFollowPattern::new(self.client.clone(), name)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-put-follow.html"]
    pub fn follow<B>(&self, index: String) -> CcrFollow<B>
    where
        B: Serialize,
    {
        CcrFollow::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-follow-info.html"]
    pub fn follow_info(&self, index: Vec<String>) -> CcrFollowInfo {
        CcrFollowInfo::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-follow-stats.html"]
    pub fn follow_stats(&self, index: Vec<String>) -> CcrFollowStats {
        CcrFollowStats::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current"]
    pub fn forget_follower<B>(&self, index: String) -> CcrForgetFollower<B>
    where
        B: Serialize,
    {
        CcrForgetFollower::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-auto-follow-pattern.html"]
    pub fn get_auto_follow_pattern(&self) -> CcrGetAutoFollowPattern {
        CcrGetAutoFollowPattern::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-post-pause-follow.html"]
    pub fn pause_follow<B>(&self, index: String) -> CcrPauseFollow<B>
    where
        B: Serialize,
    {
        CcrPauseFollow::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-put-auto-follow-pattern.html"]
    pub fn put_auto_follow_pattern<B>(&self, name: String) -> CcrPutAutoFollowPattern<B>
    where
        B: Serialize,
    {
        CcrPutAutoFollowPattern::new(self.client.clone(), name)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-post-resume-follow.html"]
    pub fn resume_follow<B>(&self, index: String) -> CcrResumeFollow<B>
    where
        B: Serialize,
    {
        CcrResumeFollow::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-stats.html"]
    pub fn stats(&self) -> CcrStats {
        CcrStats::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current"]
    pub fn unfollow<B>(&self, index: String) -> CcrUnfollow<B>
    where
        B: Serialize,
    {
        CcrUnfollow::new(self.client.clone(), index)
    }
}
impl Elasticsearch {
    #[doc = "Ccr APIs"]
    pub fn ccr(&self) -> Ccr {
        Ccr::new(self.clone())
    }
}