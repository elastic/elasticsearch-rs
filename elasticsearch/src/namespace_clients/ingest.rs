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
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct IngestDeletePipeline {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: Option<String>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl IngestDeletePipeline {
    pub fn new(client: Elasticsearch) -> Self {
        IngestDeletePipeline {
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for IngestDeletePipeline {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct IngestGetPipeline {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: Option<String>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IngestGetPipeline {
    pub fn new(client: Elasticsearch) -> Self {
        IngestGetPipeline {
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
impl Sender for IngestGetPipeline {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct IngestProcessorGrok {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IngestProcessorGrok {
    pub fn new(client: Elasticsearch) -> Self {
        IngestProcessorGrok {
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
impl Sender for IngestProcessorGrok {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct IngestPutPipeline {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: Option<String>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl IngestPutPipeline {
    pub fn new(client: Elasticsearch) -> Self {
        IngestPutPipeline {
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for IngestPutPipeline {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct IngestSimulate {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    verbose: Option<bool>,
}
impl IngestSimulate {
    pub fn new(client: Elasticsearch) -> Self {
        IngestSimulate {
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
    #[doc = "Verbose mode. Display data output for each processor in executed pipeline"]
    pub fn verbose(mut self, verbose: Option<bool>) -> Self {
        self.verbose = verbose;
        self
    }
}
impl Sender for IngestSimulate {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[doc = "Ingest APIs"]
pub struct Ingest {
    client: Elasticsearch,
}
impl Ingest {
    pub fn new(client: Elasticsearch) -> Self {
        Ingest { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/delete-pipeline-api.html"]
    pub fn delete_pipeline(&self) -> IngestDeletePipeline {
        IngestDeletePipeline::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-pipeline-api.html"]
    pub fn get_pipeline(&self) -> IngestGetPipeline {
        IngestGetPipeline::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/grok-processor.html#grok-processor-rest-get"]
    pub fn processor_grok(&self) -> IngestProcessorGrok {
        IngestProcessorGrok::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/put-pipeline-api.html"]
    pub fn put_pipeline(&self) -> IngestPutPipeline {
        IngestPutPipeline::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/simulate-pipeline-api.html"]
    pub fn simulate(&self) -> IngestSimulate {
        IngestSimulate::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Ingest APIs"]
    pub fn ingest(&self) -> Ingest {
        Ingest::new(self.clone())
    }
}