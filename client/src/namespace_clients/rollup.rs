

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
pub struct RollupNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl RollupNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        RollupNamespaceClient { client }
    }
    #[doc = ""]
    pub fn delete_job(&self) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_rollup/job/{id}")
    }
    #[doc = ""]
    pub fn get_jobs(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_rollup/job/{id}")
    }
    #[doc = ""]
    pub fn get_rollup_caps(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_rollup/data/{id}")
    }
    #[doc = ""]
    pub fn get_rollup_index_caps(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_rollup/data")
    }
    #[doc = ""]
    pub fn put_job(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_rollup/job/{id}")
    }
    #[doc = ""]
    pub fn rollup_search(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "{index}/_rollup_search")
    }
    #[doc = ""]
    pub fn start_job(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_rollup/job/{id}/_start")
    }
    #[doc = ""]
    pub fn stop_job(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_rollup/job/{id}/_stop")
    }
}
impl ElasticsearchClient {
    pub fn rollup(&self) -> RollupNamespaceClient {
        RollupNamespaceClient::new(self)
    }
}
