

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[Default]
pub struct MigrationDeprecationsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> MigrationDeprecationsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        MigrationDeprecationsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MigrationDeprecationsRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
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
    pub fn deprecations(&self) -> MigrationDeprecationsRequestBuilder {
        MigrationDeprecationsRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Migration APIs"]
    pub fn migration(&self) -> MigrationNamespaceClient {
        MigrationNamespaceClient::new(self)
    }
}
