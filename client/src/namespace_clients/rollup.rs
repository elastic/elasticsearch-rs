

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct RollupDeleteJobBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl RollupDeleteJobBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        RollupDeleteJobBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for RollupDeleteJobBuilder {
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
pub struct RollupGetJobsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl RollupGetJobsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        RollupGetJobsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for RollupGetJobsBuilder {
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
pub struct RollupGetRollupCapsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl RollupGetRollupCapsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        RollupGetRollupCapsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for RollupGetRollupCapsBuilder {
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
pub struct RollupGetRollupIndexCapsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl RollupGetRollupIndexCapsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        RollupGetRollupIndexCapsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for RollupGetRollupIndexCapsBuilder {
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
pub struct RollupPutJobBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl RollupPutJobBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        RollupPutJobBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for RollupPutJobBuilder {
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
pub struct RollupRollupSearchBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    rest_total_hits_as_int: Option<bool>,
    typed_keys: Option<bool>,
}
impl RollupRollupSearchBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        RollupRollupSearchBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for RollupRollupSearchBuilder {
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
pub struct RollupStartJobBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl RollupStartJobBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        RollupStartJobBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for RollupStartJobBuilder {
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
pub struct RollupStopJobBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_completion: Option<bool>,
}
impl RollupStopJobBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        RollupStopJobBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for RollupStopJobBuilder {
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
#[doc = "Rollup APIs"]
pub struct RollupClient {
    client: ElasticsearchClient,
}
impl RollupClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        RollupClient { client }
    }
    #[doc = ""]
    pub fn delete_job(&self) -> RollupDeleteJobBuilder {
        RollupDeleteJobBuilder::default()
    }
    #[doc = ""]
    pub fn get_jobs(&self) -> RollupGetJobsBuilder {
        RollupGetJobsBuilder::default()
    }
    #[doc = ""]
    pub fn get_rollup_caps(&self) -> RollupGetRollupCapsBuilder {
        RollupGetRollupCapsBuilder::default()
    }
    #[doc = ""]
    pub fn get_rollup_index_caps(&self) -> RollupGetRollupIndexCapsBuilder {
        RollupGetRollupIndexCapsBuilder::default()
    }
    #[doc = ""]
    pub fn put_job(&self) -> RollupPutJobBuilder {
        RollupPutJobBuilder::default()
    }
    #[doc = ""]
    pub fn rollup_search(&self) -> RollupRollupSearchBuilder {
        RollupRollupSearchBuilder::default()
    }
    #[doc = ""]
    pub fn start_job(&self) -> RollupStartJobBuilder {
        RollupStartJobBuilder::default()
    }
    #[doc = ""]
    pub fn stop_job(&self) -> RollupStopJobBuilder {
        RollupStopJobBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Rollup APIs"]
    pub fn rollup(&self) -> RollupClient {
        RollupClient::new(self.clone())
    }
}
