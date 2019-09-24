

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct MlCloseJobBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_jobs: Option<bool>,
    force: Option<bool>,
    timeout: Option<String>,
}
impl MlCloseJobBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlCloseJobBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlCloseJobBuilder {
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
pub struct MlDeleteCalendarBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteCalendarBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlDeleteCalendarBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlDeleteCalendarBuilder {
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
pub struct MlDeleteCalendarEventBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteCalendarEventBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlDeleteCalendarEventBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlDeleteCalendarEventBuilder {
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
pub struct MlDeleteCalendarJobBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteCalendarJobBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlDeleteCalendarJobBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlDeleteCalendarJobBuilder {
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
pub struct MlDeleteDataFrameAnalyticsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteDataFrameAnalyticsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlDeleteDataFrameAnalyticsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlDeleteDataFrameAnalyticsBuilder {
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
pub struct MlDeleteDatafeedBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    force: Option<bool>,
}
impl MlDeleteDatafeedBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlDeleteDatafeedBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlDeleteDatafeedBuilder {
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
pub struct MlDeleteExpiredDataBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteExpiredDataBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlDeleteExpiredDataBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlDeleteExpiredDataBuilder {
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
pub struct MlDeleteFilterBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteFilterBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlDeleteFilterBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlDeleteFilterBuilder {
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
pub struct MlDeleteForecastBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_forecasts: Option<bool>,
    timeout: Option<String>,
}
impl MlDeleteForecastBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlDeleteForecastBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlDeleteForecastBuilder {
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
pub struct MlDeleteJobBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    force: Option<bool>,
    wait_for_completion: Option<bool>,
}
impl MlDeleteJobBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlDeleteJobBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlDeleteJobBuilder {
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
pub struct MlDeleteModelSnapshotBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlDeleteModelSnapshotBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlDeleteModelSnapshotBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlDeleteModelSnapshotBuilder {
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
pub struct MlEvaluateDataFrameBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlEvaluateDataFrameBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlEvaluateDataFrameBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlEvaluateDataFrameBuilder {
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
pub struct MlFindFileStructureBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    charset: Option<String>,
    column_names: Option<Vec<String>>,
    delimiter: Option<String>,
    explain: Option<bool>,
    format: Option<i32>,
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
impl MlFindFileStructureBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlFindFileStructureBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlFindFileStructureBuilder {
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
pub struct MlFlushJobBuilder {
    client: ElasticsearchClient,
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
impl MlFlushJobBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlFlushJobBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlFlushJobBuilder {
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
pub struct MlForecastBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    duration: Option<String>,
    expires_in: Option<String>,
}
impl MlForecastBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlForecastBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlForecastBuilder {
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
pub struct MlGetBucketsBuilder {
    client: ElasticsearchClient,
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
impl MlGetBucketsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetBucketsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetBucketsBuilder {
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
pub struct MlGetCalendarEventsBuilder {
    client: ElasticsearchClient,
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
impl MlGetCalendarEventsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetCalendarEventsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetCalendarEventsBuilder {
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
pub struct MlGetCalendarsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    from: Option<i32>,
    size: Option<i32>,
}
impl MlGetCalendarsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetCalendarsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetCalendarsBuilder {
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
pub struct MlGetCategoriesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    from: Option<i32>,
    size: Option<i32>,
}
impl MlGetCategoriesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetCategoriesBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetCategoriesBuilder {
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
pub struct MlGetDataFrameAnalyticsBuilder {
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
impl MlGetDataFrameAnalyticsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetDataFrameAnalyticsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetDataFrameAnalyticsBuilder {
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
pub struct MlGetDataFrameAnalyticsStatsBuilder {
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
impl MlGetDataFrameAnalyticsStatsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetDataFrameAnalyticsStatsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetDataFrameAnalyticsStatsBuilder {
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
pub struct MlGetDatafeedStatsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_datafeeds: Option<bool>,
}
impl MlGetDatafeedStatsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetDatafeedStatsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetDatafeedStatsBuilder {
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
pub struct MlGetDatafeedsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_datafeeds: Option<bool>,
}
impl MlGetDatafeedsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetDatafeedsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetDatafeedsBuilder {
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
pub struct MlGetFiltersBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    from: Option<i32>,
    size: Option<i32>,
}
impl MlGetFiltersBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetFiltersBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetFiltersBuilder {
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
pub struct MlGetInfluencersBuilder {
    client: ElasticsearchClient,
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
impl MlGetInfluencersBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetInfluencersBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetInfluencersBuilder {
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
pub struct MlGetJobStatsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_jobs: Option<bool>,
}
impl MlGetJobStatsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetJobStatsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetJobStatsBuilder {
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
pub struct MlGetJobsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_jobs: Option<bool>,
}
impl MlGetJobsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetJobsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetJobsBuilder {
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
pub struct MlGetModelSnapshotsBuilder {
    client: ElasticsearchClient,
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
impl MlGetModelSnapshotsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetModelSnapshotsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetModelSnapshotsBuilder {
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
pub struct MlGetOverallBucketsBuilder {
    client: ElasticsearchClient,
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
impl MlGetOverallBucketsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetOverallBucketsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetOverallBucketsBuilder {
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
pub struct MlGetRecordsBuilder {
    client: ElasticsearchClient,
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
impl MlGetRecordsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlGetRecordsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlGetRecordsBuilder {
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
pub struct MlInfoBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlInfoBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlInfoBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlInfoBuilder {
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
pub struct MlOpenJobBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlOpenJobBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlOpenJobBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlOpenJobBuilder {
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
pub struct MlPostCalendarEventsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPostCalendarEventsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlPostCalendarEventsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlPostCalendarEventsBuilder {
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
pub struct MlPostDataBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    reset_end: Option<String>,
    reset_start: Option<String>,
}
impl MlPostDataBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlPostDataBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlPostDataBuilder {
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
pub struct MlPreviewDatafeedBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPreviewDatafeedBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlPreviewDatafeedBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlPreviewDatafeedBuilder {
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
pub struct MlPutCalendarBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPutCalendarBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlPutCalendarBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlPutCalendarBuilder {
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
pub struct MlPutCalendarJobBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPutCalendarJobBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlPutCalendarJobBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlPutCalendarJobBuilder {
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
pub struct MlPutDataFrameAnalyticsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPutDataFrameAnalyticsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlPutDataFrameAnalyticsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlPutDataFrameAnalyticsBuilder {
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
pub struct MlPutDatafeedBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPutDatafeedBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlPutDatafeedBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlPutDatafeedBuilder {
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
pub struct MlPutFilterBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPutFilterBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlPutFilterBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlPutFilterBuilder {
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
pub struct MlPutJobBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlPutJobBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlPutJobBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlPutJobBuilder {
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
pub struct MlRevertModelSnapshotBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    delete_intervening_results: Option<bool>,
}
impl MlRevertModelSnapshotBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlRevertModelSnapshotBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlRevertModelSnapshotBuilder {
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
pub struct MlSetUpgradeModeBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    enabled: Option<bool>,
    timeout: Option<String>,
}
impl MlSetUpgradeModeBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlSetUpgradeModeBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlSetUpgradeModeBuilder {
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
pub struct MlStartDataFrameAnalyticsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl MlStartDataFrameAnalyticsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlStartDataFrameAnalyticsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlStartDataFrameAnalyticsBuilder {
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
pub struct MlStartDatafeedBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    end: Option<String>,
    start: Option<String>,
    timeout: Option<String>,
}
impl MlStartDatafeedBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlStartDatafeedBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlStartDatafeedBuilder {
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
pub struct MlStopDataFrameAnalyticsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_match: Option<bool>,
    force: Option<bool>,
    timeout: Option<String>,
}
impl MlStopDataFrameAnalyticsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlStopDataFrameAnalyticsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlStopDataFrameAnalyticsBuilder {
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
pub struct MlStopDatafeedBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    allow_no_datafeeds: Option<bool>,
    force: Option<bool>,
    timeout: Option<String>,
}
impl MlStopDatafeedBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlStopDatafeedBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlStopDatafeedBuilder {
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
pub struct MlUpdateDatafeedBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlUpdateDatafeedBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlUpdateDatafeedBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlUpdateDatafeedBuilder {
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
pub struct MlUpdateFilterBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlUpdateFilterBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlUpdateFilterBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlUpdateFilterBuilder {
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
pub struct MlUpdateJobBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlUpdateJobBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlUpdateJobBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlUpdateJobBuilder {
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
pub struct MlUpdateModelSnapshotBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlUpdateModelSnapshotBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlUpdateModelSnapshotBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlUpdateModelSnapshotBuilder {
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
pub struct MlValidateBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlValidateBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlValidateBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlValidateBuilder {
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
pub struct MlValidateDetectorBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl MlValidateDetectorBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlValidateDetectorBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for MlValidateDetectorBuilder {
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
pub struct MlClient {
    client: ElasticsearchClient,
}
impl MlClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        MlClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-close-job.html"]
    pub fn close_job(&self) -> MlCloseJobBuilder {
        MlCloseJobBuilder::default()
    }
    pub fn delete_calendar(&self) -> MlDeleteCalendarBuilder {
        MlDeleteCalendarBuilder::default()
    }
    pub fn delete_calendar_event(&self) -> MlDeleteCalendarEventBuilder {
        MlDeleteCalendarEventBuilder::default()
    }
    pub fn delete_calendar_job(&self) -> MlDeleteCalendarJobBuilder {
        MlDeleteCalendarJobBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/delete-dfanalytics.html"]
    pub fn delete_data_frame_analytics(&self) -> MlDeleteDataFrameAnalyticsBuilder {
        MlDeleteDataFrameAnalyticsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-datafeed.html"]
    pub fn delete_datafeed(&self) -> MlDeleteDatafeedBuilder {
        MlDeleteDatafeedBuilder::default()
    }
    pub fn delete_expired_data(&self) -> MlDeleteExpiredDataBuilder {
        MlDeleteExpiredDataBuilder::default()
    }
    pub fn delete_filter(&self) -> MlDeleteFilterBuilder {
        MlDeleteFilterBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-forecast.html"]
    pub fn delete_forecast(&self) -> MlDeleteForecastBuilder {
        MlDeleteForecastBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-job.html"]
    pub fn delete_job(&self) -> MlDeleteJobBuilder {
        MlDeleteJobBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-snapshot.html"]
    pub fn delete_model_snapshot(&self) -> MlDeleteModelSnapshotBuilder {
        MlDeleteModelSnapshotBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/evaluate-dfanalytics.html"]
    pub fn evaluate_data_frame(&self) -> MlEvaluateDataFrameBuilder {
        MlEvaluateDataFrameBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-find-file-structure.html"]
    pub fn find_file_structure(&self) -> MlFindFileStructureBuilder {
        MlFindFileStructureBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-flush-job.html"]
    pub fn flush_job(&self) -> MlFlushJobBuilder {
        MlFlushJobBuilder::default()
    }
    pub fn forecast(&self) -> MlForecastBuilder {
        MlForecastBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-bucket.html"]
    pub fn get_buckets(&self) -> MlGetBucketsBuilder {
        MlGetBucketsBuilder::default()
    }
    pub fn get_calendar_events(&self) -> MlGetCalendarEventsBuilder {
        MlGetCalendarEventsBuilder::default()
    }
    pub fn get_calendars(&self) -> MlGetCalendarsBuilder {
        MlGetCalendarsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-category.html"]
    pub fn get_categories(&self) -> MlGetCategoriesBuilder {
        MlGetCategoriesBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/get-dfanalytics.html"]
    pub fn get_data_frame_analytics(&self) -> MlGetDataFrameAnalyticsBuilder {
        MlGetDataFrameAnalyticsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/get-dfanalytics-stats.html"]
    pub fn get_data_frame_analytics_stats(&self) -> MlGetDataFrameAnalyticsStatsBuilder {
        MlGetDataFrameAnalyticsStatsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-datafeed-stats.html"]
    pub fn get_datafeed_stats(&self) -> MlGetDatafeedStatsBuilder {
        MlGetDatafeedStatsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-datafeed.html"]
    pub fn get_datafeeds(&self) -> MlGetDatafeedsBuilder {
        MlGetDatafeedsBuilder::default()
    }
    pub fn get_filters(&self) -> MlGetFiltersBuilder {
        MlGetFiltersBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-influencer.html"]
    pub fn get_influencers(&self) -> MlGetInfluencersBuilder {
        MlGetInfluencersBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-job-stats.html"]
    pub fn get_job_stats(&self) -> MlGetJobStatsBuilder {
        MlGetJobStatsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-job.html"]
    pub fn get_jobs(&self) -> MlGetJobsBuilder {
        MlGetJobsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-snapshot.html"]
    pub fn get_model_snapshots(&self) -> MlGetModelSnapshotsBuilder {
        MlGetModelSnapshotsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-overall-buckets.html"]
    pub fn get_overall_buckets(&self) -> MlGetOverallBucketsBuilder {
        MlGetOverallBucketsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-record.html"]
    pub fn get_records(&self) -> MlGetRecordsBuilder {
        MlGetRecordsBuilder::default()
    }
    pub fn info(&self) -> MlInfoBuilder {
        MlInfoBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-open-job.html"]
    pub fn open_job(&self) -> MlOpenJobBuilder {
        MlOpenJobBuilder::default()
    }
    pub fn post_calendar_events(&self) -> MlPostCalendarEventsBuilder {
        MlPostCalendarEventsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-post-data.html"]
    pub fn post_data(&self) -> MlPostDataBuilder {
        MlPostDataBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-preview-datafeed.html"]
    pub fn preview_datafeed(&self) -> MlPreviewDatafeedBuilder {
        MlPreviewDatafeedBuilder::default()
    }
    pub fn put_calendar(&self) -> MlPutCalendarBuilder {
        MlPutCalendarBuilder::default()
    }
    pub fn put_calendar_job(&self) -> MlPutCalendarJobBuilder {
        MlPutCalendarJobBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/put-dfanalytics.html"]
    pub fn put_data_frame_analytics(&self) -> MlPutDataFrameAnalyticsBuilder {
        MlPutDataFrameAnalyticsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-datafeed.html"]
    pub fn put_datafeed(&self) -> MlPutDatafeedBuilder {
        MlPutDatafeedBuilder::default()
    }
    pub fn put_filter(&self) -> MlPutFilterBuilder {
        MlPutFilterBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-job.html"]
    pub fn put_job(&self) -> MlPutJobBuilder {
        MlPutJobBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-revert-snapshot.html"]
    pub fn revert_model_snapshot(&self) -> MlRevertModelSnapshotBuilder {
        MlRevertModelSnapshotBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-set-upgrade-mode.html"]
    pub fn set_upgrade_mode(&self) -> MlSetUpgradeModeBuilder {
        MlSetUpgradeModeBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/start-dfanalytics.html"]
    pub fn start_data_frame_analytics(&self) -> MlStartDataFrameAnalyticsBuilder {
        MlStartDataFrameAnalyticsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-start-datafeed.html"]
    pub fn start_datafeed(&self) -> MlStartDatafeedBuilder {
        MlStartDatafeedBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/stop-dfanalytics.html"]
    pub fn stop_data_frame_analytics(&self) -> MlStopDataFrameAnalyticsBuilder {
        MlStopDataFrameAnalyticsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-stop-datafeed.html"]
    pub fn stop_datafeed(&self) -> MlStopDatafeedBuilder {
        MlStopDatafeedBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-datafeed.html"]
    pub fn update_datafeed(&self) -> MlUpdateDatafeedBuilder {
        MlUpdateDatafeedBuilder::default()
    }
    pub fn update_filter(&self) -> MlUpdateFilterBuilder {
        MlUpdateFilterBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-job.html"]
    pub fn update_job(&self) -> MlUpdateJobBuilder {
        MlUpdateJobBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-snapshot.html"]
    pub fn update_model_snapshot(&self) -> MlUpdateModelSnapshotBuilder {
        MlUpdateModelSnapshotBuilder::default()
    }
    pub fn validate(&self) -> MlValidateBuilder {
        MlValidateBuilder::default()
    }
    pub fn validate_detector(&self) -> MlValidateDetectorBuilder {
        MlValidateDetectorBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Ml APIs"]
    pub fn ml(&self) -> MlClient {
        MlClient::new(self.clone())
    }
}
