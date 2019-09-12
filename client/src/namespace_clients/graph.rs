

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct GraphExploreRequest<'a> {
    routing: &'a String,
    timeout: &'a String,
}
pub struct GraphExploreRequestBuilder<'a> {
    routing: &'a String,
    timeout: &'a String,
}
impl<'a> GraphExploreRequestBuilder<'a> {
    pub fn build(&self) -> GraphExploreRequest<'a> {
        GraphExploreRequest {
            routing: self.routing,
            timeout: self.timeout,
        }
    }
}
#[doc = "Graph APIs"]
pub struct GraphNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> GraphNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        GraphNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/graph-explore-api.html"]
    pub fn explore(&self, request: &GraphExploreRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_graph/explore")
    }
}
impl ElasticsearchClient {
    #[doc = "Graph APIs"]
    pub fn graph(&self) -> GraphNamespaceClient {
        GraphNamespaceClient::new(self)
    }
}
