

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct SslCertificatesRequest<'a> {}
pub struct SslCertificatesRequestBuilder<'a> {}
impl<'a> SslCertificatesRequestBuilder<'a> {
    pub fn build(&self) -> SslCertificatesRequest<'a> {
        SslCertificatesRequest {}
    }
}
#[doc = "Ssl APIs"]
pub struct SslNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> SslNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SslNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-ssl.html"]
    pub fn certificates(&self, request: &SslCertificatesRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ssl/certificates")
    }
}
impl ElasticsearchClient {
    #[doc = "Ssl APIs"]
    pub fn ssl(&self) -> SslNamespaceClient {
        SslNamespaceClient::new(self)
    }
}
