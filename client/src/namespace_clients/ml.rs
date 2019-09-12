

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct MlCloseJobRequest<'a> {
    allow_no_jobs: Option<&'a bool>,
    force: Option<&'a bool>,
    timeout: &'a String,
}
pub struct MlCloseJobRequestBuilder<'a> {
    allow_no_jobs: Option<&'a bool>,
    force: Option<&'a bool>,
    timeout: &'a String,
}
impl<'a> MlCloseJobRequestBuilder<'a> {
    pub fn build(&self) -> MlCloseJobRequest<'a> {
        MlCloseJobRequest {
            allow_no_jobs: self.allow_no_jobs,
            force: self.force,
            timeout: self.timeout,
        }
    }
}
pub struct MlDeleteCalendarRequest<'a> {}
pub struct MlDeleteCalendarRequestBuilder<'a> {}
impl<'a> MlDeleteCalendarRequestBuilder<'a> {
    pub fn build(&self) -> MlDeleteCalendarRequest<'a> {
        MlDeleteCalendarRequest {}
    }
}
pub struct MlDeleteCalendarEventRequest<'a> {}
pub struct MlDeleteCalendarEventRequestBuilder<'a> {}
impl<'a> MlDeleteCalendarEventRequestBuilder<'a> {
    pub fn build(&self) -> MlDeleteCalendarEventRequest<'a> {
        MlDeleteCalendarEventRequest {}
    }
}
pub struct MlDeleteCalendarJobRequest<'a> {}
pub struct MlDeleteCalendarJobRequestBuilder<'a> {}
impl<'a> MlDeleteCalendarJobRequestBuilder<'a> {
    pub fn build(&self) -> MlDeleteCalendarJobRequest<'a> {
        MlDeleteCalendarJobRequest {}
    }
}
pub struct MlDeleteDataFrameAnalyticsRequest<'a> {}
pub struct MlDeleteDataFrameAnalyticsRequestBuilder<'a> {}
impl<'a> MlDeleteDataFrameAnalyticsRequestBuilder<'a> {
    pub fn build(&self) -> MlDeleteDataFrameAnalyticsRequest<'a> {
        MlDeleteDataFrameAnalyticsRequest {}
    }
}
pub struct MlDeleteDatafeedRequest<'a> {
    force: Option<&'a bool>,
}
pub struct MlDeleteDatafeedRequestBuilder<'a> {
    force: Option<&'a bool>,
}
impl<'a> MlDeleteDatafeedRequestBuilder<'a> {
    pub fn build(&self) -> MlDeleteDatafeedRequest<'a> {
        MlDeleteDatafeedRequest { force: self.force }
    }
}
pub struct MlDeleteExpiredDataRequest<'a> {}
pub struct MlDeleteExpiredDataRequestBuilder<'a> {}
impl<'a> MlDeleteExpiredDataRequestBuilder<'a> {
    pub fn build(&self) -> MlDeleteExpiredDataRequest<'a> {
        MlDeleteExpiredDataRequest {}
    }
}
pub struct MlDeleteFilterRequest<'a> {}
pub struct MlDeleteFilterRequestBuilder<'a> {}
impl<'a> MlDeleteFilterRequestBuilder<'a> {
    pub fn build(&self) -> MlDeleteFilterRequest<'a> {
        MlDeleteFilterRequest {}
    }
}
pub struct MlDeleteForecastRequest<'a> {
    allow_no_forecasts: Option<&'a bool>,
    timeout: &'a String,
}
pub struct MlDeleteForecastRequestBuilder<'a> {
    allow_no_forecasts: Option<&'a bool>,
    timeout: &'a String,
}
impl<'a> MlDeleteForecastRequestBuilder<'a> {
    pub fn build(&self) -> MlDeleteForecastRequest<'a> {
        MlDeleteForecastRequest {
            allow_no_forecasts: self.allow_no_forecasts,
            timeout: self.timeout,
        }
    }
}
pub struct MlDeleteJobRequest<'a> {
    force: Option<&'a bool>,
    wait_for_completion: Option<&'a bool>,
}
pub struct MlDeleteJobRequestBuilder<'a> {
    force: Option<&'a bool>,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> MlDeleteJobRequestBuilder<'a> {
    pub fn build(&self) -> MlDeleteJobRequest<'a> {
        MlDeleteJobRequest {
            force: self.force,
            wait_for_completion: self.wait_for_completion,
        }
    }
}
pub struct MlDeleteModelSnapshotRequest<'a> {}
pub struct MlDeleteModelSnapshotRequestBuilder<'a> {}
impl<'a> MlDeleteModelSnapshotRequestBuilder<'a> {
    pub fn build(&self) -> MlDeleteModelSnapshotRequest<'a> {
        MlDeleteModelSnapshotRequest {}
    }
}
pub struct MlEvaluateDataFrameRequest<'a> {}
pub struct MlEvaluateDataFrameRequestBuilder<'a> {}
impl<'a> MlEvaluateDataFrameRequestBuilder<'a> {
    pub fn build(&self) -> MlEvaluateDataFrameRequest<'a> {
        MlEvaluateDataFrameRequest {}
    }
}
pub struct MlFindFileStructureRequest<'a> {
    charset: &'a String,
    column_names: &'a Vec<String>,
    delimiter: &'a String,
    explain: Option<&'a bool>,
    format: Option<&'a i32>,
    grok_pattern: &'a String,
    has_header_row: Option<&'a bool>,
    line_merge_size_limit: Option<&'a i32>,
    lines_to_sample: Option<&'a i32>,
    quote: &'a String,
    should_trim_fields: Option<&'a bool>,
    timeout: &'a String,
    timestamp_field: &'a String,
    timestamp_format: &'a String,
}
pub struct MlFindFileStructureRequestBuilder<'a> {
    charset: &'a String,
    column_names: &'a Vec<String>,
    delimiter: &'a String,
    explain: Option<&'a bool>,
    format: Option<&'a i32>,
    grok_pattern: &'a String,
    has_header_row: Option<&'a bool>,
    line_merge_size_limit: Option<&'a i32>,
    lines_to_sample: Option<&'a i32>,
    quote: &'a String,
    should_trim_fields: Option<&'a bool>,
    timeout: &'a String,
    timestamp_field: &'a String,
    timestamp_format: &'a String,
}
impl<'a> MlFindFileStructureRequestBuilder<'a> {
    pub fn build(&self) -> MlFindFileStructureRequest<'a> {
        MlFindFileStructureRequest {
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
        }
    }
}
pub struct MlFlushJobRequest<'a> {
    advance_time: &'a String,
    calc_interim: Option<&'a bool>,
    end: &'a String,
    skip_time: &'a String,
    start: &'a String,
}
pub struct MlFlushJobRequestBuilder<'a> {
    advance_time: &'a String,
    calc_interim: Option<&'a bool>,
    end: &'a String,
    skip_time: &'a String,
    start: &'a String,
}
impl<'a> MlFlushJobRequestBuilder<'a> {
    pub fn build(&self) -> MlFlushJobRequest<'a> {
        MlFlushJobRequest {
            advance_time: self.advance_time,
            calc_interim: self.calc_interim,
            end: self.end,
            skip_time: self.skip_time,
            start: self.start,
        }
    }
}
pub struct MlForecastRequest<'a> {
    duration: &'a String,
    expires_in: &'a String,
}
pub struct MlForecastRequestBuilder<'a> {
    duration: &'a String,
    expires_in: &'a String,
}
impl<'a> MlForecastRequestBuilder<'a> {
    pub fn build(&self) -> MlForecastRequest<'a> {
        MlForecastRequest {
            duration: self.duration,
            expires_in: self.expires_in,
        }
    }
}
pub struct MlGetBucketsRequest<'a> {
    anomaly_score: Option<&'a f64>,
    desc: Option<&'a bool>,
    end: &'a String,
    exclude_interim: Option<&'a bool>,
    expand: Option<&'a bool>,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
    sort: &'a String,
    start: &'a String,
}
pub struct MlGetBucketsRequestBuilder<'a> {
    anomaly_score: Option<&'a f64>,
    desc: Option<&'a bool>,
    end: &'a String,
    exclude_interim: Option<&'a bool>,
    expand: Option<&'a bool>,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
    sort: &'a String,
    start: &'a String,
}
impl<'a> MlGetBucketsRequestBuilder<'a> {
    pub fn build(&self) -> MlGetBucketsRequest<'a> {
        MlGetBucketsRequest {
            anomaly_score: self.anomaly_score,
            desc: self.desc,
            end: self.end,
            exclude_interim: self.exclude_interim,
            expand: self.expand,
            from: self.from,
            size: self.size,
            sort: self.sort,
            start: self.start,
        }
    }
}
pub struct MlGetCalendarEventsRequest<'a> {
    end: &'a String,
    from: Option<&'a i32>,
    job_id: &'a String,
    size: Option<&'a i32>,
    start: &'a String,
}
pub struct MlGetCalendarEventsRequestBuilder<'a> {
    end: &'a String,
    from: Option<&'a i32>,
    job_id: &'a String,
    size: Option<&'a i32>,
    start: &'a String,
}
impl<'a> MlGetCalendarEventsRequestBuilder<'a> {
    pub fn build(&self) -> MlGetCalendarEventsRequest<'a> {
        MlGetCalendarEventsRequest {
            end: self.end,
            from: self.from,
            job_id: self.job_id,
            size: self.size,
            start: self.start,
        }
    }
}
pub struct MlGetCalendarsRequest<'a> {
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
pub struct MlGetCalendarsRequestBuilder<'a> {
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
impl<'a> MlGetCalendarsRequestBuilder<'a> {
    pub fn build(&self) -> MlGetCalendarsRequest<'a> {
        MlGetCalendarsRequest {
            from: self.from,
            size: self.size,
        }
    }
}
pub struct MlGetCategoriesRequest<'a> {
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
pub struct MlGetCategoriesRequestBuilder<'a> {
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
impl<'a> MlGetCategoriesRequestBuilder<'a> {
    pub fn build(&self) -> MlGetCategoriesRequest<'a> {
        MlGetCategoriesRequest {
            from: self.from,
            size: self.size,
        }
    }
}
pub struct MlGetDataFrameAnalyticsRequest<'a> {
    allow_no_match: Option<&'a bool>,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
