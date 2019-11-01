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
pub struct IndicesAnalyze<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> IndicesAnalyze<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        IndicesAnalyze {
            client,
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
    #[doc = "The name of the index to scope the operation"]
    pub fn index(mut self, index: Option<String>) -> Self {
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
impl<B> Sender for IndicesAnalyze<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index = index;
                let mut p = String::with_capacity(10usize + index.len());
                p.push_str("/");
                p.push_str(index.as_ref());
                p.push_str("/_analyze");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_analyze"),
        };
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "index")]
                index: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesClearCache<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    fielddata: Option<bool>,
    fields: Option<Vec<String>>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    pretty: Option<bool>,
    query: Option<bool>,
    request: Option<bool>,
    source: Option<String>,
}
impl<B> IndicesClearCache<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        IndicesClearCache {
            client,
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
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "Clear field data"]
    pub fn fielddata(mut self, fielddata: Option<bool>) -> Self {
        self.fielddata = fielddata;
        self
    }
    #[doc = "A comma-separated list of fields to clear when using the `fielddata` parameter (default: all)"]
    pub fn fields(mut self, fields: Option<Vec<String>>) -> Self {
        self.fields = fields;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index name to limit the operation"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Clear query caches"]
    pub fn query(mut self, query: Option<bool>) -> Self {
        self.query = query;
        self
    }
    #[doc = "Clear request cache"]
    pub fn request(mut self, request: Option<bool>) -> Self {
        self.request = request;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for IndicesClearCache<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(14usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_cache/clear");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_cache/clear"),
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "fielddata")]
                fielddata: Option<bool>,
                #[serde(rename = "fields")]
                fields: Option<Vec<String>>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "index")]
                index: Option<Vec<String>>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "query")]
                query: Option<bool>,
                #[serde(rename = "request")]
                request: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesClose<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Vec<String>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl<B> IndicesClose<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        IndicesClose {
            client,
            index: index,
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
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma separated list of indices to close"]
    pub fn index(mut self, index: Vec<String>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl<B> Sender for IndicesClose<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index_str = self.index.join(",");
            let mut p = String::with_capacity(8usize + index_str.len());
            p.push_str("/");
            p.push_str(index_str.as_ref());
            p.push_str("/_close");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesCreate<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    include_type_name: Option<bool>,
    index: String,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl<B> IndicesCreate<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IndicesCreate {
            client,
            index: index,
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
    #[doc = "Whether a type should be expected in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: Option<bool>) -> Self {
        self.include_type_name = include_type_name;
        self
    }
    #[doc = "The name of the index"]
    pub fn index(mut self, index: String) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Set the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl<B> Sender for IndicesCreate<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index = self.index;
            let mut p = String::with_capacity(1usize + index.len());
            p.push_str("/");
            p.push_str(index.as_ref());
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
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesDelete {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Vec<String>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl IndicesDelete {
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        IndicesDelete {
            client,
            index: index,
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
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether wildcard expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Ignore unavailable indexes (default: false)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of indices to delete; use `_all` or `*` string to delete all indices"]
    pub fn index(mut self, index: Vec<String>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for IndicesDelete {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index_str = self.index.join(",");
            let mut p = String::with_capacity(1usize + index_str.len());
            p.push_str("/");
            p.push_str(index_str.as_ref());
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesDeleteAlias {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Vec<String>,
    master_timeout: Option<String>,
    name: Vec<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl IndicesDeleteAlias {
    pub fn new(client: Elasticsearch, index: Vec<String>, name: Vec<String>) -> Self {
        IndicesDeleteAlias {
            client,
            index: index,
            name: name,
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
    #[doc = "A comma-separated list of index names (supports wildcards); use `_all` for all indices"]
    pub fn index(mut self, index: Vec<String>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "A comma-separated list of aliases to delete (supports wildcards); use `_all` to delete all aliases for the specified indices."]
    pub fn name(mut self, name: Vec<String>) -> Self {
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
    #[doc = "Explicit timestamp for the document"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for IndicesDeleteAlias {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index_str = self.index.join(",");
            let name_str = self.name.join(",");
            let mut p = String::with_capacity(9usize + index_str.len() + name_str.len());
            p.push_str("/");
            p.push_str(index_str.as_ref());
            p.push_str("/_alias/");
            p.push_str(name_str.as_ref());
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesDeleteTemplate {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    name: String,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl IndicesDeleteTemplate {
    pub fn new(client: Elasticsearch, name: String) -> Self {
        IndicesDeleteTemplate {
            client,
            name: name,
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "The name of the template"]
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for IndicesDeleteTemplate {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let name = self.name;
            let mut p = String::with_capacity(11usize + name.len());
            p.push_str("/_template/");
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesExists {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    index: Vec<String>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IndicesExists {
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        IndicesExists {
            client,
            index: index,
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
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether wildcard expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: Option<bool>) -> Self {
        self.flat_settings = flat_settings;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Ignore unavailable indexes (default: false)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Whether to return all default setting for each of the indices."]
    pub fn include_defaults(mut self, include_defaults: Option<bool>) -> Self {
        self.include_defaults = include_defaults;
        self
    }
    #[doc = "A comma-separated list of index names"]
    pub fn index(mut self, index: Vec<String>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
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
impl Sender for IndicesExists {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index_str = self.index.join(",");
            let mut p = String::with_capacity(1usize + index_str.len());
            p.push_str("/");
            p.push_str(index_str.as_ref());
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Head;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
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
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesExistsAlias {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    local: Option<bool>,
    name: Vec<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IndicesExistsAlias {
    pub fn new(client: Elasticsearch, name: Vec<String>) -> Self {
        IndicesExistsAlias {
            client,
            name: name,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            local: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names to filter aliases"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "A comma-separated list of alias names to return"]
    pub fn name(mut self, name: Vec<String>) -> Self {
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
impl Sender for IndicesExistsAlias {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index_str = index.join(",");
                let name_str = self.name.join(",");
                let mut p = String::with_capacity(9usize + index_str.len() + name_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_alias/");
                p.push_str(name_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            None => {
                let name_str = self.name.join(",");
                let mut p = String::with_capacity(8usize + name_str.len());
                p.push_str("/_alias/");
                p.push_str(name_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
        };
        let method = HttpMethod::Head;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesExistsTemplate {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    name: Vec<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IndicesExistsTemplate {
    pub fn new(client: Elasticsearch, name: Vec<String>) -> Self {
        IndicesExistsTemplate {
            client,
            name: name,
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
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: Option<bool>) -> Self {
        self.flat_settings = flat_settings;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "The comma separated names of the index templates"]
    pub fn name(mut self, name: Vec<String>) -> Self {
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
impl Sender for IndicesExistsTemplate {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let name_str = self.name.join(",");
            let mut p = String::with_capacity(11usize + name_str.len());
            p.push_str("/_template/");
            p.push_str(name_str.as_ref());
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Head;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesExistsType {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Vec<String>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    ty: Vec<String>,
}
impl IndicesExistsType {
    pub fn new(client: Elasticsearch, index: Vec<String>, ty: Vec<String>) -> Self {
        IndicesExistsType {
            client,
            index: index,
            ty: ty,
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
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names; use `_all` to check the types across all indices"]
    pub fn index(mut self, index: Vec<String>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
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
    #[doc = "A comma-separated list of document types to check"]
    pub fn ty(mut self, ty: Vec<String>) -> Self {
        self.ty = ty;
        self
    }
}
impl Sender for IndicesExistsType {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index_str = self.index.join(",");
            let ty_str = self.ty.join(",");
            let mut p = String::with_capacity(11usize + index_str.len() + ty_str.len());
            p.push_str("/");
            p.push_str(index_str.as_ref());
            p.push_str("/_mapping/");
            p.push_str(ty_str.as_ref());
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Head;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesFlush<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    force: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
    wait_if_ongoing: Option<bool>,
}
impl<B> IndicesFlush<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        IndicesFlush {
            client,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            force: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            pretty: None,
            source: None,
            wait_if_ongoing: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Whether a flush should be forced even if it is not necessarily needed ie. if no changes will be committed to the index. This is useful if transaction log IDs should be incremented even if no uncommitted changes are present. (This setting can be considered as internal)"]
    pub fn force(mut self, force: Option<bool>) -> Self {
        self.force = force;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names; use `_all` or empty string for all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
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
    #[doc = "If set to true the flush operation will block until the flush can be executed if another flush operation is already executing. The default is true. If set to false the flush will be skipped iff if another flush operation is already running."]
    pub fn wait_if_ongoing(mut self, wait_if_ongoing: Option<bool>) -> Self {
        self.wait_if_ongoing = wait_if_ongoing;
        self
    }
}
impl<B> Sender for IndicesFlush<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(8usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_flush");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_flush"),
        };
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "wait_if_ongoing")]
                wait_if_ongoing: Option<bool>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesFlushSynced<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> IndicesFlushSynced<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        IndicesFlushSynced {
            client,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names; use `_all` or empty string for all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
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
impl<B> Sender for IndicesFlushSynced<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(15usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_flush/synced");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_flush/synced"),
        };
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesForcemerge<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    flush: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    max_num_segments: Option<i64>,
    only_expunge_deletes: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> IndicesForcemerge<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        IndicesForcemerge {
            client,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flush: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            max_num_segments: None,
            only_expunge_deletes: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Specify whether the index should be flushed after performing the operation (default: true)"]
    pub fn flush(mut self, flush: Option<bool>) -> Self {
        self.flush = flush;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names; use `_all` or empty string to perform the operation on all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "The number of segments the index should be merged into (default: dynamic)"]
    pub fn max_num_segments(mut self, max_num_segments: Option<i64>) -> Self {
        self.max_num_segments = max_num_segments;
        self
    }
    #[doc = "Specify whether the operation should only expunge deleted documents"]
    pub fn only_expunge_deletes(mut self, only_expunge_deletes: Option<bool>) -> Self {
        self.only_expunge_deletes = only_expunge_deletes;
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
impl<B> Sender for IndicesForcemerge<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(13usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_forcemerge");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_forcemerge"),
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
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
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesFreeze<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: String,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl<B> IndicesFreeze<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IndicesFreeze {
            client,
            index: index,
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
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "The name of the index to freeze"]
    pub fn index(mut self, index: String) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl<B> Sender for IndicesFreeze<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index = self.index;
            let mut p = String::with_capacity(9usize + index.len());
            p.push_str("/");
            p.push_str(index.as_ref());
            p.push_str("/_freeze");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesGet {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    include_type_name: Option<bool>,
    index: Vec<String>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IndicesGet {
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        IndicesGet {
            client,
            index: index,
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
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether wildcard expressions should get expanded to open or closed indices (default: open)"]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: Option<bool>) -> Self {
        self.flat_settings = flat_settings;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Ignore unavailable indexes (default: false)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Whether to return all default setting for each of the indices."]
    pub fn include_defaults(mut self, include_defaults: Option<bool>) -> Self {
        self.include_defaults = include_defaults;
        self
    }
    #[doc = "Whether to add the type name to the response (default: false)"]
    pub fn include_type_name(mut self, include_type_name: Option<bool>) -> Self {
        self.include_type_name = include_type_name;
        self
    }
    #[doc = "A comma-separated list of index names"]
    pub fn index(mut self, index: Vec<String>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
impl Sender for IndicesGet {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index_str = self.index.join(",");
            let mut p = String::with_capacity(1usize + index_str.len());
            p.push_str("/");
            p.push_str(index_str.as_ref());
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
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
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesGetAlias {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    local: Option<bool>,
    name: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IndicesGetAlias {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesGetAlias {
            client,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            local: None,
            name: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names to filter aliases"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "A comma-separated list of alias names to return"]
    pub fn name(mut self, name: Option<Vec<String>>) -> Self {
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
impl Sender for IndicesGetAlias {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match (&self.index, &self.name) {
            (Some(index), Some(name)) => {
                let index_str = index.join(",");
                let name_str = name.join(",");
                let mut p = String::with_capacity(9usize + index_str.len() + name_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_alias/");
                p.push_str(name_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, Some(name)) => {
                let name_str = name.join(",");
                let mut p = String::with_capacity(8usize + name_str.len());
                p.push_str("/_alias/");
                p.push_str(name_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (Some(index), None) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(8usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_alias");
                std::borrow::Cow::Owned(p)
            }
            (None, None) => std::borrow::Cow::Borrowed("/_alias"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesGetFieldMapping {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    fields: Vec<String>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    include_type_name: Option<bool>,
    index: Option<Vec<String>>,
    local: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    ty: Option<Vec<String>>,
}
impl IndicesGetFieldMapping {
    pub fn new(client: Elasticsearch, fields: Vec<String>) -> Self {
        IndicesGetFieldMapping {
            client,
            fields: fields,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            include_defaults: None,
            include_type_name: None,
            index: None,
            local: None,
            pretty: None,
            source: None,
            ty: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of fields"]
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.fields = fields;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Whether the default mapping values should be returned as well"]
    pub fn include_defaults(mut self, include_defaults: Option<bool>) -> Self {
        self.include_defaults = include_defaults;
        self
    }
    #[doc = "Whether a type should be returned in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: Option<bool>) -> Self {
        self.include_type_name = include_type_name;
        self
    }
    #[doc = "A comma-separated list of index names"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
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
    #[doc = "A comma-separated list of document types"]
    pub fn ty(mut self, ty: Option<Vec<String>>) -> Self {
        self.ty = ty;
        self
    }
}
impl Sender for IndicesGetFieldMapping {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match (&self.index, &self.ty) {
            (Some(index), Some(ty)) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let fields_str = self.fields.join(",");
                let mut p = String::with_capacity(
                    18usize + index_str.len() + ty_str.len() + fields_str.len(),
                );
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping/");
                p.push_str(ty_str.as_ref());
                p.push_str("/field/");
                p.push_str(fields_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (Some(index), None) => {
                let index_str = index.join(",");
                let fields_str = self.fields.join(",");
                let mut p = String::with_capacity(17usize + index_str.len() + fields_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping/field/");
                p.push_str(fields_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, Some(ty)) => {
                let ty_str = ty.join(",");
                let fields_str = self.fields.join(",");
                let mut p = String::with_capacity(17usize + ty_str.len() + fields_str.len());
                p.push_str("/_mapping/");
                p.push_str(ty_str.as_ref());
                p.push_str("/field/");
                p.push_str(fields_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, None) => {
                let fields_str = self.fields.join(",");
                let mut p = String::with_capacity(16usize + fields_str.len());
                p.push_str("/_mapping/field/");
                p.push_str(fields_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
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
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesGetMapping {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_type_name: Option<bool>,
    index: Option<Vec<String>>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    ty: Option<Vec<String>>,
}
impl IndicesGetMapping {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesGetMapping {
            client,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            include_type_name: None,
            index: None,
            local: None,
            master_timeout: None,
            pretty: None,
            source: None,
            ty: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Whether to add the type name to the response (default: false)"]
    pub fn include_type_name(mut self, include_type_name: Option<bool>) -> Self {
        self.include_type_name = include_type_name;
        self
    }
    #[doc = "A comma-separated list of index names"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "A comma-separated list of document types"]
    pub fn ty(mut self, ty: Option<Vec<String>>) -> Self {
        self.ty = ty;
        self
    }
}
impl Sender for IndicesGetMapping {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match (&self.index, &self.ty) {
            (Some(index), Some(ty)) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(11usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping/");
                p.push_str(ty_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (Some(index), None) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping");
                std::borrow::Cow::Owned(p)
            }
            (None, Some(ty)) => {
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(10usize + ty_str.len());
                p.push_str("/_mapping/");
                p.push_str(ty_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, None) => std::borrow::Cow::Borrowed("/_mapping"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesGetSettings {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_defaults: Option<bool>,
    index: Option<Vec<String>>,
    local: Option<bool>,
    master_timeout: Option<String>,
    name: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IndicesGetSettings {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesGetSettings {
            client,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            ignore_unavailable: None,
            include_defaults: None,
            index: None,
            local: None,
            master_timeout: None,
            name: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: Option<bool>) -> Self {
        self.flat_settings = flat_settings;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Whether to return all default setting for each of the indices."]
    pub fn include_defaults(mut self, include_defaults: Option<bool>) -> Self {
        self.include_defaults = include_defaults;
        self
    }
    #[doc = "A comma-separated list of index names; use `_all` or empty string to perform the operation on all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "The name of the settings that should be included"]
    pub fn name(mut self, name: Option<Vec<String>>) -> Self {
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
impl Sender for IndicesGetSettings {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match (&self.index, &self.name) {
            (Some(index), Some(name)) => {
                let index_str = index.join(",");
                let name_str = name.join(",");
                let mut p = String::with_capacity(12usize + index_str.len() + name_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_settings/");
                p.push_str(name_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (Some(index), None) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(11usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_settings");
                std::borrow::Cow::Owned(p)
            }
            (None, Some(name)) => {
                let name_str = name.join(",");
                let mut p = String::with_capacity(11usize + name_str.len());
                p.push_str("/_settings/");
                p.push_str(name_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, None) => std::borrow::Cow::Borrowed("/_settings"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
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
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesGetTemplate {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    include_type_name: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    name: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IndicesGetTemplate {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesGetTemplate {
            client,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            include_type_name: None,
            local: None,
            master_timeout: None,
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
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: Option<bool>) -> Self {
        self.flat_settings = flat_settings;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether a type should be returned in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: Option<bool>) -> Self {
        self.include_type_name = include_type_name;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "The comma separated names of the index templates"]
    pub fn name(mut self, name: Option<Vec<String>>) -> Self {
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
impl Sender for IndicesGetTemplate {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.name {
            Some(name) => {
                let name_str = name.join(",");
                let mut p = String::with_capacity(11usize + name_str.len());
                p.push_str("/_template/");
                p.push_str(name_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_template"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesGetUpgrade {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IndicesGetUpgrade {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesGetUpgrade {
            client,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names; use `_all` or empty string to perform the operation on all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
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
impl Sender for IndicesGetUpgrade {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_upgrade");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_upgrade"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesOpen<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Vec<String>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl<B> IndicesOpen<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        IndicesOpen {
            client,
            index: index,
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
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma separated list of indices to open"]
    pub fn index(mut self, index: Vec<String>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl<B> Sender for IndicesOpen<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index_str = self.index.join(",");
            let mut p = String::with_capacity(7usize + index_str.len());
            p.push_str("/");
            p.push_str(index_str.as_ref());
            p.push_str("/_open");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesPutAlias<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Vec<String>,
    master_timeout: Option<String>,
    name: String,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> IndicesPutAlias<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: Vec<String>, name: String) -> Self {
        IndicesPutAlias {
            client,
            index: index,
            name: name,
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
    #[doc = "A comma-separated list of index names the alias should point to (supports wildcards); use `_all` to perform the operation on all indices."]
    pub fn index(mut self, index: Vec<String>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "The name of the alias to be created or updated"]
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
    #[doc = "Explicit timestamp for the document"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl<B> Sender for IndicesPutAlias<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index_str = self.index.join(",");
            let name = self.name;
            let mut p = String::with_capacity(9usize + index_str.len() + name.len());
            p.push_str("/");
            p.push_str(index_str.as_ref());
            p.push_str("/_alias/");
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesPutMapping<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_type_name: Option<bool>,
    index: Option<Vec<String>>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    ty: Option<String>,
}
impl<B> IndicesPutMapping<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        IndicesPutMapping {
            client,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            include_type_name: None,
            index: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
            ty: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Whether a type should be expected in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: Option<bool>) -> Self {
        self.include_type_name = include_type_name;
        self
    }
    #[doc = "A comma-separated list of index names the mapping should be added to (supports wildcards); use `_all` or omit to add the mapping on all indices."]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "The name of the document type"]
    pub fn ty(mut self, ty: Option<String>) -> Self {
        self.ty = ty;
        self
    }
}
impl<B> Sender for IndicesPutMapping<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match (&self.index, &self.ty) {
            (Some(index), Some(ty)) => {
                let index_str = index.join(",");
                let ty = ty;
                let mut p = String::with_capacity(11usize + index_str.len() + ty.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty.as_ref());
                p.push_str("/_mapping");
                std::borrow::Cow::Owned(p)
            }
            (Some(index), None) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_mapping");
                std::borrow::Cow::Owned(p)
            }
            (None, Some(ty)) => {
                let ty = ty;
                let mut p = String::with_capacity(10usize + ty.len());
                p.push_str("/_mapping/");
                p.push_str(ty.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, None) => std::borrow::Cow::Borrowed("/_all/_mapping"),
        };
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesPutSettings<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    master_timeout: Option<String>,
    preserve_existing: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> IndicesPutSettings<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        IndicesPutSettings {
            client,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            master_timeout: None,
            preserve_existing: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: Option<bool>) -> Self {
        self.flat_settings = flat_settings;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names; use `_all` or empty string to perform the operation on all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Whether to update existing settings. If set to `true` existing settings on an index remain unchanged, the default is `false`"]
    pub fn preserve_existing(mut self, preserve_existing: Option<bool>) -> Self {
        self.preserve_existing = preserve_existing;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl<B> Sender for IndicesPutSettings<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(11usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_settings");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_settings"),
        };
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "preserve_existing")]
                preserve_existing: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesPutTemplate<B> {
    client: Elasticsearch,
    body: Option<B>,
    create: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<String>,
    name: String,
    order: Option<i64>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> IndicesPutTemplate<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, name: String) -> Self {
        IndicesPutTemplate {
            client,
            name: name,
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
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Whether the index template should only be added if new or can also replace an existing one"]
    pub fn create(mut self, create: Option<bool>) -> Self {
        self.create = create;
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
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: Option<bool>) -> Self {
        self.flat_settings = flat_settings;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether a type should be returned in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: Option<bool>) -> Self {
        self.include_type_name = include_type_name;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "The name of the template"]
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    #[doc = "The order for this template when merging multiple matching ones (higher numbers are merged later, overriding the lower numbers)"]
    pub fn order(mut self, order: Option<i64>) -> Self {
        self.order = order;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl<B> Sender for IndicesPutTemplate<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let name = self.name;
            let mut p = String::with_capacity(11usize + name.len());
            p.push_str("/_template/");
            p.push_str(name.as_ref());
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "create")]
                create: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "order")]
                order: Option<i64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesRecovery {
    client: Elasticsearch,
    active_only: Option<bool>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IndicesRecovery {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesRecovery {
            client,
            active_only: None,
            detailed: None,
            error_trace: None,
            filter_path: None,
            human: None,
            index: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Display only those recoveries that are currently on-going"]
    pub fn active_only(mut self, active_only: Option<bool>) -> Self {
        self.active_only = active_only;
        self
    }
    #[doc = "Whether to display detailed information about shard recovery"]
    pub fn detailed(mut self, detailed: Option<bool>) -> Self {
        self.detailed = detailed;
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
    #[doc = "A comma-separated list of index names; use `_all` or empty string to perform the operation on all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
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
impl Sender for IndicesRecovery {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(11usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_recovery");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_recovery"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "active_only")]
                active_only: Option<bool>,
                #[serde(rename = "detailed")]
                detailed: Option<bool>,
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesRefresh<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> IndicesRefresh<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        IndicesRefresh {
            client,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names; use `_all` or empty string to perform the operation on all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
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
impl<B> Sender for IndicesRefresh<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_refresh");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_refresh"),
        };
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesReloadSearchAnalyzers<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Vec<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> IndicesReloadSearchAnalyzers<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        IndicesReloadSearchAnalyzers {
            client,
            index: index,
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
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names to reload analyzers for"]
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
impl<B> Sender for IndicesReloadSearchAnalyzers<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index_str = self.index.join(",");
            let mut p = String::with_capacity(26usize + index_str.len());
            p.push_str("/");
            p.push_str(index_str.as_ref());
            p.push_str("/_reload_search_analyzers");
            std::borrow::Cow::Owned(p)
        };
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesRollover<B> {
    client: Elasticsearch,
    alias: String,
    body: Option<B>,
    dry_run: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    include_type_name: Option<bool>,
    master_timeout: Option<String>,
    new_index: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl<B> IndicesRollover<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, alias: String) -> Self {
        IndicesRollover {
            client,
            alias: alias,
            body: None,
            dry_run: None,
            error_trace: None,
            filter_path: None,
            human: None,
            include_type_name: None,
            master_timeout: None,
            new_index: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The name of the alias to rollover"]
    pub fn alias(mut self, alias: String) -> Self {
        self.alias = alias;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "If set to true the rollover action will only be validated but not actually performed even if a condition matches. The default is false"]
    pub fn dry_run(mut self, dry_run: Option<bool>) -> Self {
        self.dry_run = dry_run;
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
    #[doc = "Whether a type should be included in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: Option<bool>) -> Self {
        self.include_type_name = include_type_name;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "The name of the rollover index"]
    pub fn new_index(mut self, new_index: Option<String>) -> Self {
        self.new_index = new_index;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Set the number of active shards to wait for on the newly created rollover index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl<B> Sender for IndicesRollover<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.new_index {
            Some(new_index) => {
                let alias = self.alias;
                let new_index = new_index;
                let mut p = String::with_capacity(12usize + alias.len() + new_index.len());
                p.push_str("/");
                p.push_str(alias.as_ref());
                p.push_str("/_rollover/");
                p.push_str(new_index.as_ref());
                std::borrow::Cow::Owned(p)
            }
            None => {
                let alias = self.alias;
                let mut p = String::with_capacity(11usize + alias.len());
                p.push_str("/");
                p.push_str(alias.as_ref());
                p.push_str("/_rollover");
                std::borrow::Cow::Owned(p)
            }
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "dry_run")]
                dry_run: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesSegments {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
    verbose: Option<bool>,
}
impl IndicesSegments {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesSegments {
            client,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            pretty: None,
            source: None,
            verbose: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names; use `_all` or empty string to perform the operation on all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
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
    #[doc = "Includes detailed memory usage by Lucene."]
    pub fn verbose(mut self, verbose: Option<bool>) -> Self {
        self.verbose = verbose;
        self
    }
}
impl Sender for IndicesSegments {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(11usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_segments");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_segments"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "verbose")]
                verbose: Option<bool>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesShardStores {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
    status: Option<Vec<String>>,
}
impl IndicesShardStores {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesShardStores {
            client,
            allow_no_indices: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            pretty: None,
            source: None,
            status: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names; use `_all` or empty string to perform the operation on all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
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
    #[doc = "A comma-separated list of statuses used to filter on shards to get store information for"]
    pub fn status(mut self, status: Option<Vec<String>>) -> Self {
        self.status = status;
        self
    }
}
impl Sender for IndicesShardStores {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(15usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_shard_stores");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_shard_stores"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "status")]
                status: Option<Vec<String>>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesShrink<B> {
    client: Elasticsearch,
    body: Option<B>,
    copy_settings: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    target: String,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl<B> IndicesShrink<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String, target: String) -> Self {
        IndicesShrink {
            client,
            index: index,
            target: target,
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
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "whether or not to copy settings from the source index (defaults to false)"]
    pub fn copy_settings(mut self, copy_settings: Option<bool>) -> Self {
        self.copy_settings = copy_settings;
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
    #[doc = "The name of the source index to shrink"]
    pub fn index(mut self, index: String) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "The name of the target index to shrink into"]
    pub fn target(mut self, target: String) -> Self {
        self.target = target;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Set the number of active shards to wait for on the shrunken index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl<B> Sender for IndicesShrink<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index = self.index;
            let target = self.target;
            let mut p = String::with_capacity(10usize + index.len() + target.len());
            p.push_str("/");
            p.push_str(index.as_ref());
            p.push_str("/_shrink/");
            p.push_str(target.as_ref());
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "copy_settings")]
                copy_settings: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesSplit<B> {
    client: Elasticsearch,
    body: Option<B>,
    copy_settings: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    target: String,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl<B> IndicesSplit<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String, target: String) -> Self {
        IndicesSplit {
            client,
            index: index,
            target: target,
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
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "whether or not to copy settings from the source index (defaults to false)"]
    pub fn copy_settings(mut self, copy_settings: Option<bool>) -> Self {
        self.copy_settings = copy_settings;
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
    #[doc = "The name of the source index to split"]
    pub fn index(mut self, index: String) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "The name of the target index to split into"]
    pub fn target(mut self, target: String) -> Self {
        self.target = target;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Set the number of active shards to wait for on the shrunken index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl<B> Sender for IndicesSplit<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index = self.index;
            let target = self.target;
            let mut p = String::with_capacity(9usize + index.len() + target.len());
            p.push_str("/");
            p.push_str(index.as_ref());
            p.push_str("/_split/");
            p.push_str(target.as_ref());
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "copy_settings")]
                copy_settings: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesStats {
    client: Elasticsearch,
    completion_fields: Option<Vec<String>>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    fielddata_fields: Option<Vec<String>>,
    fields: Option<Vec<String>>,
    filter_path: Option<Vec<String>>,
    forbid_closed_indices: Option<bool>,
    groups: Option<Vec<String>>,
    human: Option<bool>,
    include_segment_file_sizes: Option<bool>,
    include_unloaded_segments: Option<bool>,
    index: Option<Vec<String>>,
    level: Option<Level>,
    metric: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
    types: Option<Vec<String>>,
}
impl IndicesStats {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesStats {
            client,
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
            index: None,
            level: None,
            metric: None,
            pretty: None,
            source: None,
            types: None,
        }
    }
    #[doc = "A comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards)"]
    pub fn completion_fields(mut self, completion_fields: Option<Vec<String>>) -> Self {
        self.completion_fields = completion_fields;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of fields for `fielddata` index metric (supports wildcards)"]
    pub fn fielddata_fields(mut self, fielddata_fields: Option<Vec<String>>) -> Self {
        self.fielddata_fields = fielddata_fields;
        self
    }
    #[doc = "A comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards)"]
    pub fn fields(mut self, fields: Option<Vec<String>>) -> Self {
        self.fields = fields;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "If set to false stats will also collected from closed indices if explicitly specified or if expand_wildcards expands to closed indices"]
    pub fn forbid_closed_indices(mut self, forbid_closed_indices: Option<bool>) -> Self {
        self.forbid_closed_indices = forbid_closed_indices;
        self
    }
    #[doc = "A comma-separated list of search groups for `search` index metric"]
    pub fn groups(mut self, groups: Option<Vec<String>>) -> Self {
        self.groups = groups;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested)"]
    pub fn include_segment_file_sizes(mut self, include_segment_file_sizes: Option<bool>) -> Self {
        self.include_segment_file_sizes = include_segment_file_sizes;
        self
    }
    #[doc = "If set to true segment stats will include stats for segments that are not currently loaded into memory"]
    pub fn include_unloaded_segments(mut self, include_unloaded_segments: Option<bool>) -> Self {
        self.include_unloaded_segments = include_unloaded_segments;
        self
    }
    #[doc = "A comma-separated list of index names; use `_all` or empty string to perform the operation on all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Return stats aggregated at cluster, index or shard level"]
    pub fn level(mut self, level: Option<Level>) -> Self {
        self.level = level;
        self
    }
    #[doc = "Limit the information returned the specific metrics."]
    pub fn metric(mut self, metric: Option<Vec<String>>) -> Self {
        self.metric = metric;
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
    #[doc = "A comma-separated list of document types for the `indexing` index metric"]
    pub fn types(mut self, types: Option<Vec<String>>) -> Self {
        self.types = types;
        self
    }
}
impl Sender for IndicesStats {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match (&self.index, &self.metric) {
            (Some(index), Some(metric)) => {
                let index_str = index.join(",");
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(9usize + index_str.len() + metric_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_stats/");
                p.push_str(metric_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (None, Some(metric)) => {
                let metric_str = metric.join(",");
                let mut p = String::with_capacity(8usize + metric_str.len());
                p.push_str("/_stats/");
                p.push_str(metric_str.as_ref());
                std::borrow::Cow::Owned(p)
            }
            (Some(index), None) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(8usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_stats");
                std::borrow::Cow::Owned(p)
            }
            (None, None) => std::borrow::Cow::Borrowed("/_stats"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "completion_fields")]
                completion_fields: Option<Vec<String>>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "fielddata_fields")]
                fielddata_fields: Option<Vec<String>>,
                #[serde(rename = "fields")]
                fields: Option<Vec<String>>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "forbid_closed_indices")]
                forbid_closed_indices: Option<bool>,
                #[serde(rename = "groups")]
                groups: Option<Vec<String>>,
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
                source: Option<String>,
                #[serde(rename = "types")]
                types: Option<Vec<String>>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesUnfreeze<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: String,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl<B> IndicesUnfreeze<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IndicesUnfreeze {
            client,
            index: index,
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
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "The name of the index to unfreeze"]
    pub fn index(mut self, index: String) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Sets the number of active shards to wait for before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl<B> Sender for IndicesUnfreeze<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let index = self.index;
            let mut p = String::with_capacity(11usize + index.len());
            p.push_str("/");
            p.push_str(index.as_ref());
            p.push_str("/_unfreeze");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesUpdateAliases<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> IndicesUpdateAliases<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        IndicesUpdateAliases {
            client,
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
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "Request timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl<B> Sender for IndicesUpdateAliases<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = std::borrow::Cow::Borrowed("/_aliases");
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
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesUpgrade<B> {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    only_ancient_segments: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    wait_for_completion: Option<bool>,
}
impl<B> IndicesUpgrade<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        IndicesUpgrade {
            client,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            index: None,
            only_ancient_segments: None,
            pretty: None,
            source: None,
            wait_for_completion: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
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
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names; use `_all` or empty string to perform the operation on all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "If true, only ancient (an older Lucene major release) segments will be upgraded"]
    pub fn only_ancient_segments(mut self, only_ancient_segments: Option<bool>) -> Self {
        self.only_ancient_segments = only_ancient_segments;
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
    #[doc = "Specify whether the request should block until the all segments are upgraded (default: false)"]
    pub fn wait_for_completion(mut self, wait_for_completion: Option<bool>) -> Self {
        self.wait_for_completion = wait_for_completion;
        self
    }
}
impl<B> Sender for IndicesUpgrade<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.index {
            Some(index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(10usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_upgrade");
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_upgrade"),
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "only_ancient_segments")]
                only_ancient_segments: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IndicesValidateQuery<B> {
    client: Elasticsearch,
    all_shards: Option<bool>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<String>,
    body: Option<B>,
    default_operator: Option<DefaultOperator>,
    df: Option<String>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    explain: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    lenient: Option<bool>,
    pretty: Option<bool>,
    q: Option<String>,
    rewrite: Option<bool>,
    source: Option<String>,
    ty: Option<Vec<String>>,
}
impl<B> IndicesValidateQuery<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        IndicesValidateQuery {
            client,
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
            index: None,
            lenient: None,
            pretty: None,
            q: None,
            rewrite: None,
            source: None,
            ty: None,
        }
    }
    #[doc = "Execute validation on all shards instead of one random shard per index"]
    pub fn all_shards(mut self, all_shards: Option<bool>) -> Self {
        self.all_shards = all_shards;
        self
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed (default: false)"]
    pub fn analyze_wildcard(mut self, analyze_wildcard: Option<bool>) -> Self {
        self.analyze_wildcard = analyze_wildcard;
        self
    }
    #[doc = "The analyzer to use for the query string"]
    pub fn analyzer(mut self, analyzer: Option<String>) -> Self {
        self.analyzer = analyzer;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "The default operator for query string query (AND or OR)"]
    pub fn default_operator(mut self, default_operator: Option<DefaultOperator>) -> Self {
        self.default_operator = default_operator;
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string"]
    pub fn df(mut self, df: Option<String>) -> Self {
        self.df = df;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "Return detailed information about the error"]
    pub fn explain(mut self, explain: Option<bool>) -> Self {
        self.explain = explain;
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
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "A comma-separated list of index names to restrict the operation; use `_all` or empty string to perform the operation on all indices"]
    pub fn index(mut self, index: Option<Vec<String>>) -> Self {
        self.index = index;
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored"]
    pub fn lenient(mut self, lenient: Option<bool>) -> Self {
        self.lenient = lenient;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Query in the Lucene query string syntax"]
    pub fn q(mut self, q: Option<String>) -> Self {
        self.q = q;
        self
    }
    #[doc = "Provide a more detailed explanation showing the actual Lucene query that will be executed."]
    pub fn rewrite(mut self, rewrite: Option<bool>) -> Self {
        self.rewrite = rewrite;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "A comma-separated list of document types to restrict the operation; leave empty to perform the operation on all types"]
    pub fn ty(mut self, ty: Option<Vec<String>>) -> Self {
        self.ty = ty;
        self
    }
}
impl<B> Sender for IndicesValidateQuery<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match (&self.index, &self.ty) {
            (Some(index), Some(ty)) => {
                let index_str = index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(18usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_validate/query");
                std::borrow::Cow::Owned(p)
            }
            (None, Some(ty)) => {
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(22usize + ty_str.len());
                p.push_str("/_all/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_validate/query");
                std::borrow::Cow::Owned(p)
            }
            (Some(index), None) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(17usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_validate/query");
                std::borrow::Cow::Owned(p)
            }
            (None, None) => std::borrow::Cow::Borrowed("/_validate/query"),
        };
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "all_shards")]
                all_shards: Option<bool>,
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "analyze_wildcard")]
                analyze_wildcard: Option<bool>,
                #[serde(rename = "analyzer")]
                analyzer: Option<String>,
                #[serde(rename = "default_operator")]
                default_operator: Option<DefaultOperator>,
                #[serde(rename = "df")]
                df: Option<String>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "explain")]
                explain: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient")]
                lenient: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "q")]
                q: Option<String>,
                #[serde(rename = "rewrite")]
                rewrite: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[doc = "Indices APIs"]
pub struct Indices {
    client: Elasticsearch,
}
impl Indices {
    pub fn new(client: Elasticsearch) -> Self {
        Indices { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-analyze.html"]
    pub fn analyze<B>(&self) -> IndicesAnalyze<B>
    where
        B: Serialize,
    {
        IndicesAnalyze::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-clearcache.html"]
    pub fn clear_cache<B>(&self) -> IndicesClearCache<B>
    where
        B: Serialize,
    {
        IndicesClearCache::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html"]
    pub fn close<B>(&self, index: Vec<String>) -> IndicesClose<B>
    where
        B: Serialize,
    {
        IndicesClose::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-create-index.html"]
    pub fn create<B>(&self, index: String) -> IndicesCreate<B>
    where
        B: Serialize,
    {
        IndicesCreate::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-delete-index.html"]
    pub fn delete(&self, index: Vec<String>) -> IndicesDelete {
        IndicesDelete::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn delete_alias(&self, index: Vec<String>, name: Vec<String>) -> IndicesDeleteAlias {
        IndicesDeleteAlias::new(self.client.clone(), index, name)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn delete_template(&self, name: String) -> IndicesDeleteTemplate {
        IndicesDeleteTemplate::new(self.client.clone(), name)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-exists.html"]
    pub fn exists(&self, index: Vec<String>) -> IndicesExists {
        IndicesExists::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn exists_alias(&self, name: Vec<String>) -> IndicesExistsAlias {
        IndicesExistsAlias::new(self.client.clone(), name)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn exists_template(&self, name: Vec<String>) -> IndicesExistsTemplate {
        IndicesExistsTemplate::new(self.client.clone(), name)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-types-exists.html"]
    pub fn exists_type(&self, index: Vec<String>, ty: Vec<String>) -> IndicesExistsType {
        IndicesExistsType::new(self.client.clone(), index, ty)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-flush.html"]
    pub fn flush<B>(&self) -> IndicesFlush<B>
    where
        B: Serialize,
    {
        IndicesFlush::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-flush.html#synced-flush-api"]
    pub fn flush_synced<B>(&self) -> IndicesFlushSynced<B>
    where
        B: Serialize,
    {
        IndicesFlushSynced::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-forcemerge.html"]
    pub fn forcemerge<B>(&self) -> IndicesForcemerge<B>
    where
        B: Serialize,
    {
        IndicesForcemerge::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html"]
    pub fn freeze<B>(&self, index: String) -> IndicesFreeze<B>
    where
        B: Serialize,
    {
        IndicesFreeze::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-index.html"]
    pub fn get(&self, index: Vec<String>) -> IndicesGet {
        IndicesGet::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn get_alias(&self) -> IndicesGetAlias {
        IndicesGetAlias::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-field-mapping.html"]
    pub fn get_field_mapping(&self, fields: Vec<String>) -> IndicesGetFieldMapping {
        IndicesGetFieldMapping::new(self.client.clone(), fields)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-mapping.html"]
    pub fn get_mapping(&self) -> IndicesGetMapping {
        IndicesGetMapping::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-settings.html"]
    pub fn get_settings(&self) -> IndicesGetSettings {
        IndicesGetSettings::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn get_template(&self) -> IndicesGetTemplate {
        IndicesGetTemplate::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html"]
    pub fn get_upgrade(&self) -> IndicesGetUpgrade {
        IndicesGetUpgrade::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html"]
    pub fn open<B>(&self, index: Vec<String>) -> IndicesOpen<B>
    where
        B: Serialize,
    {
        IndicesOpen::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn put_alias<B>(&self, index: Vec<String>, name: String) -> IndicesPutAlias<B>
    where
        B: Serialize,
    {
        IndicesPutAlias::new(self.client.clone(), index, name)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-put-mapping.html"]
    pub fn put_mapping<B>(&self) -> IndicesPutMapping<B>
    where
        B: Serialize,
    {
        IndicesPutMapping::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-update-settings.html"]
    pub fn put_settings<B>(&self) -> IndicesPutSettings<B>
    where
        B: Serialize,
    {
        IndicesPutSettings::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn put_template<B>(&self, name: String) -> IndicesPutTemplate<B>
    where
        B: Serialize,
    {
        IndicesPutTemplate::new(self.client.clone(), name)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-recovery.html"]
    pub fn recovery(&self) -> IndicesRecovery {
        IndicesRecovery::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-refresh.html"]
    pub fn refresh<B>(&self) -> IndicesRefresh<B>
    where
        B: Serialize,
    {
        IndicesRefresh::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-reload-analyzers.html"]
    pub fn reload_search_analyzers<B>(&self, index: Vec<String>) -> IndicesReloadSearchAnalyzers<B>
    where
        B: Serialize,
    {
        IndicesReloadSearchAnalyzers::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-rollover-index.html"]
    pub fn rollover<B>(&self, alias: String) -> IndicesRollover<B>
    where
        B: Serialize,
    {
        IndicesRollover::new(self.client.clone(), alias)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-segments.html"]
    pub fn segments(&self) -> IndicesSegments {
        IndicesSegments::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shards-stores.html"]
    pub fn shard_stores(&self) -> IndicesShardStores {
        IndicesShardStores::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shrink-index.html"]
    pub fn shrink<B>(&self, index: String, target: String) -> IndicesShrink<B>
    where
        B: Serialize,
    {
        IndicesShrink::new(self.client.clone(), index, target)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-split-index.html"]
    pub fn split<B>(&self, index: String, target: String) -> IndicesSplit<B>
    where
        B: Serialize,
    {
        IndicesSplit::new(self.client.clone(), index, target)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-stats.html"]
    pub fn stats(&self) -> IndicesStats {
        IndicesStats::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html"]
    pub fn unfreeze<B>(&self, index: String) -> IndicesUnfreeze<B>
    where
        B: Serialize,
    {
        IndicesUnfreeze::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn update_aliases<B>(&self) -> IndicesUpdateAliases<B>
    where
        B: Serialize,
    {
        IndicesUpdateAliases::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html"]
    pub fn upgrade<B>(&self) -> IndicesUpgrade<B>
    where
        B: Serialize,
    {
        IndicesUpgrade::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-validate.html"]
    pub fn validate_query<B>(&self) -> IndicesValidateQuery<B>
    where
        B: Serialize,
    {
        IndicesValidateQuery::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Indices APIs"]
    pub fn indices(&self) -> Indices {
        Indices::new(self.clone())
    }
}