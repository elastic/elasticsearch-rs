

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct XpackInfoRequest<'a> {
    categories: &'a Vec<String>,
}
pub struct XpackInfoRequestBuilder<'a> {
    categories: &'a Vec<String>,
}
impl<'a> XpackInfoRequestBuilder<'a> {
    pub fn build(&self) -> XpackInfoRequest<'a> {
        XpackInfoRequest {
            categories: self.categories,
        }
    }
}
pub struct XpackUsageRequest<'a> {
    master_timeout: &'a String,
}
pub struct XpackUsageRequestBuilder<'a> {
    master_timeout: &'a String,
}
impl<'a> XpackUsageRequestBuilder<'a> {
    pub fn build(&self) -> XpackUsageRequest<'a> {
        XpackUsageRequest {
            master_timeout: self.master_timeout,
        }
    }
}
#[doc = "Xpack APIs"]
pub struct XpackNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> XpackNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        XpackNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/info-api.html"]
    pub fn info(&self, request: &XpackInfoRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_xpack")
    }
    #[doc = "Retrieve information about xpack features usage"]
    pub fn usage(&self, request: &XpackUsageRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_xpack/usage")
    }
}
impl ElasticsearchClient {
    #[doc = "Xpack APIs"]
    pub fn xpack(&self) -> XpackNamespaceClient {
        XpackNamespaceClient::new(self)
    }
}
