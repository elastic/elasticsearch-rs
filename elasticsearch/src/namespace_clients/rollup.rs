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
pub struct RollupDeleteJob {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl RollupDeleteJob {
    pub fn new(client: Elasticsearch, id: String) -> Self {
        RollupDeleteJob {
            client,
            id: id,
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
impl Sender for RollupDeleteJob {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_rollup/job/{id}";
        let method = HttpMethod::Delete;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct RollupGetJobs {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl RollupGetJobs {
    pub fn new(client: Elasticsearch) -> Self {
        RollupGetJobs {
            client,
            error_trace: None,
            filter_path: None,
            human: None,
            id: None,
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
impl Sender for RollupGetJobs {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_rollup/job/{id}";
        let method = HttpMethod::Get;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct RollupGetRollupCaps {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl RollupGetRollupCaps {
    pub fn new(client: Elasticsearch) -> Self {
        RollupGetRollupCaps {
            client,
            error_trace: None,
            filter_path: None,
            human: None,
            id: None,
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
impl Sender for RollupGetRollupCaps {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_rollup/data/{id}";
        let method = HttpMethod::Get;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct RollupGetRollupIndexCaps {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl RollupGetRollupIndexCaps {
    pub fn new(client: Elasticsearch, index: String) -> Self {
        RollupGetRollupIndexCaps {
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
impl Sender for RollupGetRollupIndexCaps {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/{index}/_rollup/data";
        let method = HttpMethod::Get;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct RollupPutJob<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> RollupPutJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, id: String) -> Self {
        RollupPutJob {
            client,
            id: id,
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
impl<B> Sender for RollupPutJob<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_rollup/job/{id}";
        let method = HttpMethod::Put;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct RollupRollupSearch<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Vec<String>,
    pretty: Option<bool>,
    rest_total_hits_as_int: Option<bool>,
    source: Option<String>,
    ty: Option<String>,
    typed_keys: Option<bool>,
}
impl<B> RollupRollupSearch<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        RollupRollupSearch {
            client,
            index: index,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            rest_total_hits_as_int: None,
            source: None,
            ty: None,
            typed_keys: None,
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
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Indicates whether hits.total should be rendered as an integer or an object in the rest search response"]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: Option<bool>) -> Self {
        self.rest_total_hits_as_int = rest_total_hits_as_int;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response"]
    pub fn typed_keys(mut self, typed_keys: Option<bool>) -> Self {
        self.typed_keys = typed_keys;
        self
    }
}
impl<B> Sender for RollupRollupSearch<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/{index}/_rollup_search";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "rest_total_hits_as_int")]
                rest_total_hits_as_int: Option<bool>,
                #[serde(rename = "typed_keys")]
                typed_keys: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                typed_keys: self.typed_keys,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct RollupStartJob<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> RollupStartJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, id: String) -> Self {
        RollupStartJob {
            client,
            id: id,
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
impl<B> Sender for RollupStartJob<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_rollup/job/{id}/_start";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct RollupStopJob<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_completion: Option<bool>,
}
impl<B> RollupStopJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, id: String) -> Self {
        RollupStopJob {
            client,
            id: id,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_completion: None,
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
    #[doc = "Block for (at maximum) the specified duration while waiting for the job to stop.  Defaults to 30s."]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "True if the API should block until the job has fully stopped, false if should be executed async. Defaults to false."]
    pub fn wait_for_completion(mut self, wait_for_completion: Option<bool>) -> Self {
        self.wait_for_completion = wait_for_completion;
        self
    }
}
impl<B> Sender for RollupStopJob<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_rollup/job/{id}/_stop";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                timeout: self.timeout,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[doc = "Rollup APIs"]
pub struct Rollup {
    client: Elasticsearch,
}
impl Rollup {
    pub fn new(client: Elasticsearch) -> Self {
        Rollup { client }
    }
    #[doc = ""]
    pub fn delete_job(&self, id: String) -> RollupDeleteJob {
        RollupDeleteJob::new(self.client.clone(), id)
    }
    #[doc = ""]
    pub fn get_jobs(&self) -> RollupGetJobs {
        RollupGetJobs::new(self.client.clone())
    }
    #[doc = ""]
    pub fn get_rollup_caps(&self) -> RollupGetRollupCaps {
        RollupGetRollupCaps::new(self.client.clone())
    }
    #[doc = ""]
    pub fn get_rollup_index_caps(&self, index: String) -> RollupGetRollupIndexCaps {
        RollupGetRollupIndexCaps::new(self.client.clone(), index)
    }
    #[doc = ""]
    pub fn put_job<B>(&self, id: String) -> RollupPutJob<B>
    where
        B: Serialize,
    {
        RollupPutJob::new(self.client.clone(), id)
    }
    #[doc = ""]
    pub fn rollup_search<B>(&self, index: Vec<String>) -> RollupRollupSearch<B>
    where
        B: Serialize,
    {
        RollupRollupSearch::new(self.client.clone(), index)
    }
    #[doc = ""]
    pub fn start_job<B>(&self, id: String) -> RollupStartJob<B>
    where
        B: Serialize,
    {
        RollupStartJob::new(self.client.clone(), id)
    }
    #[doc = ""]
    pub fn stop_job<B>(&self, id: String) -> RollupStopJob<B>
    where
        B: Serialize,
    {
        RollupStopJob::new(self.client.clone(), id)
    }
}
impl Elasticsearch {
    #[doc = "Rollup APIs"]
    pub fn rollup(&self) -> Rollup {
        Rollup::new(self.clone())
    }
}