pub struct MlGetDataFrameAnalyticsRequestBuilder<'a> {
    allow_no_match: Option<&'a bool>,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
impl<'a> MlGetDataFrameAnalyticsRequestBuilder<'a> {
    pub fn build(&self) -> MlGetDataFrameAnalyticsRequest<'a> {
        MlGetDataFrameAnalyticsRequest {
            allow_no_match: self.allow_no_match,
            from: self.from,
            size: self.size,
        }
    }
}
pub struct MlGetDataFrameAnalyticsStatsRequest<'a> {
    allow_no_match: Option<&'a bool>,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
pub struct MlGetDataFrameAnalyticsStatsRequestBuilder<'a> {
    allow_no_match: Option<&'a bool>,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
impl<'a> MlGetDataFrameAnalyticsStatsRequestBuilder<'a> {
    pub fn build(&self) -> MlGetDataFrameAnalyticsStatsRequest<'a> {
        MlGetDataFrameAnalyticsStatsRequest {
            allow_no_match: self.allow_no_match,
            from: self.from,
            size: self.size,
        }
    }
}
pub struct MlGetDatafeedStatsRequest<'a> {
    allow_no_datafeeds: Option<&'a bool>,
}
pub struct MlGetDatafeedStatsRequestBuilder<'a> {
    allow_no_datafeeds: Option<&'a bool>,
}
impl<'a> MlGetDatafeedStatsRequestBuilder<'a> {
    pub fn build(&self) -> MlGetDatafeedStatsRequest<'a> {
        MlGetDatafeedStatsRequest {
            allow_no_datafeeds: self.allow_no_datafeeds,
        }
    }
}
pub struct MlGetDatafeedsRequest<'a> {
    allow_no_datafeeds: Option<&'a bool>,
}
pub struct MlGetDatafeedsRequestBuilder<'a> {
    allow_no_datafeeds: Option<&'a bool>,
}
impl<'a> MlGetDatafeedsRequestBuilder<'a> {
    pub fn build(&self) -> MlGetDatafeedsRequest<'a> {
        MlGetDatafeedsRequest {
            allow_no_datafeeds: self.allow_no_datafeeds,
        }
    }
}
pub struct MlGetFiltersRequest<'a> {
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
pub struct MlGetFiltersRequestBuilder<'a> {
    from: Option<&'a i32>,
    size: Option<&'a i32>,
}
impl<'a> MlGetFiltersRequestBuilder<'a> {
    pub fn build(&self) -> MlGetFiltersRequest<'a> {
        MlGetFiltersRequest {
            from: self.from,
            size: self.size,
        }
    }
}
pub struct MlGetInfluencersRequest<'a> {
    desc: Option<&'a bool>,
    end: &'a String,
    exclude_interim: Option<&'a bool>,
    from: Option<&'a i32>,
    influencer_score: Option<&'a f64>,
    size: Option<&'a i32>,
    sort: &'a String,
    start: &'a String,
}
pub struct MlGetInfluencersRequestBuilder<'a> {
    desc: Option<&'a bool>,
    end: &'a String,
    exclude_interim: Option<&'a bool>,
    from: Option<&'a i32>,
    influencer_score: Option<&'a f64>,
    size: Option<&'a i32>,
    sort: &'a String,
    start: &'a String,
}
impl<'a> MlGetInfluencersRequestBuilder<'a> {
    pub fn build(&self) -> MlGetInfluencersRequest<'a> {
        MlGetInfluencersRequest {
            desc: self.desc,
            end: self.end,
            exclude_interim: self.exclude_interim,
            from: self.from,
            influencer_score: self.influencer_score,
            size: self.size,
            sort: self.sort,
            start: self.start,
        }
    }
}
pub struct MlGetJobStatsRequest<'a> {
    allow_no_jobs: Option<&'a bool>,
}
pub struct MlGetJobStatsRequestBuilder<'a> {
    allow_no_jobs: Option<&'a bool>,
}
impl<'a> MlGetJobStatsRequestBuilder<'a> {
    pub fn build(&self) -> MlGetJobStatsRequest<'a> {
        MlGetJobStatsRequest {
            allow_no_jobs: self.allow_no_jobs,
        }
    }
}
pub struct MlGetJobsRequest<'a> {
    allow_no_jobs: Option<&'a bool>,
}
pub struct MlGetJobsRequestBuilder<'a> {
    allow_no_jobs: Option<&'a bool>,
}
impl<'a> MlGetJobsRequestBuilder<'a> {
    pub fn build(&self) -> MlGetJobsRequest<'a> {
        MlGetJobsRequest {
            allow_no_jobs: self.allow_no_jobs,
        }
    }
}
pub struct MlGetModelSnapshotsRequest<'a> {
    desc: Option<&'a bool>,
    end: &'a String,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
    sort: &'a String,
    start: &'a String,
}
pub struct MlGetModelSnapshotsRequestBuilder<'a> {
    desc: Option<&'a bool>,
    end: &'a String,
    from: Option<&'a i32>,
    size: Option<&'a i32>,
    sort: &'a String,
    start: &'a String,
}
impl<'a> MlGetModelSnapshotsRequestBuilder<'a> {
    pub fn build(&self) -> MlGetModelSnapshotsRequest<'a> {
        MlGetModelSnapshotsRequest {
            desc: self.desc,
            end: self.end,
            from: self.from,
            size: self.size,
            sort: self.sort,
            start: self.start,
        }
    }
}
pub struct MlGetOverallBucketsRequest<'a> {
    allow_no_jobs: Option<&'a bool>,
    bucket_span: &'a String,
    end: &'a String,
    exclude_interim: Option<&'a bool>,
    overall_score: Option<&'a f64>,
    start: &'a String,
    top_n: Option<&'a i32>,
}
pub struct MlGetOverallBucketsRequestBuilder<'a> {
    allow_no_jobs: Option<&'a bool>,
    bucket_span: &'a String,
    end: &'a String,
    exclude_interim: Option<&'a bool>,
    overall_score: Option<&'a f64>,
    start: &'a String,
    top_n: Option<&'a i32>,
}
impl<'a> MlGetOverallBucketsRequestBuilder<'a> {
    pub fn build(&self) -> MlGetOverallBucketsRequest<'a> {
        MlGetOverallBucketsRequest {
            allow_no_jobs: self.allow_no_jobs,
            bucket_span: self.bucket_span,
            end: self.end,
            exclude_interim: self.exclude_interim,
            overall_score: self.overall_score,
            start: self.start,
            top_n: self.top_n,
        }
    }
}
pub struct MlGetRecordsRequest<'a> {
    desc: Option<&'a bool>,
    end: &'a String,
    exclude_interim: Option<&'a bool>,
    from: Option<&'a i32>,
    record_score: Option<&'a f64>,
    size: Option<&'a i32>,
    sort: &'a String,
    start: &'a String,
}
pub struct MlGetRecordsRequestBuilder<'a> {
    desc: Option<&'a bool>,
    end: &'a String,
    exclude_interim: Option<&'a bool>,
    from: Option<&'a i32>,
    record_score: Option<&'a f64>,
    size: Option<&'a i32>,
    sort: &'a String,
    start: &'a String,
}
impl<'a> MlGetRecordsRequestBuilder<'a> {
    pub fn build(&self) -> MlGetRecordsRequest<'a> {
        MlGetRecordsRequest {
            desc: self.desc,
            end: self.end,
            exclude_interim: self.exclude_interim,
            from: self.from,
            record_score: self.record_score,
            size: self.size,
            sort: self.sort,
            start: self.start,
        }
    }
}
pub struct MlInfoRequest<'a> {}
pub struct MlInfoRequestBuilder<'a> {}
impl<'a> MlInfoRequestBuilder<'a> {
    pub fn build(&self) -> MlInfoRequest<'a> {
        MlInfoRequest {}
    }
}
pub struct MlOpenJobRequest<'a> {}
pub struct MlOpenJobRequestBuilder<'a> {}
impl<'a> MlOpenJobRequestBuilder<'a> {
    pub fn build(&self) -> MlOpenJobRequest<'a> {
        MlOpenJobRequest {}
    }
}
pub struct MlPostCalendarEventsRequest<'a> {}
pub struct MlPostCalendarEventsRequestBuilder<'a> {}
impl<'a> MlPostCalendarEventsRequestBuilder<'a> {
    pub fn build(&self) -> MlPostCalendarEventsRequest<'a> {
        MlPostCalendarEventsRequest {}
    }
}
pub struct MlPostDataRequest<'a> {
    reset_end: &'a String,
    reset_start: &'a String,
}
pub struct MlPostDataRequestBuilder<'a> {
    reset_end: &'a String,
    reset_start: &'a String,
}
impl<'a> MlPostDataRequestBuilder<'a> {
    pub fn build(&self) -> MlPostDataRequest<'a> {
        MlPostDataRequest {
            reset_end: self.reset_end,
            reset_start: self.reset_start,
        }
    }
}
pub struct MlPreviewDatafeedRequest<'a> {}
pub struct MlPreviewDatafeedRequestBuilder<'a> {}
impl<'a> MlPreviewDatafeedRequestBuilder<'a> {
    pub fn build(&self) -> MlPreviewDatafeedRequest<'a> {
        MlPreviewDatafeedRequest {}
    }
}
pub struct MlPutCalendarRequest<'a> {}
pub struct MlPutCalendarRequestBuilder<'a> {}
impl<'a> MlPutCalendarRequestBuilder<'a> {
    pub fn build(&self) -> MlPutCalendarRequest<'a> {
        MlPutCalendarRequest {}
    }
}
pub struct MlPutCalendarJobRequest<'a> {}
pub struct MlPutCalendarJobRequestBuilder<'a> {}
impl<'a> MlPutCalendarJobRequestBuilder<'a> {
    pub fn build(&self) -> MlPutCalendarJobRequest<'a> {
        MlPutCalendarJobRequest {}
    }
}
pub struct MlPutDataFrameAnalyticsRequest<'a> {}
pub struct MlPutDataFrameAnalyticsRequestBuilder<'a> {}
impl<'a> MlPutDataFrameAnalyticsRequestBuilder<'a> {
    pub fn build(&self) -> MlPutDataFrameAnalyticsRequest<'a> {
        MlPutDataFrameAnalyticsRequest {}
    }
}
pub struct MlPutDatafeedRequest<'a> {}
pub struct MlPutDatafeedRequestBuilder<'a> {}
impl<'a> MlPutDatafeedRequestBuilder<'a> {
    pub fn build(&self) -> MlPutDatafeedRequest<'a> {
        MlPutDatafeedRequest {}
    }
}
pub struct MlPutFilterRequest<'a> {}
pub struct MlPutFilterRequestBuilder<'a> {}
impl<'a> MlPutFilterRequestBuilder<'a> {
    pub fn build(&self) -> MlPutFilterRequest<'a> {
        MlPutFilterRequest {}
    }
}
pub struct MlPutJobRequest<'a> {}
pub struct MlPutJobRequestBuilder<'a> {}
impl<'a> MlPutJobRequestBuilder<'a> {
    pub fn build(&self) -> MlPutJobRequest<'a> {
        MlPutJobRequest {}
    }
}
pub struct MlRevertModelSnapshotRequest<'a> {
    delete_intervening_results: Option<&'a bool>,
}
pub struct MlRevertModelSnapshotRequestBuilder<'a> {
    delete_intervening_results: Option<&'a bool>,
}
impl<'a> MlRevertModelSnapshotRequestBuilder<'a> {
    pub fn build(&self) -> MlRevertModelSnapshotRequest<'a> {
        MlRevertModelSnapshotRequest {
            delete_intervening_results: self.delete_intervening_results,
        }
    }
}
pub struct MlSetUpgradeModeRequest<'a> {
    enabled: Option<&'a bool>,
    timeout: &'a String,
}
pub struct MlSetUpgradeModeRequestBuilder<'a> {
    enabled: Option<&'a bool>,
    timeout: &'a String,
}
impl<'a> MlSetUpgradeModeRequestBuilder<'a> {
    pub fn build(&self) -> MlSetUpgradeModeRequest<'a> {
        MlSetUpgradeModeRequest {
            enabled: self.enabled,
            timeout: self.timeout,
        }
    }
}
pub struct MlStartDataFrameAnalyticsRequest<'a> {
    timeout: &'a String,
}
pub struct MlStartDataFrameAnalyticsRequestBuilder<'a> {
    timeout: &'a String,
}
impl<'a> MlStartDataFrameAnalyticsRequestBuilder<'a> {
    pub fn build(&self) -> MlStartDataFrameAnalyticsRequest<'a> {
        MlStartDataFrameAnalyticsRequest {
            timeout: self.timeout,
        }
    }
}
pub struct MlStartDatafeedRequest<'a> {
    end: &'a String,
    start: &'a String,
    timeout: &'a String,
}
pub struct MlStartDatafeedRequestBuilder<'a> {
    end: &'a String,
    start: &'a String,
    timeout: &'a String,
}
impl<'a> MlStartDatafeedRequestBuilder<'a> {
    pub fn build(&self) -> MlStartDatafeedRequest<'a> {
        MlStartDatafeedRequest {
            end: self.end,
            start: self.start,
            timeout: self.timeout,
        }
    }
}
pub struct MlStopDataFrameAnalyticsRequest<'a> {
    allow_no_match: Option<&'a bool>,
    force: Option<&'a bool>,
    timeout: &'a String,
}
pub struct MlStopDataFrameAnalyticsRequestBuilder<'a> {
    allow_no_match: Option<&'a bool>,
    force: Option<&'a bool>,
    timeout: &'a String,
}
impl<'a> MlStopDataFrameAnalyticsRequestBuilder<'a> {
    pub fn build(&self) -> MlStopDataFrameAnalyticsRequest<'a> {
        MlStopDataFrameAnalyticsRequest {
            allow_no_match: self.allow_no_match,
            force: self.force,
            timeout: self.timeout,
        }
    }
}
pub struct MlStopDatafeedRequest<'a> {
    allow_no_datafeeds: Option<&'a bool>,
    force: Option<&'a bool>,
    timeout: &'a String,
}
pub struct MlStopDatafeedRequestBuilder<'a> {
    allow_no_datafeeds: Option<&'a bool>,
    force: Option<&'a bool>,
    timeout: &'a String,
}
impl<'a> MlStopDatafeedRequestBuilder<'a> {
    pub fn build(&self) -> MlStopDatafeedRequest<'a> {
        MlStopDatafeedRequest {
            allow_no_datafeeds: self.allow_no_datafeeds,
            force: self.force,
            timeout: self.timeout,
        }
    }
}
pub struct MlUpdateDatafeedRequest<'a> {}
pub struct MlUpdateDatafeedRequestBuilder<'a> {}
impl<'a> MlUpdateDatafeedRequestBuilder<'a> {
    pub fn build(&self) -> MlUpdateDatafeedRequest<'a> {
        MlUpdateDatafeedRequest {}
    }
}
pub struct MlUpdateFilterRequest<'a> {}
pub struct MlUpdateFilterRequestBuilder<'a> {}
impl<'a> MlUpdateFilterRequestBuilder<'a> {
    pub fn build(&self) -> MlUpdateFilterRequest<'a> {
        MlUpdateFilterRequest {}
    }
}
pub struct MlUpdateJobRequest<'a> {}
pub struct MlUpdateJobRequestBuilder<'a> {}
impl<'a> MlUpdateJobRequestBuilder<'a> {
    pub fn build(&self) -> MlUpdateJobRequest<'a> {
        MlUpdateJobRequest {}
    }
}
pub struct MlUpdateModelSnapshotRequest<'a> {}
pub struct MlUpdateModelSnapshotRequestBuilder<'a> {}
impl<'a> MlUpdateModelSnapshotRequestBuilder<'a> {
    pub fn build(&self) -> MlUpdateModelSnapshotRequest<'a> {
        MlUpdateModelSnapshotRequest {}
    }
}
pub struct MlValidateRequest<'a> {}
pub struct MlValidateRequestBuilder<'a> {}
impl<'a> MlValidateRequestBuilder<'a> {
    pub fn build(&self) -> MlValidateRequest<'a> {
        MlValidateRequest {}
    }
}
pub struct MlValidateDetectorRequest<'a> {}
pub struct MlValidateDetectorRequestBuilder<'a> {}
impl<'a> MlValidateDetectorRequestBuilder<'a> {
    pub fn build(&self) -> MlValidateDetectorRequest<'a> {
        MlValidateDetectorRequest {}
    }
}
#[doc = "Ml APIs"]
pub struct MlNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> MlNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        MlNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-close-job.html"]
    pub fn close_job(&self, request: &MlCloseJobRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/anomaly_detectors/{job_id}/_close")
    }
    pub fn delete_calendar(&self, request: &MlDeleteCalendarRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ml/calendars/{calendar_id}")
    }
    pub fn delete_calendar_event(
        &self,
        request: &MlDeleteCalendarEventRequest,
    ) -> Result<Response> {
        self.client.send(
            HttpMethod::Delete,
            "/_ml/calendars/{calendar_id}/events/{event_id}",
        )
    }
    pub fn delete_calendar_job(&self, request: &MlDeleteCalendarJobRequest) -> Result<Response> {
        self.client.send(
            HttpMethod::Delete,
            "/_ml/calendars/{calendar_id}/jobs/{job_id}",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/delete-dfanalytics.html"]
    pub fn delete_data_frame_analytics(
        &self,
        request: &MlDeleteDataFrameAnalyticsRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ml/data_frame/analytics/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-datafeed.html"]
    pub fn delete_datafeed(&self, request: &MlDeleteDatafeedRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ml/datafeeds/{datafeed_id}")
    }
    pub fn delete_expired_data(&self, request: &MlDeleteExpiredDataRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ml/_delete_expired_data")
    }
    pub fn delete_filter(&self, request: &MlDeleteFilterRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ml/filters/{filter_id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-forecast.html"]
    pub fn delete_forecast(&self, request: &MlDeleteForecastRequest) -> Result<Response> {
        self.client.send(
            HttpMethod::Delete,
            "/_ml/anomaly_detectors/{job_id}/_forecast",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-job.html"]
    pub fn delete_job(&self, request: &MlDeleteJobRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ml/anomaly_detectors/{job_id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-snapshot.html"]
    pub fn delete_model_snapshot(
        &self,
        request: &MlDeleteModelSnapshotRequest,
    ) -> Result<Response> {
        self.client.send(
            HttpMethod::Delete,
            "/_ml/anomaly_detectors/{job_id}/model_snapshots/{snapshot_id}",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/evaluate-dfanalytics.html"]
    pub fn evaluate_data_frame(&self, request: &MlEvaluateDataFrameRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/data_frame/_evaluate")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-find-file-structure.html"]
    pub fn find_file_structure(&self, request: &MlFindFileStructureRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/find_file_structure")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-flush-job.html"]
    pub fn flush_job(&self, request: &MlFlushJobRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/anomaly_detectors/{job_id}/_flush")
    }
    pub fn forecast(&self, request: &MlForecastRequest) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_ml/anomaly_detectors/{job_id}/_forecast",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-bucket.html"]
    pub fn get_buckets(&self, request: &MlGetBucketsRequest) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_ml/anomaly_detectors/{job_id}/results/buckets/{timestamp}",
        )
    }
    pub fn get_calendar_events(&self, request: &MlGetCalendarEventsRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/calendars/{calendar_id}/events")
    }
    pub fn get_calendars(&self, request: &MlGetCalendarsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ml/calendars")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-category.html"]
    pub fn get_categories(&self, request: &MlGetCategoriesRequest) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_ml/anomaly_detectors/{job_id}/results/categories/{category_id}",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/get-dfanalytics.html"]
    pub fn get_data_frame_analytics(
        &self,
        request: &MlGetDataFrameAnalyticsRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/data_frame/analytics/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/get-dfanalytics-stats.html"]
    pub fn get_data_frame_analytics_stats(
        &self,
        request: &MlGetDataFrameAnalyticsStatsRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/data_frame/analytics/_stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-datafeed-stats.html"]
    pub fn get_datafeed_stats(&self, request: &MlGetDatafeedStatsRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/datafeeds/{datafeed_id}/_stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-datafeed.html"]
    pub fn get_datafeeds(&self, request: &MlGetDatafeedsRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/datafeeds/{datafeed_id}")
    }
    pub fn get_filters(&self, request: &MlGetFiltersRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ml/filters")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-influencer.html"]
    pub fn get_influencers(&self, request: &MlGetInfluencersRequest) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_ml/anomaly_detectors/{job_id}/results/influencers",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-job-stats.html"]
    pub fn get_job_stats(&self, request: &MlGetJobStatsRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/anomaly_detectors/_stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-job.html"]
    pub fn get_jobs(&self, request: &MlGetJobsRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/anomaly_detectors/{job_id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-snapshot.html"]
    pub fn get_model_snapshots(&self, request: &MlGetModelSnapshotsRequest) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_ml/anomaly_detectors/{job_id}/model_snapshots/{snapshot_id}",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-overall-buckets.html"]
    pub fn get_overall_buckets(&self, request: &MlGetOverallBucketsRequest) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_ml/anomaly_detectors/{job_id}/results/overall_buckets",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-record.html"]
    pub fn get_records(&self, request: &MlGetRecordsRequest) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_ml/anomaly_detectors/{job_id}/results/records",
        )
    }
    pub fn info(&self, request: &MlInfoRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ml/info")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-open-job.html"]
    pub fn open_job(&self, request: &MlOpenJobRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/anomaly_detectors/{job_id}/_open")
    }
    pub fn post_calendar_events(&self, request: &MlPostCalendarEventsRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/calendars/{calendar_id}/events")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-post-data.html"]
    pub fn post_data(&self, request: &MlPostDataRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/anomaly_detectors/{job_id}/_data")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-preview-datafeed.html"]
    pub fn preview_datafeed(&self, request: &MlPreviewDatafeedRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/datafeeds/{datafeed_id}/_preview")
    }
    pub fn put_calendar(&self, request: &MlPutCalendarRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_ml/calendars/{calendar_id}")
    }
    pub fn put_calendar_job(&self, request: &MlPutCalendarJobRequest) -> Result<Response> {
        self.client.send(
            HttpMethod::Put,
            "/_ml/calendars/{calendar_id}/jobs/{job_id}",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/put-dfanalytics.html"]
    pub fn put_data_frame_analytics(
        &self,
        request: &MlPutDataFrameAnalyticsRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_ml/data_frame/analytics/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-datafeed.html"]
    pub fn put_datafeed(&self, request: &MlPutDatafeedRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_ml/datafeeds/{datafeed_id}")
    }
    pub fn put_filter(&self, request: &MlPutFilterRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_ml/filters/{filter_id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-job.html"]
    pub fn put_job(&self, request: &MlPutJobRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_ml/anomaly_detectors/{job_id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-revert-snapshot.html"]
    pub fn revert_model_snapshot(
        &self,
        request: &MlRevertModelSnapshotRequest,
    ) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_ml/anomaly_detectors/{job_id}/model_snapshots/{snapshot_id}/_revert",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-set-upgrade-mode.html"]
    pub fn set_upgrade_mode(&self, request: &MlSetUpgradeModeRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_ml/set_upgrade_mode")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/start-dfanalytics.html"]
    pub fn start_data_frame_analytics(
        &self,
        request: &MlStartDataFrameAnalyticsRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/data_frame/analytics/{id}/_start")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-start-datafeed.html"]
    pub fn start_datafeed(&self, request: &MlStartDatafeedRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/datafeeds/{datafeed_id}/_start")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/stop-dfanalytics.html"]
    pub fn stop_data_frame_analytics(
        &self,
        request: &MlStopDataFrameAnalyticsRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/data_frame/analytics/{id}/_stop")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-stop-datafeed.html"]
    pub fn stop_datafeed(&self, request: &MlStopDatafeedRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/datafeeds/{datafeed_id}/_stop")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-datafeed.html"]
    pub fn update_datafeed(&self, request: &MlUpdateDatafeedRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/datafeeds/{datafeed_id}/_update")
    }
    pub fn update_filter(&self, request: &MlUpdateFilterRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/filters/{filter_id}/_update")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-job.html"]
    pub fn update_job(&self, request: &MlUpdateJobRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/anomaly_detectors/{job_id}/_update")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-snapshot.html"]
    pub fn update_model_snapshot(
        &self,
        request: &MlUpdateModelSnapshotRequest,
    ) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_ml/anomaly_detectors/{job_id}/model_snapshots/{snapshot_id}/_update",
        )
    }
    pub fn validate(&self, request: &MlValidateRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/anomaly_detectors/_validate")
    }
    pub fn validate_detector(&self, request: &MlValidateDetectorRequest) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_ml/anomaly_detectors/_validate/detector",
        )
    }
}
impl ElasticsearchClient {
    #[doc = "Ml APIs"]
    pub fn ml(&self) -> MlNamespaceClient {
        MlNamespaceClient::new(self)
    }
}
