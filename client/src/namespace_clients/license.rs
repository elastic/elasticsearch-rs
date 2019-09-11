

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
pub struct LicenseNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl LicenseNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        LicenseNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/delete-license.html"]
    pub fn delete(&self) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_license")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-license.html"]
    pub fn get(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_license")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-basic-status.html"]
    pub fn get_basic_status(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_license/basic_status")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-trial-status.html"]
    pub fn get_trial_status(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_license/trial_status")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/update-license.html"]
    pub fn post(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_license")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/start-basic.html"]
    pub fn post_start_basic(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_license/start_basic")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/start-trial.html"]
    pub fn post_start_trial(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_license/start_trial")
    }
}
impl ElasticsearchClient {
    pub fn license(&self) -> LicenseNamespaceClient {
        LicenseNamespaceClient::new(self)
    }
}
