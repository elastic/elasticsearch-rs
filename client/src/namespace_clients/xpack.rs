

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct XpackInfoBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    categories: Option<Vec<String>>,
}
impl XpackInfoBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        XpackInfoBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for XpackInfoBuilder {
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
pub struct XpackUsageBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
}
impl XpackUsageBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        XpackUsageBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for XpackUsageBuilder {
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
#[doc = "Xpack APIs"]
pub struct XpackClient {
    client: ElasticsearchClient,
}
impl XpackClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        XpackClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/info-api.html"]
    pub fn info(&self) -> XpackInfoBuilder {
        XpackInfoBuilder::default()
    }
    #[doc = "Retrieve information about xpack features usage"]
    pub fn usage(&self) -> XpackUsageBuilder {
        XpackUsageBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Xpack APIs"]
    pub fn xpack(&self) -> XpackClient {
        XpackClient::new(self.clone())
    }
}
