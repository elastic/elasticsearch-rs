use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
#[doc = "Snapshot APIs"]
pub struct SnapshotNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl SnapshotNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SnapshotNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn create(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_snapshot/{repository}/{snapshot}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn create_repository(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_snapshot/{repository}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn delete(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_snapshot/{repository}/{snapshot}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn delete_repository(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_snapshot/{repository}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn get(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_snapshot/{repository}/{snapshot}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn get_repository(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_snapshot")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn restore(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_snapshot/{repository}/{snapshot}/_restore",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn status(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_snapshot/_status")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn verify_repository(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_snapshot/{repository}/_verify")
    }
}
impl ElasticsearchClient {
    #[doc = "Snapshot APIs"]
    pub fn snapshot(&self) -> SnapshotNamespaceClient {
        SnapshotNamespaceClient::new(self)
    }
}
