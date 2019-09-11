

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
pub struct GraphNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl GraphNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        GraphNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/graph-explore-api.html"]
    pub fn explore(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_graph/explore")
    }
}
impl ElasticsearchClient {
    pub fn graph(&self) -> GraphNamespaceClient {
        GraphNamespaceClient::new(self)
    }
}
