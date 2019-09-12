

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct MigrationDeprecationsRequest<'a> {}
pub struct MigrationDeprecationsRequestBuilder<'a> {}
impl<'a> MigrationDeprecationsRequestBuilder<'a> {
    pub fn build(&self) -> MigrationDeprecationsRequest<'a> {
        MigrationDeprecationsRequest {}
    }
}
#[doc = "Migration APIs"]
pub struct MigrationNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> MigrationNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        MigrationNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/migration-api-deprecation.html"]
    pub fn deprecations(&self, request: &MigrationDeprecationsRequest) -> Result<Response> {
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
