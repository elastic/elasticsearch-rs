

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
pub struct IndicesNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl IndicesNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IndicesNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-analyze.html"]
    pub fn analyze(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_analyze")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-clearcache.html"]
    pub fn clear_cache(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_cache/clear")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html"]
    pub fn close(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_close")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-create-index.html"]
    pub fn create(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/{index}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-delete-index.html"]
    pub fn delete(&self) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/{index}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn delete_alias(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/{index}/_alias/{name}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn delete_template(&self) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_template/{name}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-exists.html"]
    pub fn exists(&self) -> Result<Response> {
        self.client.send(HttpMethod::Head, "/{index}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn exists_alias(&self) -> Result<Response> {
        self.client.send(HttpMethod::Head, "/_alias/{name}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn exists_template(&self) -> Result<Response> {
        self.client.send(HttpMethod::Head, "/_template/{name}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-types-exists.html"]
    pub fn exists_type(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Head, "/{index}/_mapping/{type}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-flush.html"]
    pub fn flush(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_flush")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-flush.html#synced-flush-api"]
    pub fn flush_synced(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_flush/synced")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-forcemerge.html"]
    pub fn forcemerge(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_forcemerge")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html"]
    pub fn freeze(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_freeze")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-index.html"]
    pub fn get(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn get_alias(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_alias")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-field-mapping.html"]
    pub fn get_field_mapping(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_mapping/field/{fields}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-mapping.html"]
    pub fn get_mapping(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_mapping")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-settings.html"]
    pub fn get_settings(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_settings")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn get_template(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_template")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html"]
    pub fn get_upgrade(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_upgrade")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html"]
    pub fn open(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_open")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn put_alias(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/{index}/_alias/{name}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-put-mapping.html"]
    pub fn put_mapping(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "{index}/_mapping")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-update-settings.html"]
    pub fn put_settings(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_settings")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html"]
    pub fn put_template(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_template/{name}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-recovery.html"]
    pub fn recovery(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_recovery")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-refresh.html"]
    pub fn refresh(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_refresh")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-reload-analyzers.html"]
    pub fn reload_search_analyzers(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/{index}/_reload_search_analyzers")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-rollover-index.html"]
    pub fn rollover(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{alias}/_rollover")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-segments.html"]
    pub fn segments(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_segments")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shards-stores.html"]
    pub fn shard_stores(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_shard_stores")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shrink-index.html"]
    pub fn shrink(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/{index}/_shrink/{target}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-split-index.html"]
    pub fn split(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/{index}/_split/{target}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-stats.html"]
    pub fn stats(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_stats")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/frozen.html"]
    pub fn unfreeze(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_unfreeze")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-aliases.html"]
    pub fn update_aliases(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_aliases")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-upgrade.html"]
    pub fn upgrade(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_upgrade")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-validate.html"]
    pub fn validate_query(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_validate/query")
    }
}
impl ElasticsearchClient {
    pub fn indices(&self) -> IndicesNamespaceClient {
        IndicesNamespaceClient::new(self)
    }
}
