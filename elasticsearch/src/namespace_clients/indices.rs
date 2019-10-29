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
use super::super::client::Elasticsearch;
use super::super::enums::*;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::error::ElasticsearchError;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
#[derive(Default)]
pub struct IndicesAnalyze {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IndicesAnalyze {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesAnalyze {
            client,
            ..Default::default()
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
impl Sender for IndicesAnalyze {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "index")]
                index: Option<String>,
            }
            let query_params = QueryParamsStruct { index: self.index };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesClearCache {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
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
impl IndicesClearCache {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesClearCache {
            client,
            ..Default::default()
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
impl Sender for IndicesClearCache {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "fielddata")]
                fielddata: Option<bool>,
                #[serde(rename = "fields")]
                fields: Option<Vec<String>>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "index")]
                index: Option<Vec<String>>,
                #[serde(rename = "query")]
                query: Option<bool>,
                #[serde(rename = "request")]
                request: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                fielddata: self.fielddata,
                fields: self.fields,
                ignore_unavailable: self.ignore_unavailable,
                index: self.index,
                query: self.query,
                request: self.request,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesClose {
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
    wait_for_active_shards: Option<String>,
}
impl IndicesClose {
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        IndicesClose {
            client,
            index: index,
            ..Default::default()
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
impl Sender for IndicesClose {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesCreate {
    client: Elasticsearch,
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
impl IndicesCreate {
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IndicesCreate {
            client,
            index: index,
            ..Default::default()
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
    #[doc = "Whether a type should be expected in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: Option<bool>) -> Self {
        self.include_type_name = include_type_name;
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
impl Sender for IndicesCreate {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                include_type_name: self.include_type_name,
                master_timeout: self.master_timeout,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                master_timeout: self.master_timeout,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                master_timeout: self.master_timeout,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_defaults")]
                include_defaults: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                flat_settings: self.flat_settings,
                ignore_unavailable: self.ignore_unavailable,
                include_defaults: self.include_defaults,
                local: self.local,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
impl Sender for IndicesExistsAlias {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                flat_settings: self.flat_settings,
                local: self.local,
                master_timeout: self.master_timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
impl Sender for IndicesExistsType {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesFlush {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
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
impl IndicesFlush {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesFlush {
            client,
            ..Default::default()
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
impl Sender for IndicesFlush {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "wait_if_ongoing")]
                wait_if_ongoing: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                force: self.force,
                ignore_unavailable: self.ignore_unavailable,
                wait_if_ongoing: self.wait_if_ongoing,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesFlushSynced {
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
impl IndicesFlushSynced {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesFlushSynced {
            client,
            ..Default::default()
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
impl Sender for IndicesFlushSynced {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesForcemerge {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
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
impl IndicesForcemerge {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesForcemerge {
            client,
            ..Default::default()
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
impl Sender for IndicesForcemerge {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "flush")]
                flush: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "max_num_segments")]
                max_num_segments: Option<i64>,
                #[serde(rename = "only_expunge_deletes")]
                only_expunge_deletes: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                flush: self.flush,
                ignore_unavailable: self.ignore_unavailable,
                max_num_segments: self.max_num_segments,
                only_expunge_deletes: self.only_expunge_deletes,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesFreeze {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
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
impl IndicesFreeze {
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IndicesFreeze {
            client,
            index: index,
            ..Default::default()
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
impl Sender for IndicesFreeze {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
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
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                flat_settings: self.flat_settings,
                ignore_unavailable: self.ignore_unavailable,
                include_defaults: self.include_defaults,
                include_type_name: self.include_type_name,
                local: self.local,
                master_timeout: self.master_timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
impl Sender for IndicesGetAlias {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
impl Sender for IndicesGetFieldMapping {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_defaults")]
                include_defaults: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                include_defaults: self.include_defaults,
                include_type_name: self.include_type_name,
                local: self.local,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
impl Sender for IndicesGetMapping {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                include_type_name: self.include_type_name,
                local: self.local,
                master_timeout: self.master_timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
impl Sender for IndicesGetSettings {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_defaults")]
                include_defaults: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                flat_settings: self.flat_settings,
                ignore_unavailable: self.ignore_unavailable,
                include_defaults: self.include_defaults,
                local: self.local,
                master_timeout: self.master_timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                flat_settings: self.flat_settings,
                include_type_name: self.include_type_name,
                local: self.local,
                master_timeout: self.master_timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesOpen {
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
    wait_for_active_shards: Option<String>,
}
impl IndicesOpen {
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        IndicesOpen {
            client,
            index: index,
            ..Default::default()
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
impl Sender for IndicesOpen {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesPutAlias {
    client: Elasticsearch,
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
impl IndicesPutAlias {
    pub fn new(client: Elasticsearch, index: Vec<String>, name: String) -> Self {
        IndicesPutAlias {
            client,
            index: index,
            name: name,
            ..Default::default()
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
impl Sender for IndicesPutAlias {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                master_timeout: self.master_timeout,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesPutMapping {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_type_name: Option<bool>,
    index: Vec<String>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    ty: Option<String>,
}
impl IndicesPutMapping {
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        IndicesPutMapping {
            client,
            index: index,
            ..Default::default()
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
    #[doc = "Whether a type should be expected in the body of the mappings."]
    pub fn include_type_name(mut self, include_type_name: Option<bool>) -> Self {
        self.include_type_name = include_type_name;
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
impl Sender for IndicesPutMapping {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                include_type_name: self.include_type_name,
                master_timeout: self.master_timeout,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesPutSettings {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
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
impl IndicesPutSettings {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesPutSettings {
            client,
            ..Default::default()
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
impl Sender for IndicesPutSettings {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "preserve_existing")]
                preserve_existing: Option<bool>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                flat_settings: self.flat_settings,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                preserve_existing: self.preserve_existing,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesPutTemplate {
    client: Elasticsearch,
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
impl IndicesPutTemplate {
    pub fn new(client: Elasticsearch, name: String) -> Self {
        IndicesPutTemplate {
            client,
            name: name,
            ..Default::default()
        }
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
impl Sender for IndicesPutTemplate {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "create")]
                create: Option<bool>,
                #[serde(rename = "flat_settings")]
                flat_settings: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "order")]
                order: Option<i64>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                create: self.create,
                flat_settings: self.flat_settings,
                include_type_name: self.include_type_name,
                master_timeout: self.master_timeout,
                order: self.order,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "active_only")]
                active_only: Option<bool>,
                #[serde(rename = "detailed")]
                detailed: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                active_only: self.active_only,
                detailed: self.detailed,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesRefresh {
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
impl IndicesRefresh {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesRefresh {
            client,
            ..Default::default()
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
impl Sender for IndicesRefresh {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesReloadSearchAnalyzers {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Vec<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IndicesReloadSearchAnalyzers {
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        IndicesReloadSearchAnalyzers {
            client,
            index: index,
            ..Default::default()
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
impl Sender for IndicesReloadSearchAnalyzers {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesRollover {
    client: Elasticsearch,
    alias: String,
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
impl IndicesRollover {
    pub fn new(client: Elasticsearch, alias: String) -> Self {
        IndicesRollover {
            client,
            alias: alias,
            ..Default::default()
        }
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
impl Sender for IndicesRollover {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "dry_run")]
                dry_run: Option<bool>,
                #[serde(rename = "include_type_name")]
                include_type_name: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                dry_run: self.dry_run,
                include_type_name: self.include_type_name,
                master_timeout: self.master_timeout,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "verbose")]
                verbose: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                verbose: self.verbose,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "status")]
                status: Option<Vec<String>>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                status: self.status,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesShrink {
    client: Elasticsearch,
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
impl IndicesShrink {
    pub fn new(client: Elasticsearch, index: String, target: String) -> Self {
        IndicesShrink {
            client,
            index: index,
            target: target,
            ..Default::default()
        }
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
    #[doc = "Set the number of active shards to wait for on the shrunken index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl Sender for IndicesShrink {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "copy_settings")]
                copy_settings: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                copy_settings: self.copy_settings,
                master_timeout: self.master_timeout,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesSplit {
    client: Elasticsearch,
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
impl IndicesSplit {
    pub fn new(client: Elasticsearch, index: String, target: String) -> Self {
        IndicesSplit {
            client,
            index: index,
            target: target,
            ..Default::default()
        }
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
    #[doc = "Set the number of active shards to wait for on the shrunken index before the operation returns."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
}
impl Sender for IndicesSplit {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "copy_settings")]
                copy_settings: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                copy_settings: self.copy_settings,
                master_timeout: self.master_timeout,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
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
            ..Default::default()
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
    #[doc = "Return stats aggregated at cluster, index or shard level"]
    pub fn level(mut self, level: Option<Level>) -> Self {
        self.level = level;
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
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "completion_fields")]
                completion_fields: Option<Vec<String>>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "fielddata_fields")]
                fielddata_fields: Option<Vec<String>>,
                #[serde(rename = "fields")]
                fields: Option<Vec<String>>,
                #[serde(rename = "forbid_closed_indices")]
                forbid_closed_indices: Option<bool>,
                #[serde(rename = "groups")]
                groups: Option<Vec<String>>,
                #[serde(rename = "include_segment_file_sizes")]
                include_segment_file_sizes: Option<bool>,
                #[serde(rename = "include_unloaded_segments")]
                include_unloaded_segments: Option<bool>,
                #[serde(rename = "level")]
                level: Option<Level>,
                #[serde(rename = "types")]
                types: Option<Vec<String>>,
            }
            let query_params = QueryParamsStruct {
                completion_fields: self.completion_fields,
                expand_wildcards: self.expand_wildcards,
                fielddata_fields: self.fielddata_fields,
                fields: self.fields,
                forbid_closed_indices: self.forbid_closed_indices,
                groups: self.groups,
                include_segment_file_sizes: self.include_segment_file_sizes,
                include_unloaded_segments: self.include_unloaded_segments,
                level: self.level,
                types: self.types,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesUnfreeze {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
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
impl IndicesUnfreeze {
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IndicesUnfreeze {
            client,
            index: index,
            ..Default::default()
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
impl Sender for IndicesUnfreeze {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_active_shards")]
                wait_for_active_shards: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesUpdateAliases {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl IndicesUpdateAliases {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesUpdateAliases {
            client,
            ..Default::default()
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
impl Sender for IndicesUpdateAliases {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                master_timeout: self.master_timeout,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesUpgrade {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
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
impl IndicesUpgrade {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesUpgrade {
            client,
            ..Default::default()
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
impl Sender for IndicesUpgrade {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_indices")]
                allow_no_indices: Option<bool>,
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "only_ancient_segments")]
                only_ancient_segments: Option<bool>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_indices: self.allow_no_indices,
                expand_wildcards: self.expand_wildcards,
                ignore_unavailable: self.ignore_unavailable,
                only_ancient_segments: self.only_ancient_segments,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IndicesValidateQuery {
    client: Elasticsearch,
    all_shards: Option<bool>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<String>,
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
impl IndicesValidateQuery {
    pub fn new(client: Elasticsearch) -> Self {
        IndicesValidateQuery {
            client,
            ..Default::default()
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
}
impl Sender for IndicesValidateQuery {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
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
                #[serde(rename = "expand_wildcards")]
                expand_wildcards: Option<ExpandWildcards>,
                #[serde(rename = "explain")]
                explain: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "lenient")]
                lenient: Option<bool>,
                #[serde(rename = "q")]
                q: Option<String>,
                #[serde(rename = "rewrite")]
                rewrite: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                all_shards: self.all_shards,
                allow_no_indices: self.allow_no_indices,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                default_operator: self.default_operator,
                df: self.df,
                expand_wildcards: self.expand_wildcards,
                explain: self.explain,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                q: self.q,
                rewrite: self.rewrite,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
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
    pub fn analyze(&self) -> IndicesAnalyze {
        IndicesAnalyze::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-clearcache.html"]
    pub fn clear_cache(&self) -> IndicesClearCache {
        IndicesClearCache::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html"]
    pub fn close(&self, index: Vec<String>) -> IndicesClose {
        IndicesClose::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-create-index.html"]
    pub fn create(&self, index: String) -> IndicesCreate {
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
    pub fn flush(&self) -> IndicesFlush {
        IndicesFlush::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-flush.html#synced-flush-api"]
    pub fn flush_synced(&self) -> IndicesFlushSynced {
        IndicesFlushSynced::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-forcemerge.html"]
    pub fn forcemerge(&self) -> IndicesForcemerge {
        IndicesForcemerge::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html"]
    pub fn freeze(&self, index: String) -> IndicesFreeze {
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
    pub fn open(&self, index: Vec<String>) -> IndicesOpen {
        IndicesOpen::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn put_alias(&self, index: Vec<String>, name: String) -> IndicesPutAlias {
        IndicesPutAlias::new(self.client.clone(), index, name)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-put-mapping.html"]
    pub fn put_mapping(&self, index: Vec<String>) -> IndicesPutMapping {
        IndicesPutMapping::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-update-settings.html"]
    pub fn put_settings(&self) -> IndicesPutSettings {
        IndicesPutSettings::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn put_template(&self, name: String) -> IndicesPutTemplate {
        IndicesPutTemplate::new(self.client.clone(), name)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-recovery.html"]
    pub fn recovery(&self) -> IndicesRecovery {
        IndicesRecovery::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-refresh.html"]
    pub fn refresh(&self) -> IndicesRefresh {
        IndicesRefresh::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-reload-analyzers.html"]
    pub fn reload_search_analyzers(&self, index: Vec<String>) -> IndicesReloadSearchAnalyzers {
        IndicesReloadSearchAnalyzers::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-rollover-index.html"]
    pub fn rollover(&self, alias: String) -> IndicesRollover {
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
    pub fn shrink(&self, index: String, target: String) -> IndicesShrink {
        IndicesShrink::new(self.client.clone(), index, target)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-split-index.html"]
    pub fn split(&self, index: String, target: String) -> IndicesSplit {
        IndicesSplit::new(self.client.clone(), index, target)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-stats.html"]
    pub fn stats(&self) -> IndicesStats {
        IndicesStats::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html"]
    pub fn unfreeze(&self, index: String) -> IndicesUnfreeze {
        IndicesUnfreeze::new(self.client.clone(), index)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn update_aliases(&self) -> IndicesUpdateAliases {
        IndicesUpdateAliases::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html"]
    pub fn upgrade(&self) -> IndicesUpgrade {
        IndicesUpgrade::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-validate.html"]
    pub fn validate_query(&self) -> IndicesValidateQuery {
        IndicesValidateQuery::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Indices APIs"]
    pub fn indices(&self) -> Indices {
        Indices::new(self.clone())
    }
}