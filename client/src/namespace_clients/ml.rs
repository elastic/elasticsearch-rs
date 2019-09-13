

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct MlCloseJobRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_jobs: Option<&'a bool>,
    force: Option<&'a bool>,
    timeout: &'a str,
}
impl<'a> MlCloseJobRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlCloseJobRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlCloseJobRequestBuilder<'a> {
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
pub struct MlDeleteCalendarRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlDeleteCalendarRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlDeleteCalendarRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlDeleteCalendarRequestBuilder<'a> {
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
pub struct MlDeleteCalendarEventRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlDeleteCalendarEventRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlDeleteCalendarEventRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlDeleteCalendarEventRequestBuilder<'a> {
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
pub struct MlDeleteCalendarJobRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlDeleteCalendarJobRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlDeleteCalendarJobRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlDeleteCalendarJobRequestBuilder<'a> {
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
pub struct MlDeleteDataFrameAnalyticsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlDeleteDataFrameAnalyticsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlDeleteDataFrameAnalyticsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlDeleteDataFrameAnalyticsRequestBuilder<'a> {
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
pub struct MlDeleteDatafeedRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    force: Option<&'a bool>,
}
impl<'a> MlDeleteDatafeedRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlDeleteDatafeedRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlDeleteDatafeedRequestBuilder<'a> {
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
pub struct MlDeleteExpiredDataRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlDeleteExpiredDataRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlDeleteExpiredDataRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlDeleteExpiredDataRequestBuilder<'a> {
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
pub struct MlDeleteFilterRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlDeleteFilterRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlDeleteFilterRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlDeleteFilterRequestBuilder<'a> {
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
pub struct MlDeleteForecastRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_forecasts: Option<&'a bool>,
    timeout: &'a str,
}
impl<'a> MlDeleteForecastRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlDeleteForecastRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlDeleteForecastRequestBuilder<'a> {
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
pub struct MlDeleteJobRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    force: Option<&'a bool>,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> MlDeleteJobRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlDeleteJobRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlDeleteJobRequestBuilder<'a> {
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
pub struct MlDeleteModelSnapshotRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlDeleteModelSnapshotRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlDeleteModelSnapshotRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlDeleteModelSnapshotRequestBuilder<'a> {
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
pub struct MlEvaluateDataFrameRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlEvaluateDataFrameRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlEvaluateDataFrameRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlEvaluateDataFrameRequestBuilder<'a> {
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
pub struct MlFindFileStructureRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    charset: &'a str,
    column_names: Option<&'a Vec<String>>,
    delimiter: &'a str,
    explain: Option<&'a bool>,
    format: Option<&'a i32>,
    grok_pattern: &'a str,
    has_header_row: Option<&'a bool>,
    line_merge_size_limit: Option<&'a i32>,
    lines_to_sample: Option<&'a i32>,
    quote: &'a str,
    should_trim_fields: Option<&'a bool>,
    timeout: &'a str,
    timestamp_field: &'a str,
    timestamp_format: &'a str,
}
impl<'a> MlFindFileStructureRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlFindFileStructureRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlFindFileStructureRequestBuilder<'a> {
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
pub struct MlFlushJobRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    advance_time: &'a str,
    calc_interim: Option<&'a bool>,
    end: &'a str,
    skip_time: &'a str,
    start: &'a str,
}
impl<'a> MlFlushJobRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlFlushJobRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlFlushJobRequestBuilder<'a> {
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
pub struct MlForecastRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    duration: &'a str,
    expires_in: &'a str,
}
impl<'a> MlForecastRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlForecastRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlForecastRequestBuilder<'a> {
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
pub struct MlGetBucketsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    anomaly_score: Option<&'a f64>,
    desc: Option<&'a bool>,
    end: &'a str,
    exclude_interim: Option<&'a bool>,
    expand: Option<&'a bool>,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
    sort: &'a str,
    start: &'a str,
}
impl<'a> MlGetBucketsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetBucketsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetBucketsRequestBuilder<'a> {
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
pub struct MlGetCalendarEventsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    end: &'a str,
    from: Option<&'a i32>,
    job_id: &'a str,
    size: Option<&'a i32>,
    start: &'a str,
}
impl<'a> MlGetCalendarEventsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetCalendarEventsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetCalendarEventsRequestBuilder<'a> {
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
pub struct MlGetCalendarsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
impl<'a> MlGetCalendarsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetCalendarsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetCalendarsRequestBuilder<'a> {
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
pub struct MlGetCategoriesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
impl<'a> MlGetCategoriesRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetCategoriesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetCategoriesRequestBuilder<'a> {
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
pub struct MlGetDataFrameAnalyticsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_match: Option<&'a bool>,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
impl<'a> MlGetDataFrameAnalyticsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetDataFrameAnalyticsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetDataFrameAnalyticsRequestBuilder<'a> {
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
pub struct MlGetDataFrameAnalyticsStatsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_match: Option<&'a bool>,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
impl<'a> MlGetDataFrameAnalyticsStatsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetDataFrameAnalyticsStatsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetDataFrameAnalyticsStatsRequestBuilder<'a> {
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
pub struct MlGetDatafeedStatsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_datafeeds: Option<&'a bool>,
}
impl<'a> MlGetDatafeedStatsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetDatafeedStatsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetDatafeedStatsRequestBuilder<'a> {
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
pub struct MlGetDatafeedsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_datafeeds: Option<&'a bool>,
}
impl<'a> MlGetDatafeedsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetDatafeedsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetDatafeedsRequestBuilder<'a> {
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
pub struct MlGetFiltersRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
impl<'a> MlGetFiltersRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetFiltersRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetFiltersRequestBuilder<'a> {
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
pub struct MlGetInfluencersRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    desc: Option<&'a bool>,
    end: &'a str,
    exclude_interim: Option<&'a bool>,
    from: Option<&'a i32>,
    influencer_score: Option<&'a f64>,
    size: Option<&'a i32>,
    sort: &'a str,
    start: &'a str,
}
impl<'a> MlGetInfluencersRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetInfluencersRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetInfluencersRequestBuilder<'a> {
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
pub struct MlGetJobStatsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_jobs: Option<&'a bool>,
}
impl<'a> MlGetJobStatsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetJobStatsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetJobStatsRequestBuilder<'a> {
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
pub struct MlGetJobsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_jobs: Option<&'a bool>,
}
impl<'a> MlGetJobsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetJobsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetJobsRequestBuilder<'a> {
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
pub struct MlGetModelSnapshotsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    desc: Option<&'a bool>,
    end: &'a str,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
    sort: &'a str,
    start: &'a str,
}
impl<'a> MlGetModelSnapshotsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetModelSnapshotsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetModelSnapshotsRequestBuilder<'a> {
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
pub struct MlGetOverallBucketsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_jobs: Option<&'a bool>,
    bucket_span: &'a str,
    end: &'a str,
    exclude_interim: Option<&'a bool>,
    overall_score: Option<&'a f64>,
    start: &'a str,
    top_n: Option<&'a i32>,
}
impl<'a> MlGetOverallBucketsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetOverallBucketsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetOverallBucketsRequestBuilder<'a> {
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
pub struct MlGetRecordsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    desc: Option<&'a bool>,
    end: &'a str,
    exclude_interim: Option<&'a bool>,
    from: Option<&'a i32>,
    record_score: Option<&'a f64>,
    size: Option<&'a i32>,
    sort: &'a str,
    start: &'a str,
}
impl<'a> MlGetRecordsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlGetRecordsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlGetRecordsRequestBuilder<'a> {
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
pub struct MlInfoRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlInfoRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlInfoRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlInfoRequestBuilder<'a> {
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
pub struct MlOpenJobRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlOpenJobRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlOpenJobRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlOpenJobRequestBuilder<'a> {
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
pub struct MlPostCalendarEventsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlPostCalendarEventsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlPostCalendarEventsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlPostCalendarEventsRequestBuilder<'a> {
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
pub struct MlPostDataRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    reset_end: &'a str,
    reset_start: &'a str,
}
impl<'a> MlPostDataRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlPostDataRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlPostDataRequestBuilder<'a> {
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
pub struct MlPreviewDatafeedRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlPreviewDatafeedRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlPreviewDatafeedRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlPreviewDatafeedRequestBuilder<'a> {
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
pub struct MlPutCalendarRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlPutCalendarRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlPutCalendarRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlPutCalendarRequestBuilder<'a> {
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
pub struct MlPutCalendarJobRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlPutCalendarJobRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlPutCalendarJobRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlPutCalendarJobRequestBuilder<'a> {
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
pub struct MlPutDataFrameAnalyticsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlPutDataFrameAnalyticsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlPutDataFrameAnalyticsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlPutDataFrameAnalyticsRequestBuilder<'a> {
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
pub struct MlPutDatafeedRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlPutDatafeedRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlPutDatafeedRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlPutDatafeedRequestBuilder<'a> {
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
pub struct MlPutFilterRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlPutFilterRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlPutFilterRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlPutFilterRequestBuilder<'a> {
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
pub struct MlPutJobRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlPutJobRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlPutJobRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlPutJobRequestBuilder<'a> {
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
pub struct MlRevertModelSnapshotRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    delete_intervening_results: Option<&'a bool>,
}
impl<'a> MlRevertModelSnapshotRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlRevertModelSnapshotRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlRevertModelSnapshotRequestBuilder<'a> {
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
pub struct MlSetUpgradeModeRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    enabled: Option<&'a bool>,
    timeout: &'a str,
}
impl<'a> MlSetUpgradeModeRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlSetUpgradeModeRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlSetUpgradeModeRequestBuilder<'a> {
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
pub struct MlStartDataFrameAnalyticsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    timeout: &'a str,
}
impl<'a> MlStartDataFrameAnalyticsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlStartDataFrameAnalyticsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlStartDataFrameAnalyticsRequestBuilder<'a> {
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
pub struct MlStartDatafeedRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    end: &'a str,
    start: &'a str,
    timeout: &'a str,
}
impl<'a> MlStartDatafeedRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlStartDatafeedRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlStartDatafeedRequestBuilder<'a> {
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
pub struct MlStopDataFrameAnalyticsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_match: Option<&'a bool>,
    force: Option<&'a bool>,
    timeout: &'a str,
}
impl<'a> MlStopDataFrameAnalyticsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlStopDataFrameAnalyticsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlStopDataFrameAnalyticsRequestBuilder<'a> {
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
pub struct MlStopDatafeedRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    allow_no_datafeeds: Option<&'a bool>,
    force: Option<&'a bool>,
    timeout: &'a str,
}
impl<'a> MlStopDatafeedRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlStopDatafeedRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlStopDatafeedRequestBuilder<'a> {
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
pub struct MlUpdateDatafeedRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlUpdateDatafeedRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlUpdateDatafeedRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlUpdateDatafeedRequestBuilder<'a> {
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
pub struct MlUpdateFilterRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlUpdateFilterRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlUpdateFilterRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlUpdateFilterRequestBuilder<'a> {
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
pub struct MlUpdateJobRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlUpdateJobRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlUpdateJobRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlUpdateJobRequestBuilder<'a> {
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
pub struct MlUpdateModelSnapshotRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlUpdateModelSnapshotRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlUpdateModelSnapshotRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlUpdateModelSnapshotRequestBuilder<'a> {
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
pub struct MlValidateRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlValidateRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlValidateRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlValidateRequestBuilder<'a> {
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
pub struct MlValidateDetectorRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> MlValidateDetectorRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlValidateDetectorRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for MlValidateDetectorRequestBuilder<'a> {
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
pub struct MlNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> MlNamespaceClient<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        MlNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-close-job.html"]
    pub fn close_job(&self) -> MlCloseJobRequestBuilder {
        MlCloseJobRequestBuilder::default()
    }
    pub fn delete_calendar(&self) -> MlDeleteCalendarRequestBuilder {
        MlDeleteCalendarRequestBuilder::default()
    }
    pub fn delete_calendar_event(&self) -> MlDeleteCalendarEventRequestBuilder {
        MlDeleteCalendarEventRequestBuilder::default()
    }
    pub fn delete_calendar_job(&self) -> MlDeleteCalendarJobRequestBuilder {
        MlDeleteCalendarJobRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/delete-dfanalytics.html"]
    pub fn delete_data_frame_analytics(&self) -> MlDeleteDataFrameAnalyticsRequestBuilder {
        MlDeleteDataFrameAnalyticsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-datafeed.html"]
    pub fn delete_datafeed(&self) -> MlDeleteDatafeedRequestBuilder {
        MlDeleteDatafeedRequestBuilder::default()
    }
    pub fn delete_expired_data(&self) -> MlDeleteExpiredDataRequestBuilder {
        MlDeleteExpiredDataRequestBuilder::default()
    }
    pub fn delete_filter(&self) -> MlDeleteFilterRequestBuilder {
        MlDeleteFilterRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-forecast.html"]
    pub fn delete_forecast(&self) -> MlDeleteForecastRequestBuilder {
        MlDeleteForecastRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-job.html"]
    pub fn delete_job(&self) -> MlDeleteJobRequestBuilder {
        MlDeleteJobRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-snapshot.html"]
    pub fn delete_model_snapshot(&self) -> MlDeleteModelSnapshotRequestBuilder {
        MlDeleteModelSnapshotRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/evaluate-dfanalytics.html"]
    pub fn evaluate_data_frame(&self) -> MlEvaluateDataFrameRequestBuilder {
        MlEvaluateDataFrameRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-find-file-structure.html"]
    pub fn find_file_structure(&self) -> MlFindFileStructureRequestBuilder {
        MlFindFileStructureRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-flush-job.html"]
    pub fn flush_job(&self) -> MlFlushJobRequestBuilder {
        MlFlushJobRequestBuilder::default()
    }
    pub fn forecast(&self) -> MlForecastRequestBuilder {
        MlForecastRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-bucket.html"]
    pub fn get_buckets(&self) -> MlGetBucketsRequestBuilder {
        MlGetBucketsRequestBuilder::default()
    }
    pub fn get_calendar_events(&self) -> MlGetCalendarEventsRequestBuilder {
        MlGetCalendarEventsRequestBuilder::default()
    }
    pub fn get_calendars(&self) -> MlGetCalendarsRequestBuilder {
        MlGetCalendarsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-category.html"]
    pub fn get_categories(&self) -> MlGetCategoriesRequestBuilder {
        MlGetCategoriesRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/get-dfanalytics.html"]
    pub fn get_data_frame_analytics(&self) -> MlGetDataFrameAnalyticsRequestBuilder {
        MlGetDataFrameAnalyticsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/get-dfanalytics-stats.html"]
    pub fn get_data_frame_analytics_stats(&self) -> MlGetDataFrameAnalyticsStatsRequestBuilder {
        MlGetDataFrameAnalyticsStatsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-datafeed-stats.html"]
    pub fn get_datafeed_stats(&self) -> MlGetDatafeedStatsRequestBuilder {
        MlGetDatafeedStatsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-datafeed.html"]
    pub fn get_datafeeds(&self) -> MlGetDatafeedsRequestBuilder {
        MlGetDatafeedsRequestBuilder::default()
    }
    pub fn get_filters(&self) -> MlGetFiltersRequestBuilder {
        MlGetFiltersRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-influencer.html"]
    pub fn get_influencers(&self) -> MlGetInfluencersRequestBuilder {
        MlGetInfluencersRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-job-stats.html"]
    pub fn get_job_stats(&self) -> MlGetJobStatsRequestBuilder {
        MlGetJobStatsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-job.html"]
    pub fn get_jobs(&self) -> MlGetJobsRequestBuilder {
        MlGetJobsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-snapshot.html"]
    pub fn get_model_snapshots(&self) -> MlGetModelSnapshotsRequestBuilder {
        MlGetModelSnapshotsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-overall-buckets.html"]
    pub fn get_overall_buckets(&self) -> MlGetOverallBucketsRequestBuilder {
        MlGetOverallBucketsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-record.html"]
    pub fn get_records(&self) -> MlGetRecordsRequestBuilder {
        MlGetRecordsRequestBuilder::default()
    }
    pub fn info(&self) -> MlInfoRequestBuilder {
        MlInfoRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-open-job.html"]
    pub fn open_job(&self) -> MlOpenJobRequestBuilder {
        MlOpenJobRequestBuilder::default()
    }
    pub fn post_calendar_events(&self) -> MlPostCalendarEventsRequestBuilder {
        MlPostCalendarEventsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-post-data.html"]
    pub fn post_data(&self) -> MlPostDataRequestBuilder {
        MlPostDataRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-preview-datafeed.html"]
    pub fn preview_datafeed(&self) -> MlPreviewDatafeedRequestBuilder {
        MlPreviewDatafeedRequestBuilder::default()
    }
    pub fn put_calendar(&self) -> MlPutCalendarRequestBuilder {
        MlPutCalendarRequestBuilder::default()
    }
    pub fn put_calendar_job(&self) -> MlPutCalendarJobRequestBuilder {
        MlPutCalendarJobRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/put-dfanalytics.html"]
    pub fn put_data_frame_analytics(&self) -> MlPutDataFrameAnalyticsRequestBuilder {
        MlPutDataFrameAnalyticsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-datafeed.html"]
    pub fn put_datafeed(&self) -> MlPutDatafeedRequestBuilder {
        MlPutDatafeedRequestBuilder::default()
    }
    pub fn put_filter(&self) -> MlPutFilterRequestBuilder {
        MlPutFilterRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-job.html"]
    pub fn put_job(&self) -> MlPutJobRequestBuilder {
        MlPutJobRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-revert-snapshot.html"]
    pub fn revert_model_snapshot(&self) -> MlRevertModelSnapshotRequestBuilder {
        MlRevertModelSnapshotRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-set-upgrade-mode.html"]
    pub fn set_upgrade_mode(&self) -> MlSetUpgradeModeRequestBuilder {
        MlSetUpgradeModeRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/start-dfanalytics.html"]
    pub fn start_data_frame_analytics(&self) -> MlStartDataFrameAnalyticsRequestBuilder {
        MlStartDataFrameAnalyticsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-start-datafeed.html"]
    pub fn start_datafeed(&self) -> MlStartDatafeedRequestBuilder {
        MlStartDatafeedRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/stop-dfanalytics.html"]
    pub fn stop_data_frame_analytics(&self) -> MlStopDataFrameAnalyticsRequestBuilder {
        MlStopDataFrameAnalyticsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-stop-datafeed.html"]
    pub fn stop_datafeed(&self) -> MlStopDatafeedRequestBuilder {
        MlStopDatafeedRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-datafeed.html"]
    pub fn update_datafeed(&self) -> MlUpdateDatafeedRequestBuilder {
        MlUpdateDatafeedRequestBuilder::default()
    }
    pub fn update_filter(&self) -> MlUpdateFilterRequestBuilder {
        MlUpdateFilterRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-job.html"]
    pub fn update_job(&self) -> MlUpdateJobRequestBuilder {
        MlUpdateJobRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-snapshot.html"]
    pub fn update_model_snapshot(&self) -> MlUpdateModelSnapshotRequestBuilder {
        MlUpdateModelSnapshotRequestBuilder::default()
    }
    pub fn validate(&self) -> MlValidateRequestBuilder {
        MlValidateRequestBuilder::default()
    }
    pub fn validate_detector(&self) -> MlValidateDetectorRequestBuilder {
        MlValidateDetectorRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Ml APIs"]
    pub fn ml(&self) -> MlNamespaceClient {
        MlNamespaceClient::new(self)
    }
}
