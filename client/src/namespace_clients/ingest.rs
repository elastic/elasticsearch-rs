

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct IngestDeletePipelineBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
    timeout: Option<String>,
}
impl IngestDeletePipelineBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IngestDeletePipelineBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IngestDeletePipelineBuilder {
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
pub struct IngestGetPipelineBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
}
impl IngestGetPipelineBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IngestGetPipelineBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IngestGetPipelineBuilder {
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
pub struct IngestProcessorGrokBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IngestProcessorGrokBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IngestProcessorGrokBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IngestProcessorGrokBuilder {
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
pub struct IngestPutPipelineBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
    timeout: Option<String>,
}
impl IngestPutPipelineBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IngestPutPipelineBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IngestPutPipelineBuilder {
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
pub struct IngestSimulateBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    verbose: Option<bool>,
}
impl IngestSimulateBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IngestSimulateBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IngestSimulateBuilder {
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
pub struct IngestClient {
    client: ElasticsearchClient,
}
impl IngestClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        IngestClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/delete-pipeline-api.html"]
    pub fn delete_pipeline(&self) -> IngestDeletePipelineBuilder {
        IngestDeletePipelineBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-pipeline-api.html"]
    pub fn get_pipeline(&self) -> IngestGetPipelineBuilder {
        IngestGetPipelineBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/grok-processor.html#grok-processor-rest-get"]
    pub fn processor_grok(&self) -> IngestProcessorGrokBuilder {
        IngestProcessorGrokBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/put-pipeline-api.html"]
    pub fn put_pipeline(&self) -> IngestPutPipelineBuilder {
        IngestPutPipelineBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/simulate-pipeline-api.html"]
    pub fn simulate(&self) -> IngestSimulateBuilder {
        IngestSimulateBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Ingest APIs"]
    pub fn ingest(&self) -> IngestClient {
        IngestClient::new(self.clone())
    }
}
