use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
#[doc = "Global APIs"]
pub struct GlobalNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl GlobalNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        GlobalNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-bulk.html"]
    pub fn bulk(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_bulk")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-request-body.html#request-body-search-scroll"]
    pub fn clear_scroll(&self) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_search/scroll")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-count.html"]
    pub fn count(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_count")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-index_.html"]
    pub fn create(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/{index}/_create/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-delete.html"]
    pub fn delete(&self) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/{index}/_doc/{id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-delete-by-query.html"]
    pub fn delete_by_query(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/{index}/_delete_by_query")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-delete-by-query.html"]
    pub fn delete_by_query_rethrottle(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_delete_by_query/{task_id}/_rethrottle")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html"]
    pub fn delete_script(&self) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_scripts/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn exists(&self) -> Result<Response> {
        self.client.send(HttpMethod::Head, "/{index}/_doc/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn exists_source(&self) -> Result<Response> {
        self.client.send(HttpMethod::Head, "/{index}/_source/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-explain.html"]
    pub fn explain(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_explain/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-field-caps.html"]
    pub fn field_caps(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_field_caps")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn get(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_doc/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html"]
    pub fn get_script(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_scripts/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html"]
    pub fn get_source(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_source/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-index_.html"]
    pub fn index(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_doc/{id}")
    }
    #[doc = "http://www.elastic.co/guide/"]
    pub fn info(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-multi-get.html"]
    pub fn mget(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_mget")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-multi-search.html"]
    pub fn msearch(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_msearch")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/search-multi-search.html"]
    pub fn msearch_template(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_msearch/template")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-multi-termvectors.html"]
    pub fn mtermvectors(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_mtermvectors")
    }
    #[doc = "http://www.elastic.co/guide/"]
    pub fn ping(&self) -> Result<Response> {
        self.client.send(HttpMethod::Head, "/")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html"]
    pub fn put_script(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_scripts/{id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/search-rank-eval.html"]
    pub fn rank_eval(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_rank_eval")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-reindex.html"]
    pub fn reindex(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_reindex")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-reindex.html"]
    pub fn reindex_rethrottle(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_reindex/{task_id}/_rethrottle")
    }
    #[doc = "http://www.elasticsearch.org/guide/en/elasticsearch/reference/master/search-template.html"]
    pub fn render_search_template(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_render/template")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/painless/master/painless-execute-api.html"]
    pub fn scripts_painless_execute(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_scripts/painless/_execute")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-request-body.html#request-body-search-scroll"]
    pub fn scroll(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_search/scroll")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html"]
    pub fn search(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_search")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/search-shards.html"]
    pub fn search_shards(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_search_shards")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/search-template.html"]
    pub fn search_template(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_search/template")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-termvectors.html"]
    pub fn termvectors(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/{index}/_termvectors/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-update.html"]
    pub fn update(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_update/{id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-update-by-query.html"]
    pub fn update_by_query(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/{index}/_update_by_query")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-update-by-query.html"]
    pub fn update_by_query_rethrottle(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_update_by_query/{task_id}/_rethrottle")
    }
}
impl ElasticsearchClient {
    #[doc = "Global APIs"]
    pub fn global(&self) -> GlobalNamespaceClient {
        GlobalNamespaceClient::new(self)
    }
}
