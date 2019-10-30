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
pub struct MlCloseJob<B> {
    client: Elasticsearch,
    allow_no_jobs: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    force: Option<bool>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> MlCloseJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlCloseJob {
            client,
            job_id: job_id,
            allow_no_jobs: None,
            body: None,
            error_trace: None,
            filter_path: None,
            force: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: Option<bool>) -> Self {
        self.allow_no_jobs = allow_no_jobs;
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
    #[doc = "True if the job should be forcefully closed"]
    pub fn force(mut self, force: Option<bool>) -> Self {
        self.force = force;
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
    #[doc = "Controls the time to wait until a job has closed. Default to 30 minutes"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl<B> Sender for MlCloseJob<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/_close";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_jobs")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_jobs: self.allow_no_jobs,
                force: self.force,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlDeleteCalendar {
    client: Elasticsearch,
    calendar_id: String,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteCalendar {
    pub fn new(client: Elasticsearch, calendar_id: String) -> Self {
        MlDeleteCalendar {
            client,
            calendar_id: calendar_id,
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
}
impl Sender for MlDeleteCalendar {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/calendars/{calendar_id}";
        let method = HttpMethod::Delete;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlDeleteCalendarEvent {
    client: Elasticsearch,
    calendar_id: String,
    error_trace: Option<bool>,
    event_id: String,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteCalendarEvent {
    pub fn new(client: Elasticsearch, calendar_id: String, event_id: String) -> Self {
        MlDeleteCalendarEvent {
            client,
            calendar_id: calendar_id,
            event_id: event_id,
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
}
impl Sender for MlDeleteCalendarEvent {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/calendars/{calendar_id}/events/{event_id}";
        let method = HttpMethod::Delete;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlDeleteCalendarJob {
    client: Elasticsearch,
    calendar_id: String,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteCalendarJob {
    pub fn new(client: Elasticsearch, calendar_id: String, job_id: String) -> Self {
        MlDeleteCalendarJob {
            client,
            calendar_id: calendar_id,
            job_id: job_id,
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
}
impl Sender for MlDeleteCalendarJob {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/calendars/{calendar_id}/jobs/{job_id}";
        let method = HttpMethod::Delete;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlDeleteDataFrameAnalytics {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteDataFrameAnalytics {
    pub fn new(client: Elasticsearch, id: String) -> Self {
        MlDeleteDataFrameAnalytics {
            client,
            id: id,
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
}
impl Sender for MlDeleteDataFrameAnalytics {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/data_frame/analytics/{id}";
        let method = HttpMethod::Delete;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlDeleteDatafeed {
    client: Elasticsearch,
    datafeed_id: String,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteDatafeed {
    pub fn new(client: Elasticsearch, datafeed_id: String) -> Self {
        MlDeleteDatafeed {
            client,
            datafeed_id: datafeed_id,
            error_trace: None,
            filter_path: None,
            force: None,
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
    #[doc = "True if the datafeed should be forcefully deleted"]
    pub fn force(mut self, force: Option<bool>) -> Self {
        self.force = force;
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
impl Sender for MlDeleteDatafeed {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/datafeeds/{datafeed_id}";
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "force")]
                force: Option<bool>,
            }
            let query_params = QueryParamsStruct { force: self.force };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlDeleteExpiredData {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteExpiredData {
    pub fn new(client: Elasticsearch) -> Self {
        MlDeleteExpiredData {
            client,
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
}
impl Sender for MlDeleteExpiredData {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/_delete_expired_data";
        let method = HttpMethod::Delete;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlDeleteFilter {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_id: String,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteFilter {
    pub fn new(client: Elasticsearch, filter_id: String) -> Self {
        MlDeleteFilter {
            client,
            filter_id: filter_id,
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
}
impl Sender for MlDeleteFilter {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/filters/{filter_id}";
        let method = HttpMethod::Delete;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlDeleteForecast {
    client: Elasticsearch,
    allow_no_forecasts: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    forecast_id: Option<String>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl MlDeleteForecast {
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlDeleteForecast {
            client,
            job_id: job_id,
            allow_no_forecasts: None,
            error_trace: None,
            filter_path: None,
            forecast_id: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Whether to ignore if `_all` matches no forecasts"]
    pub fn allow_no_forecasts(mut self, allow_no_forecasts: Option<bool>) -> Self {
        self.allow_no_forecasts = allow_no_forecasts;
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
    #[doc = "Controls the time to wait until the forecast(s) are deleted. Default to 30 seconds"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for MlDeleteForecast {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/_forecast";
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_forecasts")]
                allow_no_forecasts: Option<bool>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_forecasts: self.allow_no_forecasts,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlDeleteJob {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    force: Option<bool>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    source: Option<String>,
    wait_for_completion: Option<bool>,
}
impl MlDeleteJob {
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlDeleteJob {
            client,
            job_id: job_id,
            error_trace: None,
            filter_path: None,
            force: None,
            human: None,
            pretty: None,
            source: None,
            wait_for_completion: None,
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
    #[doc = "True if the job should be forcefully deleted"]
    pub fn force(mut self, force: Option<bool>) -> Self {
        self.force = force;
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
    #[doc = "Should this request wait until the operation has completed before returning"]
    pub fn wait_for_completion(mut self, wait_for_completion: Option<bool>) -> Self {
        self.wait_for_completion = wait_for_completion;
        self
    }
}
impl Sender for MlDeleteJob {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}";
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                force: self.force,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlDeleteModelSnapshot {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    snapshot_id: String,
    source: Option<String>,
}
impl MlDeleteModelSnapshot {
    pub fn new(client: Elasticsearch, job_id: String, snapshot_id: String) -> Self {
        MlDeleteModelSnapshot {
            client,
            job_id: job_id,
            snapshot_id: snapshot_id,
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
}
impl Sender for MlDeleteModelSnapshot {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/model_snapshots/{snapshot_id}";
        let method = HttpMethod::Delete;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlEvaluateDataFrame<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlEvaluateDataFrame<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        MlEvaluateDataFrame {
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
impl<B> Sender for MlEvaluateDataFrame<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/data_frame/_evaluate";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlFindFileStructure<B> {
    client: Elasticsearch,
    body: Option<B>,
    charset: Option<String>,
    column_names: Option<Vec<String>>,
    delimiter: Option<String>,
    error_trace: Option<bool>,
    explain: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<Format>,
    grok_pattern: Option<String>,
    has_header_row: Option<bool>,
    human: Option<bool>,
    line_merge_size_limit: Option<i32>,
    lines_to_sample: Option<i32>,
    pretty: Option<bool>,
    quote: Option<String>,
    should_trim_fields: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    timestamp_field: Option<String>,
    timestamp_format: Option<String>,
}
impl<B> MlFindFileStructure<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        MlFindFileStructure {
            client,
            body: None,
            charset: None,
            column_names: None,
            delimiter: None,
            error_trace: None,
            explain: None,
            filter_path: None,
            format: None,
            grok_pattern: None,
            has_header_row: None,
            human: None,
            line_merge_size_limit: None,
            lines_to_sample: None,
            pretty: None,
            quote: None,
            should_trim_fields: None,
            source: None,
            timeout: None,
            timestamp_field: None,
            timestamp_format: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Optional parameter to specify the character set of the file"]
    pub fn charset(mut self, charset: Option<String>) -> Self {
        self.charset = charset;
        self
    }
    #[doc = "Optional parameter containing a comma separated list of the column names for a delimited file"]
    pub fn column_names(mut self, column_names: Option<Vec<String>>) -> Self {
        self.column_names = column_names;
        self
    }
    #[doc = "Optional parameter to specify the delimiter character for a delimited file - must be a single character"]
    pub fn delimiter(mut self, delimiter: Option<String>) -> Self {
        self.delimiter = delimiter;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to include a commentary on how the structure was derived"]
    pub fn explain(mut self, explain: Option<bool>) -> Self {
        self.explain = explain;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Optional parameter to specify the high level file format"]
    pub fn format(mut self, format: Option<Format>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Optional parameter to specify the Grok pattern that should be used to extract fields from messages in a semi-structured text file"]
    pub fn grok_pattern(mut self, grok_pattern: Option<String>) -> Self {
        self.grok_pattern = grok_pattern;
        self
    }
    #[doc = "Optional parameter to specify whether a delimited file includes the column names in its first row"]
    pub fn has_header_row(mut self, has_header_row: Option<bool>) -> Self {
        self.has_header_row = has_header_row;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Maximum number of characters permitted in a single message when lines are merged to create messages."]
    pub fn line_merge_size_limit(mut self, line_merge_size_limit: Option<i32>) -> Self {
        self.line_merge_size_limit = line_merge_size_limit;
        self
    }
    #[doc = "How many lines of the file should be included in the analysis"]
    pub fn lines_to_sample(mut self, lines_to_sample: Option<i32>) -> Self {
        self.lines_to_sample = lines_to_sample;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Optional parameter to specify the quote character for a delimited file - must be a single character"]
    pub fn quote(mut self, quote: Option<String>) -> Self {
        self.quote = quote;
        self
    }
    #[doc = "Optional parameter to specify whether the values between delimiters in a delimited file should have whitespace trimmed from them"]
    pub fn should_trim_fields(mut self, should_trim_fields: Option<bool>) -> Self {
        self.should_trim_fields = should_trim_fields;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Timeout after which the analysis will be aborted"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Optional parameter to specify the timestamp field in the file"]
    pub fn timestamp_field(mut self, timestamp_field: Option<String>) -> Self {
        self.timestamp_field = timestamp_field;
        self
    }
    #[doc = "Optional parameter to specify the timestamp format in the file - may be either a Joda or Java time format"]
    pub fn timestamp_format(mut self, timestamp_format: Option<String>) -> Self {
        self.timestamp_format = timestamp_format;
        self
    }
}
impl<B> Sender for MlFindFileStructure<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/find_file_structure";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "charset")]
                charset: Option<String>,
                #[serde(rename = "column_names")]
                column_names: Option<Vec<String>>,
                #[serde(rename = "delimiter")]
                delimiter: Option<String>,
                #[serde(rename = "explain")]
                explain: Option<bool>,
                #[serde(rename = "format")]
                format: Option<Format>,
                #[serde(rename = "grok_pattern")]
                grok_pattern: Option<String>,
                #[serde(rename = "has_header_row")]
                has_header_row: Option<bool>,
                #[serde(rename = "line_merge_size_limit")]
                line_merge_size_limit: Option<i32>,
                #[serde(rename = "lines_to_sample")]
                lines_to_sample: Option<i32>,
                #[serde(rename = "quote")]
                quote: Option<String>,
                #[serde(rename = "should_trim_fields")]
                should_trim_fields: Option<bool>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "timestamp_field")]
                timestamp_field: Option<String>,
                #[serde(rename = "timestamp_format")]
                timestamp_format: Option<String>,
            }
            let query_params = QueryParamsStruct {
                charset: self.charset,
                column_names: self.column_names,
                delimiter: self.delimiter,
                explain: self.explain,
                format: self.format,
                grok_pattern: self.grok_pattern,
                has_header_row: self.has_header_row,
                line_merge_size_limit: self.line_merge_size_limit,
                lines_to_sample: self.lines_to_sample,
                quote: self.quote,
                should_trim_fields: self.should_trim_fields,
                timeout: self.timeout,
                timestamp_field: self.timestamp_field,
                timestamp_format: self.timestamp_format,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlFlushJob<B> {
    client: Elasticsearch,
    advance_time: Option<String>,
    body: Option<B>,
    calc_interim: Option<bool>,
    end: Option<String>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    skip_time: Option<String>,
    source: Option<String>,
    start: Option<String>,
}
impl<B> MlFlushJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlFlushJob {
            client,
            job_id: job_id,
            advance_time: None,
            body: None,
            calc_interim: None,
            end: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            skip_time: None,
            source: None,
            start: None,
        }
    }
    #[doc = "Advances time to the given value generating results and updating the model for the advanced interval"]
    pub fn advance_time(mut self, advance_time: Option<String>) -> Self {
        self.advance_time = advance_time;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Calculates interim results for the most recent bucket or all buckets within the latency period"]
    pub fn calc_interim(mut self, calc_interim: Option<bool>) -> Self {
        self.calc_interim = calc_interim;
        self
    }
    #[doc = "When used in conjunction with calc_interim, specifies the range of buckets on which to calculate interim results"]
    pub fn end(mut self, end: Option<String>) -> Self {
        self.end = end;
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
    #[doc = "Skips time to the given value without generating results or updating the model for the skipped interval"]
    pub fn skip_time(mut self, skip_time: Option<String>) -> Self {
        self.skip_time = skip_time;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "When used in conjunction with calc_interim, specifies the range of buckets on which to calculate interim results"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
}
impl<B> Sender for MlFlushJob<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/_flush";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "advance_time")]
                advance_time: Option<String>,
                #[serde(rename = "calc_interim")]
                calc_interim: Option<bool>,
                #[serde(rename = "end")]
                end: Option<String>,
                #[serde(rename = "skip_time")]
                skip_time: Option<String>,
                #[serde(rename = "start")]
                start: Option<String>,
            }
            let query_params = QueryParamsStruct {
                advance_time: self.advance_time,
                calc_interim: self.calc_interim,
                end: self.end,
                skip_time: self.skip_time,
                start: self.start,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlForecast<B> {
    client: Elasticsearch,
    body: Option<B>,
    duration: Option<String>,
    error_trace: Option<bool>,
    expires_in: Option<String>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlForecast<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlForecast {
            client,
            job_id: job_id,
            body: None,
            duration: None,
            error_trace: None,
            expires_in: None,
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
    #[doc = "The duration of the forecast"]
    pub fn duration(mut self, duration: Option<String>) -> Self {
        self.duration = duration;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "The time interval after which the forecast expires. Expired forecasts will be deleted at the first opportunity."]
    pub fn expires_in(mut self, expires_in: Option<String>) -> Self {
        self.expires_in = expires_in;
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
impl<B> Sender for MlForecast<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/_forecast";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "duration")]
                duration: Option<String>,
                #[serde(rename = "expires_in")]
                expires_in: Option<String>,
            }
            let query_params = QueryParamsStruct {
                duration: self.duration,
                expires_in: self.expires_in,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetBuckets<B> {
    client: Elasticsearch,
    anomaly_score: Option<f64>,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<String>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    expand: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    size: Option<i32>,
    sort: Option<String>,
    source: Option<String>,
    start: Option<String>,
    timestamp: Option<String>,
}
impl<B> MlGetBuckets<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlGetBuckets {
            client,
            job_id: job_id,
            anomaly_score: None,
            body: None,
            desc: None,
            end: None,
            error_trace: None,
            exclude_interim: None,
            expand: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            sort: None,
            source: None,
            start: None,
            timestamp: None,
        }
    }
    #[doc = "Filter for the most anomalous buckets"]
    pub fn anomaly_score(mut self, anomaly_score: Option<f64>) -> Self {
        self.anomaly_score = anomaly_score;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Set the sort direction"]
    pub fn desc(mut self, desc: Option<bool>) -> Self {
        self.desc = desc;
        self
    }
    #[doc = "End time filter for buckets"]
    pub fn end(mut self, end: Option<String>) -> Self {
        self.end = end;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Exclude interim results"]
    pub fn exclude_interim(mut self, exclude_interim: Option<bool>) -> Self {
        self.exclude_interim = exclude_interim;
        self
    }
    #[doc = "Include anomaly records"]
    pub fn expand(mut self, expand: Option<bool>) -> Self {
        self.expand = expand;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "skips a number of buckets"]
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
    #[doc = "specifies a max number of buckets to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "Sort buckets by a particular field"]
    pub fn sort(mut self, sort: Option<String>) -> Self {
        self.sort = sort;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Start time filter for buckets"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
}
impl<B> Sender for MlGetBuckets<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/results/buckets/{timestamp}";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "anomaly_score")]
                anomaly_score: Option<f64>,
                #[serde(rename = "desc")]
                desc: Option<bool>,
                #[serde(rename = "end")]
                end: Option<String>,
                #[serde(rename = "exclude_interim")]
                exclude_interim: Option<bool>,
                #[serde(rename = "expand")]
                expand: Option<bool>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "sort")]
                sort: Option<String>,
                #[serde(rename = "start")]
                start: Option<String>,
            }
            let query_params = QueryParamsStruct {
                anomaly_score: self.anomaly_score,
                desc: self.desc,
                end: self.end,
                exclude_interim: self.exclude_interim,
                expand: self.expand,
                from: self.from,
                size: self.size,
                sort: self.sort,
                start: self.start,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetCalendarEvents {
    client: Elasticsearch,
    calendar_id: String,
    end: Option<String>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    job_id: Option<String>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<String>,
    start: Option<String>,
}
impl MlGetCalendarEvents {
    pub fn new(client: Elasticsearch, calendar_id: String) -> Self {
        MlGetCalendarEvents {
            client,
            calendar_id: calendar_id,
            end: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            job_id: None,
            pretty: None,
            size: None,
            source: None,
            start: None,
        }
    }
    #[doc = "Get events before this time"]
    pub fn end(mut self, end: Option<String>) -> Self {
        self.end = end;
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
    #[doc = "Skips a number of events"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Get events for the job. When this option is used calendar_id must be '_all'"]
    pub fn job_id(mut self, job_id: Option<String>) -> Self {
        self.job_id = job_id;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Specifies a max number of events to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Get events after this time"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
}
impl Sender for MlGetCalendarEvents {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/calendars/{calendar_id}/events";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "end")]
                end: Option<String>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "job_id")]
                job_id: Option<String>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "start")]
                start: Option<String>,
            }
            let query_params = QueryParamsStruct {
                end: self.end,
                from: self.from,
                job_id: self.job_id,
                size: self.size,
                start: self.start,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetCalendars<B> {
    client: Elasticsearch,
    body: Option<B>,
    calendar_id: Option<String>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<String>,
}
impl<B> MlGetCalendars<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        MlGetCalendars {
            client,
            body: None,
            calendar_id: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
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
    #[doc = "skips a number of calendars"]
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
    #[doc = "specifies a max number of calendars to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for MlGetCalendars<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/calendars";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "size")]
                size: Option<i32>,
            }
            let query_params = QueryParamsStruct {
                from: self.from,
                size: self.size,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetCategories<B> {
    client: Elasticsearch,
    body: Option<B>,
    category_id: Option<i64>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<String>,
}
impl<B> MlGetCategories<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlGetCategories {
            client,
            job_id: job_id,
            body: None,
            category_id: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
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
    #[doc = "skips a number of categories"]
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
    #[doc = "specifies a max number of categories to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for MlGetCategories<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/results/categories/{category_id}";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "size")]
                size: Option<i32>,
            }
            let query_params = QueryParamsStruct {
                from: self.from,
                size: self.size,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetDataFrameAnalytics {
    client: Elasticsearch,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    id: Option<String>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<String>,
}
impl MlGetDataFrameAnalytics {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetDataFrameAnalytics {
            client,
            allow_no_match: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            id: None,
            pretty: None,
            size: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no data frame analytics. (This includes `_all` string or when no data frame analytics have been specified)"]
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
    #[doc = "skips a number of analytics"]
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
    #[doc = "specifies a max number of analytics to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for MlGetDataFrameAnalytics {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/data_frame/analytics/{id}";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "size")]
                size: Option<i32>,
            }
            let query_params = QueryParamsStruct {
                allow_no_match: self.allow_no_match,
                from: self.from,
                size: self.size,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetDataFrameAnalyticsStats {
    client: Elasticsearch,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    id: Option<String>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<String>,
}
impl MlGetDataFrameAnalyticsStats {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetDataFrameAnalyticsStats {
            client,
            allow_no_match: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            id: None,
            pretty: None,
            size: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no data frame analytics. (This includes `_all` string or when no data frame analytics have been specified)"]
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
    #[doc = "skips a number of analytics"]
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
    #[doc = "specifies a max number of analytics to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for MlGetDataFrameAnalyticsStats {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/data_frame/analytics/_stats";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "size")]
                size: Option<i32>,
            }
            let query_params = QueryParamsStruct {
                allow_no_match: self.allow_no_match,
                from: self.from,
                size: self.size,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetDatafeedStats {
    client: Elasticsearch,
    allow_no_datafeeds: Option<bool>,
    datafeed_id: Option<String>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlGetDatafeedStats {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetDatafeedStats {
            client,
            allow_no_datafeeds: None,
            datafeed_id: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no datafeeds. (This includes `_all` string or when no datafeeds have been specified)"]
    pub fn allow_no_datafeeds(mut self, allow_no_datafeeds: Option<bool>) -> Self {
        self.allow_no_datafeeds = allow_no_datafeeds;
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
impl Sender for MlGetDatafeedStats {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/datafeeds/{datafeed_id}/_stats";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_datafeeds")]
                allow_no_datafeeds: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_datafeeds: self.allow_no_datafeeds,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetDatafeeds {
    client: Elasticsearch,
    allow_no_datafeeds: Option<bool>,
    datafeed_id: Option<String>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlGetDatafeeds {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetDatafeeds {
            client,
            allow_no_datafeeds: None,
            datafeed_id: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no datafeeds. (This includes `_all` string or when no datafeeds have been specified)"]
    pub fn allow_no_datafeeds(mut self, allow_no_datafeeds: Option<bool>) -> Self {
        self.allow_no_datafeeds = allow_no_datafeeds;
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
impl Sender for MlGetDatafeeds {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/datafeeds/{datafeed_id}";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_datafeeds")]
                allow_no_datafeeds: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_datafeeds: self.allow_no_datafeeds,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetFilters {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_id: Option<String>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<String>,
}
impl MlGetFilters {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetFilters {
            client,
            error_trace: None,
            filter_id: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
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
    #[doc = "skips a number of filters"]
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
    #[doc = "specifies a max number of filters to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for MlGetFilters {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/filters";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "size")]
                size: Option<i32>,
            }
            let query_params = QueryParamsStruct {
                from: self.from,
                size: self.size,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetInfluencers<B> {
    client: Elasticsearch,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<String>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    influencer_score: Option<f64>,
    job_id: String,
    pretty: Option<bool>,
    size: Option<i32>,
    sort: Option<String>,
    source: Option<String>,
    start: Option<String>,
}
impl<B> MlGetInfluencers<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlGetInfluencers {
            client,
            job_id: job_id,
            body: None,
            desc: None,
            end: None,
            error_trace: None,
            exclude_interim: None,
            filter_path: None,
            from: None,
            human: None,
            influencer_score: None,
            pretty: None,
            size: None,
            sort: None,
            source: None,
            start: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "whether the results should be sorted in decending order"]
    pub fn desc(mut self, desc: Option<bool>) -> Self {
        self.desc = desc;
        self
    }
    #[doc = "end timestamp for the requested influencers"]
    pub fn end(mut self, end: Option<String>) -> Self {
        self.end = end;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Exclude interim results"]
    pub fn exclude_interim(mut self, exclude_interim: Option<bool>) -> Self {
        self.exclude_interim = exclude_interim;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "skips a number of influencers"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "influencer score threshold for the requested influencers"]
    pub fn influencer_score(mut self, influencer_score: Option<f64>) -> Self {
        self.influencer_score = influencer_score;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "specifies a max number of influencers to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "sort field for the requested influencers"]
    pub fn sort(mut self, sort: Option<String>) -> Self {
        self.sort = sort;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "start timestamp for the requested influencers"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
}
impl<B> Sender for MlGetInfluencers<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/results/influencers";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "desc")]
                desc: Option<bool>,
                #[serde(rename = "end")]
                end: Option<String>,
                #[serde(rename = "exclude_interim")]
                exclude_interim: Option<bool>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "influencer_score")]
                influencer_score: Option<f64>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "sort")]
                sort: Option<String>,
                #[serde(rename = "start")]
                start: Option<String>,
            }
            let query_params = QueryParamsStruct {
                desc: self.desc,
                end: self.end,
                exclude_interim: self.exclude_interim,
                from: self.from,
                influencer_score: self.influencer_score,
                size: self.size,
                sort: self.sort,
                start: self.start,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetJobStats {
    client: Elasticsearch,
    allow_no_jobs: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    job_id: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlGetJobStats {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetJobStats {
            client,
            allow_no_jobs: None,
            error_trace: None,
            filter_path: None,
            human: None,
            job_id: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: Option<bool>) -> Self {
        self.allow_no_jobs = allow_no_jobs;
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
impl Sender for MlGetJobStats {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/_stats";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_jobs")]
                allow_no_jobs: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_jobs: self.allow_no_jobs,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetJobs {
    client: Elasticsearch,
    allow_no_jobs: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    job_id: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlGetJobs {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetJobs {
            client,
            allow_no_jobs: None,
            error_trace: None,
            filter_path: None,
            human: None,
            job_id: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: Option<bool>) -> Self {
        self.allow_no_jobs = allow_no_jobs;
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
impl Sender for MlGetJobs {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_jobs")]
                allow_no_jobs: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                allow_no_jobs: self.allow_no_jobs,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetModelSnapshots<B> {
    client: Elasticsearch,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<String>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    size: Option<i32>,
    snapshot_id: Option<String>,
    sort: Option<String>,
    source: Option<String>,
    start: Option<String>,
}
impl<B> MlGetModelSnapshots<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlGetModelSnapshots {
            client,
            job_id: job_id,
            body: None,
            desc: None,
            end: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            snapshot_id: None,
            sort: None,
            source: None,
            start: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "True if the results should be sorted in descending order"]
    pub fn desc(mut self, desc: Option<bool>) -> Self {
        self.desc = desc;
        self
    }
    #[doc = "The filter 'end' query parameter"]
    pub fn end(mut self, end: Option<String>) -> Self {
        self.end = end;
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
    #[doc = "Skips a number of documents"]
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
    #[doc = "The default number of documents returned in queries as a string."]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "Name of the field to sort on"]
    pub fn sort(mut self, sort: Option<String>) -> Self {
        self.sort = sort;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "The filter 'start' query parameter"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
}
impl<B> Sender for MlGetModelSnapshots<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/model_snapshots/{snapshot_id}";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "desc")]
                desc: Option<bool>,
                #[serde(rename = "end")]
                end: Option<String>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "sort")]
                sort: Option<String>,
                #[serde(rename = "start")]
                start: Option<String>,
            }
            let query_params = QueryParamsStruct {
                desc: self.desc,
                end: self.end,
                from: self.from,
                size: self.size,
                sort: self.sort,
                start: self.start,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetOverallBuckets<B> {
    client: Elasticsearch,
    allow_no_jobs: Option<bool>,
    body: Option<B>,
    bucket_span: Option<String>,
    end: Option<String>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    job_id: String,
    overall_score: Option<f64>,
    pretty: Option<bool>,
    source: Option<String>,
    start: Option<String>,
    top_n: Option<i32>,
}
impl<B> MlGetOverallBuckets<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlGetOverallBuckets {
            client,
            job_id: job_id,
            allow_no_jobs: None,
            body: None,
            bucket_span: None,
            end: None,
            error_trace: None,
            exclude_interim: None,
            filter_path: None,
            human: None,
            overall_score: None,
            pretty: None,
            source: None,
            start: None,
            top_n: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: Option<bool>) -> Self {
        self.allow_no_jobs = allow_no_jobs;
        self
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "The span of the overall buckets. Defaults to the longest job bucket_span"]
    pub fn bucket_span(mut self, bucket_span: Option<String>) -> Self {
        self.bucket_span = bucket_span;
        self
    }
    #[doc = "Returns overall buckets with timestamps earlier than this time"]
    pub fn end(mut self, end: Option<String>) -> Self {
        self.end = end;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "If true overall buckets that include interim buckets will be excluded"]
    pub fn exclude_interim(mut self, exclude_interim: Option<bool>) -> Self {
        self.exclude_interim = exclude_interim;
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
    #[doc = "Returns overall buckets with overall scores higher than this value"]
    pub fn overall_score(mut self, overall_score: Option<f64>) -> Self {
        self.overall_score = overall_score;
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
    #[doc = "Returns overall buckets with timestamps after this time"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
    #[doc = "The number of top job bucket scores to be used in the overall_score calculation"]
    pub fn top_n(mut self, top_n: Option<i32>) -> Self {
        self.top_n = top_n;
        self
    }
}
impl<B> Sender for MlGetOverallBuckets<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/results/overall_buckets";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_jobs")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "bucket_span")]
                bucket_span: Option<String>,
                #[serde(rename = "end")]
                end: Option<String>,
                #[serde(rename = "exclude_interim")]
                exclude_interim: Option<bool>,
                #[serde(rename = "overall_score")]
                overall_score: Option<f64>,
                #[serde(rename = "start")]
                start: Option<String>,
                #[serde(rename = "top_n")]
                top_n: Option<i32>,
            }
            let query_params = QueryParamsStruct {
                allow_no_jobs: self.allow_no_jobs,
                bucket_span: self.bucket_span,
                end: self.end,
                exclude_interim: self.exclude_interim,
                overall_score: self.overall_score,
                start: self.start,
                top_n: self.top_n,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlGetRecords<B> {
    client: Elasticsearch,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<String>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    record_score: Option<f64>,
    size: Option<i32>,
    sort: Option<String>,
    source: Option<String>,
    start: Option<String>,
}
impl<B> MlGetRecords<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlGetRecords {
            client,
            job_id: job_id,
            body: None,
            desc: None,
            end: None,
            error_trace: None,
            exclude_interim: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            record_score: None,
            size: None,
            sort: None,
            source: None,
            start: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Set the sort direction"]
    pub fn desc(mut self, desc: Option<bool>) -> Self {
        self.desc = desc;
        self
    }
    #[doc = "End time filter for records"]
    pub fn end(mut self, end: Option<String>) -> Self {
        self.end = end;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Exclude interim results"]
    pub fn exclude_interim(mut self, exclude_interim: Option<bool>) -> Self {
        self.exclude_interim = exclude_interim;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "skips a number of records"]
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
    pub fn record_score(mut self, record_score: Option<f64>) -> Self {
        self.record_score = record_score;
        self
    }
    #[doc = "specifies a max number of records to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "Sort records by a particular field"]
    pub fn sort(mut self, sort: Option<String>) -> Self {
        self.sort = sort;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Start time filter for records"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
}
impl<B> Sender for MlGetRecords<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/results/records";
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "desc")]
                desc: Option<bool>,
                #[serde(rename = "end")]
                end: Option<String>,
                #[serde(rename = "exclude_interim")]
                exclude_interim: Option<bool>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "record_score")]
                record_score: Option<f64>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "sort")]
                sort: Option<String>,
                #[serde(rename = "start")]
                start: Option<String>,
            }
            let query_params = QueryParamsStruct {
                desc: self.desc,
                end: self.end,
                exclude_interim: self.exclude_interim,
                from: self.from,
                record_score: self.record_score,
                size: self.size,
                sort: self.sort,
                start: self.start,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlInfo {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlInfo {
    pub fn new(client: Elasticsearch) -> Self {
        MlInfo {
            client,
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
}
impl Sender for MlInfo {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/info";
        let method = HttpMethod::Get;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlOpenJob<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_downtime: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> MlOpenJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlOpenJob {
            client,
            job_id: job_id,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            ignore_downtime: None,
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
}
impl<B> Sender for MlOpenJob<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/_open";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlPostCalendarEvents<B> {
    client: Elasticsearch,
    body: Option<B>,
    calendar_id: String,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlPostCalendarEvents<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, calendar_id: String) -> Self {
        MlPostCalendarEvents {
            client,
            calendar_id: calendar_id,
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
impl<B> Sender for MlPostCalendarEvents<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/calendars/{calendar_id}/events";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlPostData<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    reset_end: Option<String>,
    reset_start: Option<String>,
    source: Option<String>,
}
impl<B> MlPostData<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlPostData {
            client,
            job_id: job_id,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            reset_end: None,
            reset_start: None,
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
    #[doc = "Optional parameter to specify the end of the bucket resetting range"]
    pub fn reset_end(mut self, reset_end: Option<String>) -> Self {
        self.reset_end = reset_end;
        self
    }
    #[doc = "Optional parameter to specify the start of the bucket resetting range"]
    pub fn reset_start(mut self, reset_start: Option<String>) -> Self {
        self.reset_start = reset_start;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for MlPostData<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/_data";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "reset_end")]
                reset_end: Option<String>,
                #[serde(rename = "reset_start")]
                reset_start: Option<String>,
            }
            let query_params = QueryParamsStruct {
                reset_end: self.reset_end,
                reset_start: self.reset_start,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlPreviewDatafeed {
    client: Elasticsearch,
    datafeed_id: String,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPreviewDatafeed {
    pub fn new(client: Elasticsearch, datafeed_id: String) -> Self {
        MlPreviewDatafeed {
            client,
            datafeed_id: datafeed_id,
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
}
impl Sender for MlPreviewDatafeed {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/datafeeds/{datafeed_id}/_preview";
        let method = HttpMethod::Get;
        let query_string = None::<()>;
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlPutCalendar<B> {
    client: Elasticsearch,
    body: Option<B>,
    calendar_id: String,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlPutCalendar<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, calendar_id: String) -> Self {
        MlPutCalendar {
            client,
            calendar_id: calendar_id,
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
impl<B> Sender for MlPutCalendar<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/calendars/{calendar_id}";
        let method = HttpMethod::Put;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlPutCalendarJob<B> {
    client: Elasticsearch,
    body: Option<B>,
    calendar_id: String,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlPutCalendarJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, calendar_id: String, job_id: String) -> Self {
        MlPutCalendarJob {
            client,
            calendar_id: calendar_id,
            job_id: job_id,
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
impl<B> Sender for MlPutCalendarJob<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/calendars/{calendar_id}/jobs/{job_id}";
        let method = HttpMethod::Put;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlPutDataFrameAnalytics<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlPutDataFrameAnalytics<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, id: String) -> Self {
        MlPutDataFrameAnalytics {
            client,
            id: id,
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
impl<B> Sender for MlPutDataFrameAnalytics<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/data_frame/analytics/{id}";
        let method = HttpMethod::Put;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlPutDatafeed<B> {
    client: Elasticsearch,
    body: Option<B>,
    datafeed_id: String,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlPutDatafeed<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, datafeed_id: String) -> Self {
        MlPutDatafeed {
            client,
            datafeed_id: datafeed_id,
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
impl<B> Sender for MlPutDatafeed<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/datafeeds/{datafeed_id}";
        let method = HttpMethod::Put;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlPutFilter<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_id: String,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlPutFilter<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, filter_id: String) -> Self {
        MlPutFilter {
            client,
            filter_id: filter_id,
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
impl<B> Sender for MlPutFilter<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/filters/{filter_id}";
        let method = HttpMethod::Put;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlPutJob<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlPutJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlPutJob {
            client,
            job_id: job_id,
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
impl<B> Sender for MlPutJob<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}";
        let method = HttpMethod::Put;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlRevertModelSnapshot<B> {
    client: Elasticsearch,
    body: Option<B>,
    delete_intervening_results: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    snapshot_id: String,
    source: Option<String>,
}
impl<B> MlRevertModelSnapshot<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String, snapshot_id: String) -> Self {
        MlRevertModelSnapshot {
            client,
            job_id: job_id,
            snapshot_id: snapshot_id,
            body: None,
            delete_intervening_results: None,
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
    #[doc = "Should we reset the results back to the time of the snapshot?"]
    pub fn delete_intervening_results(mut self, delete_intervening_results: Option<bool>) -> Self {
        self.delete_intervening_results = delete_intervening_results;
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
impl<B> Sender for MlRevertModelSnapshot<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/model_snapshots/{snapshot_id}/_revert";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "delete_intervening_results")]
                delete_intervening_results: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                delete_intervening_results: self.delete_intervening_results,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlSetUpgradeMode<B> {
    client: Elasticsearch,
    body: Option<B>,
    enabled: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> MlSetUpgradeMode<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        MlSetUpgradeMode {
            client,
            body: None,
            enabled: None,
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
    #[doc = "Whether to enable upgrade_mode ML setting or not. Defaults to false."]
    pub fn enabled(mut self, enabled: Option<bool>) -> Self {
        self.enabled = enabled;
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
    #[doc = "Controls the time to wait before action times out. Defaults to 30 seconds"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl<B> Sender for MlSetUpgradeMode<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/set_upgrade_mode";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "enabled")]
                enabled: Option<bool>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                enabled: self.enabled,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlStartDataFrameAnalytics<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    id: String,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> MlStartDataFrameAnalytics<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, id: String) -> Self {
        MlStartDataFrameAnalytics {
            client,
            id: id,
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
    #[doc = "Controls the time to wait until the task has started. Defaults to 20 seconds"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl<B> Sender for MlStartDataFrameAnalytics<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/data_frame/analytics/{id}/_start";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlStartDatafeed<B> {
    client: Elasticsearch,
    body: Option<B>,
    datafeed_id: String,
    end: Option<String>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    start: Option<String>,
    timeout: Option<String>,
}
impl<B> MlStartDatafeed<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, datafeed_id: String) -> Self {
        MlStartDatafeed {
            client,
            datafeed_id: datafeed_id,
            body: None,
            end: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            start: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "The end time when the datafeed should stop. When not set, the datafeed continues in real time"]
    pub fn end(mut self, end: Option<String>) -> Self {
        self.end = end;
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
    #[doc = "The start time from where the datafeed should begin"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
    #[doc = "Controls the time to wait until a datafeed has started. Default to 20 seconds"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl<B> Sender for MlStartDatafeed<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/datafeeds/{datafeed_id}/_start";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "end")]
                end: Option<String>,
                #[serde(rename = "start")]
                start: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                end: self.end,
                start: self.start,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlStopDataFrameAnalytics<B> {
    client: Elasticsearch,
    allow_no_match: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    force: Option<bool>,
    human: Option<bool>,
    id: String,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> MlStopDataFrameAnalytics<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, id: String) -> Self {
        MlStopDataFrameAnalytics {
            client,
            id: id,
            allow_no_match: None,
            body: None,
            error_trace: None,
            filter_path: None,
            force: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no data frame analytics. (This includes `_all` string or when no data frame analytics have been specified)"]
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
    #[doc = "True if the data frame analytics should be forcefully stopped"]
    pub fn force(mut self, force: Option<bool>) -> Self {
        self.force = force;
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
    #[doc = "Controls the time to wait until the task has stopped. Defaults to 20 seconds"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl<B> Sender for MlStopDataFrameAnalytics<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/data_frame/analytics/{id}/_stop";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_match: self.allow_no_match,
                force: self.force,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlStopDatafeed<B> {
    client: Elasticsearch,
    allow_no_datafeeds: Option<bool>,
    body: Option<B>,
    datafeed_id: String,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> MlStopDatafeed<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, datafeed_id: String) -> Self {
        MlStopDatafeed {
            client,
            datafeed_id: datafeed_id,
            allow_no_datafeeds: None,
            body: None,
            error_trace: None,
            filter_path: None,
            force: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no datafeeds. (This includes `_all` string or when no datafeeds have been specified)"]
    pub fn allow_no_datafeeds(mut self, allow_no_datafeeds: Option<bool>) -> Self {
        self.allow_no_datafeeds = allow_no_datafeeds;
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
    #[doc = "True if the datafeed should be forcefully stopped."]
    pub fn force(mut self, force: Option<bool>) -> Self {
        self.force = force;
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
    #[doc = "Controls the time to wait until a datafeed has stopped. Default to 20 seconds"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl<B> Sender for MlStopDatafeed<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/datafeeds/{datafeed_id}/_stop";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_datafeeds")]
                allow_no_datafeeds: Option<bool>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_datafeeds: self.allow_no_datafeeds,
                force: self.force,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlUpdateDatafeed<B> {
    client: Elasticsearch,
    body: Option<B>,
    datafeed_id: String,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlUpdateDatafeed<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, datafeed_id: String) -> Self {
        MlUpdateDatafeed {
            client,
            datafeed_id: datafeed_id,
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
impl<B> Sender for MlUpdateDatafeed<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/datafeeds/{datafeed_id}/_update";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlUpdateFilter<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_id: String,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlUpdateFilter<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, filter_id: String) -> Self {
        MlUpdateFilter {
            client,
            filter_id: filter_id,
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
impl<B> Sender for MlUpdateFilter<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/filters/{filter_id}/_update";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlUpdateJob<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlUpdateJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String) -> Self {
        MlUpdateJob {
            client,
            job_id: job_id,
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
impl<B> Sender for MlUpdateJob<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/_update";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlUpdateModelSnapshot<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    job_id: String,
    pretty: Option<bool>,
    snapshot_id: String,
    source: Option<String>,
}
impl<B> MlUpdateModelSnapshot<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, job_id: String, snapshot_id: String) -> Self {
        MlUpdateModelSnapshot {
            client,
            job_id: job_id,
            snapshot_id: snapshot_id,
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
impl<B> Sender for MlUpdateModelSnapshot<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/{job_id}/model_snapshots/{snapshot_id}/_update";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlValidate<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlValidate<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        MlValidate {
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
impl<B> Sender for MlValidate<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/_validate";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct MlValidateDetector<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlValidateDetector<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        MlValidateDetector {
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
impl<B> Sender for MlValidateDetector<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_ml/anomaly_detectors/_validate/detector";
        let method = HttpMethod::Post;
        let query_string = None::<()>;
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[doc = "Ml APIs"]
pub struct Ml {
    client: Elasticsearch,
}
impl Ml {
    pub fn new(client: Elasticsearch) -> Self {
        Ml { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-close-job.html"]
    pub fn close_job<B>(&self, job_id: String) -> MlCloseJob<B>
    where
        B: Serialize,
    {
        MlCloseJob::new(self.client.clone(), job_id)
    }
    pub fn delete_calendar(&self, calendar_id: String) -> MlDeleteCalendar {
        MlDeleteCalendar::new(self.client.clone(), calendar_id)
    }
    pub fn delete_calendar_event(
        &self,
        calendar_id: String,
        event_id: String,
    ) -> MlDeleteCalendarEvent {
        MlDeleteCalendarEvent::new(self.client.clone(), calendar_id, event_id)
    }
    pub fn delete_calendar_job(&self, calendar_id: String, job_id: String) -> MlDeleteCalendarJob {
        MlDeleteCalendarJob::new(self.client.clone(), calendar_id, job_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/delete-dfanalytics.html"]
    pub fn delete_data_frame_analytics(&self, id: String) -> MlDeleteDataFrameAnalytics {
        MlDeleteDataFrameAnalytics::new(self.client.clone(), id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-datafeed.html"]
    pub fn delete_datafeed(&self, datafeed_id: String) -> MlDeleteDatafeed {
        MlDeleteDatafeed::new(self.client.clone(), datafeed_id)
    }
    pub fn delete_expired_data(&self) -> MlDeleteExpiredData {
        MlDeleteExpiredData::new(self.client.clone())
    }
    pub fn delete_filter(&self, filter_id: String) -> MlDeleteFilter {
        MlDeleteFilter::new(self.client.clone(), filter_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-forecast.html"]
    pub fn delete_forecast(&self, job_id: String) -> MlDeleteForecast {
        MlDeleteForecast::new(self.client.clone(), job_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-job.html"]
    pub fn delete_job(&self, job_id: String) -> MlDeleteJob {
        MlDeleteJob::new(self.client.clone(), job_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-snapshot.html"]
    pub fn delete_model_snapshot(
        &self,
        job_id: String,
        snapshot_id: String,
    ) -> MlDeleteModelSnapshot {
        MlDeleteModelSnapshot::new(self.client.clone(), job_id, snapshot_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/evaluate-dfanalytics.html"]
    pub fn evaluate_data_frame<B>(&self) -> MlEvaluateDataFrame<B>
    where
        B: Serialize,
    {
        MlEvaluateDataFrame::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-find-file-structure.html"]
    pub fn find_file_structure<B>(&self) -> MlFindFileStructure<B>
    where
        B: Serialize,
    {
        MlFindFileStructure::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-flush-job.html"]
    pub fn flush_job<B>(&self, job_id: String) -> MlFlushJob<B>
    where
        B: Serialize,
    {
        MlFlushJob::new(self.client.clone(), job_id)
    }
    pub fn forecast<B>(&self, job_id: String) -> MlForecast<B>
    where
        B: Serialize,
    {
        MlForecast::new(self.client.clone(), job_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-bucket.html"]
    pub fn get_buckets<B>(&self, job_id: String) -> MlGetBuckets<B>
    where
        B: Serialize,
    {
        MlGetBuckets::new(self.client.clone(), job_id)
    }
    pub fn get_calendar_events(&self, calendar_id: String) -> MlGetCalendarEvents {
        MlGetCalendarEvents::new(self.client.clone(), calendar_id)
    }
    pub fn get_calendars<B>(&self) -> MlGetCalendars<B>
    where
        B: Serialize,
    {
        MlGetCalendars::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-category.html"]
    pub fn get_categories<B>(&self, job_id: String) -> MlGetCategories<B>
    where
        B: Serialize,
    {
        MlGetCategories::new(self.client.clone(), job_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/get-dfanalytics.html"]
    pub fn get_data_frame_analytics(&self) -> MlGetDataFrameAnalytics {
        MlGetDataFrameAnalytics::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/get-dfanalytics-stats.html"]
    pub fn get_data_frame_analytics_stats(&self) -> MlGetDataFrameAnalyticsStats {
        MlGetDataFrameAnalyticsStats::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-datafeed-stats.html"]
    pub fn get_datafeed_stats(&self) -> MlGetDatafeedStats {
        MlGetDatafeedStats::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-datafeed.html"]
    pub fn get_datafeeds(&self) -> MlGetDatafeeds {
        MlGetDatafeeds::new(self.client.clone())
    }
    pub fn get_filters(&self) -> MlGetFilters {
        MlGetFilters::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-influencer.html"]
    pub fn get_influencers<B>(&self, job_id: String) -> MlGetInfluencers<B>
    where
        B: Serialize,
    {
        MlGetInfluencers::new(self.client.clone(), job_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-job-stats.html"]
    pub fn get_job_stats(&self) -> MlGetJobStats {
        MlGetJobStats::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-job.html"]
    pub fn get_jobs(&self) -> MlGetJobs {
        MlGetJobs::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-snapshot.html"]
    pub fn get_model_snapshots<B>(&self, job_id: String) -> MlGetModelSnapshots<B>
    where
        B: Serialize,
    {
        MlGetModelSnapshots::new(self.client.clone(), job_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-overall-buckets.html"]
    pub fn get_overall_buckets<B>(&self, job_id: String) -> MlGetOverallBuckets<B>
    where
        B: Serialize,
    {
        MlGetOverallBuckets::new(self.client.clone(), job_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-record.html"]
    pub fn get_records<B>(&self, job_id: String) -> MlGetRecords<B>
    where
        B: Serialize,
    {
        MlGetRecords::new(self.client.clone(), job_id)
    }
    pub fn info(&self) -> MlInfo {
        MlInfo::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-open-job.html"]
    pub fn open_job<B>(&self, job_id: String) -> MlOpenJob<B>
    where
        B: Serialize,
    {
        MlOpenJob::new(self.client.clone(), job_id)
    }
    pub fn post_calendar_events<B>(&self, calendar_id: String) -> MlPostCalendarEvents<B>
    where
        B: Serialize,
    {
        MlPostCalendarEvents::new(self.client.clone(), calendar_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-post-data.html"]
    pub fn post_data<B>(&self, job_id: String) -> MlPostData<B>
    where
        B: Serialize,
    {
        MlPostData::new(self.client.clone(), job_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-preview-datafeed.html"]
    pub fn preview_datafeed(&self, datafeed_id: String) -> MlPreviewDatafeed {
        MlPreviewDatafeed::new(self.client.clone(), datafeed_id)
    }
    pub fn put_calendar<B>(&self, calendar_id: String) -> MlPutCalendar<B>
    where
        B: Serialize,
    {
        MlPutCalendar::new(self.client.clone(), calendar_id)
    }
    pub fn put_calendar_job<B>(&self, calendar_id: String, job_id: String) -> MlPutCalendarJob<B>
    where
        B: Serialize,
    {
        MlPutCalendarJob::new(self.client.clone(), calendar_id, job_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/put-dfanalytics.html"]
    pub fn put_data_frame_analytics<B>(&self, id: String) -> MlPutDataFrameAnalytics<B>
    where
        B: Serialize,
    {
        MlPutDataFrameAnalytics::new(self.client.clone(), id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-datafeed.html"]
    pub fn put_datafeed<B>(&self, datafeed_id: String) -> MlPutDatafeed<B>
    where
        B: Serialize,
    {
        MlPutDatafeed::new(self.client.clone(), datafeed_id)
    }
    pub fn put_filter<B>(&self, filter_id: String) -> MlPutFilter<B>
    where
        B: Serialize,
    {
        MlPutFilter::new(self.client.clone(), filter_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-job.html"]
    pub fn put_job<B>(&self, job_id: String) -> MlPutJob<B>
    where
        B: Serialize,
    {
        MlPutJob::new(self.client.clone(), job_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-revert-snapshot.html"]
    pub fn revert_model_snapshot<B>(
        &self,
        job_id: String,
        snapshot_id: String,
    ) -> MlRevertModelSnapshot<B>
    where
        B: Serialize,
    {
        MlRevertModelSnapshot::new(self.client.clone(), job_id, snapshot_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-set-upgrade-mode.html"]
    pub fn set_upgrade_mode<B>(&self) -> MlSetUpgradeMode<B>
    where
        B: Serialize,
    {
        MlSetUpgradeMode::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/start-dfanalytics.html"]
    pub fn start_data_frame_analytics<B>(&self, id: String) -> MlStartDataFrameAnalytics<B>
    where
        B: Serialize,
    {
        MlStartDataFrameAnalytics::new(self.client.clone(), id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-start-datafeed.html"]
    pub fn start_datafeed<B>(&self, datafeed_id: String) -> MlStartDatafeed<B>
    where
        B: Serialize,
    {
        MlStartDatafeed::new(self.client.clone(), datafeed_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/stop-dfanalytics.html"]
    pub fn stop_data_frame_analytics<B>(&self, id: String) -> MlStopDataFrameAnalytics<B>
    where
        B: Serialize,
    {
        MlStopDataFrameAnalytics::new(self.client.clone(), id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-stop-datafeed.html"]
    pub fn stop_datafeed<B>(&self, datafeed_id: String) -> MlStopDatafeed<B>
    where
        B: Serialize,
    {
        MlStopDatafeed::new(self.client.clone(), datafeed_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-datafeed.html"]
    pub fn update_datafeed<B>(&self, datafeed_id: String) -> MlUpdateDatafeed<B>
    where
        B: Serialize,
    {
        MlUpdateDatafeed::new(self.client.clone(), datafeed_id)
    }
    pub fn update_filter<B>(&self, filter_id: String) -> MlUpdateFilter<B>
    where
        B: Serialize,
    {
        MlUpdateFilter::new(self.client.clone(), filter_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-job.html"]
    pub fn update_job<B>(&self, job_id: String) -> MlUpdateJob<B>
    where
        B: Serialize,
    {
        MlUpdateJob::new(self.client.clone(), job_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-snapshot.html"]
    pub fn update_model_snapshot<B>(
        &self,
        job_id: String,
        snapshot_id: String,
    ) -> MlUpdateModelSnapshot<B>
    where
        B: Serialize,
    {
        MlUpdateModelSnapshot::new(self.client.clone(), job_id, snapshot_id)
    }
    pub fn validate<B>(&self) -> MlValidate<B>
    where
        B: Serialize,
    {
        MlValidate::new(self.client.clone())
    }
    pub fn validate_detector<B>(&self) -> MlValidateDetector<B>
    where
        B: Serialize,
    {
        MlValidateDetector::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Ml APIs"]
    pub fn ml(&self) -> Ml {
        Ml::new(self.clone())
    }
}
