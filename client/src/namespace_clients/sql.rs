

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[Default]
pub struct SqlClearCursorRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> SqlClearCursorRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SqlClearCursorRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SqlClearCursorRequestBuilder<'a> {
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
pub struct SqlQueryRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    format: &'a str,
}
impl<'a> SqlQueryRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SqlQueryRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SqlQueryRequestBuilder<'a> {
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
pub struct SqlTranslateRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> SqlTranslateRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SqlTranslateRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SqlTranslateRequestBuilder<'a> {
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
#[doc = "Sql APIs"]
pub struct SqlNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> SqlNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SqlNamespaceClient { client }
    }
    #[doc = "Clear SQL cursor"]
    pub fn clear_cursor(&self) -> SqlClearCursorRequestBuilder {
        SqlClearCursorRequestBuilder::default()
    }
    #[doc = "Execute SQL"]
    pub fn query(&self) -> SqlQueryRequestBuilder {
        SqlQueryRequestBuilder::default()
    }
    #[doc = "Translate SQL into Elasticsearch queries"]
    pub fn translate(&self) -> SqlTranslateRequestBuilder {
        SqlTranslateRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Sql APIs"]
    pub fn sql(&self) -> SqlNamespaceClient {
        SqlNamespaceClient::new(self)
    }
}
