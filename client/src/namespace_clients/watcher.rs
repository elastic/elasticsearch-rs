

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[Default]
pub struct WatcherAckWatchRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> WatcherAckWatchRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        WatcherAckWatchRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for WatcherAckWatchRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct WatcherActivateWatchRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> WatcherActivateWatchRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        WatcherActivateWatchRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for WatcherActivateWatchRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct WatcherDeactivateWatchRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> WatcherDeactivateWatchRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        WatcherDeactivateWatchRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for WatcherDeactivateWatchRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct WatcherDeleteWatchRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> WatcherDeleteWatchRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        WatcherDeleteWatchRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for WatcherDeleteWatchRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct WatcherExecuteWatchRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    debug: Option<&'a bool>,
}
impl<'a> WatcherExecuteWatchRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        WatcherExecuteWatchRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for WatcherExecuteWatchRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct WatcherGetWatchRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> WatcherGetWatchRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        WatcherGetWatchRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for WatcherGetWatchRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct WatcherPutWatchRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    active: Option<&'a bool>,
    if_primary_term: Option<&'a i64>,
    if_seq_no: Option<&'a i64>,
    version: Option<&'a i64>,
}
impl<'a> WatcherPutWatchRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        WatcherPutWatchRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for WatcherPutWatchRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct WatcherStartRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> WatcherStartRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        WatcherStartRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for WatcherStartRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct WatcherStatsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    emit_stacktraces: Option<&'a bool>,
    metric: &'a Vec<String>,
}
impl<'a> WatcherStatsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        WatcherStatsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for WatcherStatsRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[Default]
pub struct WatcherStopRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> WatcherStopRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        WatcherStopRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for WatcherStopRequestBuilder<'a> {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode(200),
            body: None,
        })
    }
}
#[doc = "Watcher APIs"]
pub struct WatcherNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> WatcherNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        WatcherNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-ack-watch.html"]
    pub fn ack_watch(&self) -> WatcherAckWatchRequestBuilder {
        WatcherAckWatchRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-activate-watch.html"]
    pub fn activate_watch(&self) -> WatcherActivateWatchRequestBuilder {
        WatcherActivateWatchRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-deactivate-watch.html"]
    pub fn deactivate_watch(&self) -> WatcherDeactivateWatchRequestBuilder {
        WatcherDeactivateWatchRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-delete-watch.html"]
    pub fn delete_watch(&self) -> WatcherDeleteWatchRequestBuilder {
        WatcherDeleteWatchRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-execute-watch.html"]
    pub fn execute_watch(&self) -> WatcherExecuteWatchRequestBuilder {
        WatcherExecuteWatchRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-get-watch.html"]
    pub fn get_watch(&self) -> WatcherGetWatchRequestBuilder {
        WatcherGetWatchRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-put-watch.html"]
    pub fn put_watch(&self) -> WatcherPutWatchRequestBuilder {
        WatcherPutWatchRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-start.html"]
    pub fn start(&self) -> WatcherStartRequestBuilder {
        WatcherStartRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-stats.html"]
    pub fn stats(&self) -> WatcherStatsRequestBuilder {
        WatcherStatsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-stop.html"]
    pub fn stop(&self) -> WatcherStopRequestBuilder {
        WatcherStopRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Watcher APIs"]
    pub fn watcher(&self) -> WatcherNamespaceClient {
        WatcherNamespaceClient::new(self)
    }
}
