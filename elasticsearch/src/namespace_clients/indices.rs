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
    request::{Body, HttpMethod, JsonBody, NdBody},
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Indices Analyze API"]
pub enum IndicesAnalyzeUrlParts<'a> {
    None,
    Index(&'a str),
}
impl<'a> IndicesAnalyzeUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesAnalyzeUrlParts::None => "/_analyze".into(),
            IndicesAnalyzeUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(10usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_analyze");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Analyze API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-analyze.html). Performs the analysis process on a text and return the tokens breakdown of the text."]
pub struct IndicesAnalyze<'a, B> {
    client: Elasticsearch,
    parts: IndicesAnalyzeUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    index: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> IndicesAnalyze<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesAnalyzeUrlParts<'a>) -> Self {
        IndicesAnalyze {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            index: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesAnalyze<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesAnalyze {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            index: self.index,
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
    #[doc = "The name of the index to scope the operation"]
    pub fn index(mut self, index: &'a str) -> Self {
        self.index = Some(index);
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
    #[doc = "Creates an asynchronous call to the Indices Analyze API that can be awaited"]
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
                #[serde(rename = "index")]
                index: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                index: self.index,
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
#[doc = "Url parts for the Indices Clear Cache API"]
pub enum IndicesClearCacheUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesClearCacheUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesClearCacheUrlParts::None => "/_cache/clear".into(),
            IndicesClearCacheUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(14usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_cache/clear");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Clear Cache API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-clearcache.html). Clears all or specific caches for one or more indices."]
pub struct IndicesClearCache<'a, B> {
    client: Elasticsearch,
    parts: IndicesClearCacheUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    fielddata: Option<bool>,
    fields: Option<&'a [&'a str]>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<&'a [&'a str]>,
    pretty: Option<bool>,
    query: Option<bool>,
    request: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> IndicesClearCache<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesClearCacheUrlParts<'a>) -> Self {
        IndicesClearCache {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            fielddata: None,
            fields: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            pretty: None,
            query: None,
            request: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesClearCache<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesClearCache {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            fielddata: self.fielddata,
            fields: self.fields,
            filter_path: self.filter_path,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            index: self.index,
            pretty: self.pretty,
            query: self.query,
            request: self.request,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "Clear field data"]
    pub fn fielddata(mut self, fielddata: bool) -> Self {
        self.fielddata = Some(fielddata);
        self
    }
    #[doc = "A comma-separated list of fields to clear when using the `fielddata` parameter (default: all)"]
    pub fn fields(mut self, fields: &'a [&'a str]) -> Self {
        self.fields = Some(fields);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "A comma-separated list of index name to limit the operation"]
    pub fn index(mut self, index: &'a [&'a str]) -> Self {
        self.index = Some(index);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Clear query caches"]
    pub fn query(mut self, query: bool) -> Self {
        self.query = Some(query);
        self
    }
    #[doc = "Clear request cache"]
    pub fn request(mut self, request: bool) -> Self {
        self.request = Some(request);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Clear Cache API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "fielddata")]
                fielddata: Option<bool>,
                #[serde(rename = "fields", serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'a [&'a str]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "index", serialize_with = "crate::client::serialize_coll_qs")]
                index: Option<&'a [&'a str]>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "query")]
                query: Option<bool>,
                #[serde(rename = "request")]
                request: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                fielddata: self.fielddata,
                fields: self.fields,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                index: self.index,
                pretty: self.pretty,
                query: self.query,
                request: self.request,
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
#[doc = "Url parts for the Indices Clone API"]
pub enum IndicesCloneUrlParts<'a> {
    IndexTarget(&'a str, &'a str),
}
impl<'a> IndicesCloneUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesCloneUrlParts::IndexTarget(ref index, ref target) => {
                let mut p = String::with_capacity(9usize + index.len() + target.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_clone/");
                p.push_str(target.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Clone API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-clone-index.html). Clones an index"]
pub struct IndicesClone<'a, B> {
    client: Elasticsearch,
    parts: IndicesCloneUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> IndicesClone<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesCloneUrlParts<'a>) -> Self {
        IndicesClone {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesClone<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesClone {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Set the number of active shards to wait for on the cloned index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Clone API that can be awaited"]
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Indices Close API"]
pub enum IndicesCloseUrlParts<'a> {
    Index(&'a [&'a str]),
}
impl<'a> IndicesCloseUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesCloseUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(8usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_close");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Close API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html). Closes an index."]
pub struct IndicesClose<'a, B> {
    client: Elasticsearch,
    parts: IndicesCloseUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> IndicesClose<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesCloseUrlParts<'a>) -> Self {
        IndicesClose {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesClose<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesClose {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Close API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Indices Create API"]
pub enum IndicesCreateUrlParts<'a> {
    Index(&'a str),
}
impl<'a> IndicesCreateUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesCreateUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(1usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Create API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-create-index.html). Creates an index with optional settings and mappings."]
pub struct IndicesCreate<'a, B> {
    client: Elasticsearch,
    parts: IndicesCreateUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> IndicesCreate<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesCreateUrlParts<'a>) -> Self {
        IndicesCreate {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            include_type_name: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesCreate<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesCreate {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            include_type_name: self.include_type_name,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
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
    #[doc = "Whether a type should be expected in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Set the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Create API that can be awaited"]
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
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                include_type_name: self.include_type_name,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Indices Delete API"]
pub enum IndicesDeleteUrlParts<'a> {
    Index(&'a [&'a str]),
}
impl<'a> IndicesDeleteUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesDeleteUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(1usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-delete-index.html). Deletes an index."]
pub struct IndicesDelete<'a> {
    client: Elasticsearch,
    parts: IndicesDeleteUrlParts<'a>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> IndicesDelete<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesDeleteUrlParts<'a>) -> Self {
        IndicesDelete {
            client,
            parts,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Ignore if a wildcard expression resolves to no concrete indices (default: false)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether wildcard expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Ignore unavailable indexes (default: false)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Delete API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Indices Delete Alias API"]
pub enum IndicesDeleteAliasUrlParts<'a> {
    IndexName(&'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesDeleteAliasUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesDeleteAliasUrlParts::IndexName(ref index, ref name) => {
                let index_str = index.join(",");
                let name_str = name.join(",");
                let mut p = String::with_capacity(9usize + index_str.len() + name_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_alias/");
                p.push_str(name_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Delete Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html). Deletes an alias."]
pub struct IndicesDeleteAlias<'a> {
    client: Elasticsearch,
    parts: IndicesDeleteAliasUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> IndicesDeleteAlias<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesDeleteAliasUrlParts<'a>) -> Self {
        IndicesDeleteAlias {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit timestamp for the document"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Delete Alias API that can be awaited"]
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Indices Delete Template API"]
pub enum IndicesDeleteTemplateUrlParts<'a> {
    Name(&'a str),
}
impl<'a> IndicesDeleteTemplateUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesDeleteTemplateUrlParts::Name(ref name) => {
                let mut p = String::with_capacity(11usize + name.len());
                p.push_str("/_template/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Delete Template API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html). Deletes an index template."]
pub struct IndicesDeleteTemplate<'a> {
    client: Elasticsearch,
    parts: IndicesDeleteTemplateUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> IndicesDeleteTemplate<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesDeleteTemplateUrlParts<'a>) -> Self {
        IndicesDeleteTemplate {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Delete Template API that can be awaited"]
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Indices Exists API"]
pub enum IndicesExistsUrlParts<'a> {
    Index(&'a [&'a str]),
}
impl<'a> IndicesExistsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesExistsUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(1usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Exists API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-exists.html). Returns information about whether a particular index exists."]
pub struct IndicesExists<'a> {
    client: Elasticsearch,
    parts: IndicesExistsUrlParts<'a>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IndicesExists<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesExistsUrlParts<'a>) -> Self {
        IndicesExists {
            client,
            parts,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            ignore_unavailable: None,
            include_defaults: None,
            local: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Ignore if a wildcard expression resolves to no concrete indices (default: false)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether wildcard expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Ignore unavailable indexes (default: false)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether to return all default setting for each of the indices."]
    pub fn include_defaults(mut self, include_defaults: bool) -> Self {
        self.include_defaults = Some(include_defaults);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Exists API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Head;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_defaults")]
                include_defaults: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_defaults: self.include_defaults,
                local: self.local,
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
#[doc = "Url parts for the Indices Exists Alias API"]
pub enum IndicesExistsAliasUrlParts<'a> {
    Name(&'a [&'a str]),
    IndexName(&'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesExistsAliasUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesExistsAliasUrlParts::Name(ref name) => {
                let name_str = name.join(",");
                let mut p = String::with_capacity(8usize + name_str.len());
                p.push_str("/_alias/");
                p.push_str(name_str.as_ref());
                p.into()
            }
            IndicesExistsAliasUrlParts::IndexName(ref index, ref name) => {
                let index_str = index.join(",");
                let name_str = name.join(",");
                let mut p = String::with_capacity(9usize + index_str.len() + name_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_alias/");
                p.push_str(name_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Exists Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html). Returns information about whether a particular alias exists."]
pub struct IndicesExistsAlias<'a> {
    client: Elasticsearch,
    parts: IndicesExistsAliasUrlParts<'a>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IndicesExistsAlias<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesExistsAliasUrlParts<'a>) -> Self {
        IndicesExistsAlias {
            client,
            parts,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            local: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Exists Alias API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Head;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
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
#[doc = "Url parts for the Indices Exists Template API"]
pub enum IndicesExistsTemplateUrlParts<'a> {
    Name(&'a [&'a str]),
}
impl<'a> IndicesExistsTemplateUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesExistsTemplateUrlParts::Name(ref name) => {
                let name_str = name.join(",");
                let mut p = String::with_capacity(11usize + name_str.len());
                p.push_str("/_template/");
                p.push_str(name_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Exists Template API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html). Returns information about whether a particular index template exists."]
pub struct IndicesExistsTemplate<'a> {
    client: Elasticsearch,
    parts: IndicesExistsTemplateUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IndicesExistsTemplate<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesExistsTemplateUrlParts<'a>) -> Self {
        IndicesExistsTemplate {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            local: None,
            master_timeout: None,
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
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous call to the Indices Exists Template API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Head;
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
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
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
#[doc = "Url parts for the Indices Exists Type API"]
pub enum IndicesExistsTypeUrlParts<'a> {
    IndexType(&'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesExistsTypeUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesExistsTypeUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(11usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping/");
                p.push_str(ty_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Exists Type API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-types-exists.html). Returns information about whether a particular document type exists. (DEPRECATED)"]
pub struct IndicesExistsType<'a> {
    client: Elasticsearch,
    parts: IndicesExistsTypeUrlParts<'a>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IndicesExistsType<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesExistsTypeUrlParts<'a>) -> Self {
        IndicesExistsType {
            client,
            parts,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            local: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Exists Type API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Head;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
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
#[doc = "Url parts for the Indices Flush API"]
pub enum IndicesFlushUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesFlushUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesFlushUrlParts::None => "/_flush".into(),
            IndicesFlushUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(8usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_flush");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Flush API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-flush.html). Performs the flush operation on one or more indices."]
pub struct IndicesFlush<'a, B> {
    client: Elasticsearch,
    parts: IndicesFlushUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    force: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    wait_if_ongoing: Option<bool>,
}
impl<'a, B> IndicesFlush<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesFlushUrlParts<'a>) -> Self {
        IndicesFlush {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            force: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            source: None,
            wait_if_ongoing: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesFlush<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesFlush {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            force: self.force,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            pretty: self.pretty,
            source: self.source,
            wait_if_ongoing: self.wait_if_ongoing,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Whether a flush should be forced even if it is not necessarily needed ie. if no changes will be committed to the index. This is useful if transaction log IDs should be incremented even if no uncommitted changes are present. (This setting can be considered as internal)"]
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "If set to true the flush operation will block until the flush can be executed if another flush operation is already executing. The default is true. If set to false the flush will be skipped iff if another flush operation is already running."]
    pub fn wait_if_ongoing(mut self, wait_if_ongoing: bool) -> Self {
        self.wait_if_ongoing = Some(wait_if_ongoing);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Flush API that can be awaited"]
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
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "wait_if_ongoing")]
                wait_if_ongoing: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                force: self.force,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                pretty: self.pretty,
                source: self.source,
                wait_if_ongoing: self.wait_if_ongoing,
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
#[doc = "Url parts for the Indices Flush Synced API"]
pub enum IndicesFlushSyncedUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesFlushSyncedUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesFlushSyncedUrlParts::None => "/_flush/synced".into(),
            IndicesFlushSyncedUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(15usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_flush/synced");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Flush Synced API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-synced-flush-api.html). Performs a synced flush operation on one or more indices."]
pub struct IndicesFlushSynced<'a, B> {
    client: Elasticsearch,
    parts: IndicesFlushSyncedUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> IndicesFlushSynced<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesFlushSyncedUrlParts<'a>) -> Self {
        IndicesFlushSynced {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesFlushSynced<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesFlushSynced {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "Creates an asynchronous call to the Indices Flush Synced API that can be awaited"]
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
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
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
#[doc = "Url parts for the Indices Forcemerge API"]
pub enum IndicesForcemergeUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesForcemergeUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesForcemergeUrlParts::None => "/_forcemerge".into(),
            IndicesForcemergeUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(13usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_forcemerge");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Forcemerge API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-forcemerge.html). Performs the force merge operation on one or more indices."]
pub struct IndicesForcemerge<'a, B> {
    client: Elasticsearch,
    parts: IndicesForcemergeUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    flush: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    max_num_segments: Option<i64>,
    only_expunge_deletes: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> IndicesForcemerge<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesForcemergeUrlParts<'a>) -> Self {
        IndicesForcemerge {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flush: None,
            human: None,
            ignore_unavailable: None,
            max_num_segments: None,
            only_expunge_deletes: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesForcemerge<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesForcemerge {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            flush: self.flush,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            max_num_segments: self.max_num_segments,
            only_expunge_deletes: self.only_expunge_deletes,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Specify whether the index should be flushed after performing the operation (default: true)"]
    pub fn flush(mut self, flush: bool) -> Self {
        self.flush = Some(flush);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "The number of segments the index should be merged into (default: dynamic)"]
    pub fn max_num_segments(mut self, max_num_segments: i64) -> Self {
        self.max_num_segments = Some(max_num_segments);
        self
    }
    #[doc = "Specify whether the operation should only expunge deleted documents"]
    pub fn only_expunge_deletes(mut self, only_expunge_deletes: bool) -> Self {
        self.only_expunge_deletes = Some(only_expunge_deletes);
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
    #[doc = "Creates an asynchronous call to the Indices Forcemerge API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "flush")]
                flush: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "max_num_segments")]
                max_num_segments: Option<i64>,
                #[serde(rename = "only_expunge_deletes")]
                only_expunge_deletes: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                flush: self.flush,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                max_num_segments: self.max_num_segments,
                only_expunge_deletes: self.only_expunge_deletes,
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
#[doc = "Url parts for the Indices Freeze API"]
pub enum IndicesFreezeUrlParts<'a> {
    Index(&'a str),
}
impl<'a> IndicesFreezeUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesFreezeUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(9usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_freeze");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Freeze API](https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html)."]
pub struct IndicesFreeze<'a, B> {
    client: Elasticsearch,
    parts: IndicesFreezeUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> IndicesFreeze<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesFreezeUrlParts<'a>) -> Self {
        IndicesFreeze {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesFreeze<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesFreeze {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Freeze API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Indices Get API"]
pub enum IndicesGetUrlParts<'a> {
    Index(&'a [&'a str]),
}
impl<'a> IndicesGetUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesGetUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(1usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Get API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-index.html). Returns information about one or more indices."]
pub struct IndicesGet<'a> {
    client: Elasticsearch,
    parts: IndicesGetUrlParts<'a>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IndicesGet<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesGetUrlParts<'a>) -> Self {
        IndicesGet {
            client,
            parts,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            ignore_unavailable: None,
            include_defaults: None,
            include_type_name: None,
            local: None,
            master_timeout: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Ignore if a wildcard expression resolves to no concrete indices (default: false)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether wildcard expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Ignore unavailable indexes (default: false)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether to return all default setting for each of the indices."]
    pub fn include_defaults(mut self, include_defaults: bool) -> Self {
        self.include_defaults = Some(include_defaults);
        self
    }
    #[doc = "Whether to add the type name to the response (default: false)"]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous call to the Indices Get API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_defaults")]
                include_defaults: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_defaults: self.include_defaults,
                include_type_name: self.include_type_name,
                local: self.local,
                master_timeout: self.master_timeout,
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
#[doc = "Url parts for the Indices Get Alias API"]
pub enum IndicesGetAliasUrlParts<'a> {
    None,
    Name(&'a [&'a str]),
    IndexName(&'a [&'a str], &'a [&'a str]),
    Index(&'a [&'a str]),
}
impl<'a> IndicesGetAliasUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesGetAliasUrlParts::None => "/_alias".into(),
            IndicesGetAliasUrlParts::Name(ref name) => {
                let name_str = name.join(",");
                let mut p = String::with_capacity(8usize + name_str.len());
                p.push_str("/_alias/");
                p.push_str(name_str.as_ref());
                p.into()
            }
            IndicesGetAliasUrlParts::IndexName(ref index, ref name) => {
                let index_str = index.join(",");
                let name_str = name.join(",");
                let mut p = String::with_capacity(9usize + index_str.len() + name_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_alias/");
                p.push_str(name_str.as_ref());
                p.into()
            }
            IndicesGetAliasUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(8usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_alias");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Get Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html). Returns an alias."]
pub struct IndicesGetAlias<'a> {
    client: Elasticsearch,
    parts: IndicesGetAliasUrlParts<'a>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IndicesGetAlias<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesGetAliasUrlParts<'a>) -> Self {
        IndicesGetAlias {
            client,
            parts,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            local: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Get Alias API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
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
#[doc = "Url parts for the Indices Get Field Mapping API"]
pub enum IndicesGetFieldMappingUrlParts<'a> {
    Fields(&'a [&'a str]),
    IndexFields(&'a [&'a str], &'a [&'a str]),
    TypeFields(&'a [&'a str], &'a [&'a str]),
    IndexTypeFields(&'a [&'a str], &'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesGetFieldMappingUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesGetFieldMappingUrlParts::Fields(ref fields) => {
                let fields_str = fields.join(",");
                let mut p = String::with_capacity(16usize + fields_str.len());
                p.push_str("/_mapping/field/");
                p.push_str(fields_str.as_ref());
                p.into()
            }
            IndicesGetFieldMappingUrlParts::IndexFields(ref index, ref fields) => {
                let index_str = index.join(",");
                let fields_str = fields.join(",");
                let mut p = String::with_capacity(17usize + index_str.len() + fields_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping/field/");
                p.push_str(fields_str.as_ref());
                p.into()
            }
            IndicesGetFieldMappingUrlParts::TypeFields(ref ty, ref fields) => {
                let ty_str = ty.join(",");
                let fields_str = fields.join(",");
                let mut p = String::with_capacity(17usize + ty_str.len() + fields_str.len());
                p.push_str("/_mapping/");
                p.push_str(ty_str.as_ref());
                p.push_str("/field/");
                p.push_str(fields_str.as_ref());
                p.into()
            }
            IndicesGetFieldMappingUrlParts::IndexTypeFields(ref index, ref ty, ref fields) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let fields_str = fields.join(",");
                let mut p = String::with_capacity(
                    18usize + index_str.len() + ty_str.len() + fields_str.len(),
                );
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping/");
                p.push_str(ty_str.as_ref());
                p.push_str("/field/");
                p.push_str(fields_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Get Field Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-field-mapping.html). Returns mapping for one or more fields."]
pub struct IndicesGetFieldMapping<'a> {
    client: Elasticsearch,
    parts: IndicesGetFieldMappingUrlParts<'a>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IndicesGetFieldMapping<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesGetFieldMappingUrlParts<'a>) -> Self {
        IndicesGetFieldMapping {
            client,
            parts,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            include_defaults: None,
            include_type_name: None,
            local: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether the default mapping values should be returned as well"]
    pub fn include_defaults(mut self, include_defaults: bool) -> Self {
        self.include_defaults = Some(include_defaults);
        self
    }
    #[doc = "Whether a type should be returned in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
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
    #[doc = "Creates an asynchronous call to the Indices Get Field Mapping API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_defaults")]
                include_defaults: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_defaults: self.include_defaults,
                include_type_name: self.include_type_name,
                local: self.local,
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
#[doc = "Url parts for the Indices Get Mapping API"]
pub enum IndicesGetMappingUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
    Type(&'a [&'a str]),
    IndexType(&'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesGetMappingUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesGetMappingUrlParts::None => "/_mapping".into(),
            IndicesGetMappingUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping");
                p.into()
            }
            IndicesGetMappingUrlParts::Type(ref ty) => {
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(10usize + ty_str.len());
                p.push_str("/_mapping/");
                p.push_str(ty_str.as_ref());
                p.into()
            }
            IndicesGetMappingUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(11usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping/");
                p.push_str(ty_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Get Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-mapping.html). Returns mappings for one or more indices."]
pub struct IndicesGetMapping<'a> {
    client: Elasticsearch,
    parts: IndicesGetMappingUrlParts<'a>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IndicesGetMapping<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesGetMappingUrlParts<'a>) -> Self {
        IndicesGetMapping {
            client,
            parts,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            include_type_name: None,
            local: None,
            master_timeout: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether to add the type name to the response (default: false)"]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous call to the Indices Get Mapping API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_type_name: self.include_type_name,
                local: self.local,
                master_timeout: self.master_timeout,
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
#[doc = "Url parts for the Indices Get Settings API"]
pub enum IndicesGetSettingsUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
    IndexName(&'a [&'a str], &'a [&'a str]),
    Name(&'a [&'a str]),
}
impl<'a> IndicesGetSettingsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesGetSettingsUrlParts::None => "/_settings".into(),
            IndicesGetSettingsUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(11usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_settings");
                p.into()
            }
            IndicesGetSettingsUrlParts::IndexName(ref index, ref name) => {
                let index_str = index.join(",");
                let name_str = name.join(",");
                let mut p = String::with_capacity(12usize + index_str.len() + name_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_settings/");
                p.push_str(name_str.as_ref());
                p.into()
            }
            IndicesGetSettingsUrlParts::Name(ref name) => {
                let name_str = name.join(",");
                let mut p = String::with_capacity(11usize + name_str.len());
                p.push_str("/_settings/");
                p.push_str(name_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Get Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-settings.html). Returns settings for one or more indices."]
pub struct IndicesGetSettings<'a> {
    client: Elasticsearch,
    parts: IndicesGetSettingsUrlParts<'a>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IndicesGetSettings<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesGetSettingsUrlParts<'a>) -> Self {
        IndicesGetSettings {
            client,
            parts,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            ignore_unavailable: None,
            include_defaults: None,
            local: None,
            master_timeout: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether to return all default setting for each of the indices."]
    pub fn include_defaults(mut self, include_defaults: bool) -> Self {
        self.include_defaults = Some(include_defaults);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous call to the Indices Get Settings API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_defaults")]
                include_defaults: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_defaults: self.include_defaults,
                local: self.local,
                master_timeout: self.master_timeout,
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
#[doc = "Url parts for the Indices Get Template API"]
pub enum IndicesGetTemplateUrlParts<'a> {
    None,
    Name(&'a [&'a str]),
}
impl<'a> IndicesGetTemplateUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesGetTemplateUrlParts::None => "/_template".into(),
            IndicesGetTemplateUrlParts::Name(ref name) => {
                let name_str = name.join(",");
                let mut p = String::with_capacity(11usize + name_str.len());
                p.push_str("/_template/");
                p.push_str(name_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Get Template API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html). Returns an index template."]
pub struct IndicesGetTemplate<'a> {
    client: Elasticsearch,
    parts: IndicesGetTemplateUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IndicesGetTemplate<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesGetTemplateUrlParts<'a>) -> Self {
        IndicesGetTemplate {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            include_type_name: None,
            local: None,
            master_timeout: None,
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
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether a type should be returned in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous call to the Indices Get Template API that can be awaited"]
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
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                include_type_name: self.include_type_name,
                local: self.local,
                master_timeout: self.master_timeout,
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
#[doc = "Url parts for the Indices Get Upgrade API"]
pub enum IndicesGetUpgradeUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesGetUpgradeUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesGetUpgradeUrlParts::None => "/_upgrade".into(),
            IndicesGetUpgradeUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_upgrade");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Get Upgrade API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html). The _upgrade API is no longer useful and will be removed."]
pub struct IndicesGetUpgrade<'a> {
    client: Elasticsearch,
    parts: IndicesGetUpgradeUrlParts<'a>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IndicesGetUpgrade<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesGetUpgradeUrlParts<'a>) -> Self {
        IndicesGetUpgrade {
            client,
            parts,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "Creates an asynchronous call to the Indices Get Upgrade API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
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
#[doc = "Url parts for the Indices Open API"]
pub enum IndicesOpenUrlParts<'a> {
    Index(&'a [&'a str]),
}
impl<'a> IndicesOpenUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesOpenUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(7usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_open");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Open API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html). Opens an index."]
pub struct IndicesOpen<'a, B> {
    client: Elasticsearch,
    parts: IndicesOpenUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> IndicesOpen<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesOpenUrlParts<'a>) -> Self {
        IndicesOpen {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesOpen<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesOpen {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Open API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Indices Put Alias API"]
pub enum IndicesPutAliasUrlParts<'a> {
    IndexName(&'a [&'a str], &'a str),
}
impl<'a> IndicesPutAliasUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesPutAliasUrlParts::IndexName(ref index, ref name) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(9usize + index_str.len() + name.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_alias/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Put Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html). Creates or updates an alias."]
pub struct IndicesPutAlias<'a, B> {
    client: Elasticsearch,
    parts: IndicesPutAliasUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> IndicesPutAlias<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesPutAliasUrlParts<'a>) -> Self {
        IndicesPutAlias {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesPutAlias<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesPutAlias {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit timestamp for the document"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Put Alias API that can be awaited"]
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Indices Put Mapping API"]
pub enum IndicesPutMappingUrlParts<'a> {
    Index(&'a [&'a str]),
    IndexType(&'a [&'a str], &'a str),
    Type(&'a str),
}
impl<'a> IndicesPutMappingUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesPutMappingUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping");
                p.into()
            }
            IndicesPutMappingUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(11usize + index_str.len() + ty.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/_mapping");
                p.into()
            }
            IndicesPutMappingUrlParts::Type(ref ty) => {
                let mut p = String::with_capacity(11usize + ty.len());
                p.push_str("/_mappings/");
                p.push_str(ty.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-put-mapping.html). Updates the index mappings."]
pub struct IndicesPutMapping<'a, B> {
    client: Elasticsearch,
    parts: IndicesPutMappingUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> IndicesPutMapping<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesPutMappingUrlParts<'a>) -> Self {
        IndicesPutMapping {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            include_type_name: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesPutMapping<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesPutMapping {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            include_type_name: self.include_type_name,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether a type should be expected in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Put Mapping API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_type_name: self.include_type_name,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Indices Put Settings API"]
pub enum IndicesPutSettingsUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesPutSettingsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesPutSettingsUrlParts::None => "/_settings".into(),
            IndicesPutSettingsUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(11usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_settings");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Put Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-update-settings.html). Updates the index settings."]
pub struct IndicesPutSettings<'a, B> {
    client: Elasticsearch,
    parts: IndicesPutSettingsUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'a str>,
    preserve_existing: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> IndicesPutSettings<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesPutSettingsUrlParts<'a>) -> Self {
        IndicesPutSettings {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            preserve_existing: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesPutSettings<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesPutSettings {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            flat_settings: self.flat_settings,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            preserve_existing: self.preserve_existing,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to update existing settings. If set to `true` existing settings on an index remain unchanged, the default is `false`"]
    pub fn preserve_existing(mut self, preserve_existing: bool) -> Self {
        self.preserve_existing = Some(preserve_existing);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Put Settings API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "preserve_existing")]
                preserve_existing: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                preserve_existing: self.preserve_existing,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Indices Put Template API"]
pub enum IndicesPutTemplateUrlParts<'a> {
    Name(&'a str),
}
impl<'a> IndicesPutTemplateUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesPutTemplateUrlParts::Name(ref name) => {
                let mut p = String::with_capacity(11usize + name.len());
                p.push_str("/_template/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Put Template API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html). Creates or updates an index template."]
pub struct IndicesPutTemplate<'a, B> {
    client: Elasticsearch,
    parts: IndicesPutTemplateUrlParts<'a>,
    body: Option<B>,
    create: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<&'a str>,
    order: Option<i64>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> IndicesPutTemplate<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesPutTemplateUrlParts<'a>) -> Self {
        IndicesPutTemplate {
            client,
            parts,
            body: None,
            create: None,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            include_type_name: None,
            master_timeout: None,
            order: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesPutTemplate<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesPutTemplate {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            create: self.create,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            flat_settings: self.flat_settings,
            human: self.human,
            include_type_name: self.include_type_name,
            master_timeout: self.master_timeout,
            order: self.order,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Whether the index template should only be added if new or can also replace an existing one"]
    pub fn create(mut self, create: bool) -> Self {
        self.create = Some(create);
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
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether a type should be returned in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "The order for this template when merging multiple matching ones (higher numbers are merged later, overriding the lower numbers)"]
    pub fn order(mut self, order: i64) -> Self {
        self.order = Some(order);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Put Template API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "create")]
                create: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "order")]
                order: Option<i64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParams {
                create: self.create,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                include_type_name: self.include_type_name,
                master_timeout: self.master_timeout,
                order: self.order,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Indices Recovery API"]
pub enum IndicesRecoveryUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesRecoveryUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesRecoveryUrlParts::None => "/_recovery".into(),
            IndicesRecoveryUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(11usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_recovery");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Recovery API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-recovery.html). Returns information about ongoing index shard recoveries."]
pub struct IndicesRecovery<'a> {
    client: Elasticsearch,
    parts: IndicesRecoveryUrlParts<'a>,
    active_only: Option<bool>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IndicesRecovery<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesRecoveryUrlParts<'a>) -> Self {
        IndicesRecovery {
            client,
            parts,
            active_only: None,
            detailed: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Display only those recoveries that are currently on-going"]
    pub fn active_only(mut self, active_only: bool) -> Self {
        self.active_only = Some(active_only);
        self
    }
    #[doc = "Whether to display detailed information about shard recovery"]
    pub fn detailed(mut self, detailed: bool) -> Self {
        self.detailed = Some(detailed);
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
    #[doc = "Creates an asynchronous call to the Indices Recovery API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "active_only")]
                active_only: Option<bool>,
                #[serde(rename = "detailed")]
                detailed: Option<bool>,
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
                active_only: self.active_only,
                detailed: self.detailed,
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
#[doc = "Url parts for the Indices Refresh API"]
pub enum IndicesRefreshUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesRefreshUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesRefreshUrlParts::None => "/_refresh".into(),
            IndicesRefreshUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_refresh");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Refresh API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-refresh.html). Performs the refresh operation in one or more indices."]
pub struct IndicesRefresh<'a, B> {
    client: Elasticsearch,
    parts: IndicesRefreshUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> IndicesRefresh<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesRefreshUrlParts<'a>) -> Self {
        IndicesRefresh {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesRefresh<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesRefresh {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "Creates an asynchronous call to the Indices Refresh API that can be awaited"]
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
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
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
#[doc = "Url parts for the Indices Rollover API"]
pub enum IndicesRolloverUrlParts<'a> {
    Alias(&'a str),
    AliasNewIndex(&'a str, &'a str),
}
impl<'a> IndicesRolloverUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesRolloverUrlParts::Alias(ref alias) => {
                let mut p = String::with_capacity(11usize + alias.len());
                p.push_str("/");
                p.push_str(alias.as_ref());
                p.push_str("/_rollover");
                p.into()
            }
            IndicesRolloverUrlParts::AliasNewIndex(ref alias, ref new_index) => {
                let mut p = String::with_capacity(12usize + alias.len() + new_index.len());
                p.push_str("/");
                p.push_str(alias.as_ref());
                p.push_str("/_rollover/");
                p.push_str(new_index.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Rollover API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-rollover-index.html). Updates an alias to point to a new index when the existing index\nis considered to be too large or too old."]
pub struct IndicesRollover<'a, B> {
    client: Elasticsearch,
    parts: IndicesRolloverUrlParts<'a>,
    body: Option<B>,
    dry_run: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> IndicesRollover<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesRolloverUrlParts<'a>) -> Self {
        IndicesRollover {
            client,
            parts,
            body: None,
            dry_run: None,
            error_trace: None,
            filter_path: None,
            human: None,
            include_type_name: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesRollover<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesRollover {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            dry_run: self.dry_run,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            include_type_name: self.include_type_name,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "If set to true the rollover action will only be validated but not actually performed even if a condition matches. The default is false"]
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = Some(dry_run);
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
    #[doc = "Whether a type should be included in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Set the number of active shards to wait for on the newly created rollover index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Rollover API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "dry_run")]
                dry_run: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParams {
                dry_run: self.dry_run,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                include_type_name: self.include_type_name,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Indices Segments API"]
pub enum IndicesSegmentsUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesSegmentsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesSegmentsUrlParts::None => "/_segments".into(),
            IndicesSegmentsUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(11usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_segments");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Segments API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-segments.html). Provides low-level information about segments in a Lucene index."]
pub struct IndicesSegments<'a> {
    client: Elasticsearch,
    parts: IndicesSegmentsUrlParts<'a>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    verbose: Option<bool>,
}
impl<'a> IndicesSegments<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesSegmentsUrlParts<'a>) -> Self {
        IndicesSegments {
            client,
            parts,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            source: None,
            verbose: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "Includes detailed memory usage by Lucene."]
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = Some(verbose);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Segments API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "verbose")]
                verbose: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                pretty: self.pretty,
                source: self.source,
                verbose: self.verbose,
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
#[doc = "Url parts for the Indices Shard Stores API"]
pub enum IndicesShardStoresUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesShardStoresUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesShardStoresUrlParts::None => "/_shard_stores".into(),
            IndicesShardStoresUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(15usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_shard_stores");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Shard Stores API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shards-stores.html). Provides store information for shard copies of indices."]
pub struct IndicesShardStores<'a> {
    client: Elasticsearch,
    parts: IndicesShardStoresUrlParts<'a>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    status: Option<&'a [&'a str]>,
}
impl<'a> IndicesShardStores<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesShardStoresUrlParts<'a>) -> Self {
        IndicesShardStores {
            client,
            parts,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            source: None,
            status: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "A comma-separated list of statuses used to filter on shards to get store information for"]
    pub fn status(mut self, status: &'a [&'a str]) -> Self {
        self.status = Some(status);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Shard Stores API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "status", serialize_with = "crate::client::serialize_coll_qs")]
                status: Option<&'a [&'a str]>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                pretty: self.pretty,
                source: self.source,
                status: self.status,
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
#[doc = "Url parts for the Indices Shrink API"]
pub enum IndicesShrinkUrlParts<'a> {
    IndexTarget(&'a str, &'a str),
}
impl<'a> IndicesShrinkUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesShrinkUrlParts::IndexTarget(ref index, ref target) => {
                let mut p = String::with_capacity(10usize + index.len() + target.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_shrink/");
                p.push_str(target.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Shrink API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shrink-index.html). Allow to shrink an existing index into a new index with fewer primary shards."]
pub struct IndicesShrink<'a, B> {
    client: Elasticsearch,
    parts: IndicesShrinkUrlParts<'a>,
    body: Option<B>,
    copy_settings: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> IndicesShrink<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesShrinkUrlParts<'a>) -> Self {
        IndicesShrink {
            client,
            parts,
            body: None,
            copy_settings: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesShrink<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesShrink {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            copy_settings: self.copy_settings,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "whether or not to copy settings from the source index (defaults to false)"]
    pub fn copy_settings(mut self, copy_settings: bool) -> Self {
        self.copy_settings = Some(copy_settings);
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Set the number of active shards to wait for on the shrunken index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Shrink API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "copy_settings")]
                copy_settings: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParams {
                copy_settings: self.copy_settings,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Indices Split API"]
pub enum IndicesSplitUrlParts<'a> {
    IndexTarget(&'a str, &'a str),
}
impl<'a> IndicesSplitUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesSplitUrlParts::IndexTarget(ref index, ref target) => {
                let mut p = String::with_capacity(9usize + index.len() + target.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_split/");
                p.push_str(target.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Split API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-split-index.html). Allows you to split an existing index into a new index with more primary shards."]
pub struct IndicesSplit<'a, B> {
    client: Elasticsearch,
    parts: IndicesSplitUrlParts<'a>,
    body: Option<B>,
    copy_settings: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> IndicesSplit<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesSplitUrlParts<'a>) -> Self {
        IndicesSplit {
            client,
            parts,
            body: None,
            copy_settings: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesSplit<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesSplit {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            copy_settings: self.copy_settings,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "whether or not to copy settings from the source index (defaults to false)"]
    pub fn copy_settings(mut self, copy_settings: bool) -> Self {
        self.copy_settings = Some(copy_settings);
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Set the number of active shards to wait for on the shrunken index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Split API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "copy_settings")]
                copy_settings: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParams {
                copy_settings: self.copy_settings,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Indices Stats API"]
pub enum IndicesStatsUrlParts<'a> {
    None,
    Metric(&'a [&'a str]),
    Index(&'a [&'a str]),
    IndexMetric(&'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesStatsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesStatsUrlParts::None => "/_stats".into(),
            IndicesStatsUrlParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(8usize + metric_str.len());
                p.push_str("/_stats/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
            IndicesStatsUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(8usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_stats");
                p.into()
            }
            IndicesStatsUrlParts::IndexMetric(ref index, ref metric) => {
                let index_str = index.join(",");
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(9usize + index_str.len() + metric_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_stats/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-stats.html). Provides statistics on operations happening in an index."]
pub struct IndicesStats<'a> {
    client: Elasticsearch,
    parts: IndicesStatsUrlParts<'a>,
    completion_fields: Option<&'a [&'a str]>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    fielddata_fields: Option<&'a [&'a str]>,
    fields: Option<&'a [&'a str]>,
    filter_path: Option<&'a [&'a str]>,
    forbid_closed_indices: Option<bool>,
    groups: Option<&'a [&'a str]>,
    human: Option<bool>,
    include_segment_file_sizes: Option<bool>,
    include_unloaded_segments: Option<bool>,
    level: Option<Level>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    types: Option<&'a [&'a str]>,
}
impl<'a> IndicesStats<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesStatsUrlParts<'a>) -> Self {
        IndicesStats {
            client,
            parts,
            completion_fields: None,
            error_trace: None,
            expand_wildcards: None,
            fielddata_fields: None,
            fields: None,
            filter_path: None,
            forbid_closed_indices: None,
            groups: None,
            human: None,
            include_segment_file_sizes: None,
            include_unloaded_segments: None,
            level: None,
            pretty: None,
            source: None,
            types: None,
        }
    }
    #[doc = "A comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards)"]
    pub fn completion_fields(mut self, completion_fields: &'a [&'a str]) -> Self {
        self.completion_fields = Some(completion_fields);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of fields for `fielddata` index metric (supports wildcards)"]
    pub fn fielddata_fields(mut self, fielddata_fields: &'a [&'a str]) -> Self {
        self.fielddata_fields = Some(fielddata_fields);
        self
    }
    #[doc = "A comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards)"]
    pub fn fields(mut self, fields: &'a [&'a str]) -> Self {
        self.fields = Some(fields);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "If set to false stats will also collected from closed indices if explicitly specified or if expand_wildcards expands to closed indices"]
    pub fn forbid_closed_indices(mut self, forbid_closed_indices: bool) -> Self {
        self.forbid_closed_indices = Some(forbid_closed_indices);
        self
    }
    #[doc = "A comma-separated list of search groups for `search` index metric"]
    pub fn groups(mut self, groups: &'a [&'a str]) -> Self {
        self.groups = Some(groups);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested)"]
    pub fn include_segment_file_sizes(mut self, include_segment_file_sizes: bool) -> Self {
        self.include_segment_file_sizes = Some(include_segment_file_sizes);
        self
    }
    #[doc = "If set to true segment stats will include stats for segments that are not currently loaded into memory"]
    pub fn include_unloaded_segments(mut self, include_unloaded_segments: bool) -> Self {
        self.include_unloaded_segments = Some(include_unloaded_segments);
        self
    }
    #[doc = "Return stats aggregated at cluster, index or shard level"]
    pub fn level(mut self, level: Level) -> Self {
        self.level = Some(level);
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
    #[doc = "A comma-separated list of document types for the `indexing` index metric"]
    pub fn types(mut self, types: &'a [&'a str]) -> Self {
        self.types = Some(types);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(
                    rename = "completion_fields",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                completion_fields: Option<&'a [&'a str]>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "fielddata_fields",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                fielddata_fields: Option<&'a [&'a str]>,
                #[serde(rename = "fields", serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'a [&'a str]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "forbid_closed_indices")]
                forbid_closed_indices: Option<bool>,
                #[serde(rename = "groups", serialize_with = "crate::client::serialize_coll_qs")]
                groups: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "include_segment_file_sizes")]
                include_segment_file_sizes: Option<bool>,
                #[serde(rename = "include_unloaded_segments")]
                include_unloaded_segments: Option<bool>,
                #[serde(rename = "level")]
                level: Option<Level>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "types", serialize_with = "crate::client::serialize_coll_qs")]
                types: Option<&'a [&'a str]>,
            }
            let query_params = QueryParams {
                completion_fields: self.completion_fields,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                fielddata_fields: self.fielddata_fields,
                fields: self.fields,
                filter_path: self.filter_path,
                forbid_closed_indices: self.forbid_closed_indices,
                groups: self.groups,
                human: self.human,
                include_segment_file_sizes: self.include_segment_file_sizes,
                include_unloaded_segments: self.include_unloaded_segments,
                level: self.level,
                pretty: self.pretty,
                source: self.source,
                types: self.types,
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
#[doc = "Url parts for the Indices Unfreeze API"]
pub enum IndicesUnfreezeUrlParts<'a> {
    Index(&'a str),
}
impl<'a> IndicesUnfreezeUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesUnfreezeUrlParts::Index(ref index) => {
                let mut p = String::with_capacity(11usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_unfreeze");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Unfreeze API](https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html)."]
pub struct IndicesUnfreeze<'a, B> {
    client: Elasticsearch,
    parts: IndicesUnfreezeUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_active_shards: Option<&'a str>,
}
impl<'a, B> IndicesUnfreeze<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesUnfreezeUrlParts<'a>) -> Self {
        IndicesUnfreeze {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesUnfreeze<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesUnfreeze {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'a str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Unfreeze API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'a str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Indices Update Aliases API"]
pub enum IndicesUpdateAliasesUrlParts {
    None,
}
impl IndicesUpdateAliasesUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesUpdateAliasesUrlParts::None => "/_aliases".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Update Aliases API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html). Updates index aliases."]
pub struct IndicesUpdateAliases<'a, B> {
    client: Elasticsearch,
    parts: IndicesUpdateAliasesUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> IndicesUpdateAliases<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch) -> Self {
        IndicesUpdateAliases {
            client,
            parts: IndicesUpdateAliasesUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesUpdateAliases<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesUpdateAliases {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Request timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Update Aliases API that can be awaited"]
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Indices Upgrade API"]
pub enum IndicesUpgradeUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesUpgradeUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesUpgradeUrlParts::None => "/_upgrade".into(),
            IndicesUpgradeUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_upgrade");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Upgrade API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html). The _upgrade API is no longer useful and will be removed."]
pub struct IndicesUpgrade<'a, B> {
    client: Elasticsearch,
    parts: IndicesUpgradeUrlParts<'a>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    only_ancient_segments: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    wait_for_completion: Option<bool>,
}
impl<'a, B> IndicesUpgrade<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesUpgradeUrlParts<'a>) -> Self {
        IndicesUpgrade {
            client,
            parts,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            only_ancient_segments: None,
            pretty: None,
            source: None,
            wait_for_completion: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesUpgrade<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesUpgrade {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            only_ancient_segments: self.only_ancient_segments,
            pretty: self.pretty,
            source: self.source,
            wait_for_completion: self.wait_for_completion,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "If true, only ancient (an older Lucene major release) segments will be upgraded"]
    pub fn only_ancient_segments(mut self, only_ancient_segments: bool) -> Self {
        self.only_ancient_segments = Some(only_ancient_segments);
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
    #[doc = "Specify whether the request should block until the all segments are upgraded (default: false)"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Upgrade API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "only_ancient_segments")]
                only_ancient_segments: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                only_ancient_segments: self.only_ancient_segments,
                pretty: self.pretty,
                source: self.source,
                wait_for_completion: self.wait_for_completion,
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
#[doc = "Url parts for the Indices Validate Query API"]
pub enum IndicesValidateQueryUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
    IndexType(&'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesValidateQueryUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            IndicesValidateQueryUrlParts::None => "/_validate/query".into(),
            IndicesValidateQueryUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(17usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_validate/query");
                p.into()
            }
            IndicesValidateQueryUrlParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(18usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_validate/query");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Validate Query API](https://www.elastic.co/guide/en/elasticsearch/reference/master/search-validate.html). Allows a user to validate a potentially expensive query without executing it."]
pub struct IndicesValidateQuery<'a, B> {
    client: Elasticsearch,
    parts: IndicesValidateQueryUrlParts<'a>,
    all_shards: Option<bool>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'a str>,
    body: Option<B>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'a str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    explain: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    pretty: Option<bool>,
    q: Option<&'a str>,
    rewrite: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> IndicesValidateQuery<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: IndicesValidateQueryUrlParts<'a>) -> Self {
        IndicesValidateQuery {
            client,
            parts,
            all_shards: None,
            allow_no_indices: None,
            analyze_wildcard: None,
            analyzer: None,
            body: None,
            default_operator: None,
            df: None,
            error_trace: None,
            expand_wildcards: None,
            explain: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            lenient: None,
            pretty: None,
            q: None,
            rewrite: None,
            source: None,
        }
    }
    #[doc = "Execute validation on all shards instead of one random shard per index"]
    pub fn all_shards(mut self, all_shards: bool) -> Self {
        self.all_shards = Some(all_shards);
        self
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "The analyzer to use for the query string"]
    pub fn analyzer(mut self, analyzer: &'a str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesValidateQuery<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesValidateQuery {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            all_shards: self.all_shards,
            allow_no_indices: self.allow_no_indices,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            default_operator: self.default_operator,
            df: self.df,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            explain: self.explain,
            filter_path: self.filter_path,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            lenient: self.lenient,
            pretty: self.pretty,
            q: self.q,
            rewrite: self.rewrite,
            source: self.source,
        }
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: &'a str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: ExpandWildcards) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "Return detailed information about the error"]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: &'a str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "Provide a more detailed explanation showing the actual Lucene query that will be executed."]
    pub fn rewrite(mut self, rewrite: bool) -> Self {
        self.rewrite = Some(rewrite);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Validate Query API that can be awaited"]
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
                #[serde(rename = "all_shards")]
                all_shards: Option<bool>,
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer")]
                analyzer: Option<&'a str>,
                #[serde(rename = "default_operator")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df")]
                df: Option<&'a str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "explain")]
                explain: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient")]
                lenient: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "q")]
                q: Option<&'a str>,
                #[serde(rename = "rewrite")]
                rewrite: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                all_shards: self.all_shards,
                allow_no_indices: self.allow_no_indices,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                explain: self.explain,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                pretty: self.pretty,
                q: self.q,
                rewrite: self.rewrite,
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
#[doc = "Namespace client for Indices APIs"]
pub struct Indices {
    client: Elasticsearch,
}
impl Indices {
    pub fn new(client: Elasticsearch) -> Self {
        Indices { client }
    }
    #[doc = "Performs the analysis process on a text and return the tokens breakdown of the text."]
    pub fn analyze<'a>(&self, parts: IndicesAnalyzeUrlParts<'a>) -> IndicesAnalyze<'a, ()> {
        IndicesAnalyze::new(self.client.clone(), parts)
    }
    #[doc = "Clears all or specific caches for one or more indices."]
    pub fn clear_cache<'a>(
        &self,
        parts: IndicesClearCacheUrlParts<'a>,
    ) -> IndicesClearCache<'a, ()> {
        IndicesClearCache::new(self.client.clone(), parts)
    }
    #[doc = "Clones an index"]
    pub fn clone<'a>(&self, parts: IndicesCloneUrlParts<'a>) -> IndicesClone<'a, ()> {
        IndicesClone::new(self.client.clone(), parts)
    }
    #[doc = "Closes an index."]
    pub fn close<'a>(&self, parts: IndicesCloseUrlParts<'a>) -> IndicesClose<'a, ()> {
        IndicesClose::new(self.client.clone(), parts)
    }
    #[doc = "Creates an index with optional settings and mappings."]
    pub fn create<'a>(&self, parts: IndicesCreateUrlParts<'a>) -> IndicesCreate<'a, ()> {
        IndicesCreate::new(self.client.clone(), parts)
    }
    #[doc = "Deletes an index."]
    pub fn delete<'a>(&self, parts: IndicesDeleteUrlParts<'a>) -> IndicesDelete<'a> {
        IndicesDelete::new(self.client.clone(), parts)
    }
    #[doc = "Deletes an alias."]
    pub fn delete_alias<'a>(
        &self,
        parts: IndicesDeleteAliasUrlParts<'a>,
    ) -> IndicesDeleteAlias<'a> {
        IndicesDeleteAlias::new(self.client.clone(), parts)
    }
    #[doc = "Deletes an index template."]
    pub fn delete_template<'a>(
        &self,
        parts: IndicesDeleteTemplateUrlParts<'a>,
    ) -> IndicesDeleteTemplate<'a> {
        IndicesDeleteTemplate::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about whether a particular index exists."]
    pub fn exists<'a>(&self, parts: IndicesExistsUrlParts<'a>) -> IndicesExists<'a> {
        IndicesExists::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about whether a particular alias exists."]
    pub fn exists_alias<'a>(
        &self,
        parts: IndicesExistsAliasUrlParts<'a>,
    ) -> IndicesExistsAlias<'a> {
        IndicesExistsAlias::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about whether a particular index template exists."]
    pub fn exists_template<'a>(
        &self,
        parts: IndicesExistsTemplateUrlParts<'a>,
    ) -> IndicesExistsTemplate<'a> {
        IndicesExistsTemplate::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about whether a particular document type exists. (DEPRECATED)"]
    pub fn exists_type<'a>(&self, parts: IndicesExistsTypeUrlParts<'a>) -> IndicesExistsType<'a> {
        IndicesExistsType::new(self.client.clone(), parts)
    }
    #[doc = "Performs the flush operation on one or more indices."]
    pub fn flush<'a>(&self, parts: IndicesFlushUrlParts<'a>) -> IndicesFlush<'a, ()> {
        IndicesFlush::new(self.client.clone(), parts)
    }
    #[doc = "Performs a synced flush operation on one or more indices."]
    pub fn flush_synced<'a>(
        &self,
        parts: IndicesFlushSyncedUrlParts<'a>,
    ) -> IndicesFlushSynced<'a, ()> {
        IndicesFlushSynced::new(self.client.clone(), parts)
    }
    #[doc = "Performs the force merge operation on one or more indices."]
    pub fn forcemerge<'a>(
        &self,
        parts: IndicesForcemergeUrlParts<'a>,
    ) -> IndicesForcemerge<'a, ()> {
        IndicesForcemerge::new(self.client.clone(), parts)
    }
    pub fn freeze<'a>(&self, parts: IndicesFreezeUrlParts<'a>) -> IndicesFreeze<'a, ()> {
        IndicesFreeze::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about one or more indices."]
    pub fn get<'a>(&self, parts: IndicesGetUrlParts<'a>) -> IndicesGet<'a> {
        IndicesGet::new(self.client.clone(), parts)
    }
    #[doc = "Returns an alias."]
    pub fn get_alias<'a>(&self, parts: IndicesGetAliasUrlParts<'a>) -> IndicesGetAlias<'a> {
        IndicesGetAlias::new(self.client.clone(), parts)
    }
    #[doc = "Returns mapping for one or more fields."]
    pub fn get_field_mapping<'a>(
        &self,
        parts: IndicesGetFieldMappingUrlParts<'a>,
    ) -> IndicesGetFieldMapping<'a> {
        IndicesGetFieldMapping::new(self.client.clone(), parts)
    }
    #[doc = "Returns mappings for one or more indices."]
    pub fn get_mapping<'a>(&self, parts: IndicesGetMappingUrlParts<'a>) -> IndicesGetMapping<'a> {
        IndicesGetMapping::new(self.client.clone(), parts)
    }
    #[doc = "Returns settings for one or more indices."]
    pub fn get_settings<'a>(
        &self,
        parts: IndicesGetSettingsUrlParts<'a>,
    ) -> IndicesGetSettings<'a> {
        IndicesGetSettings::new(self.client.clone(), parts)
    }
    #[doc = "Returns an index template."]
    pub fn get_template<'a>(
        &self,
        parts: IndicesGetTemplateUrlParts<'a>,
    ) -> IndicesGetTemplate<'a> {
        IndicesGetTemplate::new(self.client.clone(), parts)
    }
    #[doc = "The _upgrade API is no longer useful and will be removed."]
    pub fn get_upgrade<'a>(&self, parts: IndicesGetUpgradeUrlParts<'a>) -> IndicesGetUpgrade<'a> {
        IndicesGetUpgrade::new(self.client.clone(), parts)
    }
    #[doc = "Opens an index."]
    pub fn open<'a>(&self, parts: IndicesOpenUrlParts<'a>) -> IndicesOpen<'a, ()> {
        IndicesOpen::new(self.client.clone(), parts)
    }
    #[doc = "Creates or updates an alias."]
    pub fn put_alias<'a>(&self, parts: IndicesPutAliasUrlParts<'a>) -> IndicesPutAlias<'a, ()> {
        IndicesPutAlias::new(self.client.clone(), parts)
    }
    #[doc = "Updates the index mappings."]
    pub fn put_mapping<'a>(
        &self,
        parts: IndicesPutMappingUrlParts<'a>,
    ) -> IndicesPutMapping<'a, ()> {
        IndicesPutMapping::new(self.client.clone(), parts)
    }
    #[doc = "Updates the index settings."]
    pub fn put_settings<'a>(
        &self,
        parts: IndicesPutSettingsUrlParts<'a>,
    ) -> IndicesPutSettings<'a, ()> {
        IndicesPutSettings::new(self.client.clone(), parts)
    }
    #[doc = "Creates or updates an index template."]
    pub fn put_template<'a>(
        &self,
        parts: IndicesPutTemplateUrlParts<'a>,
    ) -> IndicesPutTemplate<'a, ()> {
        IndicesPutTemplate::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about ongoing index shard recoveries."]
    pub fn recovery<'a>(&self, parts: IndicesRecoveryUrlParts<'a>) -> IndicesRecovery<'a> {
        IndicesRecovery::new(self.client.clone(), parts)
    }
    #[doc = "Performs the refresh operation in one or more indices."]
    pub fn refresh<'a>(&self, parts: IndicesRefreshUrlParts<'a>) -> IndicesRefresh<'a, ()> {
        IndicesRefresh::new(self.client.clone(), parts)
    }
    #[doc = "Updates an alias to point to a new index when the existing index\nis considered to be too large or too old."]
    pub fn rollover<'a>(&self, parts: IndicesRolloverUrlParts<'a>) -> IndicesRollover<'a, ()> {
        IndicesRollover::new(self.client.clone(), parts)
    }
    #[doc = "Provides low-level information about segments in a Lucene index."]
    pub fn segments<'a>(&self, parts: IndicesSegmentsUrlParts<'a>) -> IndicesSegments<'a> {
        IndicesSegments::new(self.client.clone(), parts)
    }
    #[doc = "Provides store information for shard copies of indices."]
    pub fn shard_stores<'a>(
        &self,
        parts: IndicesShardStoresUrlParts<'a>,
    ) -> IndicesShardStores<'a> {
        IndicesShardStores::new(self.client.clone(), parts)
    }
    #[doc = "Allow to shrink an existing index into a new index with fewer primary shards."]
    pub fn shrink<'a>(&self, parts: IndicesShrinkUrlParts<'a>) -> IndicesShrink<'a, ()> {
        IndicesShrink::new(self.client.clone(), parts)
    }
    #[doc = "Allows you to split an existing index into a new index with more primary shards."]
    pub fn split<'a>(&self, parts: IndicesSplitUrlParts<'a>) -> IndicesSplit<'a, ()> {
        IndicesSplit::new(self.client.clone(), parts)
    }
    #[doc = "Provides statistics on operations happening in an index."]
    pub fn stats<'a>(&self, parts: IndicesStatsUrlParts<'a>) -> IndicesStats<'a> {
        IndicesStats::new(self.client.clone(), parts)
    }
    pub fn unfreeze<'a>(&self, parts: IndicesUnfreezeUrlParts<'a>) -> IndicesUnfreeze<'a, ()> {
        IndicesUnfreeze::new(self.client.clone(), parts)
    }
    #[doc = "Updates index aliases."]
    pub fn update_aliases<'a>(&self) -> IndicesUpdateAliases<'a, ()> {
        IndicesUpdateAliases::new(self.client.clone())
    }
    #[doc = "The _upgrade API is no longer useful and will be removed."]
    pub fn upgrade<'a>(&self, parts: IndicesUpgradeUrlParts<'a>) -> IndicesUpgrade<'a, ()> {
        IndicesUpgrade::new(self.client.clone(), parts)
    }
    #[doc = "Allows a user to validate a potentially expensive query without executing it."]
    pub fn validate_query<'a>(
        &self,
        parts: IndicesValidateQueryUrlParts<'a>,
    ) -> IndicesValidateQuery<'a, ()> {
        IndicesValidateQuery::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Indices APIs"]
    pub fn indices(&self) -> Indices {
        Indices::new(self.clone())
    }
}
