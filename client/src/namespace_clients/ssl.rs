

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct SslCertificatesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SslCertificatesRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SslCertificatesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SslCertificatesRequestBuilder<'a> {
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
pub struct SslNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> SslNamespaceClient<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        SslNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-ssl.html"]
    pub fn certificates(&self) -> SslCertificatesRequestBuilder {
        SslCertificatesRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Ssl APIs"]
    pub fn ssl(&self) -> SslNamespaceClient {
        SslNamespaceClient::new(self)
    }
}
