use super::super::client::Elasticsearch;
use super::super::enums::*;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct MlCloseJob {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_jobs: Option<bool>,
    force: Option<bool>,
    timeout: Option<String>,
}
impl MlCloseJob {
    pub fn new(client: Elasticsearch) -> Self {
        MlCloseJob {
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
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: Option<bool>) -> Self {
        self.allow_no_jobs = allow_no_jobs;
        self
    }
    #[doc = "True if the job should be forcefully closed"]
    pub fn force(mut self, force: Option<bool>) -> Self {
        self.force = force;
        self
    }
    #[doc = "Controls the time to wait until a job has closed. Default to 30 minutes"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for MlCloseJob {
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
pub struct MlDeleteCalendar {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteCalendar {
    pub fn new(client: Elasticsearch) -> Self {
        MlDeleteCalendar {
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
impl Sender for MlDeleteCalendar {
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
pub struct MlDeleteCalendarEvent {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteCalendarEvent {
    pub fn new(client: Elasticsearch) -> Self {
        MlDeleteCalendarEvent {
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
impl Sender for MlDeleteCalendarEvent {
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
pub struct MlDeleteCalendarJob {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteCalendarJob {
    pub fn new(client: Elasticsearch) -> Self {
        MlDeleteCalendarJob {
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
impl Sender for MlDeleteCalendarJob {
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
pub struct MlDeleteDataFrameAnalytics {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteDataFrameAnalytics {
    pub fn new(client: Elasticsearch) -> Self {
        MlDeleteDataFrameAnalytics {
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
impl Sender for MlDeleteDataFrameAnalytics {
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
pub struct MlDeleteDatafeed {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    force: Option<bool>,
}
impl MlDeleteDatafeed {
    pub fn new(client: Elasticsearch) -> Self {
        MlDeleteDatafeed {
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
    #[doc = "True if the datafeed should be forcefully deleted"]
    pub fn force(mut self, force: Option<bool>) -> Self {
        self.force = force;
        self
    }
}
impl Sender for MlDeleteDatafeed {
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
impl Sender for MlDeleteExpiredData {
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
pub struct MlDeleteFilter {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteFilter {
    pub fn new(client: Elasticsearch) -> Self {
        MlDeleteFilter {
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
impl Sender for MlDeleteFilter {
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
pub struct MlDeleteForecast {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_forecasts: Option<bool>,
    timeout: Option<String>,
}
impl MlDeleteForecast {
    pub fn new(client: Elasticsearch) -> Self {
        MlDeleteForecast {
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
    #[doc = "Whether to ignore if `_all` matches no forecasts"]
    pub fn allow_no_forecasts(mut self, allow_no_forecasts: Option<bool>) -> Self {
        self.allow_no_forecasts = allow_no_forecasts;
        self
    }
    #[doc = "Controls the time to wait until the forecast(s) are deleted. Default to 30 seconds"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for MlDeleteForecast {
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
pub struct MlDeleteJob {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    force: Option<bool>,
    wait_for_completion: Option<bool>,
}
impl MlDeleteJob {
    pub fn new(client: Elasticsearch) -> Self {
        MlDeleteJob {
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
    #[doc = "True if the job should be forcefully deleted"]
    pub fn force(mut self, force: Option<bool>) -> Self {
        self.force = force;
        self
    }
    #[doc = "Should this request wait until the operation has completed before returning"]
    pub fn wait_for_completion(mut self, wait_for_completion: Option<bool>) -> Self {
        self.wait_for_completion = wait_for_completion;
        self
    }
}
impl Sender for MlDeleteJob {
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
pub struct MlDeleteModelSnapshot {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteModelSnapshot {
    pub fn new(client: Elasticsearch) -> Self {
        MlDeleteModelSnapshot {
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
impl Sender for MlDeleteModelSnapshot {
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
pub struct MlEvaluateDataFrame {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlEvaluateDataFrame {
    pub fn new(client: Elasticsearch) -> Self {
        MlEvaluateDataFrame {
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
impl Sender for MlEvaluateDataFrame {
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
pub struct MlFindFileStructure {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    charset: Option<String>,
    column_names: Option<Vec<String>>,
    delimiter: Option<String>,
    explain: Option<bool>,
    format: Option<Format>,
    grok_pattern: Option<String>,
    has_header_row: Option<bool>,
    line_merge_size_limit: Option<i32>,
    lines_to_sample: Option<i32>,
    quote: Option<String>,
    should_trim_fields: Option<bool>,
    timeout: Option<String>,
    timestamp_field: Option<String>,
    timestamp_format: Option<String>,
}
impl MlFindFileStructure {
    pub fn new(client: Elasticsearch) -> Self {
        MlFindFileStructure {
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
    #[doc = "Whether to include a commentary on how the structure was derived"]
    pub fn explain(mut self, explain: Option<bool>) -> Self {
        self.explain = explain;
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
impl Sender for MlFindFileStructure {
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
pub struct MlFlushJob {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    advance_time: Option<String>,
    calc_interim: Option<bool>,
    end: Option<String>,
    skip_time: Option<String>,
    start: Option<String>,
}
impl MlFlushJob {
    pub fn new(client: Elasticsearch) -> Self {
        MlFlushJob {
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
    #[doc = "Advances time to the given value generating results and updating the model for the advanced interval"]
    pub fn advance_time(mut self, advance_time: Option<String>) -> Self {
        self.advance_time = advance_time;
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
    #[doc = "Skips time to the given value without generating results or updating the model for the skipped interval"]
    pub fn skip_time(mut self, skip_time: Option<String>) -> Self {
        self.skip_time = skip_time;
        self
    }
    #[doc = "When used in conjunction with calc_interim, specifies the range of buckets on which to calculate interim results"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
}
impl Sender for MlFlushJob {
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
pub struct MlForecast {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    duration: Option<String>,
    expires_in: Option<String>,
}
impl MlForecast {
    pub fn new(client: Elasticsearch) -> Self {
        MlForecast {
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
    #[doc = "The duration of the forecast"]
    pub fn duration(mut self, duration: Option<String>) -> Self {
        self.duration = duration;
        self
    }
    #[doc = "The time interval after which the forecast expires. Expired forecasts will be deleted at the first opportunity."]
    pub fn expires_in(mut self, expires_in: Option<String>) -> Self {
        self.expires_in = expires_in;
        self
    }
}
impl Sender for MlForecast {
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
pub struct MlGetBuckets {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    anomaly_score: Option<f64>,
    desc: Option<bool>,
    end: Option<String>,
    exclude_interim: Option<bool>,
    expand: Option<bool>,
    from: Option<i32>,
    size: Option<i32>,
    sort: Option<String>,
    start: Option<String>,
}
impl MlGetBuckets {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetBuckets {
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
    #[doc = "Filter for the most anomalous buckets"]
    pub fn anomaly_score(mut self, anomaly_score: Option<f64>) -> Self {
        self.anomaly_score = anomaly_score;
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
    #[doc = "skips a number of buckets"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
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
    #[doc = "Start time filter for buckets"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
}
impl Sender for MlGetBuckets {
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
pub struct MlGetCalendarEvents {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    end: Option<String>,
    from: Option<i32>,
    job_id: Option<String>,
    size: Option<i32>,
    start: Option<String>,
}
impl MlGetCalendarEvents {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetCalendarEvents {
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
    #[doc = "Get events before this time"]
    pub fn end(mut self, end: Option<String>) -> Self {
        self.end = end;
        self
    }
    #[doc = "Skips a number of events"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
        self
    }
    #[doc = "Get events for the job. When this option is used calendar_id must be '_all'"]
    pub fn job_id(mut self, job_id: Option<String>) -> Self {
        self.job_id = job_id;
        self
    }
    #[doc = "Specifies a max number of events to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "Get events after this time"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
}
impl Sender for MlGetCalendarEvents {
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
pub struct MlGetCalendars {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    from: Option<i32>,
    size: Option<i32>,
}
impl MlGetCalendars {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetCalendars {
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
    #[doc = "skips a number of calendars"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
        self
    }
    #[doc = "specifies a max number of calendars to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
}
impl Sender for MlGetCalendars {
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
pub struct MlGetCategories {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    from: Option<i32>,
    size: Option<i32>,
}
impl MlGetCategories {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetCategories {
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
    #[doc = "skips a number of categories"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
        self
    }
    #[doc = "specifies a max number of categories to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
}
impl Sender for MlGetCategories {
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
pub struct MlGetDataFrameAnalytics {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_match: Option<bool>,
    from: Option<i32>,
    size: Option<i32>,
}
impl MlGetDataFrameAnalytics {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetDataFrameAnalytics {
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
    #[doc = "Whether to ignore if a wildcard expression matches no data frame analytics. (This includes `_all` string or when no data frame analytics have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: Option<bool>) -> Self {
        self.allow_no_match = allow_no_match;
        self
    }
    #[doc = "skips a number of analytics"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
        self
    }
    #[doc = "specifies a max number of analytics to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
}
impl Sender for MlGetDataFrameAnalytics {
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
pub struct MlGetDataFrameAnalyticsStats {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_match: Option<bool>,
    from: Option<i32>,
    size: Option<i32>,
}
impl MlGetDataFrameAnalyticsStats {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetDataFrameAnalyticsStats {
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
    #[doc = "Whether to ignore if a wildcard expression matches no data frame analytics. (This includes `_all` string or when no data frame analytics have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: Option<bool>) -> Self {
        self.allow_no_match = allow_no_match;
        self
    }
    #[doc = "skips a number of analytics"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
        self
    }
    #[doc = "specifies a max number of analytics to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
}
impl Sender for MlGetDataFrameAnalyticsStats {
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
pub struct MlGetDatafeedStats {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_datafeeds: Option<bool>,
}
impl MlGetDatafeedStats {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetDatafeedStats {
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
    #[doc = "Whether to ignore if a wildcard expression matches no datafeeds. (This includes `_all` string or when no datafeeds have been specified)"]
    pub fn allow_no_datafeeds(mut self, allow_no_datafeeds: Option<bool>) -> Self {
        self.allow_no_datafeeds = allow_no_datafeeds;
        self
    }
}
impl Sender for MlGetDatafeedStats {
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
pub struct MlGetDatafeeds {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_datafeeds: Option<bool>,
}
impl MlGetDatafeeds {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetDatafeeds {
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
    #[doc = "Whether to ignore if a wildcard expression matches no datafeeds. (This includes `_all` string or when no datafeeds have been specified)"]
    pub fn allow_no_datafeeds(mut self, allow_no_datafeeds: Option<bool>) -> Self {
        self.allow_no_datafeeds = allow_no_datafeeds;
        self
    }
}
impl Sender for MlGetDatafeeds {
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
pub struct MlGetFilters {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    from: Option<i32>,
    size: Option<i32>,
}
impl MlGetFilters {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetFilters {
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
    #[doc = "skips a number of filters"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
        self
    }
    #[doc = "specifies a max number of filters to get"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
}
impl Sender for MlGetFilters {
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
pub struct MlGetInfluencers {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    desc: Option<bool>,
    end: Option<String>,
    exclude_interim: Option<bool>,
    from: Option<i32>,
    influencer_score: Option<f64>,
    size: Option<i32>,
    sort: Option<String>,
    start: Option<String>,
}
impl MlGetInfluencers {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetInfluencers {
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
    #[doc = "Exclude interim results"]
    pub fn exclude_interim(mut self, exclude_interim: Option<bool>) -> Self {
        self.exclude_interim = exclude_interim;
        self
    }
    #[doc = "skips a number of influencers"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
        self
    }
    #[doc = "influencer score threshold for the requested influencers"]
    pub fn influencer_score(mut self, influencer_score: Option<f64>) -> Self {
        self.influencer_score = influencer_score;
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
    #[doc = "start timestamp for the requested influencers"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
}
impl Sender for MlGetInfluencers {
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
pub struct MlGetJobStats {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_jobs: Option<bool>,
}
impl MlGetJobStats {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetJobStats {
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
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: Option<bool>) -> Self {
        self.allow_no_jobs = allow_no_jobs;
        self
    }
}
impl Sender for MlGetJobStats {
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
pub struct MlGetJobs {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_jobs: Option<bool>,
}
impl MlGetJobs {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetJobs {
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
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: Option<bool>) -> Self {
        self.allow_no_jobs = allow_no_jobs;
        self
    }
}
impl Sender for MlGetJobs {
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
pub struct MlGetModelSnapshots {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    desc: Option<bool>,
    end: Option<String>,
    from: Option<i32>,
    size: Option<i32>,
    sort: Option<String>,
    start: Option<String>,
}
impl MlGetModelSnapshots {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetModelSnapshots {
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
    #[doc = "Skips a number of documents"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
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
    #[doc = "The filter 'start' query parameter"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
}
impl Sender for MlGetModelSnapshots {
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
pub struct MlGetOverallBuckets {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_jobs: Option<bool>,
    bucket_span: Option<String>,
    end: Option<String>,
    exclude_interim: Option<bool>,
    overall_score: Option<f64>,
    start: Option<String>,
    top_n: Option<i32>,
}
impl MlGetOverallBuckets {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetOverallBuckets {
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
    #[doc = "Whether to ignore if a wildcard expression matches no jobs. (This includes `_all` string or when no jobs have been specified)"]
    pub fn allow_no_jobs(mut self, allow_no_jobs: Option<bool>) -> Self {
        self.allow_no_jobs = allow_no_jobs;
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
    #[doc = "If true overall buckets that include interim buckets will be excluded"]
    pub fn exclude_interim(mut self, exclude_interim: Option<bool>) -> Self {
        self.exclude_interim = exclude_interim;
        self
    }
    #[doc = "Returns overall buckets with overall scores higher than this value"]
    pub fn overall_score(mut self, overall_score: Option<f64>) -> Self {
        self.overall_score = overall_score;
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
impl Sender for MlGetOverallBuckets {
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
pub struct MlGetRecords {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    desc: Option<bool>,
    end: Option<String>,
    exclude_interim: Option<bool>,
    from: Option<i32>,
    record_score: Option<f64>,
    size: Option<i32>,
    sort: Option<String>,
    start: Option<String>,
}
impl MlGetRecords {
    pub fn new(client: Elasticsearch) -> Self {
        MlGetRecords {
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
    #[doc = "Exclude interim results"]
    pub fn exclude_interim(mut self, exclude_interim: Option<bool>) -> Self {
        self.exclude_interim = exclude_interim;
        self
    }
    #[doc = "skips a number of records"]
    pub fn from(mut self, from: Option<i32>) -> Self {
        self.from = from;
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
    #[doc = "Start time filter for records"]
    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start;
        self
    }
}
impl Sender for MlGetRecords {
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
impl Sender for MlInfo {
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
pub struct MlOpenJob {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlOpenJob {
    pub fn new(client: Elasticsearch) -> Self {
        MlOpenJob {
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
impl Sender for MlOpenJob {
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
pub struct MlPostCalendarEvents {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPostCalendarEvents {
    pub fn new(client: Elasticsearch) -> Self {
        MlPostCalendarEvents {
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
impl Sender for MlPostCalendarEvents {
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
pub struct MlPostData {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    reset_end: Option<String>,
    reset_start: Option<String>,
}
impl MlPostData {
    pub fn new(client: Elasticsearch) -> Self {
        MlPostData {
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
}
impl Sender for MlPostData {
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
pub struct MlPreviewDatafeed {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPreviewDatafeed {
    pub fn new(client: Elasticsearch) -> Self {
        MlPreviewDatafeed {
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
impl Sender for MlPreviewDatafeed {
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
pub struct MlPutCalendar {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPutCalendar {
    pub fn new(client: Elasticsearch) -> Self {
        MlPutCalendar {
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
impl Sender for MlPutCalendar {
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
pub struct MlPutCalendarJob {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPutCalendarJob {
    pub fn new(client: Elasticsearch) -> Self {
        MlPutCalendarJob {
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
impl Sender for MlPutCalendarJob {
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
pub struct MlPutDataFrameAnalytics {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPutDataFrameAnalytics {
    pub fn new(client: Elasticsearch) -> Self {
        MlPutDataFrameAnalytics {
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
impl Sender for MlPutDataFrameAnalytics {
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
pub struct MlPutDatafeed {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPutDatafeed {
    pub fn new(client: Elasticsearch) -> Self {
        MlPutDatafeed {
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
impl Sender for MlPutDatafeed {
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
pub struct MlPutFilter {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPutFilter {
    pub fn new(client: Elasticsearch) -> Self {
        MlPutFilter {
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
impl Sender for MlPutFilter {
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
pub struct MlPutJob {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPutJob {
    pub fn new(client: Elasticsearch) -> Self {
        MlPutJob {
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
impl Sender for MlPutJob {
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
pub struct MlRevertModelSnapshot {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    delete_intervening_results: Option<bool>,
}
impl MlRevertModelSnapshot {
    pub fn new(client: Elasticsearch) -> Self {
        MlRevertModelSnapshot {
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
    #[doc = "Should we reset the results back to the time of the snapshot?"]
    pub fn delete_intervening_results(mut self, delete_intervening_results: Option<bool>) -> Self {
        self.delete_intervening_results = delete_intervening_results;
        self
    }
}
impl Sender for MlRevertModelSnapshot {
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
pub struct MlSetUpgradeMode {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    enabled: Option<bool>,
    timeout: Option<String>,
}
impl MlSetUpgradeMode {
    pub fn new(client: Elasticsearch) -> Self {
        MlSetUpgradeMode {
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
    #[doc = "Whether to enable upgrade_mode ML setting or not. Defaults to false."]
    pub fn enabled(mut self, enabled: Option<bool>) -> Self {
        self.enabled = enabled;
        self
    }
    #[doc = "Controls the time to wait before action times out. Defaults to 30 seconds"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for MlSetUpgradeMode {
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
pub struct MlStartDataFrameAnalytics {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl MlStartDataFrameAnalytics {
    pub fn new(client: Elasticsearch) -> Self {
        MlStartDataFrameAnalytics {
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
    #[doc = "Controls the time to wait until the task has started. Defaults to 20 seconds"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for MlStartDataFrameAnalytics {
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
pub struct MlStartDatafeed {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    end: Option<String>,
    start: Option<String>,
    timeout: Option<String>,
}
impl MlStartDatafeed {
    pub fn new(client: Elasticsearch) -> Self {
        MlStartDatafeed {
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
    #[doc = "The end time when the datafeed should stop. When not set, the datafeed continues in real time"]
    pub fn end(mut self, end: Option<String>) -> Self {
        self.end = end;
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
impl Sender for MlStartDatafeed {
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
pub struct MlStopDataFrameAnalytics {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_match: Option<bool>,
    force: Option<bool>,
    timeout: Option<String>,
}
impl MlStopDataFrameAnalytics {
    pub fn new(client: Elasticsearch) -> Self {
        MlStopDataFrameAnalytics {
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
    #[doc = "Whether to ignore if a wildcard expression matches no data frame analytics. (This includes `_all` string or when no data frame analytics have been specified)"]
    pub fn allow_no_match(mut self, allow_no_match: Option<bool>) -> Self {
        self.allow_no_match = allow_no_match;
        self
    }
    #[doc = "True if the data frame analytics should be forcefully stopped"]
    pub fn force(mut self, force: Option<bool>) -> Self {
        self.force = force;
        self
    }
    #[doc = "Controls the time to wait until the task has stopped. Defaults to 20 seconds"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for MlStopDataFrameAnalytics {
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
pub struct MlStopDatafeed {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_datafeeds: Option<bool>,
    force: Option<bool>,
    timeout: Option<String>,
}
impl MlStopDatafeed {
    pub fn new(client: Elasticsearch) -> Self {
        MlStopDatafeed {
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
    #[doc = "Whether to ignore if a wildcard expression matches no datafeeds. (This includes `_all` string or when no datafeeds have been specified)"]
    pub fn allow_no_datafeeds(mut self, allow_no_datafeeds: Option<bool>) -> Self {
        self.allow_no_datafeeds = allow_no_datafeeds;
        self
    }
    #[doc = "True if the datafeed should be forcefully stopped."]
    pub fn force(mut self, force: Option<bool>) -> Self {
        self.force = force;
        self
    }
    #[doc = "Controls the time to wait until a datafeed has stopped. Default to 20 seconds"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for MlStopDatafeed {
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
pub struct MlUpdateDatafeed {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlUpdateDatafeed {
    pub fn new(client: Elasticsearch) -> Self {
        MlUpdateDatafeed {
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
impl Sender for MlUpdateDatafeed {
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
pub struct MlUpdateFilter {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlUpdateFilter {
    pub fn new(client: Elasticsearch) -> Self {
        MlUpdateFilter {
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
impl Sender for MlUpdateFilter {
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
pub struct MlUpdateJob {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlUpdateJob {
    pub fn new(client: Elasticsearch) -> Self {
        MlUpdateJob {
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
impl Sender for MlUpdateJob {
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
pub struct MlUpdateModelSnapshot {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlUpdateModelSnapshot {
    pub fn new(client: Elasticsearch) -> Self {
        MlUpdateModelSnapshot {
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
impl Sender for MlUpdateModelSnapshot {
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
pub struct MlValidate {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlValidate {
    pub fn new(client: Elasticsearch) -> Self {
        MlValidate {
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
impl Sender for MlValidate {
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
pub struct MlValidateDetector {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlValidateDetector {
    pub fn new(client: Elasticsearch) -> Self {
        MlValidateDetector {
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
impl Sender for MlValidateDetector {
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
#[doc = "Ml APIs"]
pub struct Ml {
    client: Elasticsearch,
}
impl Ml {
    pub fn new(client: Elasticsearch) -> Self {
        Ml { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-close-job.html"]
    pub fn close_job(&self) -> MlCloseJob {
        MlCloseJob::new(self.client.clone())
    }
    pub fn delete_calendar(&self) -> MlDeleteCalendar {
        MlDeleteCalendar::new(self.client.clone())
    }
    pub fn delete_calendar_event(&self) -> MlDeleteCalendarEvent {
        MlDeleteCalendarEvent::new(self.client.clone())
    }
    pub fn delete_calendar_job(&self) -> MlDeleteCalendarJob {
        MlDeleteCalendarJob::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/delete-dfanalytics.html"]
    pub fn delete_data_frame_analytics(&self) -> MlDeleteDataFrameAnalytics {
        MlDeleteDataFrameAnalytics::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-datafeed.html"]
    pub fn delete_datafeed(&self) -> MlDeleteDatafeed {
        MlDeleteDatafeed::new(self.client.clone())
    }
    pub fn delete_expired_data(&self) -> MlDeleteExpiredData {
        MlDeleteExpiredData::new(self.client.clone())
    }
    pub fn delete_filter(&self) -> MlDeleteFilter {
        MlDeleteFilter::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-forecast.html"]
    pub fn delete_forecast(&self) -> MlDeleteForecast {
        MlDeleteForecast::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-job.html"]
    pub fn delete_job(&self) -> MlDeleteJob {
        MlDeleteJob::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-snapshot.html"]
    pub fn delete_model_snapshot(&self) -> MlDeleteModelSnapshot {
        MlDeleteModelSnapshot::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/evaluate-dfanalytics.html"]
    pub fn evaluate_data_frame(&self) -> MlEvaluateDataFrame {
        MlEvaluateDataFrame::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-find-file-structure.html"]
    pub fn find_file_structure(&self) -> MlFindFileStructure {
        MlFindFileStructure::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-flush-job.html"]
    pub fn flush_job(&self) -> MlFlushJob {
        MlFlushJob::new(self.client.clone())
    }
    pub fn forecast(&self) -> MlForecast {
        MlForecast::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-bucket.html"]
    pub fn get_buckets(&self) -> MlGetBuckets {
        MlGetBuckets::new(self.client.clone())
    }
    pub fn get_calendar_events(&self) -> MlGetCalendarEvents {
        MlGetCalendarEvents::new(self.client.clone())
    }
    pub fn get_calendars(&self) -> MlGetCalendars {
        MlGetCalendars::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-category.html"]
    pub fn get_categories(&self) -> MlGetCategories {
        MlGetCategories::new(self.client.clone())
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
    pub fn get_influencers(&self) -> MlGetInfluencers {
        MlGetInfluencers::new(self.client.clone())
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
    pub fn get_model_snapshots(&self) -> MlGetModelSnapshots {
        MlGetModelSnapshots::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-overall-buckets.html"]
    pub fn get_overall_buckets(&self) -> MlGetOverallBuckets {
        MlGetOverallBuckets::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-record.html"]
    pub fn get_records(&self) -> MlGetRecords {
        MlGetRecords::new(self.client.clone())
    }
    pub fn info(&self) -> MlInfo {
        MlInfo::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-open-job.html"]
    pub fn open_job(&self) -> MlOpenJob {
        MlOpenJob::new(self.client.clone())
    }
    pub fn post_calendar_events(&self) -> MlPostCalendarEvents {
        MlPostCalendarEvents::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-post-data.html"]
    pub fn post_data(&self) -> MlPostData {
        MlPostData::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-preview-datafeed.html"]
    pub fn preview_datafeed(&self) -> MlPreviewDatafeed {
        MlPreviewDatafeed::new(self.client.clone())
    }
    pub fn put_calendar(&self) -> MlPutCalendar {
        MlPutCalendar::new(self.client.clone())
    }
    pub fn put_calendar_job(&self) -> MlPutCalendarJob {
        MlPutCalendarJob::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/put-dfanalytics.html"]
    pub fn put_data_frame_analytics(&self) -> MlPutDataFrameAnalytics {
        MlPutDataFrameAnalytics::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-datafeed.html"]
    pub fn put_datafeed(&self) -> MlPutDatafeed {
        MlPutDatafeed::new(self.client.clone())
    }
    pub fn put_filter(&self) -> MlPutFilter {
        MlPutFilter::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-job.html"]
    pub fn put_job(&self) -> MlPutJob {
        MlPutJob::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-revert-snapshot.html"]
    pub fn revert_model_snapshot(&self) -> MlRevertModelSnapshot {
        MlRevertModelSnapshot::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-set-upgrade-mode.html"]
    pub fn set_upgrade_mode(&self) -> MlSetUpgradeMode {
        MlSetUpgradeMode::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/start-dfanalytics.html"]
    pub fn start_data_frame_analytics(&self) -> MlStartDataFrameAnalytics {
        MlStartDataFrameAnalytics::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-start-datafeed.html"]
    pub fn start_datafeed(&self) -> MlStartDatafeed {
        MlStartDatafeed::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/stop-dfanalytics.html"]
    pub fn stop_data_frame_analytics(&self) -> MlStopDataFrameAnalytics {
        MlStopDataFrameAnalytics::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-stop-datafeed.html"]
    pub fn stop_datafeed(&self) -> MlStopDatafeed {
        MlStopDatafeed::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-datafeed.html"]
    pub fn update_datafeed(&self) -> MlUpdateDatafeed {
        MlUpdateDatafeed::new(self.client.clone())
    }
    pub fn update_filter(&self) -> MlUpdateFilter {
        MlUpdateFilter::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-job.html"]
    pub fn update_job(&self) -> MlUpdateJob {
        MlUpdateJob::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-snapshot.html"]
    pub fn update_model_snapshot(&self) -> MlUpdateModelSnapshot {
        MlUpdateModelSnapshot::new(self.client.clone())
    }
    pub fn validate(&self) -> MlValidate {
        MlValidate::new(self.client.clone())
    }
    pub fn validate_detector(&self) -> MlValidateDetector {
        MlValidateDetector::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Ml APIs"]
    pub fn ml(&self) -> Ml {
        Ml::new(self.clone())
    }
}
