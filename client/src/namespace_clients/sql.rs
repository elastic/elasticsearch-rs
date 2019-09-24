

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct SqlClearCursorBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SqlClearCursorBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SqlClearCursorBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SqlClearCursorBuilder {
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
pub struct SqlQueryBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    format: Option<String>,
}
impl SqlQueryBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SqlQueryBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SqlQueryBuilder {
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
pub struct SqlTranslateBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl SqlTranslateBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SqlTranslateBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SqlTranslateBuilder {
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
pub struct SqlClient {
    client: ElasticsearchClient,
}
impl SqlClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        SqlClient { client }
    }
    #[doc = "Clear SQL cursor"]
    pub fn clear_cursor(&self) -> SqlClearCursorBuilder {
        SqlClearCursorBuilder::new(self.client.clone())
    }
    #[doc = "Execute SQL"]
    pub fn query(&self) -> SqlQueryBuilder {
        SqlQueryBuilder::new(self.client.clone())
    }
    #[doc = "Translate SQL into Elasticsearch queries"]
    pub fn translate(&self) -> SqlTranslateBuilder {
        SqlTranslateBuilder::new(self.client.clone())
    }
}
impl ElasticsearchClient {
    #[doc = "Sql APIs"]
    pub fn sql(&self) -> SqlClient {
        SqlClient::new(self.clone())
    }
}
