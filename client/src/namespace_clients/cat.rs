use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
#[doc = "Cat APIs"]
pub struct CatNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl CatNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-alias.html"]
    pub fn aliases(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/aliases")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-allocation.html"]
    pub fn allocation(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/allocation")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-count.html"]
    pub fn count(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/count")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-fielddata.html"]
    pub fn fielddata(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/fielddata")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-health.html"]
    pub fn health(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/health")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat.html"]
    pub fn help(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-indices.html"]
    pub fn indices(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/indices")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-master.html"]
    pub fn master(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/master")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-nodeattrs.html"]
    pub fn nodeattrs(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/nodeattrs")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-nodes.html"]
    pub fn nodes(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/nodes")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-pending-tasks.html"]
    pub fn pending_tasks(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/pending_tasks")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-plugins.html"]
    pub fn plugins(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/plugins")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-recovery.html"]
    pub fn recovery(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/recovery")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-repositories.html"]
    pub fn repositories(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/repositories")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-segments.html"]
    pub fn segments(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/segments")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-shards.html"]
    pub fn shards(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/shards")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-snapshots.html"]
    pub fn snapshots(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/snapshots")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn tasks(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/tasks")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-templates.html"]
    pub fn templates(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/templates")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-thread-pool.html"]
    pub fn thread_pool(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/thread_pool")
    }
}
impl ElasticsearchClient {
    #[doc = "Cat APIs"]
    pub fn cat(&self) -> CatNamespaceClient {
        CatNamespaceClient::new(self)
    }
}
