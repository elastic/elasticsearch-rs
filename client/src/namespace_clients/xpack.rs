

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct XpackInfoRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    categories: Option<&'a Vec<String>>,
}
impl<'a> XpackInfoRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        XpackInfoRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for XpackInfoRequestBuilder<'a> {
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
pub struct XpackUsageRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    master_timeout: &'a str,
}
impl<'a> XpackUsageRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        XpackUsageRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for XpackUsageRequestBuilder<'a> {
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
pub struct XpackNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> XpackNamespaceClient<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        XpackNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/info-api.html"]
    pub fn info(&self) -> XpackInfoRequestBuilder {
        XpackInfoRequestBuilder::default()
    }
    #[doc = "Retrieve information about xpack features usage"]
    pub fn usage(&self) -> XpackUsageRequestBuilder {
        XpackUsageRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Xpack APIs"]
    pub fn xpack(&self) -> XpackNamespaceClient {
        XpackNamespaceClient::new(self)
    }
}
