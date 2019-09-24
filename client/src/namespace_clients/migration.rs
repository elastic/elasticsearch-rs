

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct MigrationDeprecationsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MigrationDeprecationsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MigrationDeprecationsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MigrationDeprecationsBuilder {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[doc = "Migration APIs"]
pub struct MigrationClient {
    client: ElasticsearchClient,
}
impl MigrationClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        MigrationClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/migration-api-deprecation.html"]
    pub fn deprecations(&self) -> MigrationDeprecationsBuilder {
        MigrationDeprecationsBuilder::new(self.client.clone())
    }
}
impl ElasticsearchClient {
    #[doc = "Migration APIs"]
    pub fn migration(&self) -> MigrationClient {
        MigrationClient::new(self.clone())
    }
}
