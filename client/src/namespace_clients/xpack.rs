

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
pub struct XpackNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl XpackNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        XpackNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/info-api.html"]
    pub fn info(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_xpack")
    }
    #[doc = "Retrieve information about xpack features usage"]
    pub fn usage(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_xpack/usage")
    }
}
impl ElasticsearchClient {
    pub fn xpack(&self) -> XpackNamespaceClient {
        XpackNamespaceClient::new(self)
    }
}
