// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
use crate::{
    client::Elasticsearch, enums::*, error::ElasticsearchError, http_method::HttpMethod,
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Data Frame Delete Data Frame Transform API"]
pub enum DataFrameDeleteDataFrameTransformUrlParts<'a> {
    TransformId(&'a str),
}
impl<'a> DataFrameDeleteDataFrameTransformUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DataFrameDeleteDataFrameTransformUrlParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(24usize + transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(transform_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Data Frame Delete Data Frame Transform API"]
pub struct DataFrameDeleteDataFrameTransform<'a> {
    client: Elasticsearch,
    parts: DataFrameDeleteDataFrameTransformUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> DataFrameDeleteDataFrameTransform<'a> {
    pub fn new(
        client: Elasticsearch,
        parts: DataFrameDeleteDataFrameTransformUrlParts<'a>,
    ) -> Self {
        DataFrameDeleteDataFrameTransform {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            force: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "When `true`, the transform is deleted regardless of its current state. The default value is `false`, meaning that the transform must be `stopped` before it can be deleted."]
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Data Frame Delete Data Frame Transform API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
                force: Option<bool>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                force: self.force,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Data Frame Get Data Frame Transform API"]
pub enum DataFrameGetDataFrameTransformUrlParts<'a> {
    TransformId(&'a str),
    None,
}
impl<'a> DataFrameGetDataFrameTransformUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DataFrameGetDataFrameTransformUrlParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(24usize + transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(transform_id.as_ref());
                p.into()
            }
            DataFrameGetDataFrameTransformUrlParts::None => "/_data_frame/transforms".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Data Frame Get Data Frame Transform API"]
pub struct DataFrameGetDataFrameTransform<'a> {
    client: Elasticsearch,
    parts: DataFrameGetDataFrameTransformUrlParts<'a>,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<&'a str>,
}
impl<'a> DataFrameGetDataFrameTransform<'a> {
    pub fn new(client: Elasticsearch, parts: DataFrameGetDataFrameTransformUrlParts<'a>) -> Self {
        DataFrameGetDataFrameTransform {
            client,
            parts,
            allow_no_match: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no data frame transforms. (This includes `_all` string or when no data frame transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: bool) -> Self {
        self.allow_no_match = Some(allow_no_match);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of transform configs, defaults to 0"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "specifies a max number of transforms to get, defaults to 100"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Data Frame Get Data Frame Transform API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "allow_no_match", skip_serializing_if = "Option::is_none")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i32>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i32>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                allow_no_match: self.allow_no_match,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                size: self.size,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Data Frame Get Data Frame Transform Stats API"]
pub enum DataFrameGetDataFrameTransformStatsUrlParts<'a> {
    TransformId(&'a str),
}
impl<'a> DataFrameGetDataFrameTransformStatsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DataFrameGetDataFrameTransformStatsUrlParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(31usize + transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(transform_id.as_ref());
                p.push_str("/_stats");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Data Frame Get Data Frame Transform Stats API"]
pub struct DataFrameGetDataFrameTransformStats<'a> {
    client: Elasticsearch,
    parts: DataFrameGetDataFrameTransformStatsUrlParts<'a>,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i64>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i64>,
    source: Option<&'a str>,
}
impl<'a> DataFrameGetDataFrameTransformStats<'a> {
    pub fn new(
        client: Elasticsearch,
        parts: DataFrameGetDataFrameTransformStatsUrlParts<'a>,
    ) -> Self {
        DataFrameGetDataFrameTransformStats {
            client,
            parts,
            allow_no_match: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no data frame transforms. (This includes `_all` string or when no data frame transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: bool) -> Self {
        self.allow_no_match = Some(allow_no_match);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of transform stats, defaults to 0"]
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "specifies a max number of transform stats to get, defaults to 100"]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Data Frame Get Data Frame Transform Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "allow_no_match", skip_serializing_if = "Option::is_none")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i64>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i64>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                allow_no_match: self.allow_no_match,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                size: self.size,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Data Frame Preview Data Frame Transform API"]
pub enum DataFramePreviewDataFrameTransformUrlParts {
    None,
}
impl DataFramePreviewDataFrameTransformUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DataFramePreviewDataFrameTransformUrlParts::None => {
                "/_data_frame/transforms/_preview".into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Data Frame Preview Data Frame Transform API"]
pub struct DataFramePreviewDataFrameTransform<'a, B> {
    client: Elasticsearch,
    parts: DataFramePreviewDataFrameTransformUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> DataFramePreviewDataFrameTransform<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        DataFramePreviewDataFrameTransform {
            client,
            parts: DataFramePreviewDataFrameTransformUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> DataFramePreviewDataFrameTransform<'a, T>
    where
        T: Serialize,
    {
        DataFramePreviewDataFrameTransform {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Data Frame Preview Data Frame Transform API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Data Frame Put Data Frame Transform API"]
pub enum DataFramePutDataFrameTransformUrlParts<'a> {
    TransformId(&'a str),
}
impl<'a> DataFramePutDataFrameTransformUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DataFramePutDataFrameTransformUrlParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(24usize + transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(transform_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Data Frame Put Data Frame Transform API"]
pub struct DataFramePutDataFrameTransform<'a, B> {
    client: Elasticsearch,
    parts: DataFramePutDataFrameTransformUrlParts<'a>,
    body: Option<B>,
    defer_validation: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> DataFramePutDataFrameTransform<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: DataFramePutDataFrameTransformUrlParts<'a>) -> Self {
        DataFramePutDataFrameTransform {
            client,
            parts,
            body: None,
            defer_validation: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> DataFramePutDataFrameTransform<'a, T>
    where
        T: Serialize,
    {
        DataFramePutDataFrameTransform {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            defer_validation: self.defer_validation,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "If validations should be deferred until data frame transform starts, defaults to false."]
    pub fn defer_validation(mut self, defer_validation: bool) -> Self {
        self.defer_validation = Some(defer_validation);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Data Frame Put Data Frame Transform API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "defer_validation", skip_serializing_if = "Option::is_none")]
                defer_validation: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                defer_validation: self.defer_validation,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Data Frame Start Data Frame Transform API"]
pub enum DataFrameStartDataFrameTransformUrlParts<'a> {
    TransformId(&'a str),
}
impl<'a> DataFrameStartDataFrameTransformUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DataFrameStartDataFrameTransformUrlParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(31usize + transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(transform_id.as_ref());
                p.push_str("/_start");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Data Frame Start Data Frame Transform API"]
pub struct DataFrameStartDataFrameTransform<'a, B> {
    client: Elasticsearch,
    parts: DataFrameStartDataFrameTransformUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> DataFrameStartDataFrameTransform<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: DataFrameStartDataFrameTransformUrlParts<'a>) -> Self {
        DataFrameStartDataFrameTransform {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> DataFrameStartDataFrameTransform<'a, T>
    where
        T: Serialize,
    {
        DataFrameStartDataFrameTransform {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait for the transform to start"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Data Frame Start Data Frame Transform API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Data Frame Stop Data Frame Transform API"]
pub enum DataFrameStopDataFrameTransformUrlParts<'a> {
    TransformId(&'a str),
}
impl<'a> DataFrameStopDataFrameTransformUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DataFrameStopDataFrameTransformUrlParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(30usize + transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(transform_id.as_ref());
                p.push_str("/_stop");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Data Frame Stop Data Frame Transform API"]
pub struct DataFrameStopDataFrameTransform<'a, B> {
    client: Elasticsearch,
    parts: DataFrameStopDataFrameTransformUrlParts<'a>,
    allow_no_match: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_completion: Option<bool>,
}
impl<'a, B> DataFrameStopDataFrameTransform<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: DataFrameStopDataFrameTransformUrlParts<'a>) -> Self {
        DataFrameStopDataFrameTransform {
            client,
            parts,
            allow_no_match: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_completion: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no data frame transforms. (This includes `_all` string or when no data frame transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: bool) -> Self {
        self.allow_no_match = Some(allow_no_match);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> DataFrameStopDataFrameTransform<'a, T>
    where
        T: Serialize,
    {
        DataFrameStopDataFrameTransform {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            allow_no_match: self.allow_no_match,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
            wait_for_completion: self.wait_for_completion,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait until the transform has stopped. Default to 30 seconds"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Whether to wait for the transform to fully stop before returning or not. Default to false"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous request to the Data Frame Stop Data Frame Transform API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "allow_no_match", skip_serializing_if = "Option::is_none")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
                #[serde(
                    rename = "wait_for_completion",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_match: self.allow_no_match,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Data Frame Update Data Frame Transform API"]
pub enum DataFrameUpdateDataFrameTransformUrlParts<'a> {
    TransformId(&'a str),
}
impl<'a> DataFrameUpdateDataFrameTransformUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            DataFrameUpdateDataFrameTransformUrlParts::TransformId(ref transform_id) => {
                let mut p = String::with_capacity(32usize + transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(transform_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Data Frame Update Data Frame Transform API"]
pub struct DataFrameUpdateDataFrameTransform<'a, B> {
    client: Elasticsearch,
    parts: DataFrameUpdateDataFrameTransformUrlParts<'a>,
    body: Option<B>,
    defer_validation: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> DataFrameUpdateDataFrameTransform<'a, B>
where
    B: Serialize,
{
    pub fn new(
        client: Elasticsearch,
        parts: DataFrameUpdateDataFrameTransformUrlParts<'a>,
    ) -> Self {
        DataFrameUpdateDataFrameTransform {
            client,
            parts,
            body: None,
            defer_validation: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> DataFrameUpdateDataFrameTransform<'a, T>
    where
        T: Serialize,
    {
        DataFrameUpdateDataFrameTransform {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            defer_validation: self.defer_validation,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "If validations should be deferred until data frame transform starts, defaults to false."]
    pub fn defer_validation(mut self, defer_validation: bool) -> Self {
        self.defer_validation = Some(defer_validation);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Data Frame Update Data Frame Transform API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "defer_validation", skip_serializing_if = "Option::is_none")]
                defer_validation: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
                defer_validation: self.defer_validation,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
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
    pub fn delete_data_frame_transform<'a>(
        &self,
        parts: DataFrameDeleteDataFrameTransformUrlParts<'a>,
    ) -> DataFrameDeleteDataFrameTransform<'a> {
        DataFrameDeleteDataFrameTransform::new(self.client.clone(), parts)
    }
    pub fn get_data_frame_transform<'a>(
        &self,
        parts: DataFrameGetDataFrameTransformUrlParts<'a>,
    ) -> DataFrameGetDataFrameTransform<'a> {
        DataFrameGetDataFrameTransform::new(self.client.clone(), parts)
    }
    pub fn get_data_frame_transform_stats<'a>(
        &self,
        parts: DataFrameGetDataFrameTransformStatsUrlParts<'a>,
    ) -> DataFrameGetDataFrameTransformStats<'a> {
        DataFrameGetDataFrameTransformStats::new(self.client.clone(), parts)
    }
    pub fn preview_data_frame_transform<'a>(&self) -> DataFramePreviewDataFrameTransform<'a, ()> {
        DataFramePreviewDataFrameTransform::new(self.client.clone())
    }
    pub fn put_data_frame_transform<'a>(
        &self,
        parts: DataFramePutDataFrameTransformUrlParts<'a>,
    ) -> DataFramePutDataFrameTransform<'a, ()> {
        DataFramePutDataFrameTransform::new(self.client.clone(), parts)
    }
    pub fn start_data_frame_transform<'a>(
        &self,
        parts: DataFrameStartDataFrameTransformUrlParts<'a>,
    ) -> DataFrameStartDataFrameTransform<'a, ()> {
        DataFrameStartDataFrameTransform::new(self.client.clone(), parts)
    }
    pub fn stop_data_frame_transform<'a>(
        &self,
        parts: DataFrameStopDataFrameTransformUrlParts<'a>,
    ) -> DataFrameStopDataFrameTransform<'a, ()> {
        DataFrameStopDataFrameTransform::new(self.client.clone(), parts)
    }
    pub fn update_data_frame_transform<'a>(
        &self,
        parts: DataFrameUpdateDataFrameTransformUrlParts<'a>,
    ) -> DataFrameUpdateDataFrameTransform<'a, ()> {
        DataFrameUpdateDataFrameTransform::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "DataFrame APIs"]
    pub fn data_frame(&self) -> DataFrame {
        DataFrame::new(self.clone())
    }
}
