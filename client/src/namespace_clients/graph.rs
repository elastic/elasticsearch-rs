

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct GraphExploreBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    routing: Option<String>,
    timeout: Option<String>,
}
impl GraphExploreBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        GraphExploreBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for GraphExploreBuilder {
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
#[doc = "Graph APIs"]
pub struct GraphClient {
    client: ElasticsearchClient,
}
impl GraphClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        GraphClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/graph-explore-api.html"]
    pub fn explore(&self) -> GraphExploreBuilder {
        GraphExploreBuilder::new(self.client.clone())
    }
}
impl ElasticsearchClient {
    #[doc = "Graph APIs"]
    pub fn graph(&self) -> GraphClient {
        GraphClient::new(self.clone())
    }
}
