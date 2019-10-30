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
pub struct IlmDeleteLifecycle {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    policy: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmDeleteLifecycle {
    pub fn new(client: Elasticsearch, policy: String) -> Self {
        IlmDeleteLifecycle {
            client,
            policy: policy,
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
impl Sender for IlmDeleteLifecycle {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ilm/policy/{policy}";
        let method = HttpMethod::Delete;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IlmExplainLifecycle {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmExplainLifecycle {
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IlmExplainLifecycle {
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
impl Sender for IlmExplainLifecycle {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/{index}/_ilm/explain";
        let method = HttpMethod::Get;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IlmGetLifecycle {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    policy: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmGetLifecycle {
    pub fn new(client: Elasticsearch) -> Self {
        IlmGetLifecycle {
            client,
            error_trace: None,
            filter_path: None,
            human: None,
            policy: None,
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
impl Sender for IlmGetLifecycle {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ilm/policy/{policy}";
        let method = HttpMethod::Get;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IlmGetStatus {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmGetStatus {
    pub fn new(client: Elasticsearch) -> Self {
        IlmGetStatus {
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
impl Sender for IlmGetStatus {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ilm/status";
        let method = HttpMethod::Get;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IlmMoveToStep<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> IlmMoveToStep<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IlmMoveToStep {
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
impl<B> Sender for IlmMoveToStep<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ilm/move/{index}";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IlmPutLifecycle<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    policy: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> IlmPutLifecycle<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, policy: String) -> Self {
        IlmPutLifecycle {
            client,
            policy: policy,
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
impl<B> Sender for IlmPutLifecycle<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ilm/policy/{policy}";
        let method = HttpMethod::Put;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IlmRemovePolicy<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> IlmRemovePolicy<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IlmRemovePolicy {
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
impl<B> Sender for IlmRemovePolicy<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/{index}/_ilm/remove";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IlmRetry<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> IlmRetry<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IlmRetry {
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
impl<B> Sender for IlmRetry<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/{index}/_ilm/retry";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IlmStart<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> IlmStart<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        IlmStart {
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
impl<B> Sender for IlmStart<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ilm/start";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct IlmStop<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> IlmStop<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        IlmStop {
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
impl<B> Sender for IlmStop<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ilm/stop";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[doc = "Ilm APIs"]
pub struct Ilm {
    client: Elasticsearch,
}
impl Ilm {
    pub fn new(client: Elasticsearch) -> Self {
        Ilm { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-delete-lifecycle.html"]
    pub fn delete_lifecycle(&self, policy: String) -> IlmDeleteLifecycle {
        IlmDeleteLifecycle::new(self.client.clone(), policy)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-explain-lifecycle.html"]
    pub fn explain_lifecycle(&self, index: String) -> IlmExplainLifecycle {
        IlmExplainLifecycle::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-lifecycle.html"]
    pub fn get_lifecycle(&self) -> IlmGetLifecycle {
        IlmGetLifecycle::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-status.html"]
    pub fn get_status(&self) -> IlmGetStatus {
        IlmGetStatus::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-move-to-step.html"]
    pub fn move_to_step<B>(&self, index: String) -> IlmMoveToStep<B>
    where
        B: Serialize,
    {
        IlmMoveToStep::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-put-lifecycle.html"]
    pub fn put_lifecycle<B>(&self, policy: String) -> IlmPutLifecycle<B>
    where
        B: Serialize,
    {
        IlmPutLifecycle::new(self.client.clone(), policy)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-remove-policy.html"]
    pub fn remove_policy<B>(&self, index: String) -> IlmRemovePolicy<B>
    where
        B: Serialize,
    {
        IlmRemovePolicy::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-retry-policy.html"]
    pub fn retry<B>(&self, index: String) -> IlmRetry<B>
    where
        B: Serialize,
    {
        IlmRetry::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-start.html"]
    pub fn start<B>(&self) -> IlmStart<B>
    where
        B: Serialize,
    {
        IlmStart::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-stop.html"]
    pub fn stop<B>(&self) -> IlmStop<B>
    where
        B: Serialize,
    {
        IlmStop::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Ilm APIs"]
    pub fn ilm(&self) -> Ilm {
        Ilm::new(self.clone())
    }
}
