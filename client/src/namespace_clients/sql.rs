

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct SqlClearCursorRequest<'a> {}
pub struct SqlClearCursorRequestBuilder<'a> {}
impl<'a> SqlClearCursorRequestBuilder<'a> {
    pub fn build(&self) -> SqlClearCursorRequest<'a> {
        SqlClearCursorRequest {}
    }
}
pub struct SqlQueryRequest<'a> {
    format: &'a String,
}
pub struct SqlQueryRequestBuilder<'a> {
    format: &'a String,
}
impl<'a> SqlQueryRequestBuilder<'a> {
    pub fn build(&self) -> SqlQueryRequest<'a> {
        SqlQueryRequest {
            format: self.format,
        }
    }
}
pub struct SqlTranslateRequest<'a> {}
pub struct SqlTranslateRequestBuilder<'a> {}
impl<'a> SqlTranslateRequestBuilder<'a> {
    pub fn build(&self) -> SqlTranslateRequest<'a> {
        SqlTranslateRequest {}
    }
}
#[doc = "Sql APIs"]
pub struct SqlNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> SqlNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SqlNamespaceClient { client }
    }
    #[doc = "Clear SQL cursor"]
    pub fn clear_cursor(&self, request: &SqlClearCursorRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_sql/close")
    }
    #[doc = "Execute SQL"]
    pub fn query(&self, request: &SqlQueryRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_sql")
    }
    #[doc = "Translate SQL into Elasticsearch queries"]
    pub fn translate(&self, request: &SqlTranslateRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_sql/translate")
    }
}
impl ElasticsearchClient {
    #[doc = "Sql APIs"]
    pub fn sql(&self) -> SqlNamespaceClient {
        SqlNamespaceClient::new(self)
    }
}
