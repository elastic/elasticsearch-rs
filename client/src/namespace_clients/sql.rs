

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct SqlClearCursorRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SqlClearCursorRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
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
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct SqlQueryRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    format: &'a str,
}
impl<'a> SqlQueryRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
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
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct SqlTranslateRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> SqlTranslateRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
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
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[doc = "Sql APIs"]
pub struct SqlNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> SqlNamespaceClient<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
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
