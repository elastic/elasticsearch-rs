

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
pub struct SslNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl SslNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SslNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-ssl.html"]
    pub fn certificates(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ssl/certificates")
    }
}
impl ElasticsearchClient {
    pub fn ssl(&self) -> SslNamespaceClient {
        SslNamespaceClient::new(self)
    }
}
