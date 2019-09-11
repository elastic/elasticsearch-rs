

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
pub struct DataFrameNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl DataFrameNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        DataFrameNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/delete-data-frame-transform.html"]
    pub fn delete_data_frame_transform(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_data_frame/transforms/{transform_id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/get-data-frame-transform.html"]
    pub fn get_data_frame_transform(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_data_frame/transforms/{transform_id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/get-data-frame-transform-stats.html"]
    pub fn get_data_frame_transform_stats(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_data_frame/transforms/{transform_id}/_stats",
        )
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/preview-data-frame-transform.html"]
    pub fn preview_data_frame_transform(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_data_frame/transforms/_preview")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/put-data-frame-transform.html"]
    pub fn put_data_frame_transform(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_data_frame/transforms/{transform_id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/start-data-frame-transform.html"]
    pub fn start_data_frame_transform(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_data_frame/transforms/{transform_id}/_start",
        )
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/stop-data-frame-transform.html"]
    pub fn stop_data_frame_transform(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_data_frame/transforms/{transform_id}/_stop",
        )
    }
}
impl ElasticsearchClient {
    pub fn data_frame(&self) -> DataFrameNamespaceClient {
        DataFrameNamespaceClient::new(self)
    }
}
