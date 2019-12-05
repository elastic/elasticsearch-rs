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
    client::Elasticsearch,
    enums::*,
    error::ElasticsearchError,
    http_method::HttpMethod,
    request::{Body, JsonBody, NdBody},
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Ml Close Job API"]
pub enum MlCloseJobUrlParts<'a> {
    JobId(&'a str),
}
impl<'a> MlCloseJobUrlParts<'a> {
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
pub struct MlCloseJob<'a, B> {
    client: Elasticsearch,
    parts: MlCloseJobUrlParts<'a>,
    allow_no_jobs: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> MlCloseJob<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlCloseJobUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlCloseJob<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlCloseJob {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait until a job has closed. Default to 30 minutes"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Close Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_jobs")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlDeleteCalendarUrlParts<'a> {
    CalendarId(&'a str),
}
impl<'a> MlDeleteCalendarUrlParts<'a> {
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
pub struct MlDeleteCalendar<'a> {
    client: Elasticsearch,
    parts: MlDeleteCalendarUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteCalendar<'a> {
    pub fn new(client: Elasticsearch, parts: MlDeleteCalendarUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Ml Delete Calendar API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlDeleteCalendarEventUrlParts<'a> {
    CalendarIdEventId(&'a str, &'a str),
}
impl<'a> MlDeleteCalendarEventUrlParts<'a> {
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
pub struct MlDeleteCalendarEvent<'a> {
    client: Elasticsearch,
    parts: MlDeleteCalendarEventUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteCalendarEvent<'a> {
    pub fn new(client: Elasticsearch, parts: MlDeleteCalendarEventUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Ml Delete Calendar Event API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlDeleteCalendarJobUrlParts<'a> {
    CalendarIdJobId(&'a str, &'a str),
}
impl<'a> MlDeleteCalendarJobUrlParts<'a> {
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
pub struct MlDeleteCalendarJob<'a> {
    client: Elasticsearch,
    parts: MlDeleteCalendarJobUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteCalendarJob<'a> {
    pub fn new(client: Elasticsearch, parts: MlDeleteCalendarJobUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Ml Delete Calendar Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlDeleteDataFrameAnalyticsUrlParts<'a> {
    Id(&'a str),
}
impl<'a> MlDeleteDataFrameAnalyticsUrlParts<'a> {
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
pub struct MlDeleteDataFrameAnalytics<'a> {
    client: Elasticsearch,
    parts: MlDeleteDataFrameAnalyticsUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteDataFrameAnalytics<'a> {
    pub fn new(client: Elasticsearch, parts: MlDeleteDataFrameAnalyticsUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Ml Delete Data Frame Analytics API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlDeleteDatafeedUrlParts<'a> {
    DatafeedId(&'a str),
}
impl<'a> MlDeleteDatafeedUrlParts<'a> {
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
pub struct MlDeleteDatafeed<'a> {
    client: Elasticsearch,
    parts: MlDeleteDatafeedUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteDatafeed<'a> {
    pub fn new(client: Elasticsearch, parts: MlDeleteDatafeedUrlParts<'a>) -> Self {
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Delete Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub struct MlDeleteExpiredData<'a> {
    client: Elasticsearch,
    parts: MlDeleteExpiredDataUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteExpiredData<'a> {
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
    #[doc = "Creates an asynchronous request to the Ml Delete Expired Data API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlDeleteFilterUrlParts<'a> {
    FilterId(&'a str),
}
impl<'a> MlDeleteFilterUrlParts<'a> {
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
pub struct MlDeleteFilter<'a> {
    client: Elasticsearch,
    parts: MlDeleteFilterUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteFilter<'a> {
    pub fn new(client: Elasticsearch, parts: MlDeleteFilterUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Ml Delete Filter API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlDeleteForecastUrlParts<'a> {
    JobId(&'a str),
    JobIdForecastId(&'a str, &'a str),
}
impl<'a> MlDeleteForecastUrlParts<'a> {
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
pub struct MlDeleteForecast<'a> {
    client: Elasticsearch,
    parts: MlDeleteForecastUrlParts<'a>,
    allow_no_forecasts: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> MlDeleteForecast<'a> {
    pub fn new(client: Elasticsearch, parts: MlDeleteForecastUrlParts<'a>) -> Self {
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
    #[doc = "Controls the time to wait until the forecast(s) are deleted. Default to 30 seconds"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Delete Forecast API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_forecasts")]
                allow_no_forecasts: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlDeleteJobUrlParts<'a> {
    JobId(&'a str),
}
impl<'a> MlDeleteJobUrlParts<'a> {
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
pub struct MlDeleteJob<'a> {
    client: Elasticsearch,
    parts: MlDeleteJobUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    wait_for_completion: Option<bool>,
}
impl<'a> MlDeleteJob<'a> {
    pub fn new(client: Elasticsearch, parts: MlDeleteJobUrlParts<'a>) -> Self {
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn source(mut self, source: &'a str) -> Self {
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
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
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
pub enum MlDeleteModelSnapshotUrlParts<'a> {
    JobIdSnapshotId(&'a str, &'a str),
}
impl<'a> MlDeleteModelSnapshotUrlParts<'a> {
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
pub struct MlDeleteModelSnapshot<'a> {
    client: Elasticsearch,
    parts: MlDeleteModelSnapshotUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteModelSnapshot<'a> {
    pub fn new(client: Elasticsearch, parts: MlDeleteModelSnapshotUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Ml Delete Model Snapshot API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Delete;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub struct MlEstimateMemoryUsage<'a, B> {
    client: Elasticsearch,
    parts: MlEstimateMemoryUsageUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlEstimateMemoryUsage<'a, B>
where
    B: Body,
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
    pub fn body<T>(self, body: T) -> MlEstimateMemoryUsage<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlEstimateMemoryUsage {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Estimate Memory Usage API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub struct MlEvaluateDataFrame<'a, B> {
    client: Elasticsearch,
    parts: MlEvaluateDataFrameUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlEvaluateDataFrame<'a, B>
where
    B: Body,
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
    pub fn body<T>(self, body: T) -> MlEvaluateDataFrame<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlEvaluateDataFrame {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Evaluate Data Frame API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub struct MlFindFileStructure<'a, B> {
    client: Elasticsearch,
    parts: MlFindFileStructureUrlParts,
    body: Option<B>,
    charset: Option<&'a str>,
    column_names: Option<&'a [&'a str]>,
    delimiter: Option<&'a str>,
    error_trace: Option<bool>,
    explain: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<Format>,
    grok_pattern: Option<&'a str>,
    has_header_row: Option<bool>,
    human: Option<bool>,
    line_merge_size_limit: Option<i32>,
    lines_to_sample: Option<i32>,
    pretty: Option<bool>,
    quote: Option<&'a str>,
    should_trim_fields: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    timestamp_field: Option<&'a str>,
    timestamp_format: Option<&'a str>,
}
impl<'a, B> MlFindFileStructure<'a, B>
where
    B: Body,
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
    pub fn body<T>(self, body: Vec<T>) -> MlFindFileStructure<'a, NdBody<T>>
    where
        T: Body,
    {
        MlFindFileStructure {
            client: self.client,
            parts: self.parts,
            body: Some(NdBody(body)),
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
    pub fn charset(mut self, charset: &'a str) -> Self {
        self.charset = Some(charset);
        self
    }
    #[doc = "Optional parameter containing a comma separated list of the column names for a delimited file"]
    pub fn column_names(mut self, column_names: &'a [&'a str]) -> Self {
        self.column_names = Some(column_names);
        self
    }
    #[doc = "Optional parameter to specify the delimiter character for a delimited file - must be a single character"]
    pub fn delimiter(mut self, delimiter: &'a str) -> Self {
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Optional parameter to specify the high level file format"]
    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Optional parameter to specify the Grok pattern that should be used to extract fields from messages in a semi-structured text file"]
    pub fn grok_pattern(mut self, grok_pattern: &'a str) -> Self {
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
    pub fn quote(mut self, quote: &'a str) -> Self {
        self.quote = Some(quote);
        self
    }
    #[doc = "Optional parameter to specify whether the values between delimiters in a delimited file should have whitespace trimmed from them"]
    pub fn should_trim_fields(mut self, should_trim_fields: bool) -> Self {
        self.should_trim_fields = Some(should_trim_fields);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Timeout after which the analysis will be aborted"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Optional parameter to specify the timestamp field in the file"]
    pub fn timestamp_field(mut self, timestamp_field: &'a str) -> Self {
        self.timestamp_field = Some(timestamp_field);
        self
    }
    #[doc = "Optional parameter to specify the timestamp format in the file - may be either a Joda or Java time format"]
    pub fn timestamp_format(mut self, timestamp_format: &'a str) -> Self {
        self.timestamp_format = Some(timestamp_format);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Find File Structure API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "charset")]
                charset: Option<&'a str>,
                #[serde(
                    rename = "column_names",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                column_names: Option<&'a [&'a str]>,
                #[serde(rename = "delimiter")]
                delimiter: Option<&'a str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "explain")]
                explain: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<Format>,
                #[serde(rename = "grok_pattern")]
                grok_pattern: Option<&'a str>,
                #[serde(rename = "has_header_row")]
                has_header_row: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "line_merge_size_limit")]
                line_merge_size_limit: Option<i32>,
                #[serde(rename = "lines_to_sample")]
                lines_to_sample: Option<i32>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "quote")]
                quote: Option<&'a str>,
                #[serde(rename = "should_trim_fields")]
                should_trim_fields: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "timestamp_field")]
                timestamp_field: Option<&'a str>,
                #[serde(rename = "timestamp_format")]
                timestamp_format: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlFlushJobUrlParts<'a> {
    JobId(&'a str),
}
impl<'a> MlFlushJobUrlParts<'a> {
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
pub struct MlFlushJob<'a, B> {
    client: Elasticsearch,
    parts: MlFlushJobUrlParts<'a>,
    advance_time: Option<&'a str>,
    body: Option<B>,
    calc_interim: Option<bool>,
    end: Option<&'a str>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    skip_time: Option<&'a str>,
    source: Option<&'a str>,
    start: Option<&'a str>,
}
impl<'a, B> MlFlushJob<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlFlushJobUrlParts<'a>) -> Self {
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
    pub fn advance_time(mut self, advance_time: &'a str) -> Self {
        self.advance_time = Some(advance_time);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> MlFlushJob<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlFlushJob {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    pub fn end(mut self, end: &'a str) -> Self {
        self.end = Some(end);
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
    #[doc = "Skips time to the given value without generating results or updating the model for the skipped interval"]
    pub fn skip_time(mut self, skip_time: &'a str) -> Self {
        self.skip_time = Some(skip_time);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "When used in conjunction with calc_interim, specifies the range of buckets on which to calculate interim results"]
    pub fn start(mut self, start: &'a str) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Flush Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "advance_time")]
                advance_time: Option<&'a str>,
                #[serde(rename = "calc_interim")]
                calc_interim: Option<bool>,
                #[serde(rename = "end")]
                end: Option<&'a str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "skip_time")]
                skip_time: Option<&'a str>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "start")]
                start: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlForecastUrlParts<'a> {
    JobId(&'a str),
}
impl<'a> MlForecastUrlParts<'a> {
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
pub struct MlForecast<'a, B> {
    client: Elasticsearch,
    parts: MlForecastUrlParts<'a>,
    body: Option<B>,
    duration: Option<&'a str>,
    error_trace: Option<bool>,
    expires_in: Option<&'a str>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlForecast<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlForecastUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlForecast<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlForecast {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    pub fn duration(mut self, duration: &'a str) -> Self {
        self.duration = Some(duration);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "The time interval after which the forecast expires. Expired forecasts will be deleted at the first opportunity."]
    pub fn expires_in(mut self, expires_in: &'a str) -> Self {
        self.expires_in = Some(expires_in);
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
    #[doc = "Creates an asynchronous request to the Ml Forecast API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "duration")]
                duration: Option<&'a str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "expires_in")]
                expires_in: Option<&'a str>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetBucketsUrlParts<'a> {
    JobIdTimestamp(&'a str, &'a str),
    JobId(&'a str),
}
impl<'a> MlGetBucketsUrlParts<'a> {
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
pub struct MlGetBuckets<'a, B> {
    client: Elasticsearch,
    parts: MlGetBucketsUrlParts<'a>,
    anomaly_score: Option<f64>,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<&'a str>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    expand: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    sort: Option<&'a str>,
    source: Option<&'a str>,
    start: Option<&'a str>,
}
impl<'a, B> MlGetBuckets<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlGetBucketsUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlGetBuckets<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetBuckets {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    pub fn end(mut self, end: &'a str) -> Self {
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn sort(mut self, sort: &'a str) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Start time filter for buckets"]
    pub fn start(mut self, start: &'a str) -> Self {
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
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "anomaly_score")]
                anomaly_score: Option<f64>,
                #[serde(rename = "desc")]
                desc: Option<bool>,
                #[serde(rename = "end")]
                end: Option<&'a str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "exclude_interim")]
                exclude_interim: Option<bool>,
                #[serde(rename = "expand")]
                expand: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "sort")]
                sort: Option<&'a str>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "start")]
                start: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetCalendarEventsUrlParts<'a> {
    CalendarId(&'a str),
}
impl<'a> MlGetCalendarEventsUrlParts<'a> {
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
pub struct MlGetCalendarEvents<'a> {
    client: Elasticsearch,
    parts: MlGetCalendarEventsUrlParts<'a>,
    end: Option<&'a str>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i32>,
    human: Option<bool>,
    job_id: Option<&'a str>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<&'a str>,
    start: Option<&'a str>,
}
impl<'a> MlGetCalendarEvents<'a> {
    pub fn new(client: Elasticsearch, parts: MlGetCalendarEventsUrlParts<'a>) -> Self {
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
    pub fn end(mut self, end: &'a str) -> Self {
        self.end = Some(end);
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
    pub fn job_id(mut self, job_id: &'a str) -> Self {
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Get events after this time"]
    pub fn start(mut self, start: &'a str) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Calendar Events API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "end")]
                end: Option<&'a str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "job_id")]
                job_id: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "start")]
                start: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetCalendarsUrlParts<'a> {
    None,
    CalendarId(&'a str),
}
impl<'a> MlGetCalendarsUrlParts<'a> {
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
pub struct MlGetCalendars<'a, B> {
    client: Elasticsearch,
    parts: MlGetCalendarsUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<&'a str>,
}
impl<'a, B> MlGetCalendars<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlGetCalendarsUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlGetCalendars<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetCalendars {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn source(mut self, source: &'a str) -> Self {
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
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetCategoriesUrlParts<'a> {
    JobIdCategoryId(&'a str, i64),
    JobId(&'a str),
}
impl<'a> MlGetCategoriesUrlParts<'a> {
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
pub struct MlGetCategories<'a, B> {
    client: Elasticsearch,
    parts: MlGetCategoriesUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<&'a str>,
}
impl<'a, B> MlGetCategories<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlGetCategoriesUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlGetCategories<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetCategories {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn source(mut self, source: &'a str) -> Self {
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
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetDataFrameAnalyticsUrlParts<'a> {
    Id(&'a str),
    None,
}
impl<'a> MlGetDataFrameAnalyticsUrlParts<'a> {
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
pub struct MlGetDataFrameAnalytics<'a> {
    client: Elasticsearch,
    parts: MlGetDataFrameAnalyticsUrlParts<'a>,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<&'a str>,
}
impl<'a> MlGetDataFrameAnalytics<'a> {
    pub fn new(client: Elasticsearch, parts: MlGetDataFrameAnalyticsUrlParts<'a>) -> Self {
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Data Frame Analytics API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetDataFrameAnalyticsStatsUrlParts<'a> {
    None,
    Id(&'a str),
}
impl<'a> MlGetDataFrameAnalyticsStatsUrlParts<'a> {
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
pub struct MlGetDataFrameAnalyticsStats<'a> {
    client: Elasticsearch,
    parts: MlGetDataFrameAnalyticsStatsUrlParts<'a>,
    allow_no_match: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<&'a str>,
}
impl<'a> MlGetDataFrameAnalyticsStats<'a> {
    pub fn new(client: Elasticsearch, parts: MlGetDataFrameAnalyticsStatsUrlParts<'a>) -> Self {
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Data Frame Analytics Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetDatafeedStatsUrlParts<'a> {
    DatafeedId(&'a str),
    None,
}
impl<'a> MlGetDatafeedStatsUrlParts<'a> {
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
pub struct MlGetDatafeedStats<'a> {
    client: Elasticsearch,
    parts: MlGetDatafeedStatsUrlParts<'a>,
    allow_no_datafeeds: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlGetDatafeedStats<'a> {
    pub fn new(client: Elasticsearch, parts: MlGetDatafeedStatsUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Ml Get Datafeed Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_datafeeds")]
                allow_no_datafeeds: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetDatafeedsUrlParts<'a> {
    DatafeedId(&'a str),
    None,
}
impl<'a> MlGetDatafeedsUrlParts<'a> {
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
pub struct MlGetDatafeeds<'a> {
    client: Elasticsearch,
    parts: MlGetDatafeedsUrlParts<'a>,
    allow_no_datafeeds: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlGetDatafeeds<'a> {
    pub fn new(client: Elasticsearch, parts: MlGetDatafeedsUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Ml Get Datafeeds API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_datafeeds")]
                allow_no_datafeeds: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetFiltersUrlParts<'a> {
    None,
    FilterId(&'a str),
}
impl<'a> MlGetFiltersUrlParts<'a> {
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
pub struct MlGetFilters<'a> {
    client: Elasticsearch,
    parts: MlGetFiltersUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<&'a str>,
}
impl<'a> MlGetFilters<'a> {
    pub fn new(client: Elasticsearch, parts: MlGetFiltersUrlParts<'a>) -> Self {
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Get Filters API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetInfluencersUrlParts<'a> {
    JobId(&'a str),
}
impl<'a> MlGetInfluencersUrlParts<'a> {
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
pub struct MlGetInfluencers<'a, B> {
    client: Elasticsearch,
    parts: MlGetInfluencersUrlParts<'a>,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<&'a str>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i32>,
    human: Option<bool>,
    influencer_score: Option<f64>,
    pretty: Option<bool>,
    size: Option<i32>,
    sort: Option<&'a str>,
    source: Option<&'a str>,
    start: Option<&'a str>,
}
impl<'a, B> MlGetInfluencers<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlGetInfluencersUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlGetInfluencers<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetInfluencers {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    pub fn end(mut self, end: &'a str) -> Self {
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn sort(mut self, sort: &'a str) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "start timestamp for the requested influencers"]
    pub fn start(mut self, start: &'a str) -> Self {
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
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "desc")]
                desc: Option<bool>,
                #[serde(rename = "end")]
                end: Option<&'a str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "exclude_interim")]
                exclude_interim: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "influencer_score")]
                influencer_score: Option<f64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "sort")]
                sort: Option<&'a str>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "start")]
                start: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetJobStatsUrlParts<'a> {
    None,
    JobId(&'a str),
}
impl<'a> MlGetJobStatsUrlParts<'a> {
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
pub struct MlGetJobStats<'a> {
    client: Elasticsearch,
    parts: MlGetJobStatsUrlParts<'a>,
    allow_no_jobs: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlGetJobStats<'a> {
    pub fn new(client: Elasticsearch, parts: MlGetJobStatsUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Ml Get Job Stats API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_jobs")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetJobsUrlParts<'a> {
    JobId(&'a str),
    None,
}
impl<'a> MlGetJobsUrlParts<'a> {
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
pub struct MlGetJobs<'a> {
    client: Elasticsearch,
    parts: MlGetJobsUrlParts<'a>,
    allow_no_jobs: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlGetJobs<'a> {
    pub fn new(client: Elasticsearch, parts: MlGetJobsUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Ml Get Jobs API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_jobs")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetModelSnapshotsUrlParts<'a> {
    JobIdSnapshotId(&'a str, &'a str),
    JobId(&'a str),
}
impl<'a> MlGetModelSnapshotsUrlParts<'a> {
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
pub struct MlGetModelSnapshots<'a, B> {
    client: Elasticsearch,
    parts: MlGetModelSnapshotsUrlParts<'a>,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<&'a str>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    sort: Option<&'a str>,
    source: Option<&'a str>,
    start: Option<&'a str>,
}
impl<'a, B> MlGetModelSnapshots<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlGetModelSnapshotsUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlGetModelSnapshots<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetModelSnapshots {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    pub fn end(mut self, end: &'a str) -> Self {
        self.end = Some(end);
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
    pub fn sort(mut self, sort: &'a str) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The filter 'start' query parameter"]
    pub fn start(mut self, start: &'a str) -> Self {
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
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "desc")]
                desc: Option<bool>,
                #[serde(rename = "end")]
                end: Option<&'a str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "sort")]
                sort: Option<&'a str>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "start")]
                start: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlGetOverallBucketsUrlParts<'a> {
    JobId(&'a str),
}
impl<'a> MlGetOverallBucketsUrlParts<'a> {
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
pub struct MlGetOverallBuckets<'a, B> {
    client: Elasticsearch,
    parts: MlGetOverallBucketsUrlParts<'a>,
    allow_no_jobs: Option<bool>,
    body: Option<B>,
    bucket_span: Option<&'a str>,
    end: Option<&'a str>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    overall_score: Option<f64>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    start: Option<&'a str>,
    top_n: Option<i32>,
}
impl<'a, B> MlGetOverallBuckets<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlGetOverallBucketsUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlGetOverallBuckets<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetOverallBuckets {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    pub fn bucket_span(mut self, bucket_span: &'a str) -> Self {
        self.bucket_span = Some(bucket_span);
        self
    }
    #[doc = "Returns overall buckets with timestamps earlier than this time"]
    pub fn end(mut self, end: &'a str) -> Self {
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Returns overall buckets with timestamps after this time"]
    pub fn start(mut self, start: &'a str) -> Self {
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
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_jobs")]
                allow_no_jobs: Option<bool>,
                #[serde(rename = "bucket_span")]
                bucket_span: Option<&'a str>,
                #[serde(rename = "end")]
                end: Option<&'a str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "exclude_interim")]
                exclude_interim: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "overall_score")]
                overall_score: Option<f64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "start")]
                start: Option<&'a str>,
                #[serde(rename = "top_n")]
                top_n: Option<i32>,
            }
            let query_params = QueryParams {
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
pub enum MlGetRecordsUrlParts<'a> {
    JobId(&'a str),
}
impl<'a> MlGetRecordsUrlParts<'a> {
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
pub struct MlGetRecords<'a, B> {
    client: Elasticsearch,
    parts: MlGetRecordsUrlParts<'a>,
    body: Option<B>,
    desc: Option<bool>,
    end: Option<&'a str>,
    error_trace: Option<bool>,
    exclude_interim: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    record_score: Option<f64>,
    size: Option<i32>,
    sort: Option<&'a str>,
    source: Option<&'a str>,
    start: Option<&'a str>,
}
impl<'a, B> MlGetRecords<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlGetRecordsUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlGetRecords<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetRecords {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    pub fn end(mut self, end: &'a str) -> Self {
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn sort(mut self, sort: &'a str) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Start time filter for records"]
    pub fn start(mut self, start: &'a str) -> Self {
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
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "desc")]
                desc: Option<bool>,
                #[serde(rename = "end")]
                end: Option<&'a str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "exclude_interim")]
                exclude_interim: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "from")]
                from: Option<i32>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "record_score")]
                record_score: Option<f64>,
                #[serde(rename = "size")]
                size: Option<i32>,
                #[serde(rename = "sort")]
                sort: Option<&'a str>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "start")]
                start: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub struct MlInfo<'a> {
    client: Elasticsearch,
    parts: MlInfoUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlInfo<'a> {
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
    #[doc = "Creates an asynchronous request to the Ml Info API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlOpenJobUrlParts<'a> {
    JobId(&'a str),
}
impl<'a> MlOpenJobUrlParts<'a> {
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
pub struct MlOpenJob<'a, B> {
    client: Elasticsearch,
    parts: MlOpenJobUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlOpenJob<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlOpenJobUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlOpenJob<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlOpenJob {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Open Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlPostCalendarEventsUrlParts<'a> {
    CalendarId(&'a str),
}
impl<'a> MlPostCalendarEventsUrlParts<'a> {
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
pub struct MlPostCalendarEvents<'a, B> {
    client: Elasticsearch,
    parts: MlPostCalendarEventsUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlPostCalendarEvents<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlPostCalendarEventsUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlPostCalendarEvents<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPostCalendarEvents {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Post Calendar Events API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlPostDataUrlParts<'a> {
    JobId(&'a str),
}
impl<'a> MlPostDataUrlParts<'a> {
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
pub struct MlPostData<'a, B> {
    client: Elasticsearch,
    parts: MlPostDataUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    reset_end: Option<&'a str>,
    reset_start: Option<&'a str>,
    source: Option<&'a str>,
}
impl<'a, B> MlPostData<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlPostDataUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: Vec<T>) -> MlPostData<'a, NdBody<T>>
    where
        T: Body,
    {
        MlPostData {
            client: self.client,
            parts: self.parts,
            body: Some(NdBody(body)),
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
    #[doc = "Optional parameter to specify the end of the bucket resetting range"]
    pub fn reset_end(mut self, reset_end: &'a str) -> Self {
        self.reset_end = Some(reset_end);
        self
    }
    #[doc = "Optional parameter to specify the start of the bucket resetting range"]
    pub fn reset_start(mut self, reset_start: &'a str) -> Self {
        self.reset_start = Some(reset_start);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Post Data API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "reset_end")]
                reset_end: Option<&'a str>,
                #[serde(rename = "reset_start")]
                reset_start: Option<&'a str>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlPreviewDatafeedUrlParts<'a> {
    DatafeedId(&'a str),
}
impl<'a> MlPreviewDatafeedUrlParts<'a> {
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
pub struct MlPreviewDatafeed<'a> {
    client: Elasticsearch,
    parts: MlPreviewDatafeedUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlPreviewDatafeed<'a> {
    pub fn new(client: Elasticsearch, parts: MlPreviewDatafeedUrlParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous request to the Ml Preview Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlPutCalendarUrlParts<'a> {
    CalendarId(&'a str),
}
impl<'a> MlPutCalendarUrlParts<'a> {
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
pub struct MlPutCalendar<'a, B> {
    client: Elasticsearch,
    parts: MlPutCalendarUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlPutCalendar<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlPutCalendarUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlPutCalendar<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPutCalendar {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Put Calendar API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlPutCalendarJobUrlParts<'a> {
    CalendarIdJobId(&'a str, &'a str),
}
impl<'a> MlPutCalendarJobUrlParts<'a> {
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
pub struct MlPutCalendarJob<'a, B> {
    client: Elasticsearch,
    parts: MlPutCalendarJobUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlPutCalendarJob<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlPutCalendarJobUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlPutCalendarJob<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPutCalendarJob {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Put Calendar Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlPutDataFrameAnalyticsUrlParts<'a> {
    Id(&'a str),
}
impl<'a> MlPutDataFrameAnalyticsUrlParts<'a> {
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
pub struct MlPutDataFrameAnalytics<'a, B> {
    client: Elasticsearch,
    parts: MlPutDataFrameAnalyticsUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlPutDataFrameAnalytics<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlPutDataFrameAnalyticsUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlPutDataFrameAnalytics<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPutDataFrameAnalytics {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Put Data Frame Analytics API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlPutDatafeedUrlParts<'a> {
    DatafeedId(&'a str),
}
impl<'a> MlPutDatafeedUrlParts<'a> {
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
pub struct MlPutDatafeed<'a, B> {
    client: Elasticsearch,
    parts: MlPutDatafeedUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlPutDatafeed<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlPutDatafeedUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlPutDatafeed<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPutDatafeed {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Put Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlPutFilterUrlParts<'a> {
    FilterId(&'a str),
}
impl<'a> MlPutFilterUrlParts<'a> {
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
pub struct MlPutFilter<'a, B> {
    client: Elasticsearch,
    parts: MlPutFilterUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlPutFilter<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlPutFilterUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlPutFilter<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPutFilter {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Put Filter API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlPutJobUrlParts<'a> {
    JobId(&'a str),
}
impl<'a> MlPutJobUrlParts<'a> {
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
pub struct MlPutJob<'a, B> {
    client: Elasticsearch,
    parts: MlPutJobUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlPutJob<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlPutJobUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlPutJob<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPutJob {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Put Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Put;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlRevertModelSnapshotUrlParts<'a> {
    JobIdSnapshotId(&'a str, &'a str),
}
impl<'a> MlRevertModelSnapshotUrlParts<'a> {
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
pub struct MlRevertModelSnapshot<'a, B> {
    client: Elasticsearch,
    parts: MlRevertModelSnapshotUrlParts<'a>,
    body: Option<B>,
    delete_intervening_results: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlRevertModelSnapshot<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlRevertModelSnapshotUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlRevertModelSnapshot<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlRevertModelSnapshot {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Revert Model Snapshot API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "delete_intervening_results")]
                delete_intervening_results: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub struct MlSetUpgradeMode<'a, B> {
    client: Elasticsearch,
    parts: MlSetUpgradeModeUrlParts,
    body: Option<B>,
    enabled: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> MlSetUpgradeMode<'a, B>
where
    B: Body,
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
    pub fn body<T>(self, body: T) -> MlSetUpgradeMode<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlSetUpgradeMode {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Controls the time to wait before action times out. Defaults to 30 seconds"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Set Upgrade Mode API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "enabled")]
                enabled: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlStartDataFrameAnalyticsUrlParts<'a> {
    Id(&'a str),
}
impl<'a> MlStartDataFrameAnalyticsUrlParts<'a> {
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
pub struct MlStartDataFrameAnalytics<'a, B> {
    client: Elasticsearch,
    parts: MlStartDataFrameAnalyticsUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> MlStartDataFrameAnalytics<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlStartDataFrameAnalyticsUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlStartDataFrameAnalytics<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlStartDataFrameAnalytics {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Controls the time to wait until the task has started. Defaults to 20 seconds"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Start Data Frame Analytics API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlStartDatafeedUrlParts<'a> {
    DatafeedId(&'a str),
}
impl<'a> MlStartDatafeedUrlParts<'a> {
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
pub struct MlStartDatafeed<'a, B> {
    client: Elasticsearch,
    parts: MlStartDatafeedUrlParts<'a>,
    body: Option<B>,
    end: Option<&'a str>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    start: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> MlStartDatafeed<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlStartDatafeedUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlStartDatafeed<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlStartDatafeed {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    pub fn end(mut self, end: &'a str) -> Self {
        self.end = Some(end);
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
    #[doc = "The start time from where the datafeed should begin"]
    pub fn start(mut self, start: &'a str) -> Self {
        self.start = Some(start);
        self
    }
    #[doc = "Controls the time to wait until a datafeed has started. Default to 20 seconds"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Start Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "end")]
                end: Option<&'a str>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "start")]
                start: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlStopDataFrameAnalyticsUrlParts<'a> {
    Id(&'a str),
}
impl<'a> MlStopDataFrameAnalyticsUrlParts<'a> {
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
pub struct MlStopDataFrameAnalytics<'a, B> {
    client: Elasticsearch,
    parts: MlStopDataFrameAnalyticsUrlParts<'a>,
    allow_no_match: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> MlStopDataFrameAnalytics<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlStopDataFrameAnalyticsUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlStopDataFrameAnalytics<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlStopDataFrameAnalytics {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait until the task has stopped. Defaults to 20 seconds"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Stop Data Frame Analytics API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_match")]
                allow_no_match: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlStopDatafeedUrlParts<'a> {
    DatafeedId(&'a str),
}
impl<'a> MlStopDatafeedUrlParts<'a> {
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
pub struct MlStopDatafeed<'a, B> {
    client: Elasticsearch,
    parts: MlStopDatafeedUrlParts<'a>,
    allow_no_datafeeds: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> MlStopDatafeed<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlStopDatafeedUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlStopDatafeed<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlStopDatafeed {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Controls the time to wait until a datafeed has stopped. Default to 20 seconds"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous request to the Ml Stop Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "allow_no_datafeeds")]
                allow_no_datafeeds: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "force")]
                force: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlUpdateDatafeedUrlParts<'a> {
    DatafeedId(&'a str),
}
impl<'a> MlUpdateDatafeedUrlParts<'a> {
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
pub struct MlUpdateDatafeed<'a, B> {
    client: Elasticsearch,
    parts: MlUpdateDatafeedUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlUpdateDatafeed<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlUpdateDatafeedUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlUpdateDatafeed<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateDatafeed {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Update Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlUpdateFilterUrlParts<'a> {
    FilterId(&'a str),
}
impl<'a> MlUpdateFilterUrlParts<'a> {
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
pub struct MlUpdateFilter<'a, B> {
    client: Elasticsearch,
    parts: MlUpdateFilterUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlUpdateFilter<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlUpdateFilterUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlUpdateFilter<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateFilter {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Update Filter API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlUpdateJobUrlParts<'a> {
    JobId(&'a str),
}
impl<'a> MlUpdateJobUrlParts<'a> {
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
pub struct MlUpdateJob<'a, B> {
    client: Elasticsearch,
    parts: MlUpdateJobUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlUpdateJob<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlUpdateJobUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlUpdateJob<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateJob {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Update Job API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub enum MlUpdateModelSnapshotUrlParts<'a> {
    JobIdSnapshotId(&'a str, &'a str),
}
impl<'a> MlUpdateModelSnapshotUrlParts<'a> {
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
pub struct MlUpdateModelSnapshot<'a, B> {
    client: Elasticsearch,
    parts: MlUpdateModelSnapshotUrlParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlUpdateModelSnapshot<'a, B>
where
    B: Body,
{
    pub fn new(client: Elasticsearch, parts: MlUpdateModelSnapshotUrlParts<'a>) -> Self {
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
    pub fn body<T>(self, body: T) -> MlUpdateModelSnapshot<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateModelSnapshot {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Update Model Snapshot API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub struct MlValidate<'a, B> {
    client: Elasticsearch,
    parts: MlValidateUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlValidate<'a, B>
where
    B: Body,
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
    pub fn body<T>(self, body: T) -> MlValidate<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlValidate {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Validate API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
pub struct MlValidateDetector<'a, B> {
    client: Elasticsearch,
    parts: MlValidateDetectorUrlParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> MlValidateDetector<'a, B>
where
    B: Body,
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
    pub fn body<T>(self, body: T) -> MlValidateDetector<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        MlValidateDetector {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
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
    #[doc = "Creates an asynchronous request to the Ml Validate Detector API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
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
    pub fn close_job<'a>(&self, parts: MlCloseJobUrlParts<'a>) -> MlCloseJob<'a, ()> {
        MlCloseJob::new(self.client.clone(), parts)
    }
    pub fn delete_calendar<'a>(&self, parts: MlDeleteCalendarUrlParts<'a>) -> MlDeleteCalendar<'a> {
        MlDeleteCalendar::new(self.client.clone(), parts)
    }
    pub fn delete_calendar_event<'a>(
        &self,
        parts: MlDeleteCalendarEventUrlParts<'a>,
    ) -> MlDeleteCalendarEvent<'a> {
        MlDeleteCalendarEvent::new(self.client.clone(), parts)
    }
    pub fn delete_calendar_job<'a>(
        &self,
        parts: MlDeleteCalendarJobUrlParts<'a>,
    ) -> MlDeleteCalendarJob<'a> {
        MlDeleteCalendarJob::new(self.client.clone(), parts)
    }
    pub fn delete_data_frame_analytics<'a>(
        &self,
        parts: MlDeleteDataFrameAnalyticsUrlParts<'a>,
    ) -> MlDeleteDataFrameAnalytics<'a> {
        MlDeleteDataFrameAnalytics::new(self.client.clone(), parts)
    }
    pub fn delete_datafeed<'a>(&self, parts: MlDeleteDatafeedUrlParts<'a>) -> MlDeleteDatafeed<'a> {
        MlDeleteDatafeed::new(self.client.clone(), parts)
    }
    pub fn delete_expired_data<'a>(&self) -> MlDeleteExpiredData<'a> {
        MlDeleteExpiredData::new(self.client.clone())
    }
    pub fn delete_filter<'a>(&self, parts: MlDeleteFilterUrlParts<'a>) -> MlDeleteFilter<'a> {
        MlDeleteFilter::new(self.client.clone(), parts)
    }
    pub fn delete_forecast<'a>(&self, parts: MlDeleteForecastUrlParts<'a>) -> MlDeleteForecast<'a> {
        MlDeleteForecast::new(self.client.clone(), parts)
    }
    pub fn delete_job<'a>(&self, parts: MlDeleteJobUrlParts<'a>) -> MlDeleteJob<'a> {
        MlDeleteJob::new(self.client.clone(), parts)
    }
    pub fn delete_model_snapshot<'a>(
        &self,
        parts: MlDeleteModelSnapshotUrlParts<'a>,
    ) -> MlDeleteModelSnapshot<'a> {
        MlDeleteModelSnapshot::new(self.client.clone(), parts)
    }
    pub fn estimate_memory_usage<'a>(&self) -> MlEstimateMemoryUsage<'a, ()> {
        MlEstimateMemoryUsage::new(self.client.clone())
    }
    pub fn evaluate_data_frame<'a>(&self) -> MlEvaluateDataFrame<'a, ()> {
        MlEvaluateDataFrame::new(self.client.clone())
    }
    pub fn find_file_structure<'a>(&self) -> MlFindFileStructure<'a, ()> {
        MlFindFileStructure::new(self.client.clone())
    }
    pub fn flush_job<'a>(&self, parts: MlFlushJobUrlParts<'a>) -> MlFlushJob<'a, ()> {
        MlFlushJob::new(self.client.clone(), parts)
    }
    pub fn forecast<'a>(&self, parts: MlForecastUrlParts<'a>) -> MlForecast<'a, ()> {
        MlForecast::new(self.client.clone(), parts)
    }
    pub fn get_buckets<'a>(&self, parts: MlGetBucketsUrlParts<'a>) -> MlGetBuckets<'a, ()> {
        MlGetBuckets::new(self.client.clone(), parts)
    }
    pub fn get_calendar_events<'a>(
        &self,
        parts: MlGetCalendarEventsUrlParts<'a>,
    ) -> MlGetCalendarEvents<'a> {
        MlGetCalendarEvents::new(self.client.clone(), parts)
    }
    pub fn get_calendars<'a>(&self, parts: MlGetCalendarsUrlParts<'a>) -> MlGetCalendars<'a, ()> {
        MlGetCalendars::new(self.client.clone(), parts)
    }
    pub fn get_categories<'a>(
        &self,
        parts: MlGetCategoriesUrlParts<'a>,
    ) -> MlGetCategories<'a, ()> {
        MlGetCategories::new(self.client.clone(), parts)
    }
    pub fn get_data_frame_analytics<'a>(
        &self,
        parts: MlGetDataFrameAnalyticsUrlParts<'a>,
    ) -> MlGetDataFrameAnalytics<'a> {
        MlGetDataFrameAnalytics::new(self.client.clone(), parts)
    }
    pub fn get_data_frame_analytics_stats<'a>(
        &self,
        parts: MlGetDataFrameAnalyticsStatsUrlParts<'a>,
    ) -> MlGetDataFrameAnalyticsStats<'a> {
        MlGetDataFrameAnalyticsStats::new(self.client.clone(), parts)
    }
    pub fn get_datafeed_stats<'a>(
        &self,
        parts: MlGetDatafeedStatsUrlParts<'a>,
    ) -> MlGetDatafeedStats<'a> {
        MlGetDatafeedStats::new(self.client.clone(), parts)
    }
    pub fn get_datafeeds<'a>(&self, parts: MlGetDatafeedsUrlParts<'a>) -> MlGetDatafeeds<'a> {
        MlGetDatafeeds::new(self.client.clone(), parts)
    }
    pub fn get_filters<'a>(&self, parts: MlGetFiltersUrlParts<'a>) -> MlGetFilters<'a> {
        MlGetFilters::new(self.client.clone(), parts)
    }
    pub fn get_influencers<'a>(
        &self,
        parts: MlGetInfluencersUrlParts<'a>,
    ) -> MlGetInfluencers<'a, ()> {
        MlGetInfluencers::new(self.client.clone(), parts)
    }
    pub fn get_job_stats<'a>(&self, parts: MlGetJobStatsUrlParts<'a>) -> MlGetJobStats<'a> {
        MlGetJobStats::new(self.client.clone(), parts)
    }
    pub fn get_jobs<'a>(&self, parts: MlGetJobsUrlParts<'a>) -> MlGetJobs<'a> {
        MlGetJobs::new(self.client.clone(), parts)
    }
    pub fn get_model_snapshots<'a>(
        &self,
        parts: MlGetModelSnapshotsUrlParts<'a>,
    ) -> MlGetModelSnapshots<'a, ()> {
        MlGetModelSnapshots::new(self.client.clone(), parts)
    }
    pub fn get_overall_buckets<'a>(
        &self,
        parts: MlGetOverallBucketsUrlParts<'a>,
    ) -> MlGetOverallBuckets<'a, ()> {
        MlGetOverallBuckets::new(self.client.clone(), parts)
    }
    pub fn get_records<'a>(&self, parts: MlGetRecordsUrlParts<'a>) -> MlGetRecords<'a, ()> {
        MlGetRecords::new(self.client.clone(), parts)
    }
    pub fn info<'a>(&self) -> MlInfo<'a> {
        MlInfo::new(self.client.clone())
    }
    pub fn open_job<'a>(&self, parts: MlOpenJobUrlParts<'a>) -> MlOpenJob<'a, ()> {
        MlOpenJob::new(self.client.clone(), parts)
    }
    pub fn post_calendar_events<'a>(
        &self,
        parts: MlPostCalendarEventsUrlParts<'a>,
    ) -> MlPostCalendarEvents<'a, ()> {
        MlPostCalendarEvents::new(self.client.clone(), parts)
    }
    pub fn post_data<'a>(&self, parts: MlPostDataUrlParts<'a>) -> MlPostData<'a, ()> {
        MlPostData::new(self.client.clone(), parts)
    }
    pub fn preview_datafeed<'a>(
        &self,
        parts: MlPreviewDatafeedUrlParts<'a>,
    ) -> MlPreviewDatafeed<'a> {
        MlPreviewDatafeed::new(self.client.clone(), parts)
    }
    pub fn put_calendar<'a>(&self, parts: MlPutCalendarUrlParts<'a>) -> MlPutCalendar<'a, ()> {
        MlPutCalendar::new(self.client.clone(), parts)
    }
    pub fn put_calendar_job<'a>(
        &self,
        parts: MlPutCalendarJobUrlParts<'a>,
    ) -> MlPutCalendarJob<'a, ()> {
        MlPutCalendarJob::new(self.client.clone(), parts)
    }
    pub fn put_data_frame_analytics<'a>(
        &self,
        parts: MlPutDataFrameAnalyticsUrlParts<'a>,
    ) -> MlPutDataFrameAnalytics<'a, ()> {
        MlPutDataFrameAnalytics::new(self.client.clone(), parts)
    }
    pub fn put_datafeed<'a>(&self, parts: MlPutDatafeedUrlParts<'a>) -> MlPutDatafeed<'a, ()> {
        MlPutDatafeed::new(self.client.clone(), parts)
    }
    pub fn put_filter<'a>(&self, parts: MlPutFilterUrlParts<'a>) -> MlPutFilter<'a, ()> {
        MlPutFilter::new(self.client.clone(), parts)
    }
    pub fn put_job<'a>(&self, parts: MlPutJobUrlParts<'a>) -> MlPutJob<'a, ()> {
        MlPutJob::new(self.client.clone(), parts)
    }
    pub fn revert_model_snapshot<'a>(
        &self,
        parts: MlRevertModelSnapshotUrlParts<'a>,
    ) -> MlRevertModelSnapshot<'a, ()> {
        MlRevertModelSnapshot::new(self.client.clone(), parts)
    }
    pub fn set_upgrade_mode<'a>(&self) -> MlSetUpgradeMode<'a, ()> {
        MlSetUpgradeMode::new(self.client.clone())
    }
    pub fn start_data_frame_analytics<'a>(
        &self,
        parts: MlStartDataFrameAnalyticsUrlParts<'a>,
    ) -> MlStartDataFrameAnalytics<'a, ()> {
        MlStartDataFrameAnalytics::new(self.client.clone(), parts)
    }
    pub fn start_datafeed<'a>(
        &self,
        parts: MlStartDatafeedUrlParts<'a>,
    ) -> MlStartDatafeed<'a, ()> {
        MlStartDatafeed::new(self.client.clone(), parts)
    }
    pub fn stop_data_frame_analytics<'a>(
        &self,
        parts: MlStopDataFrameAnalyticsUrlParts<'a>,
    ) -> MlStopDataFrameAnalytics<'a, ()> {
        MlStopDataFrameAnalytics::new(self.client.clone(), parts)
    }
    pub fn stop_datafeed<'a>(&self, parts: MlStopDatafeedUrlParts<'a>) -> MlStopDatafeed<'a, ()> {
        MlStopDatafeed::new(self.client.clone(), parts)
    }
    pub fn update_datafeed<'a>(
        &self,
        parts: MlUpdateDatafeedUrlParts<'a>,
    ) -> MlUpdateDatafeed<'a, ()> {
        MlUpdateDatafeed::new(self.client.clone(), parts)
    }
    pub fn update_filter<'a>(&self, parts: MlUpdateFilterUrlParts<'a>) -> MlUpdateFilter<'a, ()> {
        MlUpdateFilter::new(self.client.clone(), parts)
    }
    pub fn update_job<'a>(&self, parts: MlUpdateJobUrlParts<'a>) -> MlUpdateJob<'a, ()> {
        MlUpdateJob::new(self.client.clone(), parts)
    }
    pub fn update_model_snapshot<'a>(
        &self,
        parts: MlUpdateModelSnapshotUrlParts<'a>,
    ) -> MlUpdateModelSnapshot<'a, ()> {
        MlUpdateModelSnapshot::new(self.client.clone(), parts)
    }
    pub fn validate<'a>(&self) -> MlValidate<'a, ()> {
        MlValidate::new(self.client.clone())
    }
    pub fn validate_detector<'a>(&self) -> MlValidateDetector<'a, ()> {
        MlValidateDetector::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Ml APIs"]
    pub fn ml(&self) -> Ml {
        Ml::new(self.clone())
    }
}
