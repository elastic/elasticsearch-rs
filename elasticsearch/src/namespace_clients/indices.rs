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
    error::Error,
    http::{
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
};
use serde::Serialize;
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Analyze API"]
pub enum IndicesAnalyzeParts<'a> {
    None,
    Index(&'a str),
}
impl<'a> IndicesAnalyzeParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Analyze API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesAnalyzeParts::None => "/_analyze".into(),
            IndicesAnalyzeParts::Index(ref index) => {
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
    parts: IndicesAnalyzeParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesAnalyzeParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
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
#[doc = "API parts for the Indices Clear Cache API"]
pub enum IndicesClearCacheParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesClearCacheParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Clear Cache API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesClearCacheParts::None => "/_cache/clear".into(),
            IndicesClearCacheParts::Index(ref index) => {
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
    parts: IndicesClearCacheParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesClearCacheParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Indices Clone API"]
pub enum IndicesCloneParts<'a> {
    IndexTarget(&'a str, &'a str),
}
impl<'a> IndicesCloneParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Clone API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesCloneParts::IndexTarget(ref index, ref target) => {
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
    parts: IndicesCloneParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesCloneParts<'a>) -> Self {
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
#[doc = "API parts for the Indices Close API"]
pub enum IndicesCloseParts<'a> {
    Index(&'a [&'a str]),
}
impl<'a> IndicesCloseParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Close API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesCloseParts::Index(ref index) => {
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
    parts: IndicesCloseParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesCloseParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Indices Create API"]
pub enum IndicesCreateParts<'a> {
    Index(&'a str),
}
impl<'a> IndicesCreateParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Create API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesCreateParts::Index(ref index) => {
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
    parts: IndicesCreateParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesCreateParts<'a>) -> Self {
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
#[doc = "API parts for the Indices Delete API"]
pub enum IndicesDeleteParts<'a> {
    Index(&'a [&'a str]),
}
impl<'a> IndicesDeleteParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesDeleteParts::Index(ref index) => {
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
    parts: IndicesDeleteParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesDeleteParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
#[doc = "API parts for the Indices Delete Alias API"]
pub enum IndicesDeleteAliasParts<'a> {
    IndexName(&'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesDeleteAliasParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Delete Alias API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesDeleteAliasParts::IndexName(ref index, ref name) => {
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
    parts: IndicesDeleteAliasParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> IndicesDeleteAlias<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesDeleteAliasParts<'a>) -> Self {
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
#[doc = "API parts for the Indices Delete Template API"]
pub enum IndicesDeleteTemplateParts<'a> {
    Name(&'a str),
}
impl<'a> IndicesDeleteTemplateParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Delete Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesDeleteTemplateParts::Name(ref name) => {
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
    parts: IndicesDeleteTemplateParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> IndicesDeleteTemplate<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesDeleteTemplateParts<'a>) -> Self {
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
#[doc = "API parts for the Indices Exists API"]
pub enum IndicesExistsParts<'a> {
    Index(&'a [&'a str]),
}
impl<'a> IndicesExistsParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Exists API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesExistsParts::Index(ref index) => {
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
    parts: IndicesExistsParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesExistsParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
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
#[doc = "API parts for the Indices Exists Alias API"]
pub enum IndicesExistsAliasParts<'a> {
    Name(&'a [&'a str]),
    IndexName(&'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesExistsAliasParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Exists Alias API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesExistsAliasParts::Name(ref name) => {
                let name_str = name.join(",");
                let mut p = String::with_capacity(8usize + name_str.len());
                p.push_str("/_alias/");
                p.push_str(name_str.as_ref());
                p.into()
            }
            IndicesExistsAliasParts::IndexName(ref index, ref name) => {
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
    parts: IndicesExistsAliasParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesExistsAliasParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
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
#[doc = "API parts for the Indices Exists Template API"]
pub enum IndicesExistsTemplateParts<'a> {
    Name(&'a [&'a str]),
}
impl<'a> IndicesExistsTemplateParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Exists Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesExistsTemplateParts::Name(ref name) => {
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
    parts: IndicesExistsTemplateParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesExistsTemplateParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
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
#[doc = "API parts for the Indices Exists Type API"]
pub enum IndicesExistsTypeParts<'a> {
    IndexType(&'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesExistsTypeParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Exists Type API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesExistsTypeParts::IndexType(ref index, ref ty) => {
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
    parts: IndicesExistsTypeParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesExistsTypeParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
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
#[doc = "API parts for the Indices Flush API"]
pub enum IndicesFlushParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesFlushParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Flush API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesFlushParts::None => "/_flush".into(),
            IndicesFlushParts::Index(ref index) => {
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
    parts: IndicesFlushParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesFlushParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
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
#[doc = "API parts for the Indices Flush Synced API"]
pub enum IndicesFlushSyncedParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesFlushSyncedParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Flush Synced API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesFlushSyncedParts::None => "/_flush/synced".into(),
            IndicesFlushSyncedParts::Index(ref index) => {
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
    parts: IndicesFlushSyncedParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesFlushSyncedParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
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
#[doc = "API parts for the Indices Forcemerge API"]
pub enum IndicesForcemergeParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesForcemergeParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Forcemerge API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesForcemergeParts::None => "/_forcemerge".into(),
            IndicesForcemergeParts::Index(ref index) => {
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
    parts: IndicesForcemergeParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesForcemergeParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Indices Freeze API"]
pub enum IndicesFreezeParts<'a> {
    Index(&'a str),
}
impl<'a> IndicesFreezeParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Freeze API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesFreezeParts::Index(ref index) => {
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
    parts: IndicesFreezeParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesFreezeParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Indices Get API"]
pub enum IndicesGetParts<'a> {
    Index(&'a [&'a str]),
}
impl<'a> IndicesGetParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetParts::Index(ref index) => {
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
    parts: IndicesGetParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesGetParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Indices Get Alias API"]
pub enum IndicesGetAliasParts<'a> {
    None,
    Name(&'a [&'a str]),
    IndexName(&'a [&'a str], &'a [&'a str]),
    Index(&'a [&'a str]),
}
impl<'a> IndicesGetAliasParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Get Alias API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetAliasParts::None => "/_alias".into(),
            IndicesGetAliasParts::Name(ref name) => {
                let name_str = name.join(",");
                let mut p = String::with_capacity(8usize + name_str.len());
                p.push_str("/_alias/");
                p.push_str(name_str.as_ref());
                p.into()
            }
            IndicesGetAliasParts::IndexName(ref index, ref name) => {
                let index_str = index.join(",");
                let name_str = name.join(",");
                let mut p = String::with_capacity(9usize + index_str.len() + name_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_alias/");
                p.push_str(name_str.as_ref());
                p.into()
            }
            IndicesGetAliasParts::Index(ref index) => {
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
    parts: IndicesGetAliasParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesGetAliasParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Indices Get Field Mapping API"]
pub enum IndicesGetFieldMappingParts<'a> {
    Fields(&'a [&'a str]),
    IndexFields(&'a [&'a str], &'a [&'a str]),
    TypeFields(&'a [&'a str], &'a [&'a str]),
    IndexTypeFields(&'a [&'a str], &'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesGetFieldMappingParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Get Field Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetFieldMappingParts::Fields(ref fields) => {
                let fields_str = fields.join(",");
                let mut p = String::with_capacity(16usize + fields_str.len());
                p.push_str("/_mapping/field/");
                p.push_str(fields_str.as_ref());
                p.into()
            }
            IndicesGetFieldMappingParts::IndexFields(ref index, ref fields) => {
                let index_str = index.join(",");
                let fields_str = fields.join(",");
                let mut p = String::with_capacity(17usize + index_str.len() + fields_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping/field/");
                p.push_str(fields_str.as_ref());
                p.into()
            }
            IndicesGetFieldMappingParts::TypeFields(ref ty, ref fields) => {
                let ty_str = ty.join(",");
                let fields_str = fields.join(",");
                let mut p = String::with_capacity(17usize + ty_str.len() + fields_str.len());
                p.push_str("/_mapping/");
                p.push_str(ty_str.as_ref());
                p.push_str("/field/");
                p.push_str(fields_str.as_ref());
                p.into()
            }
            IndicesGetFieldMappingParts::IndexTypeFields(ref index, ref ty, ref fields) => {
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
    parts: IndicesGetFieldMappingParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesGetFieldMappingParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Indices Get Mapping API"]
pub enum IndicesGetMappingParts<'a> {
    None,
    Index(&'a [&'a str]),
    Type(&'a [&'a str]),
    IndexType(&'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesGetMappingParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Get Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetMappingParts::None => "/_mapping".into(),
            IndicesGetMappingParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping");
                p.into()
            }
            IndicesGetMappingParts::Type(ref ty) => {
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(10usize + ty_str.len());
                p.push_str("/_mapping/");
                p.push_str(ty_str.as_ref());
                p.into()
            }
            IndicesGetMappingParts::IndexType(ref index, ref ty) => {
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
    parts: IndicesGetMappingParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesGetMappingParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Indices Get Settings API"]
pub enum IndicesGetSettingsParts<'a> {
    None,
    Index(&'a [&'a str]),
    IndexName(&'a [&'a str], &'a [&'a str]),
    Name(&'a [&'a str]),
}
impl<'a> IndicesGetSettingsParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Get Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetSettingsParts::None => "/_settings".into(),
            IndicesGetSettingsParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(11usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_settings");
                p.into()
            }
            IndicesGetSettingsParts::IndexName(ref index, ref name) => {
                let index_str = index.join(",");
                let name_str = name.join(",");
                let mut p = String::with_capacity(12usize + index_str.len() + name_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_settings/");
                p.push_str(name_str.as_ref());
                p.into()
            }
            IndicesGetSettingsParts::Name(ref name) => {
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
    parts: IndicesGetSettingsParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesGetSettingsParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Indices Get Template API"]
pub enum IndicesGetTemplateParts<'a> {
    None,
    Name(&'a [&'a str]),
}
impl<'a> IndicesGetTemplateParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Get Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetTemplateParts::None => "/_template".into(),
            IndicesGetTemplateParts::Name(ref name) => {
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
    parts: IndicesGetTemplateParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesGetTemplateParts<'a>) -> Self {
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
#[doc = "API parts for the Indices Get Upgrade API"]
pub enum IndicesGetUpgradeParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesGetUpgradeParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Get Upgrade API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesGetUpgradeParts::None => "/_upgrade".into(),
            IndicesGetUpgradeParts::Index(ref index) => {
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
    parts: IndicesGetUpgradeParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesGetUpgradeParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Indices Open API"]
pub enum IndicesOpenParts<'a> {
    Index(&'a [&'a str]),
}
impl<'a> IndicesOpenParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Open API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesOpenParts::Index(ref index) => {
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
    parts: IndicesOpenParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesOpenParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Indices Put Alias API"]
pub enum IndicesPutAliasParts<'a> {
    IndexName(&'a [&'a str], &'a str),
}
impl<'a> IndicesPutAliasParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Put Alias API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesPutAliasParts::IndexName(ref index, ref name) => {
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
    parts: IndicesPutAliasParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesPutAliasParts<'a>) -> Self {
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
#[doc = "API parts for the Indices Put Mapping API"]
pub enum IndicesPutMappingParts<'a> {
    Index(&'a [&'a str]),
    IndexType(&'a [&'a str], &'a str),
    Type(&'a str),
}
impl<'a> IndicesPutMappingParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Put Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesPutMappingParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping");
                p.into()
            }
            IndicesPutMappingParts::IndexType(ref index, ref ty) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(11usize + index_str.len() + ty.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/_mapping");
                p.into()
            }
            IndicesPutMappingParts::Type(ref ty) => {
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
    parts: IndicesPutMappingParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesPutMappingParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
#[doc = "API parts for the Indices Put Settings API"]
pub enum IndicesPutSettingsParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesPutSettingsParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Put Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesPutSettingsParts::None => "/_settings".into(),
            IndicesPutSettingsParts::Index(ref index) => {
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
    parts: IndicesPutSettingsParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesPutSettingsParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
#[doc = "API parts for the Indices Put Template API"]
pub enum IndicesPutTemplateParts<'a> {
    Name(&'a str),
}
impl<'a> IndicesPutTemplateParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Put Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesPutTemplateParts::Name(ref name) => {
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
    parts: IndicesPutTemplateParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesPutTemplateParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
#[doc = "API parts for the Indices Recovery API"]
pub enum IndicesRecoveryParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesRecoveryParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Recovery API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesRecoveryParts::None => "/_recovery".into(),
            IndicesRecoveryParts::Index(ref index) => {
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
    parts: IndicesRecoveryParts<'a>,
    active_only: Option<bool>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> IndicesRecovery<'a> {
    pub fn new(client: Elasticsearch, parts: IndicesRecoveryParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Indices Refresh API"]
pub enum IndicesRefreshParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesRefreshParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Refresh API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesRefreshParts::None => "/_refresh".into(),
            IndicesRefreshParts::Index(ref index) => {
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
    parts: IndicesRefreshParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesRefreshParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
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
#[doc = "API parts for the Indices Rollover API"]
pub enum IndicesRolloverParts<'a> {
    Alias(&'a str),
    AliasNewIndex(&'a str, &'a str),
}
impl<'a> IndicesRolloverParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Rollover API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesRolloverParts::Alias(ref alias) => {
                let mut p = String::with_capacity(11usize + alias.len());
                p.push_str("/");
                p.push_str(alias.as_ref());
                p.push_str("/_rollover");
                p.into()
            }
            IndicesRolloverParts::AliasNewIndex(ref alias, ref new_index) => {
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
    parts: IndicesRolloverParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesRolloverParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Indices Segments API"]
pub enum IndicesSegmentsParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesSegmentsParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Segments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesSegmentsParts::None => "/_segments".into(),
            IndicesSegmentsParts::Index(ref index) => {
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
    parts: IndicesSegmentsParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesSegmentsParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Indices Shard Stores API"]
pub enum IndicesShardStoresParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesShardStoresParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Shard Stores API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesShardStoresParts::None => "/_shard_stores".into(),
            IndicesShardStoresParts::Index(ref index) => {
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
    parts: IndicesShardStoresParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesShardStoresParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Indices Shrink API"]
pub enum IndicesShrinkParts<'a> {
    IndexTarget(&'a str, &'a str),
}
impl<'a> IndicesShrinkParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Shrink API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesShrinkParts::IndexTarget(ref index, ref target) => {
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
    parts: IndicesShrinkParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesShrinkParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Indices Split API"]
pub enum IndicesSplitParts<'a> {
    IndexTarget(&'a str, &'a str),
}
impl<'a> IndicesSplitParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Split API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesSplitParts::IndexTarget(ref index, ref target) => {
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
    parts: IndicesSplitParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesSplitParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Indices Stats API"]
pub enum IndicesStatsParts<'a> {
    None,
    Metric(&'a [&'a str]),
    Index(&'a [&'a str]),
    IndexMetric(&'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesStatsParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesStatsParts::None => "/_stats".into(),
            IndicesStatsParts::Metric(ref metric) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(8usize + metric_str.len());
                p.push_str("/_stats/");
                p.push_str(metric_str.as_ref());
                p.into()
            }
            IndicesStatsParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(8usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_stats");
                p.into()
            }
            IndicesStatsParts::IndexMetric(ref index, ref metric) => {
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
    parts: IndicesStatsParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesStatsParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Indices Unfreeze API"]
pub enum IndicesUnfreezeParts<'a> {
    Index(&'a str),
}
impl<'a> IndicesUnfreezeParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Unfreeze API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesUnfreezeParts::Index(ref index) => {
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
    parts: IndicesUnfreezeParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesUnfreezeParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Indices Update Aliases API"]
pub enum IndicesUpdateAliasesParts {
    None,
}
impl IndicesUpdateAliasesParts {
    #[doc = "Builds a relative URL path to the Indices Update Aliases API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesUpdateAliasesParts::None => "/_aliases".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Update Aliases API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html). Updates index aliases."]
pub struct IndicesUpdateAliases<'a, B> {
    client: Elasticsearch,
    parts: IndicesUpdateAliasesParts,
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
            parts: IndicesUpdateAliasesParts::None,
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
#[doc = "API parts for the Indices Upgrade API"]
pub enum IndicesUpgradeParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> IndicesUpgradeParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Upgrade API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesUpgradeParts::None => "/_upgrade".into(),
            IndicesUpgradeParts::Index(ref index) => {
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
    parts: IndicesUpgradeParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesUpgradeParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Indices Validate Query API"]
pub enum IndicesValidateQueryParts<'a> {
    None,
    Index(&'a [&'a str]),
    IndexType(&'a [&'a str], &'a [&'a str]),
}
impl<'a> IndicesValidateQueryParts<'a> {
    #[doc = "Builds a relative URL path to the Indices Validate Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesValidateQueryParts::None => "/_validate/query".into(),
            IndicesValidateQueryParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(17usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_validate/query");
                p.into()
            }
            IndicesValidateQueryParts::IndexType(ref index, ref ty) => {
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
    parts: IndicesValidateQueryParts<'a>,
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
    pub fn new(client: Elasticsearch, parts: IndicesValidateQueryParts<'a>) -> Self {
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
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
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
    pub fn analyze<'a>(&self, parts: IndicesAnalyzeParts<'a>) -> IndicesAnalyze<'a, ()> {
        IndicesAnalyze::new(self.client.clone(), parts)
    }
    #[doc = "Clears all or specific caches for one or more indices."]
    pub fn clear_cache<'a>(&self, parts: IndicesClearCacheParts<'a>) -> IndicesClearCache<'a, ()> {
        IndicesClearCache::new(self.client.clone(), parts)
    }
    #[doc = "Clones an index"]
    pub fn clone<'a>(&self, parts: IndicesCloneParts<'a>) -> IndicesClone<'a, ()> {
        IndicesClone::new(self.client.clone(), parts)
    }
    #[doc = "Closes an index."]
    pub fn close<'a>(&self, parts: IndicesCloseParts<'a>) -> IndicesClose<'a, ()> {
        IndicesClose::new(self.client.clone(), parts)
    }
    #[doc = "Creates an index with optional settings and mappings."]
    pub fn create<'a>(&self, parts: IndicesCreateParts<'a>) -> IndicesCreate<'a, ()> {
        IndicesCreate::new(self.client.clone(), parts)
    }
    #[doc = "Deletes an index."]
    pub fn delete<'a>(&self, parts: IndicesDeleteParts<'a>) -> IndicesDelete<'a> {
        IndicesDelete::new(self.client.clone(), parts)
    }
    #[doc = "Deletes an alias."]
    pub fn delete_alias<'a>(&self, parts: IndicesDeleteAliasParts<'a>) -> IndicesDeleteAlias<'a> {
        IndicesDeleteAlias::new(self.client.clone(), parts)
    }
    #[doc = "Deletes an index template."]
    pub fn delete_template<'a>(
        &self,
        parts: IndicesDeleteTemplateParts<'a>,
    ) -> IndicesDeleteTemplate<'a> {
        IndicesDeleteTemplate::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about whether a particular index exists."]
    pub fn exists<'a>(&self, parts: IndicesExistsParts<'a>) -> IndicesExists<'a> {
        IndicesExists::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about whether a particular alias exists."]
    pub fn exists_alias<'a>(&self, parts: IndicesExistsAliasParts<'a>) -> IndicesExistsAlias<'a> {
        IndicesExistsAlias::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about whether a particular index template exists."]
    pub fn exists_template<'a>(
        &self,
        parts: IndicesExistsTemplateParts<'a>,
    ) -> IndicesExistsTemplate<'a> {
        IndicesExistsTemplate::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about whether a particular document type exists. (DEPRECATED)"]
    pub fn exists_type<'a>(&self, parts: IndicesExistsTypeParts<'a>) -> IndicesExistsType<'a> {
        IndicesExistsType::new(self.client.clone(), parts)
    }
    #[doc = "Performs the flush operation on one or more indices."]
    pub fn flush<'a>(&self, parts: IndicesFlushParts<'a>) -> IndicesFlush<'a, ()> {
        IndicesFlush::new(self.client.clone(), parts)
    }
    #[doc = "Performs a synced flush operation on one or more indices."]
    pub fn flush_synced<'a>(
        &self,
        parts: IndicesFlushSyncedParts<'a>,
    ) -> IndicesFlushSynced<'a, ()> {
        IndicesFlushSynced::new(self.client.clone(), parts)
    }
    #[doc = "Performs the force merge operation on one or more indices."]
    pub fn forcemerge<'a>(&self, parts: IndicesForcemergeParts<'a>) -> IndicesForcemerge<'a, ()> {
        IndicesForcemerge::new(self.client.clone(), parts)
    }
    pub fn freeze<'a>(&self, parts: IndicesFreezeParts<'a>) -> IndicesFreeze<'a, ()> {
        IndicesFreeze::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about one or more indices."]
    pub fn get<'a>(&self, parts: IndicesGetParts<'a>) -> IndicesGet<'a> {
        IndicesGet::new(self.client.clone(), parts)
    }
    #[doc = "Returns an alias."]
    pub fn get_alias<'a>(&self, parts: IndicesGetAliasParts<'a>) -> IndicesGetAlias<'a> {
        IndicesGetAlias::new(self.client.clone(), parts)
    }
    #[doc = "Returns mapping for one or more fields."]
    pub fn get_field_mapping<'a>(
        &self,
        parts: IndicesGetFieldMappingParts<'a>,
    ) -> IndicesGetFieldMapping<'a> {
        IndicesGetFieldMapping::new(self.client.clone(), parts)
    }
    #[doc = "Returns mappings for one or more indices."]
    pub fn get_mapping<'a>(&self, parts: IndicesGetMappingParts<'a>) -> IndicesGetMapping<'a> {
        IndicesGetMapping::new(self.client.clone(), parts)
    }
    #[doc = "Returns settings for one or more indices."]
    pub fn get_settings<'a>(&self, parts: IndicesGetSettingsParts<'a>) -> IndicesGetSettings<'a> {
        IndicesGetSettings::new(self.client.clone(), parts)
    }
    #[doc = "Returns an index template."]
    pub fn get_template<'a>(&self, parts: IndicesGetTemplateParts<'a>) -> IndicesGetTemplate<'a> {
        IndicesGetTemplate::new(self.client.clone(), parts)
    }
    #[doc = "The _upgrade API is no longer useful and will be removed."]
    pub fn get_upgrade<'a>(&self, parts: IndicesGetUpgradeParts<'a>) -> IndicesGetUpgrade<'a> {
        IndicesGetUpgrade::new(self.client.clone(), parts)
    }
    #[doc = "Opens an index."]
    pub fn open<'a>(&self, parts: IndicesOpenParts<'a>) -> IndicesOpen<'a, ()> {
        IndicesOpen::new(self.client.clone(), parts)
    }
    #[doc = "Creates or updates an alias."]
    pub fn put_alias<'a>(&self, parts: IndicesPutAliasParts<'a>) -> IndicesPutAlias<'a, ()> {
        IndicesPutAlias::new(self.client.clone(), parts)
    }
    #[doc = "Updates the index mappings."]
    pub fn put_mapping<'a>(&self, parts: IndicesPutMappingParts<'a>) -> IndicesPutMapping<'a, ()> {
        IndicesPutMapping::new(self.client.clone(), parts)
    }
    #[doc = "Updates the index settings."]
    pub fn put_settings<'a>(
        &self,
        parts: IndicesPutSettingsParts<'a>,
    ) -> IndicesPutSettings<'a, ()> {
        IndicesPutSettings::new(self.client.clone(), parts)
    }
    #[doc = "Creates or updates an index template."]
    pub fn put_template<'a>(
        &self,
        parts: IndicesPutTemplateParts<'a>,
    ) -> IndicesPutTemplate<'a, ()> {
        IndicesPutTemplate::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about ongoing index shard recoveries."]
    pub fn recovery<'a>(&self, parts: IndicesRecoveryParts<'a>) -> IndicesRecovery<'a> {
        IndicesRecovery::new(self.client.clone(), parts)
    }
    #[doc = "Performs the refresh operation in one or more indices."]
    pub fn refresh<'a>(&self, parts: IndicesRefreshParts<'a>) -> IndicesRefresh<'a, ()> {
        IndicesRefresh::new(self.client.clone(), parts)
    }
    #[doc = "Updates an alias to point to a new index when the existing index\nis considered to be too large or too old."]
    pub fn rollover<'a>(&self, parts: IndicesRolloverParts<'a>) -> IndicesRollover<'a, ()> {
        IndicesRollover::new(self.client.clone(), parts)
    }
    #[doc = "Provides low-level information about segments in a Lucene index."]
    pub fn segments<'a>(&self, parts: IndicesSegmentsParts<'a>) -> IndicesSegments<'a> {
        IndicesSegments::new(self.client.clone(), parts)
    }
    #[doc = "Provides store information for shard copies of indices."]
    pub fn shard_stores<'a>(&self, parts: IndicesShardStoresParts<'a>) -> IndicesShardStores<'a> {
        IndicesShardStores::new(self.client.clone(), parts)
    }
    #[doc = "Allow to shrink an existing index into a new index with fewer primary shards."]
    pub fn shrink<'a>(&self, parts: IndicesShrinkParts<'a>) -> IndicesShrink<'a, ()> {
        IndicesShrink::new(self.client.clone(), parts)
    }
    #[doc = "Allows you to split an existing index into a new index with more primary shards."]
    pub fn split<'a>(&self, parts: IndicesSplitParts<'a>) -> IndicesSplit<'a, ()> {
        IndicesSplit::new(self.client.clone(), parts)
    }
    #[doc = "Provides statistics on operations happening in an index."]
    pub fn stats<'a>(&self, parts: IndicesStatsParts<'a>) -> IndicesStats<'a> {
        IndicesStats::new(self.client.clone(), parts)
    }
    pub fn unfreeze<'a>(&self, parts: IndicesUnfreezeParts<'a>) -> IndicesUnfreeze<'a, ()> {
        IndicesUnfreeze::new(self.client.clone(), parts)
    }
    #[doc = "Updates index aliases."]
    pub fn update_aliases<'a>(&self) -> IndicesUpdateAliases<'a, ()> {
        IndicesUpdateAliases::new(self.client.clone())
    }
    #[doc = "The _upgrade API is no longer useful and will be removed."]
    pub fn upgrade<'a>(&self, parts: IndicesUpgradeParts<'a>) -> IndicesUpgrade<'a, ()> {
        IndicesUpgrade::new(self.client.clone(), parts)
    }
    #[doc = "Allows a user to validate a potentially expensive query without executing it."]
    pub fn validate_query<'a>(
        &self,
        parts: IndicesValidateQueryParts<'a>,
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