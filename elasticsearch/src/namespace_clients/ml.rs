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
#[doc = "Url parts for the Ml Close Job API"]
pub enum MlCloseJobUrlParts {
    JobId(String),
}
impl MlCloseJobUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlCloseJobUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(30usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/_close");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Close Job API"]
pub struct MlCloseJob<B> {
    client: Elasticsearch,
    parts: MlCloseJobUrlParts,
    allow_no_jobs: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> MlCloseJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlCloseJobUrlParts) -> Self {
        MlCloseJob {
            client,
            parts,
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
    pub fn allow_no_jobs(mut self, allow_no_jobs: bool) -> Self {
        self.allow_no_jobs = Some(allow_no_jobs);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlCloseJob<T>
    where
        T: Serialize,
    {
        MlCloseJob {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            allow_no_jobs: self.allow_no_jobs,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            force: self.force,
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "True if the job should be forcefully closed"]
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait until a job has closed. Default to 30 minutes"]
    pub fn timeout(mut self, timeout: String) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Close Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_jobs", skip_serializing_if = "Option::is_none")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
                force: Option<bool>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_jobs: self.allow_no_jobs,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                force: self.force,
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
#[doc = "Url parts for the Ml Delete Calendar API"]
pub enum MlDeleteCalendarUrlParts {
    CalendarId(String),
}
impl MlDeleteCalendarUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlDeleteCalendarUrlParts::CalendarId(ref calendar_id) => {
                let mut p = String::with_capacity(15usize + calendar_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(calendar_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Delete Calendar API"]
pub struct MlDeleteCalendar {
    client: Elasticsearch,
    parts: MlDeleteCalendarUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteCalendar {
    pub fn new(client: Elasticsearch, parts: MlDeleteCalendarUrlParts) -> Self {
        MlDeleteCalendar {
            client,
            parts,
            error_trace: None,
            filter_path: None,
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Delete Calendar API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Delete Calendar Event API"]
pub enum MlDeleteCalendarEventUrlParts {
    CalendarIdEventId(String, String),
}
impl MlDeleteCalendarEventUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlDeleteCalendarEventUrlParts::CalendarIdEventId(ref calendar_id, ref event_id) => {
                let mut p = String::with_capacity(23usize + calendar_id.len() + event_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(calendar_id.as_ref());
                p.push_str("/events/");
                p.push_str(event_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Delete Calendar Event API"]
pub struct MlDeleteCalendarEvent {
    client: Elasticsearch,
    parts: MlDeleteCalendarEventUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteCalendarEvent {
    pub fn new(client: Elasticsearch, parts: MlDeleteCalendarEventUrlParts) -> Self {
        MlDeleteCalendarEvent {
            client,
            parts,
            error_trace: None,
            filter_path: None,
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Delete Calendar Event API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Delete Calendar Job API"]
pub enum MlDeleteCalendarJobUrlParts {
    CalendarIdJobId(String, String),
}
impl MlDeleteCalendarJobUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlDeleteCalendarJobUrlParts::CalendarIdJobId(ref calendar_id, ref job_id) => {
                let mut p = String::with_capacity(21usize + calendar_id.len() + job_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(calendar_id.as_ref());
                p.push_str("/jobs/");
                p.push_str(job_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Delete Calendar Job API"]
pub struct MlDeleteCalendarJob {
    client: Elasticsearch,
    parts: MlDeleteCalendarJobUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteCalendarJob {
    pub fn new(client: Elasticsearch, parts: MlDeleteCalendarJobUrlParts) -> Self {
        MlDeleteCalendarJob {
            client,
            parts,
            error_trace: None,
            filter_path: None,
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Delete Calendar Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Delete Data Frame Analytics API"]
pub enum MlDeleteDataFrameAnalyticsUrlParts {
    Id(String),
}
impl MlDeleteDataFrameAnalyticsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlDeleteDataFrameAnalyticsUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(26usize + id.len());
                p.push_str("/_ml/data_frame/analytics/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Delete Data Frame Analytics API"]
pub struct MlDeleteDataFrameAnalytics {
    client: Elasticsearch,
    parts: MlDeleteDataFrameAnalyticsUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteDataFrameAnalytics {
    pub fn new(client: Elasticsearch, parts: MlDeleteDataFrameAnalyticsUrlParts) -> Self {
        MlDeleteDataFrameAnalytics {
            client,
            parts,
            error_trace: None,
            filter_path: None,
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Delete Data Frame Analytics API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Delete Datafeed API"]
pub enum MlDeleteDatafeedUrlParts {
    DatafeedId(String),
}
impl MlDeleteDatafeedUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlDeleteDatafeedUrlParts::DatafeedId(ref datafeed_id) => {
                let mut p = String::with_capacity(15usize + datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(datafeed_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Delete Datafeed API"]
pub struct MlDeleteDatafeed {
    client: Elasticsearch,
    parts: MlDeleteDatafeedUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteDatafeed {
    pub fn new(client: Elasticsearch, parts: MlDeleteDatafeedUrlParts) -> Self {
        MlDeleteDatafeed {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "True if the datafeed should be forcefully deleted"]
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Delete Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
                force: Option<bool>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
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
#[doc = "Url parts for the Ml Delete Expired Data API"]
pub enum MlDeleteExpiredDataUrlParts {
    None,
}
impl MlDeleteExpiredDataUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlDeleteExpiredDataUrlParts::None => "/_ml/_delete_expired_data".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Delete Expired Data API"]
pub struct MlDeleteExpiredData {
    client: Elasticsearch,
    parts: MlDeleteExpiredDataUrlParts,
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
            parts: MlDeleteExpiredDataUrlParts::None,
            error_trace: None,
            filter_path: None,
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Delete Expired Data API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Delete Filter API"]
pub enum MlDeleteFilterUrlParts {
    FilterId(String),
}
impl MlDeleteFilterUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlDeleteFilterUrlParts::FilterId(ref filter_id) => {
                let mut p = String::with_capacity(13usize + filter_id.len());
                p.push_str("/_ml/filters/");
                p.push_str(filter_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Delete Filter API"]
pub struct MlDeleteFilter {
    client: Elasticsearch,
    parts: MlDeleteFilterUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteFilter {
    pub fn new(client: Elasticsearch, parts: MlDeleteFilterUrlParts) -> Self {
        MlDeleteFilter {
            client,
            parts,
            error_trace: None,
            filter_path: None,
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Delete Filter API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Delete Forecast API"]
pub enum MlDeleteForecastUrlParts {
    JobId(String),
    JobIdForecastId(String, String),
}
impl MlDeleteForecastUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlDeleteForecastUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(33usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/_forecast");
                p.into()
            }
            MlDeleteForecastUrlParts::JobIdForecastId(ref job_id, ref forecast_id) => {
                let mut p = String::with_capacity(34usize + job_id.len() + forecast_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/_forecast/");
                p.push_str(forecast_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Delete Forecast API"]
pub struct MlDeleteForecast {
    client: Elasticsearch,
    parts: MlDeleteForecastUrlParts,
    allow_no_forecasts: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl MlDeleteForecast {
    pub fn new(client: Elasticsearch, parts: MlDeleteForecastUrlParts) -> Self {
        MlDeleteForecast {
            client,
            parts,
            allow_no_forecasts: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Whether to ignore if `_all` matches no forecasts"]
    pub fn allow_no_forecasts(mut self, allow_no_forecasts: bool) -> Self {
        self.allow_no_forecasts = Some(allow_no_forecasts);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait until the forecast(s) are deleted. Default to 30 seconds"]
    pub fn timeout(mut self, timeout: String) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Delete Forecast API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_forecasts", skip_serializing_if = "Option::is_none")]
                allow_no_forecasts: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_forecasts: self.allow_no_forecasts,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Url parts for the Ml Delete Job API"]
pub enum MlDeleteJobUrlParts {
    JobId(String),
}
impl MlDeleteJobUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlDeleteJobUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(23usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Delete Job API"]
pub struct MlDeleteJob {
    client: Elasticsearch,
    parts: MlDeleteJobUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    wait_for_completion: Option<bool>,
}
impl MlDeleteJob {
    pub fn new(client: Elasticsearch, parts: MlDeleteJobUrlParts) -> Self {
        MlDeleteJob {
            client,
            parts,
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
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "True if the job should be forcefully deleted"]
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Should this request wait until the operation has completed before returning"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Delete Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
                force: Option<bool>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(
                    rename = "wait_for_completion",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                force: self.force,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                wait_for_completion: self.wait_for_completion,
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
#[doc = "Url parts for the Ml Delete Model Snapshot API"]
pub enum MlDeleteModelSnapshotUrlParts {
    JobIdSnapshotId(String, String),
}
impl MlDeleteModelSnapshotUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlDeleteModelSnapshotUrlParts::JobIdSnapshotId(ref job_id, ref snapshot_id) => {
                let mut p = String::with_capacity(40usize + job_id.len() + snapshot_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/model_snapshots/");
                p.push_str(snapshot_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Delete Model Snapshot API"]
pub struct MlDeleteModelSnapshot {
    client: Elasticsearch,
    parts: MlDeleteModelSnapshotUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteModelSnapshot {
    pub fn new(client: Elasticsearch, parts: MlDeleteModelSnapshotUrlParts) -> Self {
        MlDeleteModelSnapshot {
            client,
            parts,
            error_trace: None,
            filter_path: None,
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Delete Model Snapshot API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Estimate Memory Usage API"]
pub enum MlEstimateMemoryUsageUrlParts {
    None,
}
impl MlEstimateMemoryUsageUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlEstimateMemoryUsageUrlParts::None => {
                "/_ml/data_frame/analytics/_estimate_memory_usage".into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Estimate Memory Usage API"]
pub struct MlEstimateMemoryUsage<B> {
    client: Elasticsearch,
    parts: MlEstimateMemoryUsageUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlEstimateMemoryUsage<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        MlEstimateMemoryUsage {
            client,
            parts: MlEstimateMemoryUsageUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlEstimateMemoryUsage<T>
    where
        T: Serialize,
    {
        MlEstimateMemoryUsage {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Estimate Memory Usage API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Evaluate Data Frame API"]
pub enum MlEvaluateDataFrameUrlParts {
    None,
}
impl MlEvaluateDataFrameUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlEvaluateDataFrameUrlParts::None => "/_ml/data_frame/_evaluate".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Evaluate Data Frame API"]
pub struct MlEvaluateDataFrame<B> {
    client: Elasticsearch,
    parts: MlEvaluateDataFrameUrlParts,
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
            parts: MlEvaluateDataFrameUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlEvaluateDataFrame<T>
    where
        T: Serialize,
    {
        MlEvaluateDataFrame {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Evaluate Data Frame API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Find File Structure API"]
pub enum MlFindFileStructureUrlParts {
    None,
}
impl MlFindFileStructureUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlFindFileStructureUrlParts::None => "/_ml/find_file_structure".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Find File Structure API"]
pub struct MlFindFileStructure<B> {
    client: Elasticsearch,
    parts: MlFindFileStructureUrlParts,
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
            parts: MlFindFileStructureUrlParts::None,
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
    pub fn body<T>(self, body: T) -> MlFindFileStructure<T>
    where
        T: Serialize,
    {
        MlFindFileStructure {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            charset: self.charset,
            column_names: self.column_names,
            delimiter: self.delimiter,
            error_trace: self.error_trace,
            explain: self.explain,
            filter_path: self.filter_path,
            format: self.format,
            grok_pattern: self.grok_pattern,
            has_header_row: self.has_header_row,
            human: self.human,
            line_merge_size_limit: self.line_merge_size_limit,
            lines_to_sample: self.lines_to_sample,
            pretty: self.pretty,
            quote: self.quote,
            should_trim_fields: self.should_trim_fields,
            source: self.source,
            timeout: self.timeout,
            timestamp_field: self.timestamp_field,
            timestamp_format: self.timestamp_format,
        }
    }
    #[doc = "Optional parameter to specify the character set of the file"]
    pub fn charset(mut self, charset: String) -> Self {
        self.charset = Some(charset);
        self
    }
    #[doc = "Optional parameter containing a comma separated list of the column names for a delimited file"]
    pub fn column_names(mut self, column_names: Vec<String>) -> Self {
        self.column_names = Some(column_names);
        self
    }
    #[doc = "Optional parameter to specify the delimiter character for a delimited file - must be a single character"]
    pub fn delimiter(mut self, delimiter: String) -> Self {
        self.delimiter = Some(delimiter);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to include a commentary on how the structure was derived"]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Optional parameter to specify the high level file format"]
    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Optional parameter to specify the Grok pattern that should be used to extract fields from messages in a semi-structured text file"]
    pub fn grok_pattern(mut self, grok_pattern: String) -> Self {
        self.grok_pattern = Some(grok_pattern);
        self
    }
    #[doc = "Optional parameter to specify whether a delimited file includes the column names in its first row"]
    pub fn has_header_row(mut self, has_header_row: bool) -> Self {
        self.has_header_row = Some(has_header_row);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Maximum number of characters permitted in a single message when lines are merged to create messages."]
    pub fn line_merge_size_limit(mut self, line_merge_size_limit: i32) -> Self {
        self.line_merge_size_limit = Some(line_merge_size_limit);
        self
    }
    #[doc = "How many lines of the file should be included in the analysis"]
    pub fn lines_to_sample(mut self, lines_to_sample: i32) -> Self {
        self.lines_to_sample = Some(lines_to_sample);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Optional parameter to specify the quote character for a delimited file - must be a single character"]
    pub fn quote(mut self, quote: String) -> Self {
        self.quote = Some(quote);
        self
    }
    #[doc = "Optional parameter to specify whether the values between delimiters in a delimited file should have whitespace trimmed from them"]
    pub fn should_trim_fields(mut self, should_trim_fields: bool) -> Self {
        self.should_trim_fields = Some(should_trim_fields);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Timeout after which the analysis will be aborted"]
    pub fn timeout(mut self, timeout: String) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Optional parameter to specify the timestamp field in the file"]
    pub fn timestamp_field(mut self, timestamp_field: String) -> Self {
        self.timestamp_field = Some(timestamp_field);
        self
    }
    #[doc = "Optional parameter to specify the timestamp format in the file - may be either a Joda or Java time format"]
    pub fn timestamp_format(mut self, timestamp_format: String) -> Self {
        self.timestamp_format = Some(timestamp_format);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Find File Structure API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "charset", skip_serializing_if = "Option::is_none")]
                charset: Option<String>,
                #[serde(
                    rename = "column_names",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                column_names: Option<Vec<String>>,
                #[serde(rename = "delimiter", skip_serializing_if = "Option::is_none")]
                delimiter: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "explain", skip_serializing_if = "Option::is_none")]
                explain: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
                format: Option<Format>,
                #[serde(rename = "grok_pattern", skip_serializing_if = "Option::is_none")]
                grok_pattern: Option<String>,
                #[serde(rename = "has_header_row", skip_serializing_if = "Option::is_none")]
                has_header_row: Option<bool>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(
                    rename = "line_merge_size_limit",
                    skip_serializing_if = "Option::is_none"
                )]
                line_merge_size_limit: Option<i32>,
                #[serde(rename = "lines_to_sample", skip_serializing_if = "Option::is_none")]
                lines_to_sample: Option<i32>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
                quote: Option<String>,
                #[serde(rename = "should_trim_fields", skip_serializing_if = "Option::is_none")]
                should_trim_fields: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
                #[serde(rename = "timestamp_field", skip_serializing_if = "Option::is_none")]
                timestamp_field: Option<String>,
                #[serde(rename = "timestamp_format", skip_serializing_if = "Option::is_none")]
                timestamp_format: Option<String>,
            }
            let query_params = QueryParamsStruct {
                charset: self.charset,
                column_names: self.column_names,
                delimiter: self.delimiter,
                error_trace: self.error_trace,
                explain: self.explain,
                filter_path: self.filter_path,
                format: self.format,
                grok_pattern: self.grok_pattern,
                has_header_row: self.has_header_row,
                human: self.human,
                line_merge_size_limit: self.line_merge_size_limit,
                lines_to_sample: self.lines_to_sample,
                pretty: self.pretty,
                quote: self.quote,
                should_trim_fields: self.should_trim_fields,
                source: self.source,
                timeout: self.timeout,
                timestamp_field: self.timestamp_field,
                timestamp_format: self.timestamp_format,
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
#[doc = "Url parts for the Ml Flush Job API"]
pub enum MlFlushJobUrlParts {
    JobId(String),
}
impl MlFlushJobUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlFlushJobUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(30usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/_flush");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Flush Job API"]
pub struct MlFlushJob<B> {
    client: Elasticsearch,
    parts: MlFlushJobUrlParts,
    advance_time: Option<String>,
    body: Option<B>,
    calc_interim: Option<bool>,
    end: Option<String>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    skip_time: Option<String>,
    source: Option<String>,
    start: Option<String>,
}
impl<B> MlFlushJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlFlushJobUrlParts) -> Self {
        MlFlushJob {
            client,
            parts,
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
    pub fn advance_time(mut self, advance_time: String) -> Self {
        self.advance_time = Some(advance_time);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlFlushJob<T>
    where
        T: Serialize,
    {
        MlFlushJob {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            advance_time: self.advance_time,
            calc_interim: self.calc_interim,
            end: self.end,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            skip_time: self.skip_time,
            source: self.source,
            start: self.start,
        }
    }
    #[doc = "Calculates interim results for the most recent bucket or all buckets within the latency period"]
    pub fn calc_interim(mut self, calc_interim: bool) -> Self {
        self.calc_interim = Some(calc_interim);
        self
    }
    #[doc = "When used in conjunction with calc_interim, specifies the range of buckets on which to calculate interim results"]
    pub fn end(mut self, end: String) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    #[doc = "Skips time to the given value without generating results or updating the model for the skipped interval"]
    pub fn skip_time(mut self, skip_time: String) -> Self {
        self.skip_time = Some(skip_time);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "When used in conjunction with calc_interim, specifies the range of buckets on which to calculate interim results"]
    pub fn start(mut self, start: String) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Flush Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "advance_time", skip_serializing_if = "Option::is_none")]
                advance_time: Option<String>,
                #[serde(rename = "calc_interim", skip_serializing_if = "Option::is_none")]
                calc_interim: Option<bool>,
                #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
                end: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "skip_time", skip_serializing_if = "Option::is_none")]
                skip_time: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
                start: Option<String>,
            }
            let query_params = QueryParamsStruct {
                advance_time: self.advance_time,
                calc_interim: self.calc_interim,
                end: self.end,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                skip_time: self.skip_time,
                source: self.source,
                start: self.start,
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
#[doc = "Url parts for the Ml Forecast API"]
pub enum MlForecastUrlParts {
    JobId(String),
}
impl MlForecastUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlForecastUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(33usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/_forecast");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Forecast API"]
pub struct MlForecast<B> {
    client: Elasticsearch,
    parts: MlForecastUrlParts,
    body: Option<B>,
    duration: Option<String>,
    error_trace: Option<bool>,
    expires_in: Option<String>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlForecast<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlForecastUrlParts) -> Self {
        MlForecast {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> MlForecast<T>
    where
        T: Serialize,
    {
        MlForecast {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            duration: self.duration,
            error_trace: self.error_trace,
            expires_in: self.expires_in,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "The duration of the forecast"]
    pub fn duration(mut self, duration: String) -> Self {
        self.duration = Some(duration);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "The time interval after which the forecast expires. Expired forecasts will be deleted at the first opportunity."]
    pub fn expires_in(mut self, expires_in: String) -> Self {
        self.expires_in = Some(expires_in);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Forecast API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
                duration: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "expires_in", skip_serializing_if = "Option::is_none")]
                expires_in: Option<String>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                duration: self.duration,
                error_trace: self.error_trace,
                expires_in: self.expires_in,
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
#[doc = "Url parts for the Ml Get Buckets API"]
pub enum MlGetBucketsUrlParts {
    JobIdTimestamp(String, String),
    JobId(String),
}
impl MlGetBucketsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetBucketsUrlParts::JobIdTimestamp(ref job_id, ref timestamp) => {
                let mut p = String::with_capacity(40usize + job_id.len() + timestamp.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/results/buckets/");
                p.push_str(timestamp.as_ref());
                p.into()
            }
            MlGetBucketsUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(39usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/results/buckets");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Buckets API"]
pub struct MlGetBuckets<B> {
    client: Elasticsearch,
    parts: MlGetBucketsUrlParts,
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
    pretty: Option<bool>,
    size: Option<i32>,
    sort: Option<String>,
    source: Option<String>,
    start: Option<String>,
}
impl<B> MlGetBuckets<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlGetBucketsUrlParts) -> Self {
        MlGetBuckets {
            client,
            parts,
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
        }
    }
    #[doc = "Filter for the most anomalous buckets"]
    pub fn anomaly_score(mut self, anomaly_score: f64) -> Self {
        self.anomaly_score = Some(anomaly_score);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlGetBuckets<T>
    where
        T: Serialize,
    {
        MlGetBuckets {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            anomaly_score: self.anomaly_score,
            desc: self.desc,
            end: self.end,
            error_trace: self.error_trace,
            exclude_interim: self.exclude_interim,
            expand: self.expand,
            filter_path: self.filter_path,
            from: self.from,
            human: self.human,
            pretty: self.pretty,
            size: self.size,
            sort: self.sort,
            source: self.source,
            start: self.start,
        }
    }
    #[doc = "Set the sort direction"]
    pub fn desc(mut self, desc: bool) -> Self {
        self.desc = Some(desc);
        self
    }
    #[doc = "End time filter for buckets"]
    pub fn end(mut self, end: String) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Exclude interim results"]
    pub fn exclude_interim(mut self, exclude_interim: bool) -> Self {
        self.exclude_interim = Some(exclude_interim);
        self
    }
    #[doc = "Include anomaly records"]
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of buckets"]
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
    #[doc = "specifies a max number of buckets to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "Sort buckets by a particular field"]
    pub fn sort(mut self, sort: String) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Start time filter for buckets"]
    pub fn start(mut self, start: String) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Buckets API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "anomaly_score", skip_serializing_if = "Option::is_none")]
                anomaly_score: Option<f64>,
                #[serde(rename = "desc", skip_serializing_if = "Option::is_none")]
                desc: Option<bool>,
                #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
                end: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "exclude_interim", skip_serializing_if = "Option::is_none")]
                exclude_interim: Option<bool>,
                #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
                expand: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i32>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i32>,
                #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
                sort: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
                start: Option<String>,
            }
            let query_params = QueryParamsStruct {
                anomaly_score: self.anomaly_score,
                desc: self.desc,
                end: self.end,
                error_trace: self.error_trace,
                exclude_interim: self.exclude_interim,
                expand: self.expand,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                size: self.size,
                sort: self.sort,
                source: self.source,
                start: self.start,
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
#[doc = "Url parts for the Ml Get Calendar Events API"]
pub enum MlGetCalendarEventsUrlParts {
    CalendarId(String),
}
impl MlGetCalendarEventsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetCalendarEventsUrlParts::CalendarId(ref calendar_id) => {
                let mut p = String::with_capacity(22usize + calendar_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(calendar_id.as_ref());
                p.push_str("/events");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Calendar Events API"]
pub struct MlGetCalendarEvents {
    client: Elasticsearch,
    parts: MlGetCalendarEventsUrlParts,
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
    pub fn new(client: Elasticsearch, parts: MlGetCalendarEventsUrlParts) -> Self {
        MlGetCalendarEvents {
            client,
            parts,
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
    pub fn end(mut self, end: String) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Skips a number of events"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Get events for the job. When this option is used calendar_id must be '_all'"]
    pub fn job_id(mut self, job_id: String) -> Self {
        self.job_id = Some(job_id);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Specifies a max number of events to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Get events after this time"]
    pub fn start(mut self, start: String) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Calendar Events API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
                end: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i32>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "job_id", skip_serializing_if = "Option::is_none")]
                job_id: Option<String>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i32>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
                start: Option<String>,
            }
            let query_params = QueryParamsStruct {
                end: self.end,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                job_id: self.job_id,
                pretty: self.pretty,
                size: self.size,
                source: self.source,
                start: self.start,
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
#[doc = "Url parts for the Ml Get Calendars API"]
pub enum MlGetCalendarsUrlParts {
    None,
    CalendarId(String),
}
impl MlGetCalendarsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetCalendarsUrlParts::None => "/_ml/calendars".into(),
            MlGetCalendarsUrlParts::CalendarId(ref calendar_id) => {
                let mut p = String::with_capacity(15usize + calendar_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(calendar_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Calendars API"]
pub struct MlGetCalendars<B> {
    client: Elasticsearch,
    parts: MlGetCalendarsUrlParts,
    body: Option<B>,
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
    pub fn new(client: Elasticsearch, parts: MlGetCalendarsUrlParts) -> Self {
        MlGetCalendars {
            client,
            parts,
            body: None,
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
    pub fn body<T>(self, body: T) -> MlGetCalendars<T>
    where
        T: Serialize,
    {
        MlGetCalendars {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            from: self.from,
            human: self.human,
            pretty: self.pretty,
            size: self.size,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of calendars"]
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
    #[doc = "specifies a max number of calendars to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Calendars API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i32>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i32>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Get Categories API"]
pub enum MlGetCategoriesUrlParts {
    JobIdCategoryId(String, i64),
    JobId(String),
}
impl MlGetCategoriesUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetCategoriesUrlParts::JobIdCategoryId(ref job_id, ref category_id) => {
                let category_id_str = category_id.to_string();
                let mut p = String::with_capacity(43usize + job_id.len() + category_id_str.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/results/categories/");
                p.push_str(category_id_str.as_ref());
                p.into()
            }
            MlGetCategoriesUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(43usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/results/categories/");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Categories API"]
pub struct MlGetCategories<B> {
    client: Elasticsearch,
    parts: MlGetCategoriesUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<String>,
}
impl<B> MlGetCategories<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlGetCategoriesUrlParts) -> Self {
        MlGetCategories {
            client,
            parts,
            body: None,
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
    pub fn body<T>(self, body: T) -> MlGetCategories<T>
    where
        T: Serialize,
    {
        MlGetCategories {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            from: self.from,
            human: self.human,
            pretty: self.pretty,
            size: self.size,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of categories"]
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
    #[doc = "specifies a max number of categories to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Categories API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i32>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i32>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Get Data Frame Analytics API"]
pub enum MlGetDataFrameAnalyticsUrlParts {
    Id(String),
    None,
}
impl MlGetDataFrameAnalyticsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetDataFrameAnalyticsUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(26usize + id.len());
                p.push_str("/_ml/data_frame/analytics/");
                p.push_str(id.as_ref());
                p.into()
            }
            MlGetDataFrameAnalyticsUrlParts::None => "/_ml/data_frame/analytics".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Data Frame Analytics API"]
pub struct MlGetDataFrameAnalytics {
    client: Elasticsearch,
    parts: MlGetDataFrameAnalyticsUrlParts,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<String>,
}
impl MlGetDataFrameAnalytics {
    pub fn new(client: Elasticsearch, parts: MlGetDataFrameAnalyticsUrlParts) -> Self {
        MlGetDataFrameAnalytics {
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
    #[doc = "Whether to ignore if a wildcard expression matches no data frame analytics. (This includes `_all` string or when no data frame analytics have been specified)"]
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of analytics"]
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
    #[doc = "specifies a max number of analytics to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Data Frame Analytics API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_match", skip_serializing_if = "Option::is_none")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i32>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i32>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Get Data Frame Analytics Stats API"]
pub enum MlGetDataFrameAnalyticsStatsUrlParts {
    None,
    Id(String),
}
impl MlGetDataFrameAnalyticsStatsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetDataFrameAnalyticsStatsUrlParts::None => "/_ml/data_frame/analytics/_stats".into(),
            MlGetDataFrameAnalyticsStatsUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(33usize + id.len());
                p.push_str("/_ml/data_frame/analytics/");
                p.push_str(id.as_ref());
                p.push_str("/_stats");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Data Frame Analytics Stats API"]
pub struct MlGetDataFrameAnalyticsStats {
    client: Elasticsearch,
    parts: MlGetDataFrameAnalyticsStatsUrlParts,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<String>,
}
impl MlGetDataFrameAnalyticsStats {
    pub fn new(client: Elasticsearch, parts: MlGetDataFrameAnalyticsStatsUrlParts) -> Self {
        MlGetDataFrameAnalyticsStats {
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
    #[doc = "Whether to ignore if a wildcard expression matches no data frame analytics. (This includes `_all` string or when no data frame analytics have been specified)"]
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of analytics"]
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
    #[doc = "specifies a max number of analytics to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Data Frame Analytics Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_match", skip_serializing_if = "Option::is_none")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i32>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i32>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Get Datafeed Stats API"]
pub enum MlGetDatafeedStatsUrlParts {
    DatafeedId(String),
    None,
}
impl MlGetDatafeedStatsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetDatafeedStatsUrlParts::DatafeedId(ref datafeed_id) => {
                let mut p = String::with_capacity(22usize + datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(datafeed_id.as_ref());
                p.push_str("/_stats");
                p.into()
            }
            MlGetDatafeedStatsUrlParts::None => "/_ml/datafeeds/_stats".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Datafeed Stats API"]
pub struct MlGetDatafeedStats {
    client: Elasticsearch,
    parts: MlGetDatafeedStatsUrlParts,
    allow_no_datafeeds: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlGetDatafeedStats {
    pub fn new(client: Elasticsearch, parts: MlGetDatafeedStatsUrlParts) -> Self {
        MlGetDatafeedStats {
            client,
            parts,
            allow_no_datafeeds: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no datafeeds. (This includes `_all` string or when no datafeeds have been specified)"]
    pub fn allow_no_datafeeds(mut self, allow_no_datafeeds: bool) -> Self {
        self.allow_no_datafeeds = Some(allow_no_datafeeds);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Datafeed Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_datafeeds", skip_serializing_if = "Option::is_none")]
                allow_no_datafeeds: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_datafeeds: self.allow_no_datafeeds,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Get Datafeeds API"]
pub enum MlGetDatafeedsUrlParts {
    DatafeedId(String),
    None,
}
impl MlGetDatafeedsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetDatafeedsUrlParts::DatafeedId(ref datafeed_id) => {
                let mut p = String::with_capacity(15usize + datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(datafeed_id.as_ref());
                p.into()
            }
            MlGetDatafeedsUrlParts::None => "/_ml/datafeeds".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Datafeeds API"]
pub struct MlGetDatafeeds {
    client: Elasticsearch,
    parts: MlGetDatafeedsUrlParts,
    allow_no_datafeeds: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlGetDatafeeds {
    pub fn new(client: Elasticsearch, parts: MlGetDatafeedsUrlParts) -> Self {
        MlGetDatafeeds {
            client,
            parts,
            allow_no_datafeeds: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no datafeeds. (This includes `_all` string or when no datafeeds have been specified)"]
    pub fn allow_no_datafeeds(mut self, allow_no_datafeeds: bool) -> Self {
        self.allow_no_datafeeds = Some(allow_no_datafeeds);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Datafeeds API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_datafeeds", skip_serializing_if = "Option::is_none")]
                allow_no_datafeeds: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_datafeeds: self.allow_no_datafeeds,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Get Filters API"]
pub enum MlGetFiltersUrlParts {
    None,
    FilterId(String),
}
impl MlGetFiltersUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetFiltersUrlParts::None => "/_ml/filters".into(),
            MlGetFiltersUrlParts::FilterId(ref filter_id) => {
                let mut p = String::with_capacity(13usize + filter_id.len());
                p.push_str("/_ml/filters/");
                p.push_str(filter_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Filters API"]
pub struct MlGetFilters {
    client: Elasticsearch,
    parts: MlGetFiltersUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<String>,
}
impl MlGetFilters {
    pub fn new(client: Elasticsearch, parts: MlGetFiltersUrlParts) -> Self {
        MlGetFilters {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of filters"]
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
    #[doc = "specifies a max number of filters to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Filters API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i32>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i32>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
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
#[doc = "Url parts for the Ml Get Influencers API"]
pub enum MlGetInfluencersUrlParts {
    JobId(String),
}
impl MlGetInfluencersUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetInfluencersUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(43usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/results/influencers");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Influencers API"]
pub struct MlGetInfluencers<B> {
    client: Elasticsearch,
    parts: MlGetInfluencersUrlParts,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<String>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    influencer_score: Option<f64>,
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
    pub fn new(client: Elasticsearch, parts: MlGetInfluencersUrlParts) -> Self {
        MlGetInfluencers {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> MlGetInfluencers<T>
    where
        T: Serialize,
    {
        MlGetInfluencers {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            desc: self.desc,
            end: self.end,
            error_trace: self.error_trace,
            exclude_interim: self.exclude_interim,
            filter_path: self.filter_path,
            from: self.from,
            human: self.human,
            influencer_score: self.influencer_score,
            pretty: self.pretty,
            size: self.size,
            sort: self.sort,
            source: self.source,
            start: self.start,
        }
    }
    #[doc = "whether the results should be sorted in decending order"]
    pub fn desc(mut self, desc: bool) -> Self {
        self.desc = Some(desc);
        self
    }
    #[doc = "end timestamp for the requested influencers"]
    pub fn end(mut self, end: String) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Exclude interim results"]
    pub fn exclude_interim(mut self, exclude_interim: bool) -> Self {
        self.exclude_interim = Some(exclude_interim);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of influencers"]
    pub fn from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "influencer score threshold for the requested influencers"]
    pub fn influencer_score(mut self, influencer_score: f64) -> Self {
        self.influencer_score = Some(influencer_score);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "specifies a max number of influencers to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "sort field for the requested influencers"]
    pub fn sort(mut self, sort: String) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "start timestamp for the requested influencers"]
    pub fn start(mut self, start: String) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Influencers API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "desc", skip_serializing_if = "Option::is_none")]
                desc: Option<bool>,
                #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
                end: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "exclude_interim", skip_serializing_if = "Option::is_none")]
                exclude_interim: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i32>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "influencer_score", skip_serializing_if = "Option::is_none")]
                influencer_score: Option<f64>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i32>,
                #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
                sort: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
                start: Option<String>,
            }
            let query_params = QueryParamsStruct {
                desc: self.desc,
                end: self.end,
                error_trace: self.error_trace,
                exclude_interim: self.exclude_interim,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                influencer_score: self.influencer_score,
                pretty: self.pretty,
                size: self.size,
                sort: self.sort,
                source: self.source,
                start: self.start,
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
#[doc = "Url parts for the Ml Get Job Stats API"]
pub enum MlGetJobStatsUrlParts {
    None,
    JobId(String),
}
impl MlGetJobStatsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetJobStatsUrlParts::None => "/_ml/anomaly_detectors/_stats".into(),
            MlGetJobStatsUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(30usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/_stats");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Job Stats API"]
pub struct MlGetJobStats {
    client: Elasticsearch,
    parts: MlGetJobStatsUrlParts,
    allow_no_jobs: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlGetJobStats {
    pub fn new(client: Elasticsearch, parts: MlGetJobStatsUrlParts) -> Self {
        MlGetJobStats {
            client,
            parts,
            allow_no_jobs: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: bool) -> Self {
        self.allow_no_jobs = Some(allow_no_jobs);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Job Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_jobs", skip_serializing_if = "Option::is_none")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_jobs: self.allow_no_jobs,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Get Jobs API"]
pub enum MlGetJobsUrlParts {
    JobId(String),
    None,
}
impl MlGetJobsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetJobsUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(23usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.into()
            }
            MlGetJobsUrlParts::None => "/_ml/anomaly_detectors".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Jobs API"]
pub struct MlGetJobs {
    client: Elasticsearch,
    parts: MlGetJobsUrlParts,
    allow_no_jobs: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlGetJobs {
    pub fn new(client: Elasticsearch, parts: MlGetJobsUrlParts) -> Self {
        MlGetJobs {
            client,
            parts,
            allow_no_jobs: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: bool) -> Self {
        self.allow_no_jobs = Some(allow_no_jobs);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Jobs API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_jobs", skip_serializing_if = "Option::is_none")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_jobs: self.allow_no_jobs,
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Get Model Snapshots API"]
pub enum MlGetModelSnapshotsUrlParts {
    JobIdSnapshotId(String, String),
    JobId(String),
}
impl MlGetModelSnapshotsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetModelSnapshotsUrlParts::JobIdSnapshotId(ref job_id, ref snapshot_id) => {
                let mut p = String::with_capacity(40usize + job_id.len() + snapshot_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/model_snapshots/");
                p.push_str(snapshot_id.as_ref());
                p.into()
            }
            MlGetModelSnapshotsUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(39usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/model_snapshots");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Model Snapshots API"]
pub struct MlGetModelSnapshots<B> {
    client: Elasticsearch,
    parts: MlGetModelSnapshotsUrlParts,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<String>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    sort: Option<String>,
    source: Option<String>,
    start: Option<String>,
}
impl<B> MlGetModelSnapshots<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlGetModelSnapshotsUrlParts) -> Self {
        MlGetModelSnapshots {
            client,
            parts,
            body: None,
            desc: None,
            end: None,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            size: None,
            sort: None,
            source: None,
            start: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlGetModelSnapshots<T>
    where
        T: Serialize,
    {
        MlGetModelSnapshots {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            desc: self.desc,
            end: self.end,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            from: self.from,
            human: self.human,
            pretty: self.pretty,
            size: self.size,
            sort: self.sort,
            source: self.source,
            start: self.start,
        }
    }
    #[doc = "True if the results should be sorted in descending order"]
    pub fn desc(mut self, desc: bool) -> Self {
        self.desc = Some(desc);
        self
    }
    #[doc = "The filter 'end' query parameter"]
    pub fn end(mut self, end: String) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Skips a number of documents"]
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
    #[doc = "The default number of documents returned in queries as a string."]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "Name of the field to sort on"]
    pub fn sort(mut self, sort: String) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The filter 'start' query parameter"]
    pub fn start(mut self, start: String) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Model Snapshots API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "desc", skip_serializing_if = "Option::is_none")]
                desc: Option<bool>,
                #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
                end: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i32>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i32>,
                #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
                sort: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
                start: Option<String>,
            }
            let query_params = QueryParamsStruct {
                desc: self.desc,
                end: self.end,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                size: self.size,
                sort: self.sort,
                source: self.source,
                start: self.start,
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
#[doc = "Url parts for the Ml Get Overall Buckets API"]
pub enum MlGetOverallBucketsUrlParts {
    JobId(String),
}
impl MlGetOverallBucketsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetOverallBucketsUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(47usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/results/overall_buckets");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Overall Buckets API"]
pub struct MlGetOverallBuckets<B> {
    client: Elasticsearch,
    parts: MlGetOverallBucketsUrlParts,
    allow_no_jobs: Option<bool>,
    body: Option<B>,
    bucket_span: Option<String>,
    end: Option<String>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
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
    pub fn new(client: Elasticsearch, parts: MlGetOverallBucketsUrlParts) -> Self {
        MlGetOverallBuckets {
            client,
            parts,
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
    pub fn allow_no_jobs(mut self, allow_no_jobs: bool) -> Self {
        self.allow_no_jobs = Some(allow_no_jobs);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlGetOverallBuckets<T>
    where
        T: Serialize,
    {
        MlGetOverallBuckets {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            allow_no_jobs: self.allow_no_jobs,
            bucket_span: self.bucket_span,
            end: self.end,
            error_trace: self.error_trace,
            exclude_interim: self.exclude_interim,
            filter_path: self.filter_path,
            human: self.human,
            overall_score: self.overall_score,
            pretty: self.pretty,
            source: self.source,
            start: self.start,
            top_n: self.top_n,
        }
    }
    #[doc = "The span of the overall buckets. Defaults to the longest job bucket_span"]
    pub fn bucket_span(mut self, bucket_span: String) -> Self {
        self.bucket_span = Some(bucket_span);
        self
    }
    #[doc = "Returns overall buckets with timestamps earlier than this time"]
    pub fn end(mut self, end: String) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "If true overall buckets that include interim buckets will be excluded"]
    pub fn exclude_interim(mut self, exclude_interim: bool) -> Self {
        self.exclude_interim = Some(exclude_interim);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Returns overall buckets with overall scores higher than this value"]
    pub fn overall_score(mut self, overall_score: f64) -> Self {
        self.overall_score = Some(overall_score);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Returns overall buckets with timestamps after this time"]
    pub fn start(mut self, start: String) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "The number of top job bucket scores to be used in the overall_score calculation"]
    pub fn top_n(mut self, top_n: i32) -> Self {
        self.top_n = Some(top_n);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Overall Buckets API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_jobs", skip_serializing_if = "Option::is_none")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "bucket_span", skip_serializing_if = "Option::is_none")]
                bucket_span: Option<String>,
                #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
                end: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "exclude_interim", skip_serializing_if = "Option::is_none")]
                exclude_interim: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "overall_score", skip_serializing_if = "Option::is_none")]
                overall_score: Option<f64>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
                start: Option<String>,
                #[serde(rename = "top_n", skip_serializing_if = "Option::is_none")]
                top_n: Option<i32>,
            }
            let query_params = QueryParamsStruct {
                allow_no_jobs: self.allow_no_jobs,
                bucket_span: self.bucket_span,
                end: self.end,
                error_trace: self.error_trace,
                exclude_interim: self.exclude_interim,
                filter_path: self.filter_path,
                human: self.human,
                overall_score: self.overall_score,
                pretty: self.pretty,
                source: self.source,
                start: self.start,
                top_n: self.top_n,
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
#[doc = "Url parts for the Ml Get Records API"]
pub enum MlGetRecordsUrlParts {
    JobId(String),
}
impl MlGetRecordsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlGetRecordsUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(39usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/results/records");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Get Records API"]
pub struct MlGetRecords<B> {
    client: Elasticsearch,
    parts: MlGetRecordsUrlParts,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<String>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    filter_path: Option<Vec<String>>,
    from: Option<i32>,
    human: Option<bool>,
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
    pub fn new(client: Elasticsearch, parts: MlGetRecordsUrlParts) -> Self {
        MlGetRecords {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> MlGetRecords<T>
    where
        T: Serialize,
    {
        MlGetRecords {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            desc: self.desc,
            end: self.end,
            error_trace: self.error_trace,
            exclude_interim: self.exclude_interim,
            filter_path: self.filter_path,
            from: self.from,
            human: self.human,
            pretty: self.pretty,
            record_score: self.record_score,
            size: self.size,
            sort: self.sort,
            source: self.source,
            start: self.start,
        }
    }
    #[doc = "Set the sort direction"]
    pub fn desc(mut self, desc: bool) -> Self {
        self.desc = Some(desc);
        self
    }
    #[doc = "End time filter for records"]
    pub fn end(mut self, end: String) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Exclude interim results"]
    pub fn exclude_interim(mut self, exclude_interim: bool) -> Self {
        self.exclude_interim = Some(exclude_interim);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "skips a number of records"]
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
    pub fn record_score(mut self, record_score: f64) -> Self {
        self.record_score = Some(record_score);
        self
    }
    #[doc = "specifies a max number of records to get"]
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "Sort records by a particular field"]
    pub fn sort(mut self, sort: String) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Start time filter for records"]
    pub fn start(mut self, start: String) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Records API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "desc", skip_serializing_if = "Option::is_none")]
                desc: Option<bool>,
                #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
                end: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(rename = "exclude_interim", skip_serializing_if = "Option::is_none")]
                exclude_interim: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
                from: Option<i32>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "record_score", skip_serializing_if = "Option::is_none")]
                record_score: Option<f64>,
                #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
                size: Option<i32>,
                #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
                sort: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
                start: Option<String>,
            }
            let query_params = QueryParamsStruct {
                desc: self.desc,
                end: self.end,
                error_trace: self.error_trace,
                exclude_interim: self.exclude_interim,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                record_score: self.record_score,
                size: self.size,
                sort: self.sort,
                source: self.source,
                start: self.start,
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
#[doc = "Url parts for the Ml Info API"]
pub enum MlInfoUrlParts {
    None,
}
impl MlInfoUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlInfoUrlParts::None => "/_ml/info".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Info API"]
pub struct MlInfo {
    client: Elasticsearch,
    parts: MlInfoUrlParts,
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
            parts: MlInfoUrlParts::None,
            error_trace: None,
            filter_path: None,
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Info API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Open Job API"]
pub enum MlOpenJobUrlParts {
    JobId(String),
}
impl MlOpenJobUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlOpenJobUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(29usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/_open");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Open Job API"]
pub struct MlOpenJob<B> {
    client: Elasticsearch,
    parts: MlOpenJobUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlOpenJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlOpenJobUrlParts) -> Self {
        MlOpenJob {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlOpenJob<T>
    where
        T: Serialize,
    {
        MlOpenJob {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Open Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Post Calendar Events API"]
pub enum MlPostCalendarEventsUrlParts {
    CalendarId(String),
}
impl MlPostCalendarEventsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlPostCalendarEventsUrlParts::CalendarId(ref calendar_id) => {
                let mut p = String::with_capacity(22usize + calendar_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(calendar_id.as_ref());
                p.push_str("/events");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Post Calendar Events API"]
pub struct MlPostCalendarEvents<B> {
    client: Elasticsearch,
    parts: MlPostCalendarEventsUrlParts,
    body: Option<B>,
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
    pub fn new(client: Elasticsearch, parts: MlPostCalendarEventsUrlParts) -> Self {
        MlPostCalendarEvents {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlPostCalendarEvents<T>
    where
        T: Serialize,
    {
        MlPostCalendarEvents {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Post Calendar Events API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Post Data API"]
pub enum MlPostDataUrlParts {
    JobId(String),
}
impl MlPostDataUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlPostDataUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(29usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/_data");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Post Data API"]
pub struct MlPostData<B> {
    client: Elasticsearch,
    parts: MlPostDataUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    reset_end: Option<String>,
    reset_start: Option<String>,
    source: Option<String>,
}
impl<B> MlPostData<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlPostDataUrlParts) -> Self {
        MlPostData {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> MlPostData<T>
    where
        T: Serialize,
    {
        MlPostData {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            reset_end: self.reset_end,
            reset_start: self.reset_start,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    #[doc = "Optional parameter to specify the end of the bucket resetting range"]
    pub fn reset_end(mut self, reset_end: String) -> Self {
        self.reset_end = Some(reset_end);
        self
    }
    #[doc = "Optional parameter to specify the start of the bucket resetting range"]
    pub fn reset_start(mut self, reset_start: String) -> Self {
        self.reset_start = Some(reset_start);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Post Data API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "reset_end", skip_serializing_if = "Option::is_none")]
                reset_end: Option<String>,
                #[serde(rename = "reset_start", skip_serializing_if = "Option::is_none")]
                reset_start: Option<String>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                reset_end: self.reset_end,
                reset_start: self.reset_start,
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
#[doc = "Url parts for the Ml Preview Datafeed API"]
pub enum MlPreviewDatafeedUrlParts {
    DatafeedId(String),
}
impl MlPreviewDatafeedUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlPreviewDatafeedUrlParts::DatafeedId(ref datafeed_id) => {
                let mut p = String::with_capacity(24usize + datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(datafeed_id.as_ref());
                p.push_str("/_preview");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Preview Datafeed API"]
pub struct MlPreviewDatafeed {
    client: Elasticsearch,
    parts: MlPreviewDatafeedUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPreviewDatafeed {
    pub fn new(client: Elasticsearch, parts: MlPreviewDatafeedUrlParts) -> Self {
        MlPreviewDatafeed {
            client,
            parts,
            error_trace: None,
            filter_path: None,
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Preview Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Put Calendar API"]
pub enum MlPutCalendarUrlParts {
    CalendarId(String),
}
impl MlPutCalendarUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlPutCalendarUrlParts::CalendarId(ref calendar_id) => {
                let mut p = String::with_capacity(15usize + calendar_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(calendar_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Put Calendar API"]
pub struct MlPutCalendar<B> {
    client: Elasticsearch,
    parts: MlPutCalendarUrlParts,
    body: Option<B>,
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
    pub fn new(client: Elasticsearch, parts: MlPutCalendarUrlParts) -> Self {
        MlPutCalendar {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlPutCalendar<T>
    where
        T: Serialize,
    {
        MlPutCalendar {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Put Calendar API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Put Calendar Job API"]
pub enum MlPutCalendarJobUrlParts {
    CalendarIdJobId(String, String),
}
impl MlPutCalendarJobUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlPutCalendarJobUrlParts::CalendarIdJobId(ref calendar_id, ref job_id) => {
                let mut p = String::with_capacity(21usize + calendar_id.len() + job_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(calendar_id.as_ref());
                p.push_str("/jobs/");
                p.push_str(job_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Put Calendar Job API"]
pub struct MlPutCalendarJob<B> {
    client: Elasticsearch,
    parts: MlPutCalendarJobUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlPutCalendarJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlPutCalendarJobUrlParts) -> Self {
        MlPutCalendarJob {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlPutCalendarJob<T>
    where
        T: Serialize,
    {
        MlPutCalendarJob {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Put Calendar Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Put Data Frame Analytics API"]
pub enum MlPutDataFrameAnalyticsUrlParts {
    Id(String),
}
impl MlPutDataFrameAnalyticsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlPutDataFrameAnalyticsUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(26usize + id.len());
                p.push_str("/_ml/data_frame/analytics/");
                p.push_str(id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Put Data Frame Analytics API"]
pub struct MlPutDataFrameAnalytics<B> {
    client: Elasticsearch,
    parts: MlPutDataFrameAnalyticsUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlPutDataFrameAnalytics<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlPutDataFrameAnalyticsUrlParts) -> Self {
        MlPutDataFrameAnalytics {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlPutDataFrameAnalytics<T>
    where
        T: Serialize,
    {
        MlPutDataFrameAnalytics {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Put Data Frame Analytics API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Put Datafeed API"]
pub enum MlPutDatafeedUrlParts {
    DatafeedId(String),
}
impl MlPutDatafeedUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlPutDatafeedUrlParts::DatafeedId(ref datafeed_id) => {
                let mut p = String::with_capacity(15usize + datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(datafeed_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Put Datafeed API"]
pub struct MlPutDatafeed<B> {
    client: Elasticsearch,
    parts: MlPutDatafeedUrlParts,
    body: Option<B>,
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
    pub fn new(client: Elasticsearch, parts: MlPutDatafeedUrlParts) -> Self {
        MlPutDatafeed {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlPutDatafeed<T>
    where
        T: Serialize,
    {
        MlPutDatafeed {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Put Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Put Filter API"]
pub enum MlPutFilterUrlParts {
    FilterId(String),
}
impl MlPutFilterUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlPutFilterUrlParts::FilterId(ref filter_id) => {
                let mut p = String::with_capacity(13usize + filter_id.len());
                p.push_str("/_ml/filters/");
                p.push_str(filter_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Put Filter API"]
pub struct MlPutFilter<B> {
    client: Elasticsearch,
    parts: MlPutFilterUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlPutFilter<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlPutFilterUrlParts) -> Self {
        MlPutFilter {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlPutFilter<T>
    where
        T: Serialize,
    {
        MlPutFilter {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Put Filter API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Put Job API"]
pub enum MlPutJobUrlParts {
    JobId(String),
}
impl MlPutJobUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlPutJobUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(23usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Put Job API"]
pub struct MlPutJob<B> {
    client: Elasticsearch,
    parts: MlPutJobUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlPutJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlPutJobUrlParts) -> Self {
        MlPutJob {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlPutJob<T>
    where
        T: Serialize,
    {
        MlPutJob {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Put Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Revert Model Snapshot API"]
pub enum MlRevertModelSnapshotUrlParts {
    JobIdSnapshotId(String, String),
}
impl MlRevertModelSnapshotUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlRevertModelSnapshotUrlParts::JobIdSnapshotId(ref job_id, ref snapshot_id) => {
                let mut p = String::with_capacity(48usize + job_id.len() + snapshot_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/model_snapshots/");
                p.push_str(snapshot_id.as_ref());
                p.push_str("/_revert");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Revert Model Snapshot API"]
pub struct MlRevertModelSnapshot<B> {
    client: Elasticsearch,
    parts: MlRevertModelSnapshotUrlParts,
    body: Option<B>,
    delete_intervening_results: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlRevertModelSnapshot<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlRevertModelSnapshotUrlParts) -> Self {
        MlRevertModelSnapshot {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> MlRevertModelSnapshot<T>
    where
        T: Serialize,
    {
        MlRevertModelSnapshot {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            delete_intervening_results: self.delete_intervening_results,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Should we reset the results back to the time of the snapshot?"]
    pub fn delete_intervening_results(mut self, delete_intervening_results: bool) -> Self {
        self.delete_intervening_results = Some(delete_intervening_results);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Revert Model Snapshot API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(
                    rename = "delete_intervening_results",
                    skip_serializing_if = "Option::is_none"
                )]
                delete_intervening_results: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                delete_intervening_results: self.delete_intervening_results,
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
#[doc = "Url parts for the Ml Set Upgrade Mode API"]
pub enum MlSetUpgradeModeUrlParts {
    None,
}
impl MlSetUpgradeModeUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlSetUpgradeModeUrlParts::None => "/_ml/set_upgrade_mode".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Set Upgrade Mode API"]
pub struct MlSetUpgradeMode<B> {
    client: Elasticsearch,
    parts: MlSetUpgradeModeUrlParts,
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
            parts: MlSetUpgradeModeUrlParts::None,
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
    pub fn body<T>(self, body: T) -> MlSetUpgradeMode<T>
    where
        T: Serialize,
    {
        MlSetUpgradeMode {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            enabled: self.enabled,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Whether to enable upgrade_mode ML setting or not. Defaults to false."]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait before action times out. Defaults to 30 seconds"]
    pub fn timeout(mut self, timeout: String) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Set Upgrade Mode API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
                enabled: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                enabled: self.enabled,
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
#[doc = "Url parts for the Ml Start Data Frame Analytics API"]
pub enum MlStartDataFrameAnalyticsUrlParts {
    Id(String),
}
impl MlStartDataFrameAnalyticsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlStartDataFrameAnalyticsUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(33usize + id.len());
                p.push_str("/_ml/data_frame/analytics/");
                p.push_str(id.as_ref());
                p.push_str("/_start");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Start Data Frame Analytics API"]
pub struct MlStartDataFrameAnalytics<B> {
    client: Elasticsearch,
    parts: MlStartDataFrameAnalyticsUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> MlStartDataFrameAnalytics<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlStartDataFrameAnalyticsUrlParts) -> Self {
        MlStartDataFrameAnalytics {
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
    pub fn body<T>(self, body: T) -> MlStartDataFrameAnalytics<T>
    where
        T: Serialize,
    {
        MlStartDataFrameAnalytics {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait until the task has started. Defaults to 20 seconds"]
    pub fn timeout(mut self, timeout: String) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Start Data Frame Analytics API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Start Datafeed API"]
pub enum MlStartDatafeedUrlParts {
    DatafeedId(String),
}
impl MlStartDatafeedUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlStartDatafeedUrlParts::DatafeedId(ref datafeed_id) => {
                let mut p = String::with_capacity(22usize + datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(datafeed_id.as_ref());
                p.push_str("/_start");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Start Datafeed API"]
pub struct MlStartDatafeed<B> {
    client: Elasticsearch,
    parts: MlStartDatafeedUrlParts,
    body: Option<B>,
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
    pub fn new(client: Elasticsearch, parts: MlStartDatafeedUrlParts) -> Self {
        MlStartDatafeed {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> MlStartDatafeed<T>
    where
        T: Serialize,
    {
        MlStartDatafeed {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            end: self.end,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
            start: self.start,
            timeout: self.timeout,
        }
    }
    #[doc = "The end time when the datafeed should stop. When not set, the datafeed continues in real time"]
    pub fn end(mut self, end: String) -> Self {
        self.end = Some(end);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The start time from where the datafeed should begin"]
    pub fn start(mut self, start: String) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Controls the time to wait until a datafeed has started. Default to 20 seconds"]
    pub fn timeout(mut self, timeout: String) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Start Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
                end: Option<String>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
                start: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                end: self.end,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                start: self.start,
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
#[doc = "Url parts for the Ml Stop Data Frame Analytics API"]
pub enum MlStopDataFrameAnalyticsUrlParts {
    Id(String),
}
impl MlStopDataFrameAnalyticsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlStopDataFrameAnalyticsUrlParts::Id(ref id) => {
                let mut p = String::with_capacity(32usize + id.len());
                p.push_str("/_ml/data_frame/analytics/");
                p.push_str(id.as_ref());
                p.push_str("/_stop");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Stop Data Frame Analytics API"]
pub struct MlStopDataFrameAnalytics<B> {
    client: Elasticsearch,
    parts: MlStopDataFrameAnalyticsUrlParts,
    allow_no_match: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> MlStopDataFrameAnalytics<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlStopDataFrameAnalyticsUrlParts) -> Self {
        MlStopDataFrameAnalytics {
            client,
            parts,
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
    pub fn allow_no_match(mut self, allow_no_match: bool) -> Self {
        self.allow_no_match = Some(allow_no_match);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlStopDataFrameAnalytics<T>
    where
        T: Serialize,
    {
        MlStopDataFrameAnalytics {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            allow_no_match: self.allow_no_match,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            force: self.force,
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "True if the data frame analytics should be forcefully stopped"]
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait until the task has stopped. Defaults to 20 seconds"]
    pub fn timeout(mut self, timeout: String) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Stop Data Frame Analytics API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_match", skip_serializing_if = "Option::is_none")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
                force: Option<bool>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_match: self.allow_no_match,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                force: self.force,
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
#[doc = "Url parts for the Ml Stop Datafeed API"]
pub enum MlStopDatafeedUrlParts {
    DatafeedId(String),
}
impl MlStopDatafeedUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlStopDatafeedUrlParts::DatafeedId(ref datafeed_id) => {
                let mut p = String::with_capacity(21usize + datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(datafeed_id.as_ref());
                p.push_str("/_stop");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Stop Datafeed API"]
pub struct MlStopDatafeed<B> {
    client: Elasticsearch,
    parts: MlStopDatafeedUrlParts,
    allow_no_datafeeds: Option<bool>,
    body: Option<B>,
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
    pub fn new(client: Elasticsearch, parts: MlStopDatafeedUrlParts) -> Self {
        MlStopDatafeed {
            client,
            parts,
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
    pub fn allow_no_datafeeds(mut self, allow_no_datafeeds: bool) -> Self {
        self.allow_no_datafeeds = Some(allow_no_datafeeds);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlStopDatafeed<T>
    where
        T: Serialize,
    {
        MlStopDatafeed {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            allow_no_datafeeds: self.allow_no_datafeeds,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            force: self.force,
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "True if the datafeed should be forcefully stopped."]
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait until a datafeed has stopped. Default to 20 seconds"]
    pub fn timeout(mut self, timeout: String) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Stop Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "allow_no_datafeeds", skip_serializing_if = "Option::is_none")]
                allow_no_datafeeds: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
                force: Option<bool>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<String>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                allow_no_datafeeds: self.allow_no_datafeeds,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                force: self.force,
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
#[doc = "Url parts for the Ml Update Datafeed API"]
pub enum MlUpdateDatafeedUrlParts {
    DatafeedId(String),
}
impl MlUpdateDatafeedUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlUpdateDatafeedUrlParts::DatafeedId(ref datafeed_id) => {
                let mut p = String::with_capacity(23usize + datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(datafeed_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Update Datafeed API"]
pub struct MlUpdateDatafeed<B> {
    client: Elasticsearch,
    parts: MlUpdateDatafeedUrlParts,
    body: Option<B>,
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
    pub fn new(client: Elasticsearch, parts: MlUpdateDatafeedUrlParts) -> Self {
        MlUpdateDatafeed {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlUpdateDatafeed<T>
    where
        T: Serialize,
    {
        MlUpdateDatafeed {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Update Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Update Filter API"]
pub enum MlUpdateFilterUrlParts {
    FilterId(String),
}
impl MlUpdateFilterUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlUpdateFilterUrlParts::FilterId(ref filter_id) => {
                let mut p = String::with_capacity(21usize + filter_id.len());
                p.push_str("/_ml/filters/");
                p.push_str(filter_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Update Filter API"]
pub struct MlUpdateFilter<B> {
    client: Elasticsearch,
    parts: MlUpdateFilterUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlUpdateFilter<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlUpdateFilterUrlParts) -> Self {
        MlUpdateFilter {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlUpdateFilter<T>
    where
        T: Serialize,
    {
        MlUpdateFilter {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Update Filter API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Update Job API"]
pub enum MlUpdateJobUrlParts {
    JobId(String),
}
impl MlUpdateJobUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlUpdateJobUrlParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(31usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Update Job API"]
pub struct MlUpdateJob<B> {
    client: Elasticsearch,
    parts: MlUpdateJobUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlUpdateJob<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlUpdateJobUrlParts) -> Self {
        MlUpdateJob {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlUpdateJob<T>
    where
        T: Serialize,
    {
        MlUpdateJob {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Update Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Update Model Snapshot API"]
pub enum MlUpdateModelSnapshotUrlParts {
    JobIdSnapshotId(String, String),
}
impl MlUpdateModelSnapshotUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlUpdateModelSnapshotUrlParts::JobIdSnapshotId(ref job_id, ref snapshot_id) => {
                let mut p = String::with_capacity(48usize + job_id.len() + snapshot_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/model_snapshots/");
                p.push_str(snapshot_id.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Update Model Snapshot API"]
pub struct MlUpdateModelSnapshot<B> {
    client: Elasticsearch,
    parts: MlUpdateModelSnapshotUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl<B> MlUpdateModelSnapshot<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: MlUpdateModelSnapshotUrlParts) -> Self {
        MlUpdateModelSnapshot {
            client,
            parts,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlUpdateModelSnapshot<T>
    where
        T: Serialize,
    {
        MlUpdateModelSnapshot {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Update Model Snapshot API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Validate API"]
pub enum MlValidateUrlParts {
    None,
}
impl MlValidateUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlValidateUrlParts::None => "/_ml/anomaly_detectors/_validate".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Validate API"]
pub struct MlValidate<B> {
    client: Elasticsearch,
    parts: MlValidateUrlParts,
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
            parts: MlValidateUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlValidate<T>
    where
        T: Serialize,
    {
        MlValidate {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Validate API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Validate Detector API"]
pub enum MlValidateDetectorUrlParts {
    None,
}
impl MlValidateDetectorUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            MlValidateDetectorUrlParts::None => "/_ml/anomaly_detectors/_validate/detector".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Ml Validate Detector API"]
pub struct MlValidateDetector<B> {
    client: Elasticsearch,
    parts: MlValidateDetectorUrlParts,
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
            parts: MlValidateDetectorUrlParts::None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlValidateDetector<T>
    where
        T: Serialize,
    {
        MlValidateDetector {
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
    pub fn filter_path(mut self, filter_path: Vec<String>) -> Self {
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
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Validate Detector API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_vec_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
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
    pub fn close_job(&self, parts: MlCloseJobUrlParts) -> MlCloseJob<()> {
        MlCloseJob::new(self.client.clone(), parts)
    }
    pub fn delete_calendar(&self, parts: MlDeleteCalendarUrlParts) -> MlDeleteCalendar {
        MlDeleteCalendar::new(self.client.clone(), parts)
    }
    pub fn delete_calendar_event(
        &self,
        parts: MlDeleteCalendarEventUrlParts,
    ) -> MlDeleteCalendarEvent {
        MlDeleteCalendarEvent::new(self.client.clone(), parts)
    }
    pub fn delete_calendar_job(&self, parts: MlDeleteCalendarJobUrlParts) -> MlDeleteCalendarJob {
        MlDeleteCalendarJob::new(self.client.clone(), parts)
    }
    pub fn delete_data_frame_analytics(
        &self,
        parts: MlDeleteDataFrameAnalyticsUrlParts,
    ) -> MlDeleteDataFrameAnalytics {
        MlDeleteDataFrameAnalytics::new(self.client.clone(), parts)
    }
    pub fn delete_datafeed(&self, parts: MlDeleteDatafeedUrlParts) -> MlDeleteDatafeed {
        MlDeleteDatafeed::new(self.client.clone(), parts)
    }
    pub fn delete_expired_data(&self) -> MlDeleteExpiredData {
        MlDeleteExpiredData::new(self.client.clone())
    }
    pub fn delete_filter(&self, parts: MlDeleteFilterUrlParts) -> MlDeleteFilter {
        MlDeleteFilter::new(self.client.clone(), parts)
    }
    pub fn delete_forecast(&self, parts: MlDeleteForecastUrlParts) -> MlDeleteForecast {
        MlDeleteForecast::new(self.client.clone(), parts)
    }
    pub fn delete_job(&self, parts: MlDeleteJobUrlParts) -> MlDeleteJob {
        MlDeleteJob::new(self.client.clone(), parts)
    }
    pub fn delete_model_snapshot(
        &self,
        parts: MlDeleteModelSnapshotUrlParts,
    ) -> MlDeleteModelSnapshot {
        MlDeleteModelSnapshot::new(self.client.clone(), parts)
    }
    pub fn estimate_memory_usage(&self) -> MlEstimateMemoryUsage<()> {
        MlEstimateMemoryUsage::new(self.client.clone())
    }
    pub fn evaluate_data_frame(&self) -> MlEvaluateDataFrame<()> {
        MlEvaluateDataFrame::new(self.client.clone())
    }
    pub fn find_file_structure(&self) -> MlFindFileStructure<()> {
        MlFindFileStructure::new(self.client.clone())
    }
    pub fn flush_job(&self, parts: MlFlushJobUrlParts) -> MlFlushJob<()> {
        MlFlushJob::new(self.client.clone(), parts)
    }
    pub fn forecast(&self, parts: MlForecastUrlParts) -> MlForecast<()> {
        MlForecast::new(self.client.clone(), parts)
    }
    pub fn get_buckets(&self, parts: MlGetBucketsUrlParts) -> MlGetBuckets<()> {
        MlGetBuckets::new(self.client.clone(), parts)
    }
    pub fn get_calendar_events(&self, parts: MlGetCalendarEventsUrlParts) -> MlGetCalendarEvents {
        MlGetCalendarEvents::new(self.client.clone(), parts)
    }
    pub fn get_calendars(&self, parts: MlGetCalendarsUrlParts) -> MlGetCalendars<()> {
        MlGetCalendars::new(self.client.clone(), parts)
    }
    pub fn get_categories(&self, parts: MlGetCategoriesUrlParts) -> MlGetCategories<()> {
        MlGetCategories::new(self.client.clone(), parts)
    }
    pub fn get_data_frame_analytics(
        &self,
        parts: MlGetDataFrameAnalyticsUrlParts,
    ) -> MlGetDataFrameAnalytics {
        MlGetDataFrameAnalytics::new(self.client.clone(), parts)
    }
    pub fn get_data_frame_analytics_stats(
        &self,
        parts: MlGetDataFrameAnalyticsStatsUrlParts,
    ) -> MlGetDataFrameAnalyticsStats {
        MlGetDataFrameAnalyticsStats::new(self.client.clone(), parts)
    }
    pub fn get_datafeed_stats(&self, parts: MlGetDatafeedStatsUrlParts) -> MlGetDatafeedStats {
        MlGetDatafeedStats::new(self.client.clone(), parts)
    }
    pub fn get_datafeeds(&self, parts: MlGetDatafeedsUrlParts) -> MlGetDatafeeds {
        MlGetDatafeeds::new(self.client.clone(), parts)
    }
    pub fn get_filters(&self, parts: MlGetFiltersUrlParts) -> MlGetFilters {
        MlGetFilters::new(self.client.clone(), parts)
    }
    pub fn get_influencers(&self, parts: MlGetInfluencersUrlParts) -> MlGetInfluencers<()> {
        MlGetInfluencers::new(self.client.clone(), parts)
    }
    pub fn get_job_stats(&self, parts: MlGetJobStatsUrlParts) -> MlGetJobStats {
        MlGetJobStats::new(self.client.clone(), parts)
    }
    pub fn get_jobs(&self, parts: MlGetJobsUrlParts) -> MlGetJobs {
        MlGetJobs::new(self.client.clone(), parts)
    }
    pub fn get_model_snapshots(
        &self,
        parts: MlGetModelSnapshotsUrlParts,
    ) -> MlGetModelSnapshots<()> {
        MlGetModelSnapshots::new(self.client.clone(), parts)
    }
    pub fn get_overall_buckets(
        &self,
        parts: MlGetOverallBucketsUrlParts,
    ) -> MlGetOverallBuckets<()> {
        MlGetOverallBuckets::new(self.client.clone(), parts)
    }
    pub fn get_records(&self, parts: MlGetRecordsUrlParts) -> MlGetRecords<()> {
        MlGetRecords::new(self.client.clone(), parts)
    }
    pub fn info(&self) -> MlInfo {
        MlInfo::new(self.client.clone())
    }
    pub fn open_job(&self, parts: MlOpenJobUrlParts) -> MlOpenJob<()> {
        MlOpenJob::new(self.client.clone(), parts)
    }
    pub fn post_calendar_events(
        &self,
        parts: MlPostCalendarEventsUrlParts,
    ) -> MlPostCalendarEvents<()> {
        MlPostCalendarEvents::new(self.client.clone(), parts)
    }
    pub fn post_data(&self, parts: MlPostDataUrlParts) -> MlPostData<()> {
        MlPostData::new(self.client.clone(), parts)
    }
    pub fn preview_datafeed(&self, parts: MlPreviewDatafeedUrlParts) -> MlPreviewDatafeed {
        MlPreviewDatafeed::new(self.client.clone(), parts)
    }
    pub fn put_calendar(&self, parts: MlPutCalendarUrlParts) -> MlPutCalendar<()> {
        MlPutCalendar::new(self.client.clone(), parts)
    }
    pub fn put_calendar_job(&self, parts: MlPutCalendarJobUrlParts) -> MlPutCalendarJob<()> {
        MlPutCalendarJob::new(self.client.clone(), parts)
    }
    pub fn put_data_frame_analytics(
        &self,
        parts: MlPutDataFrameAnalyticsUrlParts,
    ) -> MlPutDataFrameAnalytics<()> {
        MlPutDataFrameAnalytics::new(self.client.clone(), parts)
    }
    pub fn put_datafeed(&self, parts: MlPutDatafeedUrlParts) -> MlPutDatafeed<()> {
        MlPutDatafeed::new(self.client.clone(), parts)
    }
    pub fn put_filter(&self, parts: MlPutFilterUrlParts) -> MlPutFilter<()> {
        MlPutFilter::new(self.client.clone(), parts)
    }
    pub fn put_job(&self, parts: MlPutJobUrlParts) -> MlPutJob<()> {
        MlPutJob::new(self.client.clone(), parts)
    }
    pub fn revert_model_snapshot(
        &self,
        parts: MlRevertModelSnapshotUrlParts,
    ) -> MlRevertModelSnapshot<()> {
        MlRevertModelSnapshot::new(self.client.clone(), parts)
    }
    pub fn set_upgrade_mode(&self) -> MlSetUpgradeMode<()> {
        MlSetUpgradeMode::new(self.client.clone())
    }
    pub fn start_data_frame_analytics(
        &self,
        parts: MlStartDataFrameAnalyticsUrlParts,
    ) -> MlStartDataFrameAnalytics<()> {
        MlStartDataFrameAnalytics::new(self.client.clone(), parts)
    }
    pub fn start_datafeed(&self, parts: MlStartDatafeedUrlParts) -> MlStartDatafeed<()> {
        MlStartDatafeed::new(self.client.clone(), parts)
    }
    pub fn stop_data_frame_analytics(
        &self,
        parts: MlStopDataFrameAnalyticsUrlParts,
    ) -> MlStopDataFrameAnalytics<()> {
        MlStopDataFrameAnalytics::new(self.client.clone(), parts)
    }
    pub fn stop_datafeed(&self, parts: MlStopDatafeedUrlParts) -> MlStopDatafeed<()> {
        MlStopDatafeed::new(self.client.clone(), parts)
    }
    pub fn update_datafeed(&self, parts: MlUpdateDatafeedUrlParts) -> MlUpdateDatafeed<()> {
        MlUpdateDatafeed::new(self.client.clone(), parts)
    }
    pub fn update_filter(&self, parts: MlUpdateFilterUrlParts) -> MlUpdateFilter<()> {
        MlUpdateFilter::new(self.client.clone(), parts)
    }
    pub fn update_job(&self, parts: MlUpdateJobUrlParts) -> MlUpdateJob<()> {
        MlUpdateJob::new(self.client.clone(), parts)
    }
    pub fn update_model_snapshot(
        &self,
        parts: MlUpdateModelSnapshotUrlParts,
    ) -> MlUpdateModelSnapshot<()> {
        MlUpdateModelSnapshot::new(self.client.clone(), parts)
    }
    pub fn validate(&self) -> MlValidate<()> {
        MlValidate::new(self.client.clone())
    }
    pub fn validate_detector(&self) -> MlValidateDetector<()> {
        MlValidateDetector::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Ml APIs"]
    pub fn ml(&self) -> Ml {
        Ml::new(self.clone())
    }
}