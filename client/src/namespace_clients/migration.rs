use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
#[doc = "Migration APIs"]
pub struct MigrationNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl MigrationNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        MigrationNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/migration-api-deprecation.html"]
    pub fn deprecations(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_migration/deprecations")
    }
}
impl ElasticsearchClient {
    #[doc = "Migration APIs"]
    pub fn migration(&self) -> MigrationNamespaceClient {
        MigrationNamespaceClient::new(self)
    }
}
