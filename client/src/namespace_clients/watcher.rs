use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
#[doc = "Watcher APIs"]
pub struct WatcherNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl WatcherNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        WatcherNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-ack-watch.html"]
    pub fn ack_watch(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_watcher/watch/{watch_id}/_ack")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-activate-watch.html"]
    pub fn activate_watch(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_watcher/watch/{watch_id}/_activate")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-deactivate-watch.html"]
    pub fn deactivate_watch(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_watcher/watch/{watch_id}/_deactivate")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-delete-watch.html"]
    pub fn delete_watch(&self) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_watcher/watch/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-execute-watch.html"]
    pub fn execute_watch(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_watcher/watch/{id}/_execute")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-get-watch.html"]
    pub fn get_watch(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_watcher/watch/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-put-watch.html"]
    pub fn put_watch(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_watcher/watch/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-start.html"]
    pub fn start(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_watcher/_start")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-stats.html"]
    pub fn stats(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_watcher/stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-stop.html"]
    pub fn stop(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_watcher/_stop")
    }
}
impl ElasticsearchClient {
    #[doc = "Watcher APIs"]
    pub fn watcher(&self) -> WatcherNamespaceClient {
        WatcherNamespaceClient::new(self)
    }
}
