

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct WatcherAckWatchBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl WatcherAckWatchBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        WatcherAckWatchBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for WatcherAckWatchBuilder {
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
pub struct WatcherActivateWatchBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl WatcherActivateWatchBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        WatcherActivateWatchBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for WatcherActivateWatchBuilder {
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
pub struct WatcherDeactivateWatchBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl WatcherDeactivateWatchBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        WatcherDeactivateWatchBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for WatcherDeactivateWatchBuilder {
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
pub struct WatcherDeleteWatchBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl WatcherDeleteWatchBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        WatcherDeleteWatchBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for WatcherDeleteWatchBuilder {
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
pub struct WatcherExecuteWatchBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    debug: Option<bool>,
}
impl WatcherExecuteWatchBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        WatcherExecuteWatchBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for WatcherExecuteWatchBuilder {
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
pub struct WatcherGetWatchBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl WatcherGetWatchBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        WatcherGetWatchBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for WatcherGetWatchBuilder {
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
pub struct WatcherPutWatchBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    active: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    version: Option<i64>,
}
impl WatcherPutWatchBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        WatcherPutWatchBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for WatcherPutWatchBuilder {
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
pub struct WatcherStartBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl WatcherStartBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        WatcherStartBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for WatcherStartBuilder {
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
pub struct WatcherStatsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    emit_stacktraces: Option<bool>,
    metric: Option<Vec<String>>,
}
impl WatcherStatsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        WatcherStatsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for WatcherStatsBuilder {
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
pub struct WatcherStopBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl WatcherStopBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        WatcherStopBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for WatcherStopBuilder {
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
#[doc = "Watcher APIs"]
pub struct WatcherClient {
    client: ElasticsearchClient,
}
impl WatcherClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        WatcherClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-ack-watch.html"]
    pub fn ack_watch(&self) -> WatcherAckWatchBuilder {
        WatcherAckWatchBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-activate-watch.html"]
    pub fn activate_watch(&self) -> WatcherActivateWatchBuilder {
        WatcherActivateWatchBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-deactivate-watch.html"]
    pub fn deactivate_watch(&self) -> WatcherDeactivateWatchBuilder {
        WatcherDeactivateWatchBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-delete-watch.html"]
    pub fn delete_watch(&self) -> WatcherDeleteWatchBuilder {
        WatcherDeleteWatchBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-execute-watch.html"]
    pub fn execute_watch(&self) -> WatcherExecuteWatchBuilder {
        WatcherExecuteWatchBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-get-watch.html"]
    pub fn get_watch(&self) -> WatcherGetWatchBuilder {
        WatcherGetWatchBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-put-watch.html"]
    pub fn put_watch(&self) -> WatcherPutWatchBuilder {
        WatcherPutWatchBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-start.html"]
    pub fn start(&self) -> WatcherStartBuilder {
        WatcherStartBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-stats.html"]
    pub fn stats(&self) -> WatcherStatsBuilder {
        WatcherStatsBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-stop.html"]
    pub fn stop(&self) -> WatcherStopBuilder {
        WatcherStopBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Watcher APIs"]
    pub fn watcher(&self) -> WatcherClient {
        WatcherClient::new(self.clone())
    }
}
