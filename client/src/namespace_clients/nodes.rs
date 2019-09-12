

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct NodesHotThreadsRequest<'a> {
    ignore_idle_threads: Option<&'a bool>,
    interval: &'a String,
    snapshots: Option<&'a i64>,
    threads: Option<&'a i64>,
    timeout: &'a String,
    ty: Option<&'a i32>,
}
pub struct NodesHotThreadsRequestBuilder<'a> {
    ignore_idle_threads: Option<&'a bool>,
    interval: &'a String,
    snapshots: Option<&'a i64>,
    threads: Option<&'a i64>,
    timeout: &'a String,
    ty: Option<&'a i32>,
}
impl<'a> NodesHotThreadsRequestBuilder<'a> {
    pub fn build(&self) -> NodesHotThreadsRequest<'a> {
        NodesHotThreadsRequest {
            ignore_idle_threads: self.ignore_idle_threads,
            interval: self.interval,
            snapshots: self.snapshots,
            threads: self.threads,
            timeout: self.timeout,
            ty: self.ty,
        }
    }
}
pub struct NodesInfoRequest<'a> {
    flat_settings: Option<&'a bool>,
    timeout: &'a String,
}
pub struct NodesInfoRequestBuilder<'a> {
    flat_settings: Option<&'a bool>,
    timeout: &'a String,
}
impl<'a> NodesInfoRequestBuilder<'a> {
    pub fn build(&self) -> NodesInfoRequest<'a> {
        NodesInfoRequest {
            flat_settings: self.flat_settings,
            timeout: self.timeout,
        }
    }
}
pub struct NodesReloadSecureSettingsRequest<'a> {
    timeout: &'a String,
}
pub struct NodesReloadSecureSettingsRequestBuilder<'a> {
    timeout: &'a String,
}
impl<'a> NodesReloadSecureSettingsRequestBuilder<'a> {
    pub fn build(&self) -> NodesReloadSecureSettingsRequest<'a> {
        NodesReloadSecureSettingsRequest {
            timeout: self.timeout,
        }
    }
}
pub struct NodesStatsRequest<'a> {
    completion_fields: &'a Vec<String>,
    fielddata_fields: &'a Vec<String>,
    fields: &'a Vec<String>,
    groups: Option<&'a bool>,
    include_segment_file_sizes: Option<&'a bool>,
    level: Option<&'a i32>,
    timeout: &'a String,
    types: &'a Vec<String>,
}
pub struct NodesStatsRequestBuilder<'a> {
    completion_fields: &'a Vec<String>,
    fielddata_fields: &'a Vec<String>,
    fields: &'a Vec<String>,
    groups: Option<&'a bool>,
    include_segment_file_sizes: Option<&'a bool>,
    level: Option<&'a i32>,
    timeout: &'a String,
    types: &'a Vec<String>,
}
impl<'a> NodesStatsRequestBuilder<'a> {
    pub fn build(&self) -> NodesStatsRequest<'a> {
        NodesStatsRequest {
            completion_fields: self.completion_fields,
            fielddata_fields: self.fielddata_fields,
            fields: self.fields,
            groups: self.groups,
            include_segment_file_sizes: self.include_segment_file_sizes,
            level: self.level,
            timeout: self.timeout,
            types: self.types,
        }
    }
}
pub struct NodesUsageRequest<'a> {
    timeout: &'a String,
}
pub struct NodesUsageRequestBuilder<'a> {
    timeout: &'a String,
}
impl<'a> NodesUsageRequestBuilder<'a> {
    pub fn build(&self) -> NodesUsageRequest<'a> {
        NodesUsageRequest {
            timeout: self.timeout,
        }
    }
}
#[doc = "Nodes APIs"]
pub struct NodesNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> NodesNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        NodesNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-hot-threads.html"]
    pub fn hot_threads(&self, request: &NodesHotThreadsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_nodes/hot_threads")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-info.html"]
    pub fn info(&self, request: &NodesInfoRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_nodes")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/secure-settings.html#reloadable-secure-settings"]
    pub fn reload_secure_settings(
        &self,
        request: &NodesReloadSecureSettingsRequest,
    ) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_nodes/reload_secure_settings")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html"]
    pub fn stats(&self, request: &NodesStatsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_nodes/stats")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-usage.html"]
    pub fn usage(&self, request: &NodesUsageRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_nodes/usage")
    }
}
impl ElasticsearchClient {
    #[doc = "Nodes APIs"]
    pub fn nodes(&self) -> NodesNamespaceClient {
        NodesNamespaceClient::new(self)
    }
}
