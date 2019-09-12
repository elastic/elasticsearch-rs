use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
#[doc = "Ccr APIs"]
pub struct CcrNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl CcrNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CcrNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-delete-auto-follow-pattern.html"]
    pub fn delete_auto_follow_pattern(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ccr/auto_follow/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-put-follow.html"]
    pub fn follow(&self) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/{index}/_ccr/follow")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-follow-info.html"]
    pub fn follow_info(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_ccr/info")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-follow-stats.html"]
    pub fn follow_stats(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_ccr/stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current"]
    pub fn forget_follower(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/{index}/_ccr/forget_follower")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-auto-follow-pattern.html"]
    pub fn get_auto_follow_pattern(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ccr/auto_follow")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-post-pause-follow.html"]
    pub fn pause_follow(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/{index}/_ccr/pause_follow")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-put-auto-follow-pattern.html"]
    pub fn put_auto_follow_pattern(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_ccr/auto_follow/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-post-resume-follow.html"]
    pub fn resume_follow(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/{index}/_ccr/resume_follow")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-stats.html"]
    pub fn stats(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ccr/stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current"]
    pub fn unfollow(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_ccr/unfollow")
    }
}
impl ElasticsearchClient {
    #[doc = "Ccr APIs"]
    pub fn ccr(&self) -> CcrNamespaceClient {
        CcrNamespaceClient::new(self)
    }
}
