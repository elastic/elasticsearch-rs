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
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
    params::*,
};
use serde::Serialize;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Analyze API"]
pub enum IndicesAnalyzeParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IndicesAnalyzeParts<'b> {
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
#[doc = "Builder for the [Indices Analyze API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-analyze.html)\n\nPerforms the analysis process on a text and return the tokens breakdown of the text."]
pub struct IndicesAnalyze<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesAnalyzeParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    index: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesAnalyze<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesAnalyze] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesAnalyzeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesAnalyze {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesAnalyze<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesAnalyze {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    #[doc = "The name of the index to scope the operation"]
    pub fn index(mut self, index: &'b str) -> Self {
        self.index = Some(index);
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
    #[doc = "Creates an asynchronous call to the Indices Analyze API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
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
                #[serde(rename = "index")]
                index: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Clear Cache API"]
pub enum IndicesClearCacheParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesClearCacheParts<'b> {
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
#[doc = "Builder for the [Indices Clear Cache API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-clearcache.html)\n\nClears all or specific caches for one or more indices."]
pub struct IndicesClearCache<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesClearCacheParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    fielddata: Option<bool>,
    fields: Option<&'b [&'b str]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<&'b [&'b str]>,
    pretty: Option<bool>,
    query: Option<bool>,
    request: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesClearCache<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesClearCache] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesClearCacheParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesClearCache {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesClearCache<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    pub fn fields(mut self, fields: &'b [&'b str]) -> Self {
        self.fields = Some(fields);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "A comma-separated list of index name to limit the operation"]
    pub fn index(mut self, index: &'b [&'b str]) -> Self {
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Clear Cache API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "fielddata")]
                fielddata: Option<bool>,
                #[serde(rename = "fields", serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'b [&'b str]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "index", serialize_with = "crate::client::serialize_coll_qs")]
                index: Option<&'b [&'b str]>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "query")]
                query: Option<bool>,
                #[serde(rename = "request")]
                request: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Clone API"]
pub enum IndicesCloneParts<'b> {
    #[doc = "Index and Target"]
    IndexTarget(&'b str, &'b str),
}
impl<'b> IndicesCloneParts<'b> {
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
#[doc = "Builder for the [Indices Clone API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-clone-index.html)\n\nClones an index"]
pub struct IndicesClone<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesCloneParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesClone<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesClone] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesCloneParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesClone {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesClone<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesClone {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Set the number of active shards to wait for on the cloned index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Clone API that can be awaited"]
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Close API"]
pub enum IndicesCloseParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesCloseParts<'b> {
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
#[doc = "Builder for the [Indices Close API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-open-close.html)\n\nCloses an index."]
pub struct IndicesClose<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesCloseParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesClose<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesClose] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesCloseParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesClose {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesClose<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Close API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Create API"]
pub enum IndicesCreateParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IndicesCreateParts<'b> {
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
#[doc = "Builder for the [Indices Create API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-create-index.html)\n\nCreates an index with optional settings and mappings."]
pub struct IndicesCreate<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesCreateParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesCreate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesCreate] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesCreateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesCreate {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesCreate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesCreate {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    #[doc = "Whether a type should be expected in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Set the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Create API that can be awaited"]
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
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Delete API"]
pub enum IndicesDeleteParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesDeleteParts<'b> {
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
#[doc = "Builder for the [Indices Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-delete-index.html)\n\nDeletes an index."]
pub struct IndicesDelete<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesDeleteParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> IndicesDelete<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesDelete] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesDelete {
            client,
            parts,
            headers,
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
    #[doc = "Ignore unavailable indexes (default: false)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Delete API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Delete Alias API"]
pub enum IndicesDeleteAliasParts<'b> {
    #[doc = "Index and Name"]
    IndexName(&'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesDeleteAliasParts<'b> {
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
#[doc = "Builder for the [Indices Delete Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-aliases.html)\n\nDeletes an alias."]
pub struct IndicesDeleteAlias<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesDeleteAliasParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> IndicesDeleteAlias<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesDeleteAlias] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesDeleteAliasParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesDeleteAlias {
            client,
            parts,
            headers,
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit timestamp for the document"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Delete Alias API that can be awaited"]
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
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
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Delete Template API"]
pub enum IndicesDeleteTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> IndicesDeleteTemplateParts<'b> {
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
#[doc = "Builder for the [Indices Delete Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-templates.html)\n\nDeletes an index template."]
pub struct IndicesDeleteTemplate<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesDeleteTemplateParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> IndicesDeleteTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesDeleteTemplate] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesDeleteTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesDeleteTemplate {
            client,
            parts,
            headers,
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Delete Template API that can be awaited"]
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
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
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Exists API"]
pub enum IndicesExistsParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesExistsParts<'b> {
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
#[doc = "Builder for the [Indices Exists API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-exists.html)\n\nReturns information about whether a particular index exists."]
pub struct IndicesExists<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesExistsParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesExists<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesExists] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesExistsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesExists {
            client,
            parts,
            headers,
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
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Exists API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
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
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Exists Alias API"]
pub enum IndicesExistsAliasParts<'b> {
    #[doc = "Name"]
    Name(&'b [&'b str]),
    #[doc = "Index and Name"]
    IndexName(&'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesExistsAliasParts<'b> {
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
#[doc = "Builder for the [Indices Exists Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-aliases.html)\n\nReturns information about whether a particular alias exists."]
pub struct IndicesExistsAlias<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesExistsAliasParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesExistsAlias<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesExistsAlias] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesExistsAliasParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesExistsAlias {
            client,
            parts,
            headers,
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Exists Alias API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Exists Template API"]
pub enum IndicesExistsTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> IndicesExistsTemplateParts<'b> {
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
#[doc = "Builder for the [Indices Exists Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-templates.html)\n\nReturns information about whether a particular index template exists."]
pub struct IndicesExistsTemplate<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesExistsTemplateParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesExistsTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesExistsTemplate] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesExistsTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesExistsTemplate {
            client,
            parts,
            headers,
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
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous call to the Indices Exists Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
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
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Exists Type API"]
pub enum IndicesExistsTypeParts<'b> {
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesExistsTypeParts<'b> {
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
#[doc = "Builder for the [Indices Exists Type API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-types-exists.html)\n\nReturns information about whether a particular document type exists. (DEPRECATED)"]
pub struct IndicesExistsType<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesExistsTypeParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesExistsType<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesExistsType] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesExistsTypeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesExistsType {
            client,
            parts,
            headers,
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Exists Type API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Flush API"]
pub enum IndicesFlushParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesFlushParts<'b> {
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
#[doc = "Builder for the [Indices Flush API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-flush.html)\n\nPerforms the flush operation on one or more indices."]
pub struct IndicesFlush<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesFlushParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    force: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    wait_if_ongoing: Option<bool>,
}
impl<'a, 'b, B> IndicesFlush<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesFlush] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesFlushParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesFlush {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesFlush<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Whether a flush should be forced even if it is not necessarily needed ie. if no changes will be committed to the index. This is useful if transaction log IDs should be incremented even if no uncommitted changes are present. (This setting can be considered as internal)"]
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
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
    pub fn source(mut self, source: &'b str) -> Self {
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
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Flush Synced API"]
pub enum IndicesFlushSyncedParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesFlushSyncedParts<'b> {
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
#[doc = "Builder for the [Indices Flush Synced API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-synced-flush-api.html)\n\nPerforms a synced flush operation on one or more indices. Synced flush is deprecated and will be removed in 8.0. Use flush instead"]
pub struct IndicesFlushSynced<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesFlushSyncedParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesFlushSynced<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesFlushSynced] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesFlushSyncedParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesFlushSynced {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesFlushSynced<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    pub fn source(mut self, source: &'b str) -> Self {
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
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Forcemerge API"]
pub enum IndicesForcemergeParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesForcemergeParts<'b> {
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
#[doc = "Builder for the [Indices Forcemerge API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-forcemerge.html)\n\nPerforms the force merge operation on one or more indices."]
pub struct IndicesForcemerge<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesForcemergeParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    flush: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    max_num_segments: Option<i64>,
    only_expunge_deletes: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesForcemerge<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesForcemerge] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesForcemergeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesForcemerge {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesForcemerge<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Specify whether the index should be flushed after performing the operation (default: true)"]
    pub fn flush(mut self, flush: bool) -> Self {
        self.flush = Some(flush);
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Forcemerge API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
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
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Freeze API"]
pub enum IndicesFreezeParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IndicesFreezeParts<'b> {
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
#[doc = "Builder for the [Indices Freeze API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/freeze-index-api.html)\n\nFreezes an index. A frozen index has almost no overhead on the cluster (except for maintaining its metadata in memory) and is read-only."]
pub struct IndicesFreeze<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesFreezeParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesFreeze<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesFreeze] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesFreezeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesFreeze {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesFreeze<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Freeze API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Get API"]
pub enum IndicesGetParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesGetParts<'b> {
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
#[doc = "Builder for the [Indices Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-get-index.html)\n\nReturns information about one or more indices."]
pub struct IndicesGet<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesGetParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGet<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGet] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGet {
            client,
            parts,
            headers,
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
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous call to the Indices Get API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
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
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Get Alias API"]
pub enum IndicesGetAliasParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b [&'b str]),
    #[doc = "Index and Name"]
    IndexName(&'b [&'b str], &'b [&'b str]),
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesGetAliasParts<'b> {
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
#[doc = "Builder for the [Indices Get Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-aliases.html)\n\nReturns an alias."]
pub struct IndicesGetAlias<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesGetAliasParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetAlias<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetAlias] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesGetAliasParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetAlias {
            client,
            parts,
            headers,
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Get Alias API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Get Field Mapping API"]
pub enum IndicesGetFieldMappingParts<'b> {
    #[doc = "Fields"]
    Fields(&'b [&'b str]),
    #[doc = "Index and Fields"]
    IndexFields(&'b [&'b str], &'b [&'b str]),
    #[doc = "Type and Fields"]
    TypeFields(&'b [&'b str], &'b [&'b str]),
    #[doc = "Index, Type and Fields"]
    IndexTypeFields(&'b [&'b str], &'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesGetFieldMappingParts<'b> {
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
#[doc = "Builder for the [Indices Get Field Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-get-field-mapping.html)\n\nReturns mapping for one or more fields."]
pub struct IndicesGetFieldMapping<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesGetFieldMappingParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetFieldMapping<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetFieldMapping] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesGetFieldMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetFieldMapping {
            client,
            parts,
            headers,
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Get Field Mapping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
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
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Get Mapping API"]
pub enum IndicesGetMappingParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Type"]
    Type(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesGetMappingParts<'b> {
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
#[doc = "Builder for the [Indices Get Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-get-mapping.html)\n\nReturns mappings for one or more indices."]
pub struct IndicesGetMapping<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesGetMappingParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetMapping<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetMapping] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesGetMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetMapping {
            client,
            parts,
            headers,
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
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous call to the Indices Get Mapping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Get Settings API"]
pub enum IndicesGetSettingsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Name"]
    IndexName(&'b [&'b str], &'b [&'b str]),
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> IndicesGetSettingsParts<'b> {
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
#[doc = "Builder for the [Indices Get Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-get-settings.html)\n\nReturns settings for one or more indices."]
pub struct IndicesGetSettings<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesGetSettingsParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetSettings<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetSettings] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesGetSettingsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetSettings {
            client,
            parts,
            headers,
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
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous call to the Indices Get Settings API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
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
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Get Template API"]
pub enum IndicesGetTemplateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> IndicesGetTemplateParts<'b> {
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
#[doc = "Builder for the [Indices Get Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-templates.html)\n\nReturns an index template."]
pub struct IndicesGetTemplate<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesGetTemplateParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetTemplate] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesGetTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetTemplate {
            client,
            parts,
            headers,
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
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Creates an asynchronous call to the Indices Get Template API that can be awaited"]
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
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Get Upgrade API"]
pub enum IndicesGetUpgradeParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesGetUpgradeParts<'b> {
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
#[doc = "Builder for the [Indices Get Upgrade API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-upgrade.html)\n\nThe _upgrade API is no longer useful and will be removed."]
pub struct IndicesGetUpgrade<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesGetUpgradeParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesGetUpgrade<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesGetUpgrade] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesGetUpgradeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesGetUpgrade {
            client,
            parts,
            headers,
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Get Upgrade API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Open API"]
pub enum IndicesOpenParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesOpenParts<'b> {
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
#[doc = "Builder for the [Indices Open API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-open-close.html)\n\nOpens an index."]
pub struct IndicesOpen<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesOpenParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesOpen<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesOpen] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesOpenParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesOpen {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesOpen<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Open API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Put Alias API"]
pub enum IndicesPutAliasParts<'b> {
    #[doc = "Index and Name"]
    IndexName(&'b [&'b str], &'b str),
}
impl<'b> IndicesPutAliasParts<'b> {
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
#[doc = "Builder for the [Indices Put Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-aliases.html)\n\nCreates or updates an alias."]
pub struct IndicesPutAlias<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesPutAliasParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> IndicesPutAlias<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesPutAlias] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesPutAliasParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesPutAlias {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesPutAlias<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesPutAlias {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit timestamp for the document"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Put Alias API that can be awaited"]
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
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
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Put Mapping API"]
pub enum IndicesPutMappingParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b str),
    #[doc = "Type"]
    Type(&'b str),
}
impl<'b> IndicesPutMappingParts<'b> {
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
#[doc = "Builder for the [Indices Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-put-mapping.html)\n\nUpdates the index mappings."]
pub struct IndicesPutMapping<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesPutMappingParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> IndicesPutMapping<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesPutMapping] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesPutMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesPutMapping {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesPutMapping<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Put Mapping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Put Settings API"]
pub enum IndicesPutSettingsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesPutSettingsParts<'b> {
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
#[doc = "Builder for the [Indices Put Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-update-settings.html)\n\nUpdates the index settings."]
pub struct IndicesPutSettings<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesPutSettingsParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    preserve_existing: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> IndicesPutSettings<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesPutSettings] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesPutSettingsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesPutSettings {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesPutSettings<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Put Settings API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "preserve_existing")]
                preserve_existing: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Put Template API"]
pub enum IndicesPutTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> IndicesPutTemplateParts<'b> {
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
#[doc = "Builder for the [Indices Put Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-templates.html)\n\nCreates or updates an index template."]
pub struct IndicesPutTemplate<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesPutTemplateParts<'b>,
    body: Option<B>,
    create: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<&'b str>,
    order: Option<i64>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesPutTemplate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesPutTemplate] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesPutTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesPutTemplate {
            client,
            parts,
            headers,
            body: None,
            create: None,
            error_trace: None,
            filter_path: None,
            human: None,
            include_type_name: None,
            master_timeout: None,
            order: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesPutTemplate<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
            human: self.human,
            include_type_name: self.include_type_name,
            master_timeout: self.master_timeout,
            order: self.order,
            pretty: self.pretty,
            source: self.source,
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
    #[doc = "Whether a type should be returned in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Put Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "create")]
                create: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "order")]
                order: Option<i64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                create: self.create,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                include_type_name: self.include_type_name,
                master_timeout: self.master_timeout,
                order: self.order,
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
#[doc = "API parts for the Indices Recovery API"]
pub enum IndicesRecoveryParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesRecoveryParts<'b> {
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
#[doc = "Builder for the [Indices Recovery API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-recovery.html)\n\nReturns information about ongoing index shard recoveries."]
pub struct IndicesRecovery<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesRecoveryParts<'b>,
    active_only: Option<bool>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> IndicesRecovery<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesRecovery] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesRecoveryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesRecovery {
            client,
            parts,
            headers,
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
    #[doc = "Creates an asynchronous call to the Indices Recovery API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Refresh API"]
pub enum IndicesRefreshParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesRefreshParts<'b> {
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
#[doc = "Builder for the [Indices Refresh API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-refresh.html)\n\nPerforms the refresh operation in one or more indices."]
pub struct IndicesRefresh<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesRefreshParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesRefresh<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesRefresh] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesRefreshParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesRefresh {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesRefresh<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    pub fn source(mut self, source: &'b str) -> Self {
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
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Reload Search Analyzers API"]
pub enum IndicesReloadSearchAnalyzersParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesReloadSearchAnalyzersParts<'b> {
    #[doc = "Builds a relative URL path to the Indices Reload Search Analyzers API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndicesReloadSearchAnalyzersParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(26usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_reload_search_analyzers");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Indices Reload Search Analyzers API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-reload-analyzers.html)\n\nReloads an index's search analyzers and their resources."]
pub struct IndicesReloadSearchAnalyzers<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesReloadSearchAnalyzersParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesReloadSearchAnalyzers<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesReloadSearchAnalyzers] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesReloadSearchAnalyzersParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesReloadSearchAnalyzers {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesReloadSearchAnalyzers<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesReloadSearchAnalyzers {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Reload Search Analyzers API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Rollover API"]
pub enum IndicesRolloverParts<'b> {
    #[doc = "Alias"]
    Alias(&'b str),
    #[doc = "Alias and NewIndex"]
    AliasNewIndex(&'b str, &'b str),
}
impl<'b> IndicesRolloverParts<'b> {
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
#[doc = "Builder for the [Indices Rollover API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-rollover-index.html)\n\nUpdates an alias to point to a new index when the existing index\nis considered to be too large or too old."]
pub struct IndicesRollover<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesRolloverParts<'b>,
    body: Option<B>,
    dry_run: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesRollover<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesRollover] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesRolloverParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesRollover {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesRollover<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    #[doc = "Whether a type should be included in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: bool) -> Self {
        self.include_type_name = Some(include_type_name);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Set the number of active shards to wait for on the newly created rollover index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Rollover API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "dry_run")]
                dry_run: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Segments API"]
pub enum IndicesSegmentsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesSegmentsParts<'b> {
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
#[doc = "Builder for the [Indices Segments API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-segments.html)\n\nProvides low-level information about segments in a Lucene index."]
pub struct IndicesSegments<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesSegmentsParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    verbose: Option<bool>,
}
impl<'a, 'b> IndicesSegments<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesSegments] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesSegmentsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesSegments {
            client,
            parts,
            headers,
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
    pub fn source(mut self, source: &'b str) -> Self {
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
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Shard Stores API"]
pub enum IndicesShardStoresParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesShardStoresParts<'b> {
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
#[doc = "Builder for the [Indices Shard Stores API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-shards-stores.html)\n\nProvides store information for shard copies of indices."]
pub struct IndicesShardStores<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesShardStoresParts<'b>,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    status: Option<&'b [&'b str]>,
}
impl<'a, 'b> IndicesShardStores<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesShardStores] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesShardStoresParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesShardStores {
            client,
            parts,
            headers,
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "A comma-separated list of statuses used to filter on shards to get store information for"]
    pub fn status(mut self, status: &'b [&'b str]) -> Self {
        self.status = Some(status);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Shard Stores API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "status", serialize_with = "crate::client::serialize_coll_qs")]
                status: Option<&'b [&'b str]>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Shrink API"]
pub enum IndicesShrinkParts<'b> {
    #[doc = "Index and Target"]
    IndexTarget(&'b str, &'b str),
}
impl<'b> IndicesShrinkParts<'b> {
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
#[doc = "Builder for the [Indices Shrink API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-shrink-index.html)\n\nAllow to shrink an existing index into a new index with fewer primary shards."]
pub struct IndicesShrink<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesShrinkParts<'b>,
    body: Option<B>,
    copy_settings: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesShrink<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesShrink] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesShrinkParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesShrink {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesShrink<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Set the number of active shards to wait for on the shrunken index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Shrink API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "copy_settings")]
                copy_settings: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Split API"]
pub enum IndicesSplitParts<'b> {
    #[doc = "Index and Target"]
    IndexTarget(&'b str, &'b str),
}
impl<'b> IndicesSplitParts<'b> {
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
#[doc = "Builder for the [Indices Split API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-split-index.html)\n\nAllows you to split an existing index into a new index with more primary shards."]
pub struct IndicesSplit<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesSplitParts<'b>,
    body: Option<B>,
    copy_settings: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesSplit<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesSplit] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesSplitParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesSplit {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesSplit<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Set the number of active shards to wait for on the shrunken index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Split API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "copy_settings")]
                copy_settings: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Stats API"]
