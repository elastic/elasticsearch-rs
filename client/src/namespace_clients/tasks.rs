

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
pub struct TasksNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl TasksNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        TasksNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn cancel(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_tasks/_cancel")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn get(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_tasks/{task_id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn list(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_tasks")
    }
}
impl ElasticsearchClient {
    pub fn tasks(&self) -> TasksNamespaceClient {
        TasksNamespaceClient::new(self)
    }
}
