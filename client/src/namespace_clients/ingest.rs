use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
#[doc = "Ingest APIs"]
pub struct IngestNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl IngestNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IngestNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/delete-pipeline-api.html"]
    pub fn delete_pipeline(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ingest/pipeline/{id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-pipeline-api.html"]
    pub fn get_pipeline(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ingest/pipeline")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/grok-processor.html#grok-processor-rest-get"]
    pub fn processor_grok(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ingest/processor/grok")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/put-pipeline-api.html"]
    pub fn put_pipeline(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_ingest/pipeline/{id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/simulate-pipeline-api.html"]
    pub fn simulate(&self) -> Result<Response> {
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
