

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct IngestDeletePipelineRequest<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct IngestDeletePipelineRequestBuilder<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> IngestDeletePipelineRequestBuilder<'a> {
    pub fn build(&self) -> IngestDeletePipelineRequest<'a> {
        IngestDeletePipelineRequest {
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
pub struct IngestGetPipelineRequest<'a> {
    master_timeout: &'a String,
}
pub struct IngestGetPipelineRequestBuilder<'a> {
    master_timeout: &'a String,
}
impl<'a> IngestGetPipelineRequestBuilder<'a> {
    pub fn build(&self) -> IngestGetPipelineRequest<'a> {
        IngestGetPipelineRequest {
            master_timeout: self.master_timeout,
        }
    }
}
pub struct IngestProcessorGrokRequest<'a> {}
pub struct IngestProcessorGrokRequestBuilder<'a> {}
impl<'a> IngestProcessorGrokRequestBuilder<'a> {
    pub fn build(&self) -> IngestProcessorGrokRequest<'a> {
        IngestProcessorGrokRequest {}
    }
}
pub struct IngestPutPipelineRequest<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct IngestPutPipelineRequestBuilder<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> IngestPutPipelineRequestBuilder<'a> {
    pub fn build(&self) -> IngestPutPipelineRequest<'a> {
        IngestPutPipelineRequest {
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
pub struct IngestSimulateRequest<'a> {
    verbose: Option<&'a bool>,
}
pub struct IngestSimulateRequestBuilder<'a> {
    verbose: Option<&'a bool>,
}
impl<'a> IngestSimulateRequestBuilder<'a> {
    pub fn build(&self) -> IngestSimulateRequest<'a> {
        IngestSimulateRequest {
            verbose: self.verbose,
        }
    }
}
#[doc = "Ingest APIs"]
pub struct IngestNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IngestNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IngestNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/delete-pipeline-api.html"]
    pub fn delete_pipeline(&self, request: &IngestDeletePipelineRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ingest/pipeline/{id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-pipeline-api.html"]
    pub fn get_pipeline(&self, request: &IngestGetPipelineRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ingest/pipeline")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/grok-processor.html#grok-processor-rest-get"]
    pub fn processor_grok(&self, request: &IngestProcessorGrokRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ingest/processor/grok")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/put-pipeline-api.html"]
    pub fn put_pipeline(&self, request: &IngestPutPipelineRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_ingest/pipeline/{id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/simulate-pipeline-api.html"]
    pub fn simulate(&self, request: &IngestSimulateRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ingest/pipeline/_simulate")
    }
}
impl ElasticsearchClient {
    #[doc = "Ingest APIs"]
    pub fn ingest(&self) -> IngestNamespaceClient {
        IngestNamespaceClient::new(self)
    }
}
