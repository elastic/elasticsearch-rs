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
#[allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
    params::*,
};
use serde::Serialize;
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Ml Close Job API"]
pub enum MlCloseJobParts<'a> {
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlCloseJobParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Close Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlCloseJobParts::JobId(ref job_id) => {
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
#[doc = "Builder for the [Ml Close Job API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-close-job.html)."]
pub struct MlCloseJob<'a, B> {
    client: Elasticsearch,
    parts: MlCloseJobParts<'a>,
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
    #[doc = "Creates a new instance of [MlCloseJob] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlCloseJobParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Close Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Delete Calendar API"]
pub enum MlDeleteCalendarParts<'a> {
    #[doc = "CalendarId"]
    CalendarId(&'a str),
}
impl<'a> MlDeleteCalendarParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Delete Calendar API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteCalendarParts::CalendarId(ref calendar_id) => {
                let mut p = String::with_capacity(15usize + calendar_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(calendar_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the Ml Delete Calendar API"]
pub struct MlDeleteCalendar<'a> {
    client: Elasticsearch,
    parts: MlDeleteCalendarParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteCalendar<'a> {
    #[doc = "Creates a new instance of [MlDeleteCalendar] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlDeleteCalendarParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Calendar API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
#[doc = "API parts for the Ml Delete Calendar Event API"]
pub enum MlDeleteCalendarEventParts<'a> {
    #[doc = "CalendarId and EventId"]
    CalendarIdEventId(&'a str, &'a str),
}
impl<'a> MlDeleteCalendarEventParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Delete Calendar Event API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteCalendarEventParts::CalendarIdEventId(ref calendar_id, ref event_id) => {
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
#[doc = "Builder for the Ml Delete Calendar Event API"]
pub struct MlDeleteCalendarEvent<'a> {
    client: Elasticsearch,
    parts: MlDeleteCalendarEventParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteCalendarEvent<'a> {
    #[doc = "Creates a new instance of [MlDeleteCalendarEvent] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlDeleteCalendarEventParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Calendar Event API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
#[doc = "API parts for the Ml Delete Calendar Job API"]
pub enum MlDeleteCalendarJobParts<'a> {
    #[doc = "CalendarId and JobId"]
    CalendarIdJobId(&'a str, &'a str),
}
impl<'a> MlDeleteCalendarJobParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Delete Calendar Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteCalendarJobParts::CalendarIdJobId(ref calendar_id, ref job_id) => {
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
#[doc = "Builder for the Ml Delete Calendar Job API"]
pub struct MlDeleteCalendarJob<'a> {
    client: Elasticsearch,
    parts: MlDeleteCalendarJobParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteCalendarJob<'a> {
    #[doc = "Creates a new instance of [MlDeleteCalendarJob] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlDeleteCalendarJobParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Calendar Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
#[doc = "API parts for the Ml Delete Datafeed API"]
pub enum MlDeleteDatafeedParts<'a> {
    #[doc = "DatafeedId"]
    DatafeedId(&'a str),
}
impl<'a> MlDeleteDatafeedParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Delete Datafeed API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteDatafeedParts::DatafeedId(ref datafeed_id) => {
                let mut p = String::with_capacity(15usize + datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(datafeed_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Delete Datafeed API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-datafeed.html)."]
pub struct MlDeleteDatafeed<'a> {
    client: Elasticsearch,
    parts: MlDeleteDatafeedParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteDatafeed<'a> {
    #[doc = "Creates a new instance of [MlDeleteDatafeed] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlDeleteDatafeedParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
#[doc = "API parts for the Ml Delete Expired Data API"]
pub enum MlDeleteExpiredDataParts {
    #[doc = "No parts"]
    None,
}
impl MlDeleteExpiredDataParts {
    #[doc = "Builds a relative URL path to the Ml Delete Expired Data API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteExpiredDataParts::None => "/_ml/_delete_expired_data".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the Ml Delete Expired Data API"]
pub struct MlDeleteExpiredData<'a> {
    client: Elasticsearch,
    parts: MlDeleteExpiredDataParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteExpiredData<'a> {
    #[doc = "Creates a new instance of [MlDeleteExpiredData]"]
    pub fn new(client: Elasticsearch) -> Self {
        MlDeleteExpiredData {
            client,
            parts: MlDeleteExpiredDataParts::None,
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
    #[doc = "Creates an asynchronous call to the Ml Delete Expired Data API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
#[doc = "API parts for the Ml Delete Filter API"]
pub enum MlDeleteFilterParts<'a> {
    #[doc = "FilterId"]
    FilterId(&'a str),
}
impl<'a> MlDeleteFilterParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Delete Filter API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteFilterParts::FilterId(ref filter_id) => {
                let mut p = String::with_capacity(13usize + filter_id.len());
                p.push_str("/_ml/filters/");
                p.push_str(filter_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the Ml Delete Filter API"]
pub struct MlDeleteFilter<'a> {
    client: Elasticsearch,
    parts: MlDeleteFilterParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteFilter<'a> {
    #[doc = "Creates a new instance of [MlDeleteFilter] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlDeleteFilterParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Filter API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
#[doc = "API parts for the Ml Delete Forecast API"]
pub enum MlDeleteForecastParts<'a> {
    #[doc = "JobId"]
    JobId(&'a str),
    #[doc = "JobId and ForecastId"]
    JobIdForecastId(&'a str, &'a str),
}
impl<'a> MlDeleteForecastParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Delete Forecast API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteForecastParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(33usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/_forecast");
                p.into()
            }
            MlDeleteForecastParts::JobIdForecastId(ref job_id, ref forecast_id) => {
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
#[doc = "Builder for the [Ml Delete Forecast API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-forecast.html)."]
pub struct MlDeleteForecast<'a> {
    client: Elasticsearch,
    parts: MlDeleteForecastParts<'a>,
    allow_no_forecasts: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> MlDeleteForecast<'a> {
    #[doc = "Creates a new instance of [MlDeleteForecast] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlDeleteForecastParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Forecast API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
#[doc = "API parts for the Ml Delete Job API"]
pub enum MlDeleteJobParts<'a> {
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlDeleteJobParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Delete Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteJobParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(23usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Delete Job API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-job.html)."]
pub struct MlDeleteJob<'a> {
    client: Elasticsearch,
    parts: MlDeleteJobParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    force: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    wait_for_completion: Option<bool>,
}
impl<'a> MlDeleteJob<'a> {
    #[doc = "Creates a new instance of [MlDeleteJob] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlDeleteJobParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
#[doc = "API parts for the Ml Delete Model Snapshot API"]
pub enum MlDeleteModelSnapshotParts<'a> {
    #[doc = "JobId and SnapshotId"]
    JobIdSnapshotId(&'a str, &'a str),
}
impl<'a> MlDeleteModelSnapshotParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Delete Model Snapshot API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteModelSnapshotParts::JobIdSnapshotId(ref job_id, ref snapshot_id) => {
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
#[doc = "Builder for the [Ml Delete Model Snapshot API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-snapshot.html)."]
pub struct MlDeleteModelSnapshot<'a> {
    client: Elasticsearch,
    parts: MlDeleteModelSnapshotParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlDeleteModelSnapshot<'a> {
    #[doc = "Creates a new instance of [MlDeleteModelSnapshot] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlDeleteModelSnapshotParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Model Snapshot API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
#[doc = "API parts for the Ml Flush Job API"]
pub enum MlFlushJobParts<'a> {
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlFlushJobParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Flush Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlFlushJobParts::JobId(ref job_id) => {
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
#[doc = "Builder for the [Ml Flush Job API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-flush-job.html)."]
pub struct MlFlushJob<'a, B> {
    client: Elasticsearch,
    parts: MlFlushJobParts<'a>,
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
    #[doc = "Creates a new instance of [MlFlushJob] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlFlushJobParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Flush Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Forecast API"]
pub enum MlForecastParts<'a> {
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlForecastParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Forecast API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlForecastParts::JobId(ref job_id) => {
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
#[doc = "Builder for the Ml Forecast API"]
pub struct MlForecast<'a, B> {
    client: Elasticsearch,
    parts: MlForecastParts<'a>,
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
    #[doc = "Creates a new instance of [MlForecast] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlForecastParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Forecast API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Get Buckets API"]
pub enum MlGetBucketsParts<'a> {
    #[doc = "JobId and Timestamp"]
    JobIdTimestamp(&'a str, &'a str),
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlGetBucketsParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Get Buckets API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetBucketsParts::JobIdTimestamp(ref job_id, ref timestamp) => {
                let mut p = String::with_capacity(40usize + job_id.len() + timestamp.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/results/buckets/");
                p.push_str(timestamp.as_ref());
                p.into()
            }
            MlGetBucketsParts::JobId(ref job_id) => {
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
#[doc = "Builder for the [Ml Get Buckets API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-bucket.html)."]
pub struct MlGetBuckets<'a, B> {
    client: Elasticsearch,
    parts: MlGetBucketsParts<'a>,
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
    #[doc = "Creates a new instance of [MlGetBuckets] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlGetBucketsParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Get Buckets API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
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
#[doc = "API parts for the Ml Get Calendar Events API"]
pub enum MlGetCalendarEventsParts<'a> {
    #[doc = "CalendarId"]
    CalendarId(&'a str),
}
impl<'a> MlGetCalendarEventsParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Get Calendar Events API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetCalendarEventsParts::CalendarId(ref calendar_id) => {
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
#[doc = "Builder for the Ml Get Calendar Events API"]
pub struct MlGetCalendarEvents<'a> {
    client: Elasticsearch,
    parts: MlGetCalendarEventsParts<'a>,
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
    #[doc = "Creates a new instance of [MlGetCalendarEvents] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlGetCalendarEventsParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Get Calendar Events API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Ml Get Calendars API"]
pub enum MlGetCalendarsParts<'a> {
    #[doc = "No parts"]
    None,
    #[doc = "CalendarId"]
    CalendarId(&'a str),
}
impl<'a> MlGetCalendarsParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Get Calendars API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetCalendarsParts::None => "/_ml/calendars".into(),
            MlGetCalendarsParts::CalendarId(ref calendar_id) => {
                let mut p = String::with_capacity(15usize + calendar_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(calendar_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the Ml Get Calendars API"]
pub struct MlGetCalendars<'a, B> {
    client: Elasticsearch,
    parts: MlGetCalendarsParts<'a>,
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
    #[doc = "Creates a new instance of [MlGetCalendars] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlGetCalendarsParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Get Calendars API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
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
#[doc = "API parts for the Ml Get Categories API"]
pub enum MlGetCategoriesParts<'a> {
    #[doc = "JobId and CategoryId"]
    JobIdCategoryId(&'a str, i64),
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlGetCategoriesParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Get Categories API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetCategoriesParts::JobIdCategoryId(ref job_id, ref category_id) => {
                let category_id_str = category_id.to_string();
                let mut p = String::with_capacity(43usize + job_id.len() + category_id_str.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/results/categories/");
                p.push_str(category_id_str.as_ref());
                p.into()
            }
            MlGetCategoriesParts::JobId(ref job_id) => {
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
#[doc = "Builder for the [Ml Get Categories API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-category.html)."]
pub struct MlGetCategories<'a, B> {
    client: Elasticsearch,
    parts: MlGetCategoriesParts<'a>,
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
    #[doc = "Creates a new instance of [MlGetCategories] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlGetCategoriesParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Get Categories API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
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
#[doc = "API parts for the Ml Get Datafeed Stats API"]
pub enum MlGetDatafeedStatsParts<'a> {
    #[doc = "DatafeedId"]
    DatafeedId(&'a str),
    #[doc = "No parts"]
    None,
}
impl<'a> MlGetDatafeedStatsParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Get Datafeed Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetDatafeedStatsParts::DatafeedId(ref datafeed_id) => {
                let mut p = String::with_capacity(22usize + datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(datafeed_id.as_ref());
                p.push_str("/_stats");
                p.into()
            }
            MlGetDatafeedStatsParts::None => "/_ml/datafeeds/_stats".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Datafeed Stats API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-datafeed-stats.html)."]
pub struct MlGetDatafeedStats<'a> {
    client: Elasticsearch,
    parts: MlGetDatafeedStatsParts<'a>,
    allow_no_datafeeds: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlGetDatafeedStats<'a> {
    #[doc = "Creates a new instance of [MlGetDatafeedStats] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlGetDatafeedStatsParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Get Datafeed Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Ml Get Datafeeds API"]
pub enum MlGetDatafeedsParts<'a> {
    #[doc = "DatafeedId"]
    DatafeedId(&'a str),
    #[doc = "No parts"]
    None,
}
impl<'a> MlGetDatafeedsParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Get Datafeeds API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetDatafeedsParts::DatafeedId(ref datafeed_id) => {
                let mut p = String::with_capacity(15usize + datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(datafeed_id.as_ref());
                p.into()
            }
            MlGetDatafeedsParts::None => "/_ml/datafeeds".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Datafeeds API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-datafeed.html)."]
pub struct MlGetDatafeeds<'a> {
    client: Elasticsearch,
    parts: MlGetDatafeedsParts<'a>,
    allow_no_datafeeds: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlGetDatafeeds<'a> {
    #[doc = "Creates a new instance of [MlGetDatafeeds] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlGetDatafeedsParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Get Datafeeds API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Ml Get Filters API"]
pub enum MlGetFiltersParts<'a> {
    #[doc = "No parts"]
    None,
    #[doc = "FilterId"]
    FilterId(&'a str),
}
impl<'a> MlGetFiltersParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Get Filters API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetFiltersParts::None => "/_ml/filters".into(),
            MlGetFiltersParts::FilterId(ref filter_id) => {
                let mut p = String::with_capacity(13usize + filter_id.len());
                p.push_str("/_ml/filters/");
                p.push_str(filter_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the Ml Get Filters API"]
pub struct MlGetFilters<'a> {
    client: Elasticsearch,
    parts: MlGetFiltersParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    from: Option<i32>,
    human: Option<bool>,
    pretty: Option<bool>,
    size: Option<i32>,
    source: Option<&'a str>,
}
impl<'a> MlGetFilters<'a> {
    #[doc = "Creates a new instance of [MlGetFilters] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlGetFiltersParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Get Filters API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Ml Get Influencers API"]
pub enum MlGetInfluencersParts<'a> {
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlGetInfluencersParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Get Influencers API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetInfluencersParts::JobId(ref job_id) => {
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
#[doc = "Builder for the [Ml Get Influencers API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-influencer.html)."]
pub struct MlGetInfluencers<'a, B> {
    client: Elasticsearch,
    parts: MlGetInfluencersParts<'a>,
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
    #[doc = "Creates a new instance of [MlGetInfluencers] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlGetInfluencersParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Get Influencers API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
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
#[doc = "API parts for the Ml Get Job Stats API"]
pub enum MlGetJobStatsParts<'a> {
    #[doc = "No parts"]
    None,
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlGetJobStatsParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Get Job Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetJobStatsParts::None => "/_ml/anomaly_detectors/_stats".into(),
            MlGetJobStatsParts::JobId(ref job_id) => {
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
#[doc = "Builder for the [Ml Get Job Stats API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-job-stats.html)."]
pub struct MlGetJobStats<'a> {
    client: Elasticsearch,
    parts: MlGetJobStatsParts<'a>,
    allow_no_jobs: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlGetJobStats<'a> {
    #[doc = "Creates a new instance of [MlGetJobStats] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlGetJobStatsParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Get Job Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Ml Get Jobs API"]
pub enum MlGetJobsParts<'a> {
    #[doc = "JobId"]
    JobId(&'a str),
    #[doc = "No parts"]
    None,
}
impl<'a> MlGetJobsParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Get Jobs API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetJobsParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(23usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.into()
            }
            MlGetJobsParts::None => "/_ml/anomaly_detectors".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Get Jobs API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-job.html)."]
pub struct MlGetJobs<'a> {
    client: Elasticsearch,
    parts: MlGetJobsParts<'a>,
    allow_no_jobs: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlGetJobs<'a> {
    #[doc = "Creates a new instance of [MlGetJobs] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlGetJobsParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Get Jobs API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Ml Get Model Snapshots API"]
pub enum MlGetModelSnapshotsParts<'a> {
    #[doc = "JobId and SnapshotId"]
    JobIdSnapshotId(&'a str, &'a str),
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlGetModelSnapshotsParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Get Model Snapshots API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetModelSnapshotsParts::JobIdSnapshotId(ref job_id, ref snapshot_id) => {
                let mut p = String::with_capacity(40usize + job_id.len() + snapshot_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.push_str("/model_snapshots/");
                p.push_str(snapshot_id.as_ref());
                p.into()
            }
            MlGetModelSnapshotsParts::JobId(ref job_id) => {
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
#[doc = "Builder for the [Ml Get Model Snapshots API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-snapshot.html)."]
pub struct MlGetModelSnapshots<'a, B> {
    client: Elasticsearch,
    parts: MlGetModelSnapshotsParts<'a>,
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
    #[doc = "Creates a new instance of [MlGetModelSnapshots] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlGetModelSnapshotsParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Get Model Snapshots API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
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
#[doc = "API parts for the Ml Get Overall Buckets API"]
pub enum MlGetOverallBucketsParts<'a> {
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlGetOverallBucketsParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Get Overall Buckets API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetOverallBucketsParts::JobId(ref job_id) => {
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
#[doc = "Builder for the [Ml Get Overall Buckets API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-overall-buckets.html)."]
pub struct MlGetOverallBuckets<'a, B> {
    client: Elasticsearch,
    parts: MlGetOverallBucketsParts<'a>,
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
    #[doc = "Creates a new instance of [MlGetOverallBuckets] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlGetOverallBucketsParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Get Overall Buckets API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
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
#[doc = "API parts for the Ml Get Records API"]
pub enum MlGetRecordsParts<'a> {
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlGetRecordsParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Get Records API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetRecordsParts::JobId(ref job_id) => {
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
#[doc = "Builder for the [Ml Get Records API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-record.html)."]
pub struct MlGetRecords<'a, B> {
    client: Elasticsearch,
    parts: MlGetRecordsParts<'a>,
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
    #[doc = "Creates a new instance of [MlGetRecords] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlGetRecordsParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Get Records API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
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
#[doc = "API parts for the Ml Info API"]
pub enum MlInfoParts {
    #[doc = "No parts"]
    None,
}
impl MlInfoParts {
    #[doc = "Builds a relative URL path to the Ml Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlInfoParts::None => "/_ml/info".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the Ml Info API"]
pub struct MlInfo<'a> {
    client: Elasticsearch,
    parts: MlInfoParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlInfo<'a> {
    #[doc = "Creates a new instance of [MlInfo]"]
    pub fn new(client: Elasticsearch) -> Self {
        MlInfo {
            client,
            parts: MlInfoParts::None,
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
    #[doc = "Creates an asynchronous call to the Ml Info API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Ml Open Job API"]
pub enum MlOpenJobParts<'a> {
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlOpenJobParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Open Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlOpenJobParts::JobId(ref job_id) => {
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
#[doc = "Builder for the [Ml Open Job API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-open-job.html)."]
pub struct MlOpenJob<'a, B> {
    client: Elasticsearch,
    parts: MlOpenJobParts<'a>,
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
    #[doc = "Creates a new instance of [MlOpenJob] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlOpenJobParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Open Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Post Calendar Events API"]
pub enum MlPostCalendarEventsParts<'a> {
    #[doc = "CalendarId"]
    CalendarId(&'a str),
}
impl<'a> MlPostCalendarEventsParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Post Calendar Events API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPostCalendarEventsParts::CalendarId(ref calendar_id) => {
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
#[doc = "Builder for the Ml Post Calendar Events API"]
pub struct MlPostCalendarEvents<'a, B> {
    client: Elasticsearch,
    parts: MlPostCalendarEventsParts<'a>,
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
    #[doc = "Creates a new instance of [MlPostCalendarEvents] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlPostCalendarEventsParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Post Calendar Events API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Post Data API"]
pub enum MlPostDataParts<'a> {
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlPostDataParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Post Data API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPostDataParts::JobId(ref job_id) => {
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
#[doc = "Builder for the [Ml Post Data API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-post-data.html)."]
pub struct MlPostData<'a, B> {
    client: Elasticsearch,
    parts: MlPostDataParts<'a>,
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
    #[doc = "Creates a new instance of [MlPostData] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlPostDataParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Post Data API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Preview Datafeed API"]
pub enum MlPreviewDatafeedParts<'a> {
    #[doc = "DatafeedId"]
    DatafeedId(&'a str),
}
impl<'a> MlPreviewDatafeedParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Preview Datafeed API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPreviewDatafeedParts::DatafeedId(ref datafeed_id) => {
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
#[doc = "Builder for the [Ml Preview Datafeed API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-preview-datafeed.html)."]
pub struct MlPreviewDatafeed<'a> {
    client: Elasticsearch,
    parts: MlPreviewDatafeedParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> MlPreviewDatafeed<'a> {
    #[doc = "Creates a new instance of [MlPreviewDatafeed] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlPreviewDatafeedParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Preview Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
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
#[doc = "API parts for the Ml Put Calendar API"]
pub enum MlPutCalendarParts<'a> {
    #[doc = "CalendarId"]
    CalendarId(&'a str),
}
impl<'a> MlPutCalendarParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Put Calendar API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPutCalendarParts::CalendarId(ref calendar_id) => {
                let mut p = String::with_capacity(15usize + calendar_id.len());
                p.push_str("/_ml/calendars/");
                p.push_str(calendar_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the Ml Put Calendar API"]
pub struct MlPutCalendar<'a, B> {
    client: Elasticsearch,
    parts: MlPutCalendarParts<'a>,
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
    #[doc = "Creates a new instance of [MlPutCalendar] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlPutCalendarParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Put Calendar API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
#[doc = "API parts for the Ml Put Calendar Job API"]
pub enum MlPutCalendarJobParts<'a> {
    #[doc = "CalendarId and JobId"]
    CalendarIdJobId(&'a str, &'a str),
}
impl<'a> MlPutCalendarJobParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Put Calendar Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPutCalendarJobParts::CalendarIdJobId(ref calendar_id, ref job_id) => {
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
#[doc = "Builder for the Ml Put Calendar Job API"]
pub struct MlPutCalendarJob<'a, B> {
    client: Elasticsearch,
    parts: MlPutCalendarJobParts<'a>,
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
    #[doc = "Creates a new instance of [MlPutCalendarJob] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlPutCalendarJobParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Put Calendar Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
#[doc = "API parts for the Ml Put Datafeed API"]
pub enum MlPutDatafeedParts<'a> {
    #[doc = "DatafeedId"]
    DatafeedId(&'a str),
}
impl<'a> MlPutDatafeedParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Put Datafeed API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPutDatafeedParts::DatafeedId(ref datafeed_id) => {
                let mut p = String::with_capacity(15usize + datafeed_id.len());
                p.push_str("/_ml/datafeeds/");
                p.push_str(datafeed_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Put Datafeed API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-datafeed.html)."]
pub struct MlPutDatafeed<'a, B> {
    client: Elasticsearch,
    parts: MlPutDatafeedParts<'a>,
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
    #[doc = "Creates a new instance of [MlPutDatafeed] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlPutDatafeedParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Put Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
#[doc = "API parts for the Ml Put Filter API"]
pub enum MlPutFilterParts<'a> {
    #[doc = "FilterId"]
    FilterId(&'a str),
}
impl<'a> MlPutFilterParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Put Filter API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPutFilterParts::FilterId(ref filter_id) => {
                let mut p = String::with_capacity(13usize + filter_id.len());
                p.push_str("/_ml/filters/");
                p.push_str(filter_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the Ml Put Filter API"]
pub struct MlPutFilter<'a, B> {
    client: Elasticsearch,
    parts: MlPutFilterParts<'a>,
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
    #[doc = "Creates a new instance of [MlPutFilter] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlPutFilterParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Put Filter API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
#[doc = "API parts for the Ml Put Job API"]
pub enum MlPutJobParts<'a> {
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlPutJobParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Put Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPutJobParts::JobId(ref job_id) => {
                let mut p = String::with_capacity(23usize + job_id.len());
                p.push_str("/_ml/anomaly_detectors/");
                p.push_str(job_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Put Job API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-job.html)."]
pub struct MlPutJob<'a, B> {
    client: Elasticsearch,
    parts: MlPutJobParts<'a>,
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
    #[doc = "Creates a new instance of [MlPutJob] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlPutJobParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Put Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
#[doc = "API parts for the Ml Revert Model Snapshot API"]
pub enum MlRevertModelSnapshotParts<'a> {
    #[doc = "JobId and SnapshotId"]
    JobIdSnapshotId(&'a str, &'a str),
}
impl<'a> MlRevertModelSnapshotParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Revert Model Snapshot API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlRevertModelSnapshotParts::JobIdSnapshotId(ref job_id, ref snapshot_id) => {
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
#[doc = "Builder for the [Ml Revert Model Snapshot API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-revert-snapshot.html)."]
pub struct MlRevertModelSnapshot<'a, B> {
    client: Elasticsearch,
    parts: MlRevertModelSnapshotParts<'a>,
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
    #[doc = "Creates a new instance of [MlRevertModelSnapshot] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlRevertModelSnapshotParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Revert Model Snapshot API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Set Upgrade Mode API"]
pub enum MlSetUpgradeModeParts {
    #[doc = "No parts"]
    None,
}
impl MlSetUpgradeModeParts {
    #[doc = "Builds a relative URL path to the Ml Set Upgrade Mode API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlSetUpgradeModeParts::None => "/_ml/set_upgrade_mode".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ml Set Upgrade Mode API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-set-upgrade-mode.html)."]
pub struct MlSetUpgradeMode<'a, B> {
    client: Elasticsearch,
    parts: MlSetUpgradeModeParts,
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
    #[doc = "Creates a new instance of [MlSetUpgradeMode]"]
    pub fn new(client: Elasticsearch) -> Self {
        MlSetUpgradeMode {
            client,
            parts: MlSetUpgradeModeParts::None,
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
    #[doc = "Creates an asynchronous call to the Ml Set Upgrade Mode API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Start Datafeed API"]
pub enum MlStartDatafeedParts<'a> {
    #[doc = "DatafeedId"]
    DatafeedId(&'a str),
}
impl<'a> MlStartDatafeedParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Start Datafeed API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlStartDatafeedParts::DatafeedId(ref datafeed_id) => {
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
#[doc = "Builder for the [Ml Start Datafeed API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-start-datafeed.html)."]
pub struct MlStartDatafeed<'a, B> {
    client: Elasticsearch,
    parts: MlStartDatafeedParts<'a>,
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
    #[doc = "Creates a new instance of [MlStartDatafeed] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlStartDatafeedParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Start Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Stop Datafeed API"]
pub enum MlStopDatafeedParts<'a> {
    #[doc = "DatafeedId"]
    DatafeedId(&'a str),
}
impl<'a> MlStopDatafeedParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Stop Datafeed API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlStopDatafeedParts::DatafeedId(ref datafeed_id) => {
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
#[doc = "Builder for the [Ml Stop Datafeed API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-stop-datafeed.html)."]
pub struct MlStopDatafeed<'a, B> {
    client: Elasticsearch,
    parts: MlStopDatafeedParts<'a>,
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
    #[doc = "Creates a new instance of [MlStopDatafeed] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlStopDatafeedParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Stop Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Update Datafeed API"]
pub enum MlUpdateDatafeedParts<'a> {
    #[doc = "DatafeedId"]
    DatafeedId(&'a str),
}
impl<'a> MlUpdateDatafeedParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Update Datafeed API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateDatafeedParts::DatafeedId(ref datafeed_id) => {
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
#[doc = "Builder for the [Ml Update Datafeed API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-datafeed.html)."]
pub struct MlUpdateDatafeed<'a, B> {
    client: Elasticsearch,
    parts: MlUpdateDatafeedParts<'a>,
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
    #[doc = "Creates a new instance of [MlUpdateDatafeed] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlUpdateDatafeedParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Update Datafeed API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Update Filter API"]
pub enum MlUpdateFilterParts<'a> {
    #[doc = "FilterId"]
    FilterId(&'a str),
}
impl<'a> MlUpdateFilterParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Update Filter API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateFilterParts::FilterId(ref filter_id) => {
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
#[doc = "Builder for the Ml Update Filter API"]
pub struct MlUpdateFilter<'a, B> {
    client: Elasticsearch,
    parts: MlUpdateFilterParts<'a>,
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
    #[doc = "Creates a new instance of [MlUpdateFilter] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlUpdateFilterParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Update Filter API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Update Job API"]
pub enum MlUpdateJobParts<'a> {
    #[doc = "JobId"]
    JobId(&'a str),
}
impl<'a> MlUpdateJobParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Update Job API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateJobParts::JobId(ref job_id) => {
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
#[doc = "Builder for the [Ml Update Job API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-job.html)."]
pub struct MlUpdateJob<'a, B> {
    client: Elasticsearch,
    parts: MlUpdateJobParts<'a>,
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
    #[doc = "Creates a new instance of [MlUpdateJob] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlUpdateJobParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Update Job API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Update Model Snapshot API"]
pub enum MlUpdateModelSnapshotParts<'a> {
    #[doc = "JobId and SnapshotId"]
    JobIdSnapshotId(&'a str, &'a str),
}
impl<'a> MlUpdateModelSnapshotParts<'a> {
    #[doc = "Builds a relative URL path to the Ml Update Model Snapshot API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateModelSnapshotParts::JobIdSnapshotId(ref job_id, ref snapshot_id) => {
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
#[doc = "Builder for the [Ml Update Model Snapshot API](http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-snapshot.html)."]
pub struct MlUpdateModelSnapshot<'a, B> {
    client: Elasticsearch,
    parts: MlUpdateModelSnapshotParts<'a>,
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
    #[doc = "Creates a new instance of [MlUpdateModelSnapshot] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: MlUpdateModelSnapshotParts<'a>) -> Self {
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
    #[doc = "Creates an asynchronous call to the Ml Update Model Snapshot API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Validate API"]
pub enum MlValidateParts {
    #[doc = "No parts"]
    None,
}
impl MlValidateParts {
    #[doc = "Builds a relative URL path to the Ml Validate API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlValidateParts::None => "/_ml/anomaly_detectors/_validate".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the Ml Validate API"]
pub struct MlValidate<'a, B> {
    client: Elasticsearch,
    parts: MlValidateParts,
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
    #[doc = "Creates a new instance of [MlValidate]"]
    pub fn new(client: Elasticsearch) -> Self {
        MlValidate {
            client,
            parts: MlValidateParts::None,
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
    #[doc = "Creates an asynchronous call to the Ml Validate API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "API parts for the Ml Validate Detector API"]
pub enum MlValidateDetectorParts {
    #[doc = "No parts"]
    None,
}
impl MlValidateDetectorParts {
    #[doc = "Builds a relative URL path to the Ml Validate Detector API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlValidateDetectorParts::None => "/_ml/anomaly_detectors/_validate/detector".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the Ml Validate Detector API"]
pub struct MlValidateDetector<'a, B> {
    client: Elasticsearch,
    parts: MlValidateDetectorParts,
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
    #[doc = "Creates a new instance of [MlValidateDetector]"]
    pub fn new(client: Elasticsearch) -> Self {
        MlValidateDetector {
            client,
            parts: MlValidateDetectorParts::None,
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
    #[doc = "Creates an asynchronous call to the Ml Validate Detector API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
#[doc = "Namespace client for Machine Learning APIs"]
pub struct Ml {
    client: Elasticsearch,
}
impl Ml {
    #[doc = "Creates a new instance of [Ml]"]
    pub fn new(client: Elasticsearch) -> Self {
        Self { client }
    }
    pub fn close_job<'a>(&self, parts: MlCloseJobParts<'a>) -> MlCloseJob<'a, ()> {
        MlCloseJob::new(self.client.clone(), parts)
    }
    pub fn delete_calendar<'a>(&self, parts: MlDeleteCalendarParts<'a>) -> MlDeleteCalendar<'a> {
        MlDeleteCalendar::new(self.client.clone(), parts)
    }
    pub fn delete_calendar_event<'a>(
        &self,
        parts: MlDeleteCalendarEventParts<'a>,
    ) -> MlDeleteCalendarEvent<'a> {
        MlDeleteCalendarEvent::new(self.client.clone(), parts)
    }
    pub fn delete_calendar_job<'a>(
        &self,
        parts: MlDeleteCalendarJobParts<'a>,
    ) -> MlDeleteCalendarJob<'a> {
        MlDeleteCalendarJob::new(self.client.clone(), parts)
    }
    pub fn delete_datafeed<'a>(&self, parts: MlDeleteDatafeedParts<'a>) -> MlDeleteDatafeed<'a> {
        MlDeleteDatafeed::new(self.client.clone(), parts)
    }
    pub fn delete_expired_data<'a>(&self) -> MlDeleteExpiredData<'a> {
        MlDeleteExpiredData::new(self.client.clone())
    }
    pub fn delete_filter<'a>(&self, parts: MlDeleteFilterParts<'a>) -> MlDeleteFilter<'a> {
        MlDeleteFilter::new(self.client.clone(), parts)
    }
    pub fn delete_forecast<'a>(&self, parts: MlDeleteForecastParts<'a>) -> MlDeleteForecast<'a> {
        MlDeleteForecast::new(self.client.clone(), parts)
    }
    pub fn delete_job<'a>(&self, parts: MlDeleteJobParts<'a>) -> MlDeleteJob<'a> {
        MlDeleteJob::new(self.client.clone(), parts)
    }
    pub fn delete_model_snapshot<'a>(
        &self,
        parts: MlDeleteModelSnapshotParts<'a>,
    ) -> MlDeleteModelSnapshot<'a> {
        MlDeleteModelSnapshot::new(self.client.clone(), parts)
    }
    pub fn flush_job<'a>(&self, parts: MlFlushJobParts<'a>) -> MlFlushJob<'a, ()> {
        MlFlushJob::new(self.client.clone(), parts)
    }
    pub fn forecast<'a>(&self, parts: MlForecastParts<'a>) -> MlForecast<'a, ()> {
        MlForecast::new(self.client.clone(), parts)
    }
    pub fn get_buckets<'a>(&self, parts: MlGetBucketsParts<'a>) -> MlGetBuckets<'a, ()> {
        MlGetBuckets::new(self.client.clone(), parts)
    }
    pub fn get_calendar_events<'a>(
        &self,
        parts: MlGetCalendarEventsParts<'a>,
    ) -> MlGetCalendarEvents<'a> {
        MlGetCalendarEvents::new(self.client.clone(), parts)
    }
    pub fn get_calendars<'a>(&self, parts: MlGetCalendarsParts<'a>) -> MlGetCalendars<'a, ()> {
        MlGetCalendars::new(self.client.clone(), parts)
    }
    pub fn get_categories<'a>(&self, parts: MlGetCategoriesParts<'a>) -> MlGetCategories<'a, ()> {
        MlGetCategories::new(self.client.clone(), parts)
    }
    pub fn get_datafeed_stats<'a>(
        &self,
        parts: MlGetDatafeedStatsParts<'a>,
    ) -> MlGetDatafeedStats<'a> {
        MlGetDatafeedStats::new(self.client.clone(), parts)
    }
    pub fn get_datafeeds<'a>(&self, parts: MlGetDatafeedsParts<'a>) -> MlGetDatafeeds<'a> {
        MlGetDatafeeds::new(self.client.clone(), parts)
    }
    pub fn get_filters<'a>(&self, parts: MlGetFiltersParts<'a>) -> MlGetFilters<'a> {
        MlGetFilters::new(self.client.clone(), parts)
    }
    pub fn get_influencers<'a>(
        &self,
        parts: MlGetInfluencersParts<'a>,
    ) -> MlGetInfluencers<'a, ()> {
        MlGetInfluencers::new(self.client.clone(), parts)
    }
    pub fn get_job_stats<'a>(&self, parts: MlGetJobStatsParts<'a>) -> MlGetJobStats<'a> {
        MlGetJobStats::new(self.client.clone(), parts)
    }
    pub fn get_jobs<'a>(&self, parts: MlGetJobsParts<'a>) -> MlGetJobs<'a> {
        MlGetJobs::new(self.client.clone(), parts)
    }
    pub fn get_model_snapshots<'a>(
        &self,
        parts: MlGetModelSnapshotsParts<'a>,
    ) -> MlGetModelSnapshots<'a, ()> {
        MlGetModelSnapshots::new(self.client.clone(), parts)
    }
    pub fn get_overall_buckets<'a>(
        &self,
        parts: MlGetOverallBucketsParts<'a>,
    ) -> MlGetOverallBuckets<'a, ()> {
        MlGetOverallBuckets::new(self.client.clone(), parts)
    }
    pub fn get_records<'a>(&self, parts: MlGetRecordsParts<'a>) -> MlGetRecords<'a, ()> {
        MlGetRecords::new(self.client.clone(), parts)
    }
    pub fn info<'a>(&self) -> MlInfo<'a> {
        MlInfo::new(self.client.clone())
    }
    pub fn open_job<'a>(&self, parts: MlOpenJobParts<'a>) -> MlOpenJob<'a, ()> {
        MlOpenJob::new(self.client.clone(), parts)
    }
    pub fn post_calendar_events<'a>(
        &self,
        parts: MlPostCalendarEventsParts<'a>,
    ) -> MlPostCalendarEvents<'a, ()> {
        MlPostCalendarEvents::new(self.client.clone(), parts)
    }
    pub fn post_data<'a>(&self, parts: MlPostDataParts<'a>) -> MlPostData<'a, ()> {
        MlPostData::new(self.client.clone(), parts)
    }
    pub fn preview_datafeed<'a>(&self, parts: MlPreviewDatafeedParts<'a>) -> MlPreviewDatafeed<'a> {
        MlPreviewDatafeed::new(self.client.clone(), parts)
    }
    pub fn put_calendar<'a>(&self, parts: MlPutCalendarParts<'a>) -> MlPutCalendar<'a, ()> {
        MlPutCalendar::new(self.client.clone(), parts)
    }
    pub fn put_calendar_job<'a>(
        &self,
        parts: MlPutCalendarJobParts<'a>,
    ) -> MlPutCalendarJob<'a, ()> {
        MlPutCalendarJob::new(self.client.clone(), parts)
    }
    pub fn put_datafeed<'a>(&self, parts: MlPutDatafeedParts<'a>) -> MlPutDatafeed<'a, ()> {
        MlPutDatafeed::new(self.client.clone(), parts)
    }
    pub fn put_filter<'a>(&self, parts: MlPutFilterParts<'a>) -> MlPutFilter<'a, ()> {
        MlPutFilter::new(self.client.clone(), parts)
    }
    pub fn put_job<'a>(&self, parts: MlPutJobParts<'a>) -> MlPutJob<'a, ()> {
        MlPutJob::new(self.client.clone(), parts)
    }
    pub fn revert_model_snapshot<'a>(
        &self,
        parts: MlRevertModelSnapshotParts<'a>,
    ) -> MlRevertModelSnapshot<'a, ()> {
        MlRevertModelSnapshot::new(self.client.clone(), parts)
    }
    pub fn set_upgrade_mode<'a>(&self) -> MlSetUpgradeMode<'a, ()> {
        MlSetUpgradeMode::new(self.client.clone())
    }
    pub fn start_datafeed<'a>(&self, parts: MlStartDatafeedParts<'a>) -> MlStartDatafeed<'a, ()> {
        MlStartDatafeed::new(self.client.clone(), parts)
    }
    pub fn stop_datafeed<'a>(&self, parts: MlStopDatafeedParts<'a>) -> MlStopDatafeed<'a, ()> {
        MlStopDatafeed::new(self.client.clone(), parts)
    }
    pub fn update_datafeed<'a>(
        &self,
        parts: MlUpdateDatafeedParts<'a>,
    ) -> MlUpdateDatafeed<'a, ()> {
        MlUpdateDatafeed::new(self.client.clone(), parts)
    }
    pub fn update_filter<'a>(&self, parts: MlUpdateFilterParts<'a>) -> MlUpdateFilter<'a, ()> {
        MlUpdateFilter::new(self.client.clone(), parts)
    }
    pub fn update_job<'a>(&self, parts: MlUpdateJobParts<'a>) -> MlUpdateJob<'a, ()> {
        MlUpdateJob::new(self.client.clone(), parts)
    }
    pub fn update_model_snapshot<'a>(
        &self,
        parts: MlUpdateModelSnapshotParts<'a>,
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
    #[doc = "Creates a namespace client for Machine Learning APIs"]
    pub fn ml(&self) -> Ml {
        Ml::new(self.clone())
    }
}