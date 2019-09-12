

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[Default]
pub struct RollupDeleteJobRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> RollupDeleteJobRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        RollupDeleteJobRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for RollupDeleteJobRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct RollupGetJobsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> RollupGetJobsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        RollupGetJobsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for RollupGetJobsRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct RollupGetRollupCapsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> RollupGetRollupCapsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        RollupGetRollupCapsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for RollupGetRollupCapsRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct RollupGetRollupIndexCapsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> RollupGetRollupIndexCapsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        RollupGetRollupIndexCapsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for RollupGetRollupIndexCapsRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct RollupPutJobRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> RollupPutJobRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        RollupPutJobRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for RollupPutJobRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct RollupRollupSearchRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    rest_total_hits_as_int: Option<&'a bool>,
    typed_keys: Option<&'a bool>,
}
impl<'a> RollupRollupSearchRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        RollupRollupSearchRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for RollupRollupSearchRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct RollupStartJobRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> RollupStartJobRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        RollupStartJobRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for RollupStartJobRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct RollupStopJobRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    timeout: &'a str,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> RollupStopJobRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        RollupStopJobRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for RollupStopJobRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[doc = "Rollup APIs"]
pub struct RollupNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> RollupNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        RollupNamespaceClient { client }
    }
    #[doc = ""]
    pub fn delete_job(&self) -> RollupDeleteJobRequestBuilder {
        RollupDeleteJobRequestBuilder::default()
    }
    #[doc = ""]
    pub fn get_jobs(&self) -> RollupGetJobsRequestBuilder {
        RollupGetJobsRequestBuilder::default()
    }
    #[doc = ""]
    pub fn get_rollup_caps(&self) -> RollupGetRollupCapsRequestBuilder {
        RollupGetRollupCapsRequestBuilder::default()
    }
    #[doc = ""]
    pub fn get_rollup_index_caps(&self) -> RollupGetRollupIndexCapsRequestBuilder {
        RollupGetRollupIndexCapsRequestBuilder::default()
    }
    #[doc = ""]
    pub fn put_job(&self) -> RollupPutJobRequestBuilder {
        RollupPutJobRequestBuilder::default()
    }
    #[doc = ""]
    pub fn rollup_search(&self) -> RollupRollupSearchRequestBuilder {
        RollupRollupSearchRequestBuilder::default()
    }
    #[doc = ""]
    pub fn start_job(&self) -> RollupStartJobRequestBuilder {
        RollupStartJobRequestBuilder::default()
    }
    #[doc = ""]
    pub fn stop_job(&self) -> RollupStopJobRequestBuilder {
        RollupStopJobRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Rollup APIs"]
    pub fn rollup(&self) -> RollupNamespaceClient {
        RollupNamespaceClient::new(self)
    }
}
