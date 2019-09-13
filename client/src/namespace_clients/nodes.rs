

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct NodesHotThreadsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    ignore_idle_threads: Option<&'a bool>,
    interval: &'a str,
    snapshots: Option<&'a i64>,
    threads: Option<&'a i64>,
    timeout: &'a str,
    ty: Option<&'a i32>,
}
impl<'a> NodesHotThreadsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        NodesHotThreadsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for NodesHotThreadsRequestBuilder<'a> {
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
pub struct NodesInfoRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    flat_settings: Option<&'a bool>,
    timeout: &'a str,
}
impl<'a> NodesInfoRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        NodesInfoRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for NodesInfoRequestBuilder<'a> {
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
pub struct NodesReloadSecureSettingsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    timeout: &'a str,
}
impl<'a> NodesReloadSecureSettingsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        NodesReloadSecureSettingsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for NodesReloadSecureSettingsRequestBuilder<'a> {
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
pub struct NodesStatsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    completion_fields: Option<&'a Vec<String>>,
    fielddata_fields: Option<&'a Vec<String>>,
    fields: Option<&'a Vec<String>>,
    groups: Option<&'a bool>,
    include_segment_file_sizes: Option<&'a bool>,
    level: Option<&'a i32>,
    timeout: &'a str,
    types: Option<&'a Vec<String>>,
}
impl<'a> NodesStatsRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        NodesStatsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for NodesStatsRequestBuilder<'a> {
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
pub struct NodesUsageRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    timeout: &'a str,
}
impl<'a> NodesUsageRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        NodesUsageRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for NodesUsageRequestBuilder<'a> {
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
#[doc = "Nodes APIs"]
pub struct NodesNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> NodesNamespaceClient<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        NodesNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-hot-threads.html"]
    pub fn hot_threads(&self) -> NodesHotThreadsRequestBuilder {
        NodesHotThreadsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-info.html"]
    pub fn info(&self) -> NodesInfoRequestBuilder {
        NodesInfoRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/secure-settings.html#reloadable-secure-settings"]
    pub fn reload_secure_settings(&self) -> NodesReloadSecureSettingsRequestBuilder {
        NodesReloadSecureSettingsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html"]
    pub fn stats(&self) -> NodesStatsRequestBuilder {
        NodesStatsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-usage.html"]
    pub fn usage(&self) -> NodesUsageRequestBuilder {
        NodesUsageRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Nodes APIs"]
    pub fn nodes(&self) -> NodesNamespaceClient {
        NodesNamespaceClient::new(self)
    }
}