pub enum IndicesStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Metric"]
    Metric(&'b [&'b str]),
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Metric"]
    IndexMetric(&'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesStatsParts<'b> {
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
#[doc = "Builder for the [Indices Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-stats.html)\n\nProvides statistics on operations happening in an index."]
pub struct IndicesStats<'a, 'b> {
    client: &'a Elasticsearch,
    parts: IndicesStatsParts<'b>,
    completion_fields: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    fielddata_fields: Option<&'b [&'b str]>,
    fields: Option<&'b [&'b str]>,
    filter_path: Option<&'b [&'b str]>,
    forbid_closed_indices: Option<bool>,
    groups: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    include_segment_file_sizes: Option<bool>,
    include_unloaded_segments: Option<bool>,
    level: Option<Level>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    types: Option<&'b [&'b str]>,
}
impl<'a, 'b> IndicesStats<'a, 'b> {
    #[doc = "Creates a new instance of [IndicesStats] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesStats {
            client,
            parts,
            headers,
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
    pub fn completion_fields(mut self, completion_fields: &'b [&'b str]) -> Self {
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
    pub fn fielddata_fields(mut self, fielddata_fields: &'b [&'b str]) -> Self {
        self.fielddata_fields = Some(fielddata_fields);
        self
    }
    #[doc = "A comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards)"]
    pub fn fields(mut self, fields: &'b [&'b str]) -> Self {
        self.fields = Some(fields);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "If set to false stats will also collected from closed indices if explicitly specified or if expand_wildcards expands to closed indices"]
    pub fn forbid_closed_indices(mut self, forbid_closed_indices: bool) -> Self {
        self.forbid_closed_indices = Some(forbid_closed_indices);
        self
    }
    #[doc = "A comma-separated list of search groups for `search` index metric"]
    pub fn groups(mut self, groups: &'b [&'b str]) -> Self {
        self.groups = Some(groups);
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "A comma-separated list of document types for the `indexing` index metric"]
    pub fn types(mut self, types: &'b [&'b str]) -> Self {
        self.types = Some(types);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(
                    rename = "completion_fields",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                completion_fields: Option<&'b [&'b str]>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(
                    rename = "fielddata_fields",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                fielddata_fields: Option<&'b [&'b str]>,
                #[serde(rename = "fields", serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'b [&'b str]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "forbid_closed_indices")]
                forbid_closed_indices: Option<bool>,
                #[serde(rename = "groups", serialize_with = "crate::client::serialize_coll_qs")]
                groups: Option<&'b [&'b str]>,
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
                source: Option<&'b str>,
                #[serde(rename = "types", serialize_with = "crate::client::serialize_coll_qs")]
                types: Option<&'b [&'b str]>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Unfreeze API"]
