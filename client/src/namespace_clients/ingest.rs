

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct IngestDeletePipelineRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    master_timeout: &'a str,
    timeout: &'a str,
}
impl<'a> IngestDeletePipelineRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        IngestDeletePipelineRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IngestDeletePipelineRequestBuilder<'a> {
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
pub struct IngestGetPipelineRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    master_timeout: &'a str,
}
impl<'a> IngestGetPipelineRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        IngestGetPipelineRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IngestGetPipelineRequestBuilder<'a> {
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
pub struct IngestProcessorGrokRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> IngestProcessorGrokRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        IngestProcessorGrokRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IngestProcessorGrokRequestBuilder<'a> {
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
pub struct IngestPutPipelineRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    master_timeout: &'a str,
    timeout: &'a str,
}
impl<'a> IngestPutPipelineRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        IngestPutPipelineRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IngestPutPipelineRequestBuilder<'a> {
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
pub struct IngestSimulateRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    verbose: Option<&'a bool>,
}
impl<'a> IngestSimulateRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        IngestSimulateRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IngestSimulateRequestBuilder<'a> {
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
pub struct IngestNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IngestNamespaceClient<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        IngestNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/delete-pipeline-api.html"]
    pub fn delete_pipeline(&self) -> IngestDeletePipelineRequestBuilder {
        IngestDeletePipelineRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-pipeline-api.html"]
    pub fn get_pipeline(&self) -> IngestGetPipelineRequestBuilder {
        IngestGetPipelineRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/grok-processor.html#grok-processor-rest-get"]
    pub fn processor_grok(&self) -> IngestProcessorGrokRequestBuilder {
        IngestProcessorGrokRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/put-pipeline-api.html"]
    pub fn put_pipeline(&self) -> IngestPutPipelineRequestBuilder {
        IngestPutPipelineRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/simulate-pipeline-api.html"]
    pub fn simulate(&self) -> IngestSimulateRequestBuilder {
        IngestSimulateRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Ingest APIs"]
    pub fn ingest(&self) -> IngestNamespaceClient {
        IngestNamespaceClient::new(self)
    }
}
