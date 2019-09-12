

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct CcrDeleteAutoFollowPatternRequest<'a> {}
pub struct CcrDeleteAutoFollowPatternRequestBuilder<'a> {}
impl<'a> CcrDeleteAutoFollowPatternRequestBuilder<'a> {
    pub fn build(&self) -> CcrDeleteAutoFollowPatternRequest<'a> {
        CcrDeleteAutoFollowPatternRequest {}
    }
}
pub struct CcrFollowRequest<'a> {
    wait_for_active_shards: &'a String,
}
pub struct CcrFollowRequestBuilder<'a> {
    wait_for_active_shards: &'a String,
}
impl<'a> CcrFollowRequestBuilder<'a> {
    pub fn build(&self) -> CcrFollowRequest<'a> {
        CcrFollowRequest {
            wait_for_active_shards: self.wait_for_active_shards,
        }
    }
}
pub struct CcrFollowInfoRequest<'a> {}
pub struct CcrFollowInfoRequestBuilder<'a> {}
impl<'a> CcrFollowInfoRequestBuilder<'a> {
    pub fn build(&self) -> CcrFollowInfoRequest<'a> {
        CcrFollowInfoRequest {}
    }
}
pub struct CcrFollowStatsRequest<'a> {}
pub struct CcrFollowStatsRequestBuilder<'a> {}
impl<'a> CcrFollowStatsRequestBuilder<'a> {
    pub fn build(&self) -> CcrFollowStatsRequest<'a> {
        CcrFollowStatsRequest {}
    }
}
pub struct CcrForgetFollowerRequest<'a> {}
pub struct CcrForgetFollowerRequestBuilder<'a> {}
impl<'a> CcrForgetFollowerRequestBuilder<'a> {
    pub fn build(&self) -> CcrForgetFollowerRequest<'a> {
        CcrForgetFollowerRequest {}
    }
}
pub struct CcrGetAutoFollowPatternRequest<'a> {}
pub struct CcrGetAutoFollowPatternRequestBuilder<'a> {}
impl<'a> CcrGetAutoFollowPatternRequestBuilder<'a> {
    pub fn build(&self) -> CcrGetAutoFollowPatternRequest<'a> {
        CcrGetAutoFollowPatternRequest {}
    }
}
pub struct CcrPauseFollowRequest<'a> {}
pub struct CcrPauseFollowRequestBuilder<'a> {}
impl<'a> CcrPauseFollowRequestBuilder<'a> {
    pub fn build(&self) -> CcrPauseFollowRequest<'a> {
        CcrPauseFollowRequest {}
    }
}
pub struct CcrPutAutoFollowPatternRequest<'a> {}
pub struct CcrPutAutoFollowPatternRequestBuilder<'a> {}
impl<'a> CcrPutAutoFollowPatternRequestBuilder<'a> {
    pub fn build(&self) -> CcrPutAutoFollowPatternRequest<'a> {
        CcrPutAutoFollowPatternRequest {}
    }
}
pub struct CcrResumeFollowRequest<'a> {}
pub struct CcrResumeFollowRequestBuilder<'a> {}
impl<'a> CcrResumeFollowRequestBuilder<'a> {
    pub fn build(&self) -> CcrResumeFollowRequest<'a> {
        CcrResumeFollowRequest {}
    }
}
pub struct CcrStatsRequest<'a> {}
pub struct CcrStatsRequestBuilder<'a> {}
impl<'a> CcrStatsRequestBuilder<'a> {
    pub fn build(&self) -> CcrStatsRequest<'a> {
        CcrStatsRequest {}
    }
}
pub struct CcrUnfollowRequest<'a> {}
pub struct CcrUnfollowRequestBuilder<'a> {}
impl<'a> CcrUnfollowRequestBuilder<'a> {
    pub fn build(&self) -> CcrUnfollowRequest<'a> {
        CcrUnfollowRequest {}
    }
}
#[doc = "Ccr APIs"]
pub struct CcrNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> CcrNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CcrNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-delete-auto-follow-pattern.html"]
    pub fn delete_auto_follow_pattern(
        &self,
        request: &CcrDeleteAutoFollowPatternRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ccr/auto_follow/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-put-follow.html"]
    pub fn follow(&self, request: &CcrFollowRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/{index}/_ccr/follow")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-follow-info.html"]
    pub fn follow_info(&self, request: &CcrFollowInfoRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_ccr/info")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-follow-stats.html"]
    pub fn follow_stats(&self, request: &CcrFollowStatsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_ccr/stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current"]
    pub fn forget_follower(&self, request: &CcrForgetFollowerRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/{index}/_ccr/forget_follower")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-auto-follow-pattern.html"]
    pub fn get_auto_follow_pattern(
        &self,
        request: &CcrGetAutoFollowPatternRequest,
    ) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ccr/auto_follow")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-post-pause-follow.html"]
    pub fn pause_follow(&self, request: &CcrPauseFollowRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/{index}/_ccr/pause_follow")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-put-auto-follow-pattern.html"]
    pub fn put_auto_follow_pattern(
        &self,
        request: &CcrPutAutoFollowPatternRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_ccr/auto_follow/{name}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-post-resume-follow.html"]
    pub fn resume_follow(&self, request: &CcrResumeFollowRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/{index}/_ccr/resume_follow")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-stats.html"]
    pub fn stats(&self, request: &CcrStatsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ccr/stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current"]
    pub fn unfollow(&self, request: &CcrUnfollowRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_ccr/unfollow")
    }
}
impl ElasticsearchClient {
    #[doc = "Ccr APIs"]
    pub fn ccr(&self) -> CcrNamespaceClient {
        CcrNamespaceClient::new(self)
    }
}
