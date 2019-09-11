

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
use serde::Deserialize;
pub struct MlNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl MlNamespaceClient {
    pub fn new(client: &ElasticsearchClient) -> Self {
        MlNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-close-job.html"]
    pub fn close_job(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/anomaly_detectors/{job_id}/_close")
    }
    pub fn delete_calendar(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ml/calendars/{calendar_id}")
    }
    pub fn delete_calendar_event(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Delete,
            "/_ml/calendars/{calendar_id}/events/{event_id}",
        )
    }
    pub fn delete_calendar_job(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Delete,
            "/_ml/calendars/{calendar_id}/jobs/{job_id}",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/delete-dfanalytics.html"]
    pub fn delete_data_frame_analytics(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ml/data_frame/analytics/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-datafeed.html"]
    pub fn delete_datafeed(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ml/datafeeds/{datafeed_id}")
    }
    pub fn delete_expired_data(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ml/_delete_expired_data")
    }
    pub fn delete_filter(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ml/filters/{filter_id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-forecast.html"]
    pub fn delete_forecast(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Delete,
            "/_ml/anomaly_detectors/{job_id}/_forecast",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-job.html"]
    pub fn delete_job(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ml/anomaly_detectors/{job_id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-delete-snapshot.html"]
    pub fn delete_model_snapshot(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Delete,
            "/_ml/anomaly_detectors/{job_id}/model_snapshots/{snapshot_id}",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/evaluate-dfanalytics.html"]
    pub fn evaluate_data_frame(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/data_frame/_evaluate")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-find-file-structure.html"]
    pub fn find_file_structure(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/find_file_structure")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-flush-job.html"]
    pub fn flush_job(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/anomaly_detectors/{job_id}/_flush")
    }
    pub fn forecast(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_ml/anomaly_detectors/{job_id}/_forecast",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-bucket.html"]
    pub fn get_buckets(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_ml/anomaly_detectors/{job_id}/results/buckets/{timestamp}",
        )
    }
    pub fn get_calendar_events(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/calendars/{calendar_id}/events")
    }
    pub fn get_calendars(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ml/calendars")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-category.html"]
    pub fn get_categories(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_ml/anomaly_detectors/{job_id}/results/categories/{category_id}",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/get-dfanalytics.html"]
    pub fn get_data_frame_analytics(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/data_frame/analytics/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/get-dfanalytics-stats.html"]
    pub fn get_data_frame_analytics_stats(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/data_frame/analytics/_stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-datafeed-stats.html"]
    pub fn get_datafeed_stats(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/datafeeds/{datafeed_id}/_stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-datafeed.html"]
    pub fn get_datafeeds(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/datafeeds/{datafeed_id}")
    }
    pub fn get_filters(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ml/filters")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-influencer.html"]
    pub fn get_influencers(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_ml/anomaly_detectors/{job_id}/results/influencers",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-job-stats.html"]
    pub fn get_job_stats(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/anomaly_detectors/_stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-job.html"]
    pub fn get_jobs(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/anomaly_detectors/{job_id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-snapshot.html"]
    pub fn get_model_snapshots(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_ml/anomaly_detectors/{job_id}/model_snapshots/{snapshot_id}",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-overall-buckets.html"]
    pub fn get_overall_buckets(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_ml/anomaly_detectors/{job_id}/results/overall_buckets",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-get-record.html"]
    pub fn get_records(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Get,
            "/_ml/anomaly_detectors/{job_id}/results/records",
        )
    }
    pub fn info(&self) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ml/info")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-open-job.html"]
    pub fn open_job(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/anomaly_detectors/{job_id}/_open")
    }
    pub fn post_calendar_events(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/calendars/{calendar_id}/events")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-post-data.html"]
    pub fn post_data(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/anomaly_detectors/{job_id}/_data")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-preview-datafeed.html"]
    pub fn preview_datafeed(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_ml/datafeeds/{datafeed_id}/_preview")
    }
    pub fn put_calendar(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_ml/calendars/{calendar_id}")
    }
    pub fn put_calendar_job(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Put,
            "/_ml/calendars/{calendar_id}/jobs/{job_id}",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/put-dfanalytics.html"]
    pub fn put_data_frame_analytics(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_ml/data_frame/analytics/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-datafeed.html"]
    pub fn put_datafeed(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_ml/datafeeds/{datafeed_id}")
    }
    pub fn put_filter(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_ml/filters/{filter_id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-put-job.html"]
    pub fn put_job(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_ml/anomaly_detectors/{job_id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-revert-snapshot.html"]
    pub fn revert_model_snapshot(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_ml/anomaly_detectors/{job_id}/model_snapshots/{snapshot_id}/_revert",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-set-upgrade-mode.html"]
    pub fn set_upgrade_mode(&self) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_ml/set_upgrade_mode")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/start-dfanalytics.html"]
    pub fn start_data_frame_analytics(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/data_frame/analytics/{id}/_start")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-start-datafeed.html"]
    pub fn start_datafeed(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/datafeeds/{datafeed_id}/_start")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/stop-dfanalytics.html"]
    pub fn stop_data_frame_analytics(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/data_frame/analytics/{id}/_stop")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-stop-datafeed.html"]
    pub fn stop_datafeed(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/datafeeds/{datafeed_id}/_stop")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-datafeed.html"]
    pub fn update_datafeed(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/datafeeds/{datafeed_id}/_update")
    }
    pub fn update_filter(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/filters/{filter_id}/_update")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-job.html"]
    pub fn update_job(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/anomaly_detectors/{job_id}/_update")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/ml-update-snapshot.html"]
    pub fn update_model_snapshot(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_ml/anomaly_detectors/{job_id}/model_snapshots/{snapshot_id}/_update",
        )
    }
    pub fn validate(&self) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_ml/anomaly_detectors/_validate")
    }
    pub fn validate_detector(&self) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_ml/anomaly_detectors/_validate/detector",
        )
    }
}
impl ElasticsearchClient {
    pub fn ml(&self) -> MlNamespaceClient {
        MlNamespaceClient::new(self)
    }
}
