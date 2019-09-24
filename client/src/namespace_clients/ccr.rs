

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct CcrDeleteAutoFollowPatternBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrDeleteAutoFollowPatternBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CcrDeleteAutoFollowPatternBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for CcrDeleteAutoFollowPatternBuilder {
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
pub struct CcrFollowBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    wait_for_active_shards: Option<String>,
}
impl CcrFollowBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CcrFollowBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for CcrFollowBuilder {
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
pub struct CcrFollowInfoBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrFollowInfoBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CcrFollowInfoBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for CcrFollowInfoBuilder {
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
pub struct CcrFollowStatsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrFollowStatsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CcrFollowStatsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for CcrFollowStatsBuilder {
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
pub struct CcrForgetFollowerBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrForgetFollowerBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CcrForgetFollowerBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for CcrForgetFollowerBuilder {
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
pub struct CcrGetAutoFollowPatternBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrGetAutoFollowPatternBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CcrGetAutoFollowPatternBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for CcrGetAutoFollowPatternBuilder {
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
pub struct CcrPauseFollowBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrPauseFollowBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CcrPauseFollowBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for CcrPauseFollowBuilder {
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
pub struct CcrPutAutoFollowPatternBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrPutAutoFollowPatternBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CcrPutAutoFollowPatternBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for CcrPutAutoFollowPatternBuilder {
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
pub struct CcrResumeFollowBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrResumeFollowBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CcrResumeFollowBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for CcrResumeFollowBuilder {
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
pub struct CcrStatsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrStatsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CcrStatsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for CcrStatsBuilder {
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
pub struct CcrUnfollowBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl CcrUnfollowBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CcrUnfollowBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for CcrUnfollowBuilder {
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
pub struct CcrClient {
    client: ElasticsearchClient,
}
impl CcrClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        CcrClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-delete-auto-follow-pattern.html"]
    pub fn delete_auto_follow_pattern(&self) -> CcrDeleteAutoFollowPatternBuilder {
        CcrDeleteAutoFollowPatternBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-put-follow.html"]
    pub fn follow(&self) -> CcrFollowBuilder {
        CcrFollowBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-follow-info.html"]
    pub fn follow_info(&self) -> CcrFollowInfoBuilder {
        CcrFollowInfoBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-follow-stats.html"]
    pub fn follow_stats(&self) -> CcrFollowStatsBuilder {
        CcrFollowStatsBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current"]
    pub fn forget_follower(&self) -> CcrForgetFollowerBuilder {
        CcrForgetFollowerBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-auto-follow-pattern.html"]
    pub fn get_auto_follow_pattern(&self) -> CcrGetAutoFollowPatternBuilder {
        CcrGetAutoFollowPatternBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-post-pause-follow.html"]
    pub fn pause_follow(&self) -> CcrPauseFollowBuilder {
        CcrPauseFollowBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-put-auto-follow-pattern.html"]
    pub fn put_auto_follow_pattern(&self) -> CcrPutAutoFollowPatternBuilder {
        CcrPutAutoFollowPatternBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-post-resume-follow.html"]
    pub fn resume_follow(&self) -> CcrResumeFollowBuilder {
        CcrResumeFollowBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ccr-get-stats.html"]
    pub fn stats(&self) -> CcrStatsBuilder {
        CcrStatsBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current"]
    pub fn unfollow(&self) -> CcrUnfollowBuilder {
        CcrUnfollowBuilder::new(self.client.clone())
    }
}
impl ElasticsearchClient {
    #[doc = "Ccr APIs"]
    pub fn ccr(&self) -> CcrClient {
        CcrClient::new(self.clone())
    }
}
