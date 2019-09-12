

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct DataFrameDeleteDataFrameTransformRequest<'a> {}
pub struct DataFrameDeleteDataFrameTransformRequestBuilder<'a> {}
impl<'a> DataFrameDeleteDataFrameTransformRequestBuilder<'a> {
    pub fn build(&self) -> DataFrameDeleteDataFrameTransformRequest<'a> {
        DataFrameDeleteDataFrameTransformRequest {}
    }
}
pub struct DataFrameGetDataFrameTransformRequest<'a> {
    allow_no_match: Option<&'a bool>,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
pub struct DataFrameGetDataFrameTransformRequestBuilder<'a> {
    allow_no_match: Option<&'a bool>,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
impl<'a> DataFrameGetDataFrameTransformRequestBuilder<'a> {
    pub fn build(&self) -> DataFrameGetDataFrameTransformRequest<'a> {
        DataFrameGetDataFrameTransformRequest {
            allow_no_match: self.allow_no_match,
            from: self.from,
            size: self.size,
        }
    }
}
pub struct DataFrameGetDataFrameTransformStatsRequest<'a> {
    allow_no_match: Option<&'a bool>,
    from: Option<&'a i64>,
    size: Option<&'a i64>,
}
pub struct DataFrameGetDataFrameTransformStatsRequestBuilder<'a> {
    allow_no_match: Option<&'a bool>,
    from: Option<&'a i64>,
    size: Option<&'a i64>,
}
impl<'a> DataFrameGetDataFrameTransformStatsRequestBuilder<'a> {
    pub fn build(&self) -> DataFrameGetDataFrameTransformStatsRequest<'a> {
        DataFrameGetDataFrameTransformStatsRequest {
            allow_no_match: self.allow_no_match,
            from: self.from,
            size: self.size,
        }
    }
}
pub struct DataFramePreviewDataFrameTransformRequest<'a> {}
pub struct DataFramePreviewDataFrameTransformRequestBuilder<'a> {}
impl<'a> DataFramePreviewDataFrameTransformRequestBuilder<'a> {
    pub fn build(&self) -> DataFramePreviewDataFrameTransformRequest<'a> {
        DataFramePreviewDataFrameTransformRequest {}
    }
}
pub struct DataFramePutDataFrameTransformRequest<'a> {}
pub struct DataFramePutDataFrameTransformRequestBuilder<'a> {}
impl<'a> DataFramePutDataFrameTransformRequestBuilder<'a> {
    pub fn build(&self) -> DataFramePutDataFrameTransformRequest<'a> {
        DataFramePutDataFrameTransformRequest {}
    }
}
pub struct DataFrameStartDataFrameTransformRequest<'a> {
    timeout: &'a String,
}
pub struct DataFrameStartDataFrameTransformRequestBuilder<'a> {
    timeout: &'a String,
}
impl<'a> DataFrameStartDataFrameTransformRequestBuilder<'a> {
    pub fn build(&self) -> DataFrameStartDataFrameTransformRequest<'a> {
        DataFrameStartDataFrameTransformRequest {
            timeout: self.timeout,
        }
    }
}
pub struct DataFrameStopDataFrameTransformRequest<'a> {
    allow_no_match: Option<&'a bool>,
    timeout: &'a String,
    wait_for_completion: Option<&'a bool>,
}
pub struct DataFrameStopDataFrameTransformRequestBuilder<'a> {
    allow_no_match: Option<&'a bool>,
    timeout: &'a String,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> DataFrameStopDataFrameTransformRequestBuilder<'a> {
    pub fn build(&self) -> DataFrameStopDataFrameTransformRequest<'a> {
        DataFrameStopDataFrameTransformRequest {
            allow_no_match: self.allow_no_match,
            timeout: self.timeout,
            wait_for_completion: self.wait_for_completion,
        }
    }
}
#[doc = "DataFrame APIs"]
pub struct DataFrameNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> DataFrameNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        DataFrameNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/delete-data-frame-transform.html"]
    pub fn delete_data_frame_transform(
        &self,
        request: &DataFrameDeleteDataFrameTransformRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_data_frame/transforms/{transform_id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/get-data-frame-transform.html"]
    pub fn get_data_frame_transform(
        &self,
        request: &DataFrameGetDataFrameTransformRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_data_frame/transforms/{transform_id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/get-data-frame-transform-stats.html"]
    pub fn get_data_frame_transform_stats(
        &self,
        request: &DataFrameGetDataFrameTransformStatsRequest,
    ) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_data_frame/transforms/{transform_id}/_stats",
        )
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/preview-data-frame-transform.html"]
    pub fn preview_data_frame_transform(
        &self,
        request: &DataFramePreviewDataFrameTransformRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_data_frame/transforms/_preview")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/put-data-frame-transform.html"]
    pub fn put_data_frame_transform(
        &self,
        request: &DataFramePutDataFrameTransformRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_data_frame/transforms/{transform_id}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/start-data-frame-transform.html"]
    pub fn start_data_frame_transform(
        &self,
        request: &DataFrameStartDataFrameTransformRequest,
    ) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_data_frame/transforms/{transform_id}/_start",
        )
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/stop-data-frame-transform.html"]
    pub fn stop_data_frame_transform(
        &self,
        request: &DataFrameStopDataFrameTransformRequest,
    ) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_data_frame/transforms/{transform_id}/_stop",
        )
    }
}
impl ElasticsearchClient {
    #[doc = "DataFrame APIs"]
    pub fn data_frame(&self) -> DataFrameNamespaceClient {
        DataFrameNamespaceClient::new(self)
    }
}
