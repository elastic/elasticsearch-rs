

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct CcrDeleteAutoFollowPatternRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> CcrDeleteAutoFollowPatternRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        CcrDeleteAutoFollowPatternRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CcrDeleteAutoFollowPatternRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CcrFollowRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    wait_for_active_shards: &'a str,
}
impl<'a> CcrFollowRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        CcrFollowRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CcrFollowRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CcrFollowInfoRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> CcrFollowInfoRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        CcrFollowInfoRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CcrFollowInfoRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CcrFollowStatsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> CcrFollowStatsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        CcrFollowStatsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CcrFollowStatsRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CcrForgetFollowerRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> CcrForgetFollowerRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        CcrForgetFollowerRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CcrForgetFollowerRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CcrGetAutoFollowPatternRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> CcrGetAutoFollowPatternRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        CcrGetAutoFollowPatternRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CcrGetAutoFollowPatternRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CcrPauseFollowRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> CcrPauseFollowRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        CcrPauseFollowRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CcrPauseFollowRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CcrPutAutoFollowPatternRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> CcrPutAutoFollowPatternRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        CcrPutAutoFollowPatternRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CcrPutAutoFollowPatternRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CcrResumeFollowRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> CcrResumeFollowRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        CcrResumeFollowRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CcrResumeFollowRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CcrStatsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> CcrStatsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        CcrStatsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CcrStatsRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CcrUnfollowRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> CcrUnfollowRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        CcrUnfollowRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CcrUnfollowRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[doc = "Ccr APIs"]
pub struct CcrNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> CcrNamespaceClient<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        CcrNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-delete-auto-follow-pattern.html"]
    pub fn delete_auto_follow_pattern(&self) -> CcrDeleteAutoFollowPatternRequestBuilder {
        CcrDeleteAutoFollowPatternRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-put-follow.html"]
    pub fn follow(&self) -> CcrFollowRequestBuilder {
        CcrFollowRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-follow-info.html"]
    pub fn follow_info(&self) -> CcrFollowInfoRequestBuilder {
        CcrFollowInfoRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-follow-stats.html"]
    pub fn follow_stats(&self) -> CcrFollowStatsRequestBuilder {
        CcrFollowStatsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current"]
    pub fn forget_follower(&self) -> CcrForgetFollowerRequestBuilder {
        CcrForgetFollowerRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-auto-follow-pattern.html"]
    pub fn get_auto_follow_pattern(&self) -> CcrGetAutoFollowPatternRequestBuilder {
        CcrGetAutoFollowPatternRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-post-pause-follow.html"]
    pub fn pause_follow(&self) -> CcrPauseFollowRequestBuilder {
        CcrPauseFollowRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-put-auto-follow-pattern.html"]
    pub fn put_auto_follow_pattern(&self) -> CcrPutAutoFollowPatternRequestBuilder {
        CcrPutAutoFollowPatternRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-post-resume-follow.html"]
    pub fn resume_follow(&self) -> CcrResumeFollowRequestBuilder {
        CcrResumeFollowRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-stats.html"]
    pub fn stats(&self) -> CcrStatsRequestBuilder {
        CcrStatsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current"]
    pub fn unfollow(&self) -> CcrUnfollowRequestBuilder {
        CcrUnfollowRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Ccr APIs"]
    pub fn ccr(&self) -> CcrNamespaceClient {
        CcrNamespaceClient::new(self)
    }
}
