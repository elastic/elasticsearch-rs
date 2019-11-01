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
    client::{Elasticsearch, Sender},
    enums::*,
    error::ElasticsearchError,
    http_method::HttpMethod,
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
pub struct DataFrameDeleteDataFrameTransform {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    transform_id: String,
}
impl DataFrameDeleteDataFrameTransform {
    pub fn new(client: Elasticsearch, transform_id: String) -> Self {
        DataFrameDeleteDataFrameTransform {
            client,
            transform_id: transform_id,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
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
    #[doc = "The id of the transform to delete"]
    pub fn transform_id(mut self, transform_id: String) -> Self {
        self.transform_id = transform_id;
        self
    }
}
impl Sender for DataFrameDeleteDataFrameTransform {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let transform_id = self.transform_id;
            let mut p = String::with_capacity(24usize + transform_id.len());
            p.push_str("/_data_frame/transforms/");
            p.push_str(transform_id.as_ref());
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
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
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
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
            allow_no_match: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            source: None,
            transform_id: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no data frame transforms. (This includes `_all` string or when no data frame transforms have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: Option<bool>) -> Self {
        self.allow_no_match = allow_no_match;
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
    #[doc = "skips a number of transform configs, defaults to 0"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
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
    #[doc = "specifies a max number of transforms to get, defaults to 100"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "The id or comma delimited list of id expressions of the transforms to get, '_all' or '*' implies get all transforms"]
    pub fn transform_id(mut self, transform_id: Option<String>) -> Self {
        self.transform_id = transform_id;
        self
    }
}
impl Sender for DataFrameGetDataFrameTransform {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.transform_id {
            Some(transform_id) => {
                let transform_id = transform_id;
                let mut p = String::with_capacity(24usize + transform_id.len());
                p.push_str("/_data_frame/transforms/");
                p.push_str(transform_id.as_ref());
                std::borrow::Cow::Owned(p)
            }
            None => std::borrow::Cow::Borrowed("/_data_frame/transforms"),
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<String>,
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
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
    transform_id: String,
}
impl DataFrameGetDataFrameTransformStats {
    pub fn new(client: Elasticsearch, transform_id: String) -> Self {
        DataFrameGetDataFrameTransformStats {
            client,
            transform_id: transform_id,
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
    pub fn allow_no_match(mut self, allow_no_match: Option<bool>) -> Self {
        self.allow_no_match = allow_no_match;
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
    #[doc = "skips a number of transform stats, defaults to 0"]
    pub fn from(mut self, from: Option<i64>) -> Self {
        self.from = from;
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
    #[doc = "specifies a max number of transform stats to get, defaults to 100"]
    pub fn size(mut self, size: Option<i64>) -> Self {
        self.size = size;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "The id of the transform for which to get stats. '_all' or '*' implies all transforms"]
    pub fn transform_id(mut self, transform_id: String) -> Self {
        self.transform_id = transform_id;
        self
    }
}
impl Sender for DataFrameGetDataFrameTransformStats {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let transform_id = self.transform_id;
            let mut p = String::with_capacity(31usize + transform_id.len());
            p.push_str("/_data_frame/transforms/");
            p.push_str(transform_id.as_ref());
            p.push_str("/_stats");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from")]
                from: Option<i64>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i64>,
                #[serde(rename = "source")]
                source: Option<String>,
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct DataFramePreviewDataFrameTransform<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> DataFramePreviewDataFrameTransform<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        DataFramePreviewDataFrameTransform {
            client,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
impl<B> Sender for DataFramePreviewDataFrameTransform<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = std::borrow::Cow::Borrowed("/_data_frame/transforms/_preview");
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct DataFramePutDataFrameTransform<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    transform_id: String,
}
impl<B> DataFramePutDataFrameTransform<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, transform_id: String) -> Self {
        DataFramePutDataFrameTransform {
            client,
            transform_id: transform_id,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
    #[doc = "The id of the new transform."]
    pub fn transform_id(mut self, transform_id: String) -> Self {
        self.transform_id = transform_id;
        self
    }
}
impl<B> Sender for DataFramePutDataFrameTransform<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let transform_id = self.transform_id;
            let mut p = String::with_capacity(24usize + transform_id.len());
            p.push_str("/_data_frame/transforms/");
            p.push_str(transform_id.as_ref());
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct DataFrameStartDataFrameTransform<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    transform_id: String,
}
impl<B> DataFrameStartDataFrameTransform<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, transform_id: String) -> Self {
        DataFrameStartDataFrameTransform {
            client,
            transform_id: transform_id,
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
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
    #[doc = "Controls the time to wait for the transform to start"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "The id of the transform to start"]
    pub fn transform_id(mut self, transform_id: String) -> Self {
        self.transform_id = transform_id;
        self
    }
}
impl<B> Sender for DataFrameStartDataFrameTransform<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let transform_id = self.transform_id;
            let mut p = String::with_capacity(31usize + transform_id.len());
            p.push_str("/_data_frame/transforms/");
            p.push_str(transform_id.as_ref());
            p.push_str("/_start");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
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
            .send(method, &path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct DataFrameStopDataFrameTransform<B> {
    client: Elasticsearch,
    allow_no_match: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    transform_id: String,
    wait_for_completion: Option<bool>,
}
impl<B> DataFrameStopDataFrameTransform<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, transform_id: String) -> Self {
        DataFrameStopDataFrameTransform {
            client,
            transform_id: transform_id,
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
    pub fn allow_no_match(mut self, allow_no_match: Option<bool>) -> Self {
        self.allow_no_match = allow_no_match;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
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
    #[doc = "Controls the time to wait until the transform has stopped. Default to 30 seconds"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "The id of the transform to stop"]
    pub fn transform_id(mut self, transform_id: String) -> Self {
        self.transform_id = transform_id;
        self
    }
    #[doc = "Whether to wait for the transform to fully stop before returning or not. Default to false"]
    pub fn wait_for_completion(mut self, wait_for_completion: Option<bool>) -> Self {
        self.wait_for_completion = wait_for_completion;
        self
    }
}
impl<B> Sender for DataFrameStopDataFrameTransform<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = {
            let transform_id = self.transform_id;
            let mut p = String::with_capacity(30usize + transform_id.len());
            p.push_str("/_data_frame/transforms/");
            p.push_str(transform_id.as_ref());
            p.push_str("/_stop");
            std::borrow::Cow::Owned(p)
        };
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_completion")]
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
            .send(method, &path, query_string.as_ref(), body)?;
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
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/delete-data-frame-transform.html"]
    pub fn delete_data_frame_transform(
        &self,
        transform_id: String,
    ) -> DataFrameDeleteDataFrameTransform {
        DataFrameDeleteDataFrameTransform::new(self.client.clone(), transform_id)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/get-data-frame-transform.html"]
    pub fn get_data_frame_transform(&self) -> DataFrameGetDataFrameTransform {
        DataFrameGetDataFrameTransform::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/get-data-frame-transform-stats.html"]
    pub fn get_data_frame_transform_stats(
        &self,
        transform_id: String,
    ) -> DataFrameGetDataFrameTransformStats {
        DataFrameGetDataFrameTransformStats::new(self.client.clone(), transform_id)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/preview-data-frame-transform.html"]
    pub fn preview_data_frame_transform<B>(&self) -> DataFramePreviewDataFrameTransform<B>
    where
        B: Serialize,
    {
        DataFramePreviewDataFrameTransform::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/put-data-frame-transform.html"]
    pub fn put_data_frame_transform<B>(
        &self,
        transform_id: String,
    ) -> DataFramePutDataFrameTransform<B>
    where
        B: Serialize,
    {
        DataFramePutDataFrameTransform::new(self.client.clone(), transform_id)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/start-data-frame-transform.html"]
    pub fn start_data_frame_transform<B>(
        &self,
        transform_id: String,
    ) -> DataFrameStartDataFrameTransform<B>
    where
        B: Serialize,
    {
        DataFrameStartDataFrameTransform::new(self.client.clone(), transform_id)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/stop-data-frame-transform.html"]
    pub fn stop_data_frame_transform<B>(
        &self,
        transform_id: String,
    ) -> DataFrameStopDataFrameTransform<B>
    where
        B: Serialize,
    {
        DataFrameStopDataFrameTransform::new(self.client.clone(), transform_id)
    }
}
impl Elasticsearch {
    #[doc = "DataFrame APIs"]
    pub fn data_frame(&self) -> DataFrame {
        DataFrame::new(self.clone())
    }
}