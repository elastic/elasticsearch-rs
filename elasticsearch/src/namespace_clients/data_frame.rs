use super::super::client::Elasticsearch;
use super::super::enums::*;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct DataFrameDeleteDataFrameTransform {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    transform_id: Option<String>,
}
impl DataFrameDeleteDataFrameTransform {
    pub fn new(client: Elasticsearch) -> Self {
        DataFrameDeleteDataFrameTransform {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for DataFrameDeleteDataFrameTransform {
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
pub struct DataFrameGetDataFrameTransform {
    client: Elasticsearch,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<String>,
    transform_id: Option<String>,
}
impl DataFrameGetDataFrameTransform {
    pub fn new(client: Elasticsearch) -> Self {
        DataFrameGetDataFrameTransform {
            client,
            ..Default::default()
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no data frame transforms. (This includes `_all` string or when no data frame transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: Option<bool>) -> Self {
        self.allow_no_match = allow_no_match;
        self
    }
    #[doc = "skips a number of transform configs, defaults to 0"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
        self
    }
    #[doc = "specifies a max number of transforms to get, defaults to 100"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for DataFrameGetDataFrameTransform {
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
pub struct DataFrameGetDataFrameTransformStats {
    client: Elasticsearch,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i64>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i64>,
    source: Option<String>,
    transform_id: Option<String>,
}
impl DataFrameGetDataFrameTransformStats {
    pub fn new(client: Elasticsearch) -> Self {
        DataFrameGetDataFrameTransformStats {
            client,
            ..Default::default()
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no data frame transforms. (This includes `_all` string or when no data frame transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: Option<bool>) -> Self {
        self.allow_no_match = allow_no_match;
        self
    }
    #[doc = "skips a number of transform stats, defaults to 0"]
    pub fn from(mut self, from: Option<i64>) -> Self {
        self.from = from;
        self
    }
    #[doc = "specifies a max number of transform stats to get, defaults to 100"]
    pub fn size(mut self, size: Option<i64>) -> Self {
        self.size = size;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for DataFrameGetDataFrameTransformStats {
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
pub struct DataFramePreviewDataFrameTransform {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl DataFramePreviewDataFrameTransform {
    pub fn new(client: Elasticsearch) -> Self {
        DataFramePreviewDataFrameTransform {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for DataFramePreviewDataFrameTransform {
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
pub struct DataFramePutDataFrameTransform {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    transform_id: Option<String>,
}
impl DataFramePutDataFrameTransform {
    pub fn new(client: Elasticsearch) -> Self {
        DataFramePutDataFrameTransform {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for DataFramePutDataFrameTransform {
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
pub struct DataFrameStartDataFrameTransform {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    transform_id: Option<String>,
}
impl DataFrameStartDataFrameTransform {
    pub fn new(client: Elasticsearch) -> Self {
        DataFrameStartDataFrameTransform {
            client,
            ..Default::default()
        }
    }
    #[doc = "Controls the time to wait for the transform to start"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for DataFrameStartDataFrameTransform {
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
pub struct DataFrameStopDataFrameTransform {
    client: Elasticsearch,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    transform_id: Option<String>,
    wait_for_completion: Option<bool>,
}
impl DataFrameStopDataFrameTransform {
    pub fn new(client: Elasticsearch) -> Self {
        DataFrameStopDataFrameTransform {
            client,
            ..Default::default()
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no data frame transforms. (This includes `_all` string or when no data frame transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: Option<bool>) -> Self {
        self.allow_no_match = allow_no_match;
        self
    }
    #[doc = "Controls the time to wait until the transform has stopped. Default to 30 seconds"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Whether to wait for the transform to fully stop before returning or not. Default to false"]
    pub fn wait_for_completion(mut self, wait_for_completion: Option<bool>) -> Self {
        self.wait_for_completion = wait_for_completion;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for DataFrameStopDataFrameTransform {
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
pub struct DataFrame {
    client: Elasticsearch,
}
impl DataFrame {
    pub fn new(client: Elasticsearch) -> Self {
        DataFrame { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/delete-data-frame-transform.html"]
    pub fn delete_data_frame_transform(&self) -> DataFrameDeleteDataFrameTransform {
        DataFrameDeleteDataFrameTransform::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/get-data-frame-transform.html"]
    pub fn get_data_frame_transform(&self) -> DataFrameGetDataFrameTransform {
        DataFrameGetDataFrameTransform::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/get-data-frame-transform-stats.html"]
    pub fn get_data_frame_transform_stats(&self) -> DataFrameGetDataFrameTransformStats {
        DataFrameGetDataFrameTransformStats::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/preview-data-frame-transform.html"]
    pub fn preview_data_frame_transform(&self) -> DataFramePreviewDataFrameTransform {
        DataFramePreviewDataFrameTransform::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/put-data-frame-transform.html"]
    pub fn put_data_frame_transform(&self) -> DataFramePutDataFrameTransform {
        DataFramePutDataFrameTransform::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/start-data-frame-transform.html"]
    pub fn start_data_frame_transform(&self) -> DataFrameStartDataFrameTransform {
        DataFrameStartDataFrameTransform::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/stop-data-frame-transform.html"]
    pub fn stop_data_frame_transform(&self) -> DataFrameStopDataFrameTransform {
        DataFrameStopDataFrameTransform::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "DataFrame APIs"]
    pub fn data_frame(&self) -> DataFrame {
        DataFrame::new(self.clone())
    }
}