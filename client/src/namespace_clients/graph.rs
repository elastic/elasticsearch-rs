

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct GraphExploreRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    routing: &'a str,
    timeout: &'a str,
}
impl<'a> GraphExploreRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        GraphExploreRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for GraphExploreRequestBuilder<'a> {
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
pub struct GraphNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> GraphNamespaceClient<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        GraphNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/graph-explore-api.html"]
    pub fn explore(&self) -> GraphExploreRequestBuilder {
        GraphExploreRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Graph APIs"]
    pub fn graph(&self) -> GraphNamespaceClient {
        GraphNamespaceClient::new(self)
    }
}
