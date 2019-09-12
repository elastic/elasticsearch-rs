

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[Default]
pub struct XpackInfoRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    categories: &'a Vec<String>,
}
impl<'a> XpackInfoRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
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
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct XpackUsageRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    master_timeout: &'a str,
}
impl<'a> XpackUsageRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
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
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[doc = "Xpack APIs"]
pub struct XpackNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> XpackNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
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