pub enum IndicesUnfreezeParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IndicesUnfreezeParts<'b> {
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
#[doc = "Builder for the [Indices Unfreeze API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/unfreeze-index-api.html)\n\nUnfreezes an index. When a frozen index is unfrozen, the index goes through the normal recovery process and becomes writeable again."]
pub struct IndicesUnfreeze<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesUnfreezeParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> IndicesUnfreeze<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesUnfreeze] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesUnfreezeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesUnfreeze {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesUnfreeze<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Unfreeze API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Update Aliases API"]
pub enum IndicesUpdateAliasesParts {
    #[doc = "No parts"]
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
#[doc = "Builder for the [Indices Update Aliases API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-aliases.html)\n\nUpdates index aliases."]
pub struct IndicesUpdateAliases<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesUpdateAliasesParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> IndicesUpdateAliases<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesUpdateAliases]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        IndicesUpdateAliases {
            client,
            parts: IndicesUpdateAliasesParts::None,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesUpdateAliases<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IndicesUpdateAliases {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
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
    #[doc = "Request timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Indices Update Aliases API that can be awaited"]
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
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
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Upgrade API"]
pub enum IndicesUpgradeParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> IndicesUpgradeParts<'b> {
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
#[doc = "Builder for the [Indices Upgrade API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-upgrade.html)\n\nThe _upgrade API is no longer useful and will be removed."]
pub struct IndicesUpgrade<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesUpgradeParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    only_ancient_segments: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> IndicesUpgrade<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesUpgrade] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesUpgradeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesUpgrade {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> IndicesUpgrade<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    pub fn source(mut self, source: &'b str) -> Self {
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
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "only_ancient_segments")]
                only_ancient_segments: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Indices Validate Query API"]
pub enum IndicesValidateQueryParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
    #[doc = "Index and Type"]
    IndexType(&'b [&'b str], &'b [&'b str]),
}
impl<'b> IndicesValidateQueryParts<'b> {
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
#[doc = "Builder for the [Indices Validate Query API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/search-validate.html)\n\nAllows a user to validate a potentially expensive query without executing it."]
pub struct IndicesValidateQuery<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: IndicesValidateQueryParts<'b>,
    all_shards: Option<bool>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    body: Option<B>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    explain: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    rewrite: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IndicesValidateQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IndicesValidateQuery] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: IndicesValidateQueryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IndicesValidateQuery {
            client,
            parts,
            headers,
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
    pub fn analyzer(mut self, analyzer: &'b str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IndicesValidateQuery<'a, 'b, JsonBody<T>>
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
            headers: self.headers,
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
    pub fn df(mut self, df: &'b str) -> Self {
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
    pub fn q(mut self, q: &'b str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "Provide a more detailed explanation showing the actual Lucene query that will be executed."]
    pub fn rewrite(mut self, rewrite: bool) -> Self {
        self.rewrite = Some(rewrite);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
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
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "all_shards")]
                all_shards: Option<bool>,
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer")]
                analyzer: Option<&'b str>,
                #[serde(rename = "default_operator")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df")]
                df: Option<&'b str>,
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
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient")]
                lenient: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "q")]
                q: Option<&'b str>,
                #[serde(rename = "rewrite")]
                rewrite: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Indices APIs"]
pub struct Indices<'a> {
    client: &'a Elasticsearch,
}
impl<'a> Indices<'a> {
    #[doc = "Creates a new instance of [Indices]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        Self { client }
    }
    #[doc = "[Indices Analyze API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-analyze.html)\n\nPerforms the analysis process on a text and return the tokens breakdown of the text."]
    pub fn analyze<'b>(&'a self, parts: IndicesAnalyzeParts<'b>) -> IndicesAnalyze<'a, 'b, ()> {
        IndicesAnalyze::new(&self.client, parts)
    }
    #[doc = "[Indices Clear Cache API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-clearcache.html)\n\nClears all or specific caches for one or more indices."]
    pub fn clear_cache<'b>(
        &'a self,
        parts: IndicesClearCacheParts<'b>,
    ) -> IndicesClearCache<'a, 'b, ()> {
        IndicesClearCache::new(&self.client, parts)
    }
    #[doc = "[Indices Clone API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-clone-index.html)\n\nClones an index"]
    pub fn clone<'b>(&'a self, parts: IndicesCloneParts<'b>) -> IndicesClone<'a, 'b, ()> {
        IndicesClone::new(&self.client, parts)
    }
    #[doc = "[Indices Close API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-open-close.html)\n\nCloses an index."]
    pub fn close<'b>(&'a self, parts: IndicesCloseParts<'b>) -> IndicesClose<'a, 'b, ()> {
        IndicesClose::new(&self.client, parts)
    }
    #[doc = "[Indices Create API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-create-index.html)\n\nCreates an index with optional settings and mappings.\n\n# Examples\n\nCreate an index with a mapping\n\n```rust,norun\n# use elasticsearch::{Elasticsearch, indices::IndicesCreateParts};\n# use serde_json::{json, Value};\n# async fn run() -> Result<(), Box<dyn std::error::Error>> { \n# let client = Elasticsearch::default();\nlet response = client\n    .indices()\n    .create(IndicesCreateParts::Index(\"test_index\"))\n    .body(json!({\n        \"mappings\" : {\n            \"properties\" : {\n                \"field1\" : { \"type\" : \"text\" }\n            }\n        }\n    }))\n    .send()\n    .await?;\n    \n# Ok(())\n# }\n```"]
    pub fn create<'b>(&'a self, parts: IndicesCreateParts<'b>) -> IndicesCreate<'a, 'b, ()> {
        IndicesCreate::new(&self.client, parts)
    }
    #[doc = "[Indices Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-delete-index.html)\n\nDeletes an index."]
    pub fn delete<'b>(&'a self, parts: IndicesDeleteParts<'b>) -> IndicesDelete<'a, 'b> {
        IndicesDelete::new(&self.client, parts)
    }
    #[doc = "[Indices Delete Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-aliases.html)\n\nDeletes an alias."]
    pub fn delete_alias<'b>(
        &'a self,
        parts: IndicesDeleteAliasParts<'b>,
    ) -> IndicesDeleteAlias<'a, 'b> {
        IndicesDeleteAlias::new(&self.client, parts)
    }
    #[doc = "[Indices Delete Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-templates.html)\n\nDeletes an index template."]
    pub fn delete_template<'b>(
        &'a self,
        parts: IndicesDeleteTemplateParts<'b>,
    ) -> IndicesDeleteTemplate<'a, 'b> {
        IndicesDeleteTemplate::new(&self.client, parts)
    }
    #[doc = "[Indices Exists API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-exists.html)\n\nReturns information about whether a particular index exists."]
    pub fn exists<'b>(&'a self, parts: IndicesExistsParts<'b>) -> IndicesExists<'a, 'b> {
        IndicesExists::new(&self.client, parts)
    }
    #[doc = "[Indices Exists Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-aliases.html)\n\nReturns information about whether a particular alias exists."]
    pub fn exists_alias<'b>(
        &'a self,
        parts: IndicesExistsAliasParts<'b>,
    ) -> IndicesExistsAlias<'a, 'b> {
        IndicesExistsAlias::new(&self.client, parts)
    }
    #[doc = "[Indices Exists Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-templates.html)\n\nReturns information about whether a particular index template exists."]
    pub fn exists_template<'b>(
        &'a self,
        parts: IndicesExistsTemplateParts<'b>,
    ) -> IndicesExistsTemplate<'a, 'b> {
        IndicesExistsTemplate::new(&self.client, parts)
    }
    #[doc = "[Indices Exists Type API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-types-exists.html)\n\nReturns information about whether a particular document type exists. (DEPRECATED)"]
    pub fn exists_type<'b>(
        &'a self,
        parts: IndicesExistsTypeParts<'b>,
    ) -> IndicesExistsType<'a, 'b> {
        IndicesExistsType::new(&self.client, parts)
    }
    #[doc = "[Indices Flush API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-flush.html)\n\nPerforms the flush operation on one or more indices."]
    pub fn flush<'b>(&'a self, parts: IndicesFlushParts<'b>) -> IndicesFlush<'a, 'b, ()> {
        IndicesFlush::new(&self.client, parts)
    }
    #[doc = "[Indices Flush Synced API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-synced-flush-api.html)\n\nPerforms a synced flush operation on one or more indices. Synced flush is deprecated and will be removed in 8.0. Use flush instead"]
    pub fn flush_synced<'b>(
        &'a self,
        parts: IndicesFlushSyncedParts<'b>,
    ) -> IndicesFlushSynced<'a, 'b, ()> {
        IndicesFlushSynced::new(&self.client, parts)
    }
    #[doc = "[Indices Forcemerge API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-forcemerge.html)\n\nPerforms the force merge operation on one or more indices."]
    pub fn forcemerge<'b>(
        &'a self,
        parts: IndicesForcemergeParts<'b>,
    ) -> IndicesForcemerge<'a, 'b, ()> {
        IndicesForcemerge::new(&self.client, parts)
    }
    #[doc = "[Indices Freeze API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/freeze-index-api.html)\n\nFreezes an index. A frozen index has almost no overhead on the cluster (except for maintaining its metadata in memory) and is read-only."]
    pub fn freeze<'b>(&'a self, parts: IndicesFreezeParts<'b>) -> IndicesFreeze<'a, 'b, ()> {
        IndicesFreeze::new(&self.client, parts)
    }
    #[doc = "[Indices Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-get-index.html)\n\nReturns information about one or more indices."]
    pub fn get<'b>(&'a self, parts: IndicesGetParts<'b>) -> IndicesGet<'a, 'b> {
        IndicesGet::new(&self.client, parts)
    }
    #[doc = "[Indices Get Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-aliases.html)\n\nReturns an alias."]
    pub fn get_alias<'b>(&'a self, parts: IndicesGetAliasParts<'b>) -> IndicesGetAlias<'a, 'b> {
        IndicesGetAlias::new(&self.client, parts)
    }
    #[doc = "[Indices Get Field Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-get-field-mapping.html)\n\nReturns mapping for one or more fields."]
    pub fn get_field_mapping<'b>(
        &'a self,
        parts: IndicesGetFieldMappingParts<'b>,
    ) -> IndicesGetFieldMapping<'a, 'b> {
        IndicesGetFieldMapping::new(&self.client, parts)
    }
    #[doc = "[Indices Get Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-get-mapping.html)\n\nReturns mappings for one or more indices."]
    pub fn get_mapping<'b>(
        &'a self,
        parts: IndicesGetMappingParts<'b>,
    ) -> IndicesGetMapping<'a, 'b> {
        IndicesGetMapping::new(&self.client, parts)
    }
    #[doc = "[Indices Get Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-get-settings.html)\n\nReturns settings for one or more indices."]
    pub fn get_settings<'b>(
        &'a self,
        parts: IndicesGetSettingsParts<'b>,
    ) -> IndicesGetSettings<'a, 'b> {
        IndicesGetSettings::new(&self.client, parts)
    }
    #[doc = "[Indices Get Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-templates.html)\n\nReturns an index template."]
    pub fn get_template<'b>(
        &'a self,
        parts: IndicesGetTemplateParts<'b>,
    ) -> IndicesGetTemplate<'a, 'b> {
        IndicesGetTemplate::new(&self.client, parts)
    }
    #[doc = "[Indices Get Upgrade API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-upgrade.html)\n\nThe _upgrade API is no longer useful and will be removed."]
    pub fn get_upgrade<'b>(
        &'a self,
        parts: IndicesGetUpgradeParts<'b>,
    ) -> IndicesGetUpgrade<'a, 'b> {
        IndicesGetUpgrade::new(&self.client, parts)
    }
    #[doc = "[Indices Open API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-open-close.html)\n\nOpens an index."]
    pub fn open<'b>(&'a self, parts: IndicesOpenParts<'b>) -> IndicesOpen<'a, 'b, ()> {
        IndicesOpen::new(&self.client, parts)
    }
    #[doc = "[Indices Put Alias API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-aliases.html)\n\nCreates or updates an alias."]
    pub fn put_alias<'b>(&'a self, parts: IndicesPutAliasParts<'b>) -> IndicesPutAlias<'a, 'b, ()> {
        IndicesPutAlias::new(&self.client, parts)
    }
    #[doc = "[Indices Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-put-mapping.html)\n\nUpdates the index mappings.\n\n# Examples\n\nPut a mapping into an existing index, assuming the index does not have a mapping, \nor that any properties specified do not conflict with existing properties\n\n```rust,norun\n# use elasticsearch::{Elasticsearch, indices::IndicesPutMappingParts};\n# use serde_json::{json, Value};\n# async fn run() -> Result<(), Box<dyn std::error::Error>> { \n# let client = Elasticsearch::default();\nlet response = client\n    .indices()\n    .put_mapping(IndicesPutMappingParts::Index(&[\"test_index\"]))\n    .body(json!({\n        \"properties\" : {\n            \"field1\" : { \"type\" : \"text\" }\n        }\n    }))\n    .send()\n    .await?;\n    \n# Ok(())\n# }\n```"]
    pub fn put_mapping<'b>(
        &'a self,
        parts: IndicesPutMappingParts<'b>,
    ) -> IndicesPutMapping<'a, 'b, ()> {
        IndicesPutMapping::new(&self.client, parts)
    }
    #[doc = "[Indices Put Settings API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-update-settings.html)\n\nUpdates the index settings."]
    pub fn put_settings<'b>(
        &'a self,
        parts: IndicesPutSettingsParts<'b>,
    ) -> IndicesPutSettings<'a, 'b, ()> {
        IndicesPutSettings::new(&self.client, parts)
    }
    #[doc = "[Indices Put Template API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-templates.html)\n\nCreates or updates an index template."]
    pub fn put_template<'b>(
        &'a self,
        parts: IndicesPutTemplateParts<'b>,
    ) -> IndicesPutTemplate<'a, 'b, ()> {
        IndicesPutTemplate::new(&self.client, parts)
    }
    #[doc = "[Indices Recovery API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-recovery.html)\n\nReturns information about ongoing index shard recoveries."]
    pub fn recovery<'b>(&'a self, parts: IndicesRecoveryParts<'b>) -> IndicesRecovery<'a, 'b> {
        IndicesRecovery::new(&self.client, parts)
    }
    #[doc = "[Indices Refresh API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-refresh.html)\n\nPerforms the refresh operation in one or more indices."]
    pub fn refresh<'b>(&'a self, parts: IndicesRefreshParts<'b>) -> IndicesRefresh<'a, 'b, ()> {
        IndicesRefresh::new(&self.client, parts)
    }
    #[doc = "[Indices Reload Search Analyzers API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-reload-analyzers.html)\n\nReloads an index's search analyzers and their resources."]
    pub fn reload_search_analyzers<'b>(
        &'a self,
        parts: IndicesReloadSearchAnalyzersParts<'b>,
    ) -> IndicesReloadSearchAnalyzers<'a, 'b, ()> {
        IndicesReloadSearchAnalyzers::new(&self.client, parts)
    }
    #[doc = "[Indices Rollover API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-rollover-index.html)\n\nUpdates an alias to point to a new index when the existing index\nis considered to be too large or too old."]
    pub fn rollover<'b>(&'a self, parts: IndicesRolloverParts<'b>) -> IndicesRollover<'a, 'b, ()> {
        IndicesRollover::new(&self.client, parts)
    }
    #[doc = "[Indices Segments API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-segments.html)\n\nProvides low-level information about segments in a Lucene index."]
    pub fn segments<'b>(&'a self, parts: IndicesSegmentsParts<'b>) -> IndicesSegments<'a, 'b> {
        IndicesSegments::new(&self.client, parts)
    }
    #[doc = "[Indices Shard Stores API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-shards-stores.html)\n\nProvides store information for shard copies of indices."]
    pub fn shard_stores<'b>(
        &'a self,
        parts: IndicesShardStoresParts<'b>,
    ) -> IndicesShardStores<'a, 'b> {
        IndicesShardStores::new(&self.client, parts)
    }
    #[doc = "[Indices Shrink API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-shrink-index.html)\n\nAllow to shrink an existing index into a new index with fewer primary shards."]
    pub fn shrink<'b>(&'a self, parts: IndicesShrinkParts<'b>) -> IndicesShrink<'a, 'b, ()> {
        IndicesShrink::new(&self.client, parts)
    }
    #[doc = "[Indices Split API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-split-index.html)\n\nAllows you to split an existing index into a new index with more primary shards."]
    pub fn split<'b>(&'a self, parts: IndicesSplitParts<'b>) -> IndicesSplit<'a, 'b, ()> {
        IndicesSplit::new(&self.client, parts)
    }
    #[doc = "[Indices Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-stats.html)\n\nProvides statistics on operations happening in an index."]
    pub fn stats<'b>(&'a self, parts: IndicesStatsParts<'b>) -> IndicesStats<'a, 'b> {
        IndicesStats::new(&self.client, parts)
    }
    #[doc = "[Indices Unfreeze API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/unfreeze-index-api.html)\n\nUnfreezes an index. When a frozen index is unfrozen, the index goes through the normal recovery process and becomes writeable again."]
    pub fn unfreeze<'b>(&'a self, parts: IndicesUnfreezeParts<'b>) -> IndicesUnfreeze<'a, 'b, ()> {
        IndicesUnfreeze::new(&self.client, parts)
    }
    #[doc = "[Indices Update Aliases API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-aliases.html)\n\nUpdates index aliases."]
    pub fn update_aliases<'b>(&'a self) -> IndicesUpdateAliases<'a, 'b, ()> {
        IndicesUpdateAliases::new(&self.client)
    }
    #[doc = "[Indices Upgrade API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/indices-upgrade.html)\n\nThe _upgrade API is no longer useful and will be removed."]
    pub fn upgrade<'b>(&'a self, parts: IndicesUpgradeParts<'b>) -> IndicesUpgrade<'a, 'b, ()> {
        IndicesUpgrade::new(&self.client, parts)
    }
    #[doc = "[Indices Validate Query API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/search-validate.html)\n\nAllows a user to validate a potentially expensive query without executing it."]
    pub fn validate_query<'b>(
        &'a self,
        parts: IndicesValidateQueryParts<'b>,
    ) -> IndicesValidateQuery<'a, 'b, ()> {
        IndicesValidateQuery::new(&self.client, parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Indices APIs"]
    pub fn indices(&self) -> Indices {
        Indices::new(&self)
    }
}
