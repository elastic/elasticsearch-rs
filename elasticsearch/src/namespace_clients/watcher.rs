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
pub struct WatcherAckWatch<B> {
    client: Elasticsearch,
    action_id: Option<Vec<String>>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    watch_id: String,
}
impl<B> WatcherAckWatch<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, watch_id: String) -> Self {
        WatcherAckWatch {
            client,
            watch_id: watch_id,
            action_id: None,
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
impl<B> Sender for WatcherAckWatch<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_watcher/watch/{watch_id}/_ack";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct WatcherActivateWatch<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    watch_id: String,
}
impl<B> WatcherActivateWatch<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, watch_id: String) -> Self {
        WatcherActivateWatch {
            client,
            watch_id: watch_id,
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
impl<B> Sender for WatcherActivateWatch<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_watcher/watch/{watch_id}/_activate";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct WatcherDeactivateWatch<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    watch_id: String,
}
impl<B> WatcherDeactivateWatch<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, watch_id: String) -> Self {
        WatcherDeactivateWatch {
            client,
            watch_id: watch_id,
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
impl<B> Sender for WatcherDeactivateWatch<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_watcher/watch/{watch_id}/_deactivate";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct WatcherDeleteWatch {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl WatcherDeleteWatch {
    pub fn new(client: Elasticsearch, id: String) -> Self {
        WatcherDeleteWatch {
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
impl Sender for WatcherDeleteWatch {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_watcher/watch/{id}";
        let method = HttpMethod::Delete;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct WatcherExecuteWatch<B> {
    client: Elasticsearch,
    body: Option<B>,
    debug: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> WatcherExecuteWatch<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        WatcherExecuteWatch {
            client,
            body: None,
            debug: None,
            error_trace: None,
            filter_path: None,
            human: None,
            id: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "indicates whether the watch should execute in debug mode"]
    pub fn debug(mut self, debug: Option<bool>) -> Self {
        self.debug = debug;
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
impl<B> Sender for WatcherExecuteWatch<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_watcher/watch/{id}/_execute";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "debug")]
                debug: Option<bool>,
            }
            let query_params = QueryParamsStruct { debug: self.debug };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct WatcherGetWatch {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl WatcherGetWatch {
    pub fn new(client: Elasticsearch, id: String) -> Self {
        WatcherGetWatch {
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
impl Sender for WatcherGetWatch {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_watcher/watch/{id}";
        let method = HttpMethod::Get;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct WatcherPutWatch<B> {
    client: Elasticsearch,
    active: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    pretty: Option<bool>,
    source: Option<String>,
    version: Option<i64>,
}
impl<B> WatcherPutWatch<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, id: String) -> Self {
        WatcherPutWatch {
            client,
            id: id,
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
    pub fn active(mut self, active: Option<bool>) -> Self {
        self.active = active;
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
    #[doc = "only update the watch if the last operation that has changed the watch has the specified primary term"]
    pub fn if_primary_term(mut self, if_primary_term: Option<i64>) -> Self {
        self.if_primary_term = if_primary_term;
        self
    }
    #[doc = "only update the watch if the last operation that has changed the watch has the specified sequence number"]
    pub fn if_seq_no(mut self, if_seq_no: Option<i64>) -> Self {
        self.if_seq_no = if_seq_no;
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
    #[doc = "Explicit version number for concurrency control"]
    pub fn version(mut self, version: Option<i64>) -> Self {
        self.version = version;
        self
    }
}
impl<B> Sender for WatcherPutWatch<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_watcher/watch/{id}";
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "active")]
                active: Option<bool>,
                #[serde(rename = "if_primary_term")]
                if_primary_term: Option<i64>,
                #[serde(rename = "if_seq_no")]
                if_seq_no: Option<i64>,
                #[serde(rename = "version")]
                version: Option<i64>,
            }
            let query_params = QueryParamsStruct {
                active: self.active,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                version: self.version,
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
pub struct WatcherStart<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> WatcherStart<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        WatcherStart {
            client,
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
impl<B> Sender for WatcherStart<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_watcher/_start";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct WatcherStats {
    client: Elasticsearch,
    emit_stacktraces: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    metric: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl WatcherStats {
    pub fn new(client: Elasticsearch) -> Self {
        WatcherStats {
            client,
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
    pub fn emit_stacktraces(mut self, emit_stacktraces: Option<bool>) -> Self {
        self.emit_stacktraces = emit_stacktraces;
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
    #[doc = "Controls what additional stat metrics should be include in the response"]
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
}
impl Sender for WatcherStats {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_watcher/stats";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "emit_stacktraces")]
                emit_stacktraces: Option<bool>,
                #[serde(rename = "metric")]
                metric: Option<Vec<String>>,
            }
            let query_params = QueryParamsStruct {
                emit_stacktraces: self.emit_stacktraces,
                metric: self.metric,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct WatcherStop<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> WatcherStop<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        WatcherStop {
            client,
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
impl<B> Sender for WatcherStop<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_watcher/_stop";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[doc = "Watcher APIs"]
pub struct Watcher {
    client: Elasticsearch,
}
impl Watcher {
    pub fn new(client: Elasticsearch) -> Self {
        Watcher { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-ack-watch.html"]
    pub fn ack_watch<B>(&self, watch_id: String) -> WatcherAckWatch<B>
    where
        B: Serialize,
    {
        WatcherAckWatch::new(self.client.clone(), watch_id)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-activate-watch.html"]
    pub fn activate_watch<B>(&self, watch_id: String) -> WatcherActivateWatch<B>
    where
        B: Serialize,
    {
        WatcherActivateWatch::new(self.client.clone(), watch_id)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-deactivate-watch.html"]
    pub fn deactivate_watch<B>(&self, watch_id: String) -> WatcherDeactivateWatch<B>
    where
        B: Serialize,
    {
        WatcherDeactivateWatch::new(self.client.clone(), watch_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-delete-watch.html"]
    pub fn delete_watch(&self, id: String) -> WatcherDeleteWatch {
        WatcherDeleteWatch::new(self.client.clone(), id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-execute-watch.html"]
    pub fn execute_watch<B>(&self) -> WatcherExecuteWatch<B>
    where
        B: Serialize,
    {
        WatcherExecuteWatch::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-get-watch.html"]
    pub fn get_watch(&self, id: String) -> WatcherGetWatch {
        WatcherGetWatch::new(self.client.clone(), id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-put-watch.html"]
    pub fn put_watch<B>(&self, id: String) -> WatcherPutWatch<B>
    where
        B: Serialize,
    {
        WatcherPutWatch::new(self.client.clone(), id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-start.html"]
    pub fn start<B>(&self) -> WatcherStart<B>
    where
        B: Serialize,
    {
        WatcherStart::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-stats.html"]
    pub fn stats(&self) -> WatcherStats {
        WatcherStats::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-stop.html"]
    pub fn stop<B>(&self) -> WatcherStop<B>
    where
        B: Serialize,
    {
        WatcherStop::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Watcher APIs"]
    pub fn watcher(&self) -> Watcher {
        Watcher::new(self.clone())
    }
}
