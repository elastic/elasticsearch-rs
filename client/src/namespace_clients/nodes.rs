

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct NodesHotThreadsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    ignore_idle_threads: Option<bool>,
    interval: Option<String>,
    snapshots: Option<i64>,
    threads: Option<i64>,
    timeout: Option<String>,
    ty: Option<i32>,
}
impl NodesHotThreadsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        NodesHotThreadsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for NodesHotThreadsBuilder {
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
pub struct NodesInfoBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    flat_settings: Option<bool>,
    timeout: Option<String>,
}
impl NodesInfoBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        NodesInfoBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for NodesInfoBuilder {
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
pub struct NodesReloadSecureSettingsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl NodesReloadSecureSettingsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        NodesReloadSecureSettingsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for NodesReloadSecureSettingsBuilder {
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
pub struct NodesStatsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    completion_fields: Option<Vec<String>>,
    fielddata_fields: Option<Vec<String>>,
    fields: Option<Vec<String>>,
    groups: Option<bool>,
    include_segment_file_sizes: Option<bool>,
    level: Option<i32>,
    timeout: Option<String>,
    types: Option<Vec<String>>,
}
impl NodesStatsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        NodesStatsBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for NodesStatsBuilder {
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
pub struct NodesUsageBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl NodesUsageBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        NodesUsageBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for NodesUsageBuilder {
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
pub struct NodesClient {
    client: ElasticsearchClient,
}
impl NodesClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        NodesClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-hot-threads.html"]
    pub fn hot_threads(&self) -> NodesHotThreadsBuilder {
        NodesHotThreadsBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-info.html"]
    pub fn info(&self) -> NodesInfoBuilder {
        NodesInfoBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/secure-settings.html#reloadable-secure-settings"]
    pub fn reload_secure_settings(&self) -> NodesReloadSecureSettingsBuilder {
        NodesReloadSecureSettingsBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html"]
    pub fn stats(&self) -> NodesStatsBuilder {
        NodesStatsBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-usage.html"]
    pub fn usage(&self) -> NodesUsageBuilder {
        NodesUsageBuilder::new(self.client.clone())
    }
}
impl ElasticsearchClient {
    #[doc = "Nodes APIs"]
    pub fn nodes(&self) -> NodesClient {
        NodesClient::new(self.clone())
    }
}
