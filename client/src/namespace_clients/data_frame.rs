

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct DataFrameDeleteDataFrameTransformBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl DataFrameDeleteDataFrameTransformBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        DataFrameDeleteDataFrameTransformBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for DataFrameDeleteDataFrameTransformBuilder {
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
pub struct DataFrameGetDataFrameTransformBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_match: Option<bool>,
    from: Option<i32>,
    size: Option<i32>,
}
impl DataFrameGetDataFrameTransformBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        DataFrameGetDataFrameTransformBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for DataFrameGetDataFrameTransformBuilder {
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
pub struct DataFrameGetDataFrameTransformStatsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_match: Option<bool>,
    from: Option<i64>,
    size: Option<i64>,
}
impl DataFrameGetDataFrameTransformStatsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        DataFrameGetDataFrameTransformStatsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for DataFrameGetDataFrameTransformStatsBuilder {
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
pub struct DataFramePreviewDataFrameTransformBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl DataFramePreviewDataFrameTransformBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        DataFramePreviewDataFrameTransformBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for DataFramePreviewDataFrameTransformBuilder {
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
pub struct DataFramePutDataFrameTransformBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl DataFramePutDataFrameTransformBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        DataFramePutDataFrameTransformBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for DataFramePutDataFrameTransformBuilder {
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
pub struct DataFrameStartDataFrameTransformBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl DataFrameStartDataFrameTransformBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        DataFrameStartDataFrameTransformBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for DataFrameStartDataFrameTransformBuilder {
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
pub struct DataFrameStopDataFrameTransformBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_match: Option<bool>,
    timeout: Option<String>,
    wait_for_completion: Option<bool>,
}
impl DataFrameStopDataFrameTransformBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        DataFrameStopDataFrameTransformBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for DataFrameStopDataFrameTransformBuilder {
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
pub struct DataFrameClient {
    client: ElasticsearchClient,
}
impl DataFrameClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        DataFrameClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/delete-data-frame-transform.html"]
    pub fn delete_data_frame_transform(&self) -> DataFrameDeleteDataFrameTransformBuilder {
        DataFrameDeleteDataFrameTransformBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/get-data-frame-transform.html"]
    pub fn get_data_frame_transform(&self) -> DataFrameGetDataFrameTransformBuilder {
        DataFrameGetDataFrameTransformBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/get-data-frame-transform-stats.html"]
    pub fn get_data_frame_transform_stats(&self) -> DataFrameGetDataFrameTransformStatsBuilder {
        DataFrameGetDataFrameTransformStatsBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/preview-data-frame-transform.html"]
    pub fn preview_data_frame_transform(&self) -> DataFramePreviewDataFrameTransformBuilder {
        DataFramePreviewDataFrameTransformBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/put-data-frame-transform.html"]
    pub fn put_data_frame_transform(&self) -> DataFramePutDataFrameTransformBuilder {
        DataFramePutDataFrameTransformBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/start-data-frame-transform.html"]
    pub fn start_data_frame_transform(&self) -> DataFrameStartDataFrameTransformBuilder {
        DataFrameStartDataFrameTransformBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/stop-data-frame-transform.html"]
    pub fn stop_data_frame_transform(&self) -> DataFrameStopDataFrameTransformBuilder {
        DataFrameStopDataFrameTransformBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "DataFrame APIs"]
    pub fn data_frame(&self) -> DataFrameClient {
        DataFrameClient::new(self.clone())
    }
}
