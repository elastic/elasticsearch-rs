use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
#[doc = "Ilm APIs"]
pub struct IlmNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl IlmNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IlmNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-delete-lifecycle.html"]
    pub fn delete_lifecycle(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ilm/policy/{policy}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-explain-lifecycle.html"]
    pub fn explain_lifecycle(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_ilm/explain")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-lifecycle.html"]
    pub fn get_lifecycle(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ilm/policy/{policy}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-status.html"]
    pub fn get_status(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ilm/status")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-move-to-step.html"]
    pub fn move_to_step(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_ilm/move/{index}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-put-lifecycle.html"]
    pub fn put_lifecycle(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_ilm/policy/{policy}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-remove-policy.html"]
    pub fn remove_policy(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_ilm/remove")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-retry-policy.html"]
    pub fn retry(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_ilm/retry")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-start.html"]
    pub fn start(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_ilm/start")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-stop.html"]
    pub fn stop(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_ilm/stop")
    }
}
impl ElasticsearchClient {
    #[doc = "Ilm APIs"]
    pub fn ilm(&self) -> IlmNamespaceClient {
        IlmNamespaceClient::new(self)
    }
}
