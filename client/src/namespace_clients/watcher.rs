

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct WatcherAckWatchRequest<'a> {}
pub struct WatcherAckWatchRequestBuilder<'a> {}
impl<'a> WatcherAckWatchRequestBuilder<'a> {
    pub fn build(&self) -> WatcherAckWatchRequest<'a> {
        WatcherAckWatchRequest {}
    }
}
pub struct WatcherActivateWatchRequest<'a> {}
pub struct WatcherActivateWatchRequestBuilder<'a> {}
impl<'a> WatcherActivateWatchRequestBuilder<'a> {
    pub fn build(&self) -> WatcherActivateWatchRequest<'a> {
        WatcherActivateWatchRequest {}
    }
}
pub struct WatcherDeactivateWatchRequest<'a> {}
pub struct WatcherDeactivateWatchRequestBuilder<'a> {}
impl<'a> WatcherDeactivateWatchRequestBuilder<'a> {
    pub fn build(&self) -> WatcherDeactivateWatchRequest<'a> {
        WatcherDeactivateWatchRequest {}
    }
}
pub struct WatcherDeleteWatchRequest<'a> {}
pub struct WatcherDeleteWatchRequestBuilder<'a> {}
impl<'a> WatcherDeleteWatchRequestBuilder<'a> {
    pub fn build(&self) -> WatcherDeleteWatchRequest<'a> {
        WatcherDeleteWatchRequest {}
    }
}
pub struct WatcherExecuteWatchRequest<'a> {
    debug: Option<&'a bool>,
}
pub struct WatcherExecuteWatchRequestBuilder<'a> {
    debug: Option<&'a bool>,
}
impl<'a> WatcherExecuteWatchRequestBuilder<'a> {
    pub fn build(&self) -> WatcherExecuteWatchRequest<'a> {
        WatcherExecuteWatchRequest { debug: self.debug }
    }
}
pub struct WatcherGetWatchRequest<'a> {}
pub struct WatcherGetWatchRequestBuilder<'a> {}
impl<'a> WatcherGetWatchRequestBuilder<'a> {
    pub fn build(&self) -> WatcherGetWatchRequest<'a> {
        WatcherGetWatchRequest {}
    }
}
pub struct WatcherPutWatchRequest<'a> {
    active: Option<&'a bool>,
    if_primary_term: Option<&'a i64>,
    if_seq_no: Option<&'a i64>,
    version: Option<&'a i64>,
}
pub struct WatcherPutWatchRequestBuilder<'a> {
    active: Option<&'a bool>,
    if_primary_term: Option<&'a i64>,
    if_seq_no: Option<&'a i64>,
    version: Option<&'a i64>,
}
impl<'a> WatcherPutWatchRequestBuilder<'a> {
    pub fn build(&self) -> WatcherPutWatchRequest<'a> {
        WatcherPutWatchRequest {
            active: self.active,
            if_primary_term: self.if_primary_term,
            if_seq_no: self.if_seq_no,
            version: self.version,
        }
    }
}
pub struct WatcherStartRequest<'a> {}
pub struct WatcherStartRequestBuilder<'a> {}
impl<'a> WatcherStartRequestBuilder<'a> {
    pub fn build(&self) -> WatcherStartRequest<'a> {
        WatcherStartRequest {}
    }
}
pub struct WatcherStatsRequest<'a> {
    emit_stacktraces: Option<&'a bool>,
    metric: &'a Vec<String>,
}
pub struct WatcherStatsRequestBuilder<'a> {
    emit_stacktraces: Option<&'a bool>,
    metric: &'a Vec<String>,
}
impl<'a> WatcherStatsRequestBuilder<'a> {
    pub fn build(&self) -> WatcherStatsRequest<'a> {
        WatcherStatsRequest {
            emit_stacktraces: self.emit_stacktraces,
            metric: self.metric,
        }
    }
}
pub struct WatcherStopRequest<'a> {}
pub struct WatcherStopRequestBuilder<'a> {}
impl<'a> WatcherStopRequestBuilder<'a> {
    pub fn build(&self) -> WatcherStopRequest<'a> {
        WatcherStopRequest {}
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
    pub fn ack_watch(&self, request: &WatcherAckWatchRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_watcher/watch/{watch_id}/_ack")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-activate-watch.html"]
    pub fn activate_watch(&self, request: &WatcherActivateWatchRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_watcher/watch/{watch_id}/_activate")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-deactivate-watch.html"]
    pub fn deactivate_watch(&self, request: &WatcherDeactivateWatchRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_watcher/watch/{watch_id}/_deactivate")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-delete-watch.html"]
    pub fn delete_watch(&self, request: &WatcherDeleteWatchRequest) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_watcher/watch/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-execute-watch.html"]
    pub fn execute_watch(&self, request: &WatcherExecuteWatchRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_watcher/watch/{id}/_execute")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-get-watch.html"]
    pub fn get_watch(&self, request: &WatcherGetWatchRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_watcher/watch/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-put-watch.html"]
    pub fn put_watch(&self, request: &WatcherPutWatchRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_watcher/watch/{id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-start.html"]
    pub fn start(&self, request: &WatcherStartRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_watcher/_start")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-stats.html"]
    pub fn stats(&self, request: &WatcherStatsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_watcher/stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/current/watcher-api-stop.html"]
    pub fn stop(&self, request: &WatcherStopRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_watcher/_stop")
    }
}
impl ElasticsearchClient {
    #[doc = "Watcher APIs"]
    pub fn watcher(&self) -> WatcherNamespaceClient {
        WatcherNamespaceClient::new(self)
    }
}
