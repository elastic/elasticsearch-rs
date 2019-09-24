

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct SslCertificatesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SslCertificatesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SslCertificatesBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SslCertificatesBuilder {
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
#[doc = "Ssl APIs"]
pub struct SslClient {
    client: ElasticsearchClient,
}
impl SslClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        SslClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-ssl.html"]
    pub fn certificates(&self) -> SslCertificatesBuilder {
        SslCertificatesBuilder::new(self.client.clone())
    }
}
impl ElasticsearchClient {
    #[doc = "Ssl APIs"]
    pub fn ssl(&self) -> SslClient {
        SslClient::new(self.clone())
    }
}
