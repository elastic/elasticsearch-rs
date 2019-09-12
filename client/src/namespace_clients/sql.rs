use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
#[doc = "Sql APIs"]
pub struct SqlNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl SqlNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SqlNamespaceClient { client }
    }
    #[doc = "Clear SQL cursor"]
    pub fn clear_cursor(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_sql/close")
    }
    #[doc = "Execute SQL"]
    pub fn query(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_sql")
    }
    #[doc = "Translate SQL into Elasticsearch queries"]
    pub fn translate(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_sql/translate")
    }
}
impl ElasticsearchClient {
    #[doc = "Sql APIs"]
    pub fn sql(&self) -> SqlNamespaceClient {
        SqlNamespaceClient::new(self)
    }
}
