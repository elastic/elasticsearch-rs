

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct DataFrameDeleteDataFrameTransformRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> DataFrameDeleteDataFrameTransformRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        DataFrameDeleteDataFrameTransformRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for DataFrameDeleteDataFrameTransformRequestBuilder<'a> {
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
pub struct DataFrameGetDataFrameTransformRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_match: Option<&'a bool>,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
impl<'a> DataFrameGetDataFrameTransformRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        DataFrameGetDataFrameTransformRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for DataFrameGetDataFrameTransformRequestBuilder<'a> {
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
pub struct DataFrameGetDataFrameTransformStatsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_match: Option<&'a bool>,
    from: Option<&'a i64>,
    size: Option<&'a i64>,
}
impl<'a> DataFrameGetDataFrameTransformStatsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        DataFrameGetDataFrameTransformStatsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for DataFrameGetDataFrameTransformStatsRequestBuilder<'a> {
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
pub struct DataFramePreviewDataFrameTransformRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> DataFramePreviewDataFrameTransformRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        DataFramePreviewDataFrameTransformRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for DataFramePreviewDataFrameTransformRequestBuilder<'a> {
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
pub struct DataFramePutDataFrameTransformRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> DataFramePutDataFrameTransformRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        DataFramePutDataFrameTransformRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for DataFramePutDataFrameTransformRequestBuilder<'a> {
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
pub struct DataFrameStartDataFrameTransformRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    timeout: &'a str,
}
impl<'a> DataFrameStartDataFrameTransformRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        DataFrameStartDataFrameTransformRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for DataFrameStartDataFrameTransformRequestBuilder<'a> {
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
pub struct DataFrameStopDataFrameTransformRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_match: Option<&'a bool>,
    timeout: &'a str,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> DataFrameStopDataFrameTransformRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        DataFrameStopDataFrameTransformRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for DataFrameStopDataFrameTransformRequestBuilder<'a> {
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
#[doc = "DataFrame APIs"]
pub struct DataFrameNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> DataFrameNamespaceClient<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        DataFrameNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/delete-data-frame-transform.html"]
    pub fn delete_data_frame_transform(&self) -> DataFrameDeleteDataFrameTransformRequestBuilder {
        DataFrameDeleteDataFrameTransformRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/get-data-frame-transform.html"]
    pub fn get_data_frame_transform(&self) -> DataFrameGetDataFrameTransformRequestBuilder {
        DataFrameGetDataFrameTransformRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/get-data-frame-transform-stats.html"]
    pub fn get_data_frame_transform_stats(
        &self,
    ) -> DataFrameGetDataFrameTransformStatsRequestBuilder {
        DataFrameGetDataFrameTransformStatsRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/preview-data-frame-transform.html"]
    pub fn preview_data_frame_transform(&self) -> DataFramePreviewDataFrameTransformRequestBuilder {
        DataFramePreviewDataFrameTransformRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/put-data-frame-transform.html"]
    pub fn put_data_frame_transform(&self) -> DataFramePutDataFrameTransformRequestBuilder {
        DataFramePutDataFrameTransformRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/start-data-frame-transform.html"]
    pub fn start_data_frame_transform(&self) -> DataFrameStartDataFrameTransformRequestBuilder {
        DataFrameStartDataFrameTransformRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/stop-data-frame-transform.html"]
    pub fn stop_data_frame_transform(&self) -> DataFrameStopDataFrameTransformRequestBuilder {
        DataFrameStopDataFrameTransformRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "DataFrame APIs"]
    pub fn data_frame(&self) -> DataFrameNamespaceClient {
        DataFrameNamespaceClient::new(self)
    }
}
