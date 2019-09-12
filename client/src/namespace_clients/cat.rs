

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[Default]
pub struct CatAliasesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatAliasesRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatAliasesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatAliasesRequestBuilder<'a> {
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
pub struct CatAllocationRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    bytes: Option<&'a i32>,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatAllocationRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatAllocationRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatAllocationRequestBuilder<'a> {
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
pub struct CatCountRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatCountRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatCountRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatCountRequestBuilder<'a> {
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
pub struct CatFielddataRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    bytes: Option<&'a i32>,
    fields: &'a Vec<String>,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatFielddataRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatFielddataRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatFielddataRequestBuilder<'a> {
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
pub struct CatHealthRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    ts: Option<&'a bool>,
    v: Option<&'a bool>,
}
impl<'a> CatHealthRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatHealthRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatHealthRequestBuilder<'a> {
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
pub struct CatHelpRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    help: Option<&'a bool>,
    s: &'a Vec<String>,
}
impl<'a> CatHelpRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatHelpRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatHelpRequestBuilder<'a> {
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
pub struct CatIndicesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    bytes: Option<&'a i32>,
    format: &'a str,
    h: &'a Vec<String>,
    health: Option<&'a i32>,
    help: Option<&'a bool>,
    include_unloaded_segments: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    pri: Option<&'a bool>,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatIndicesRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatIndicesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatIndicesRequestBuilder<'a> {
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
pub struct CatMasterRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatMasterRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatMasterRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatMasterRequestBuilder<'a> {
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
pub struct CatNodeattrsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatNodeattrsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatNodeattrsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatNodeattrsRequestBuilder<'a> {
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
pub struct CatNodesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    format: &'a str,
    full_id: Option<&'a bool>,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatNodesRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatNodesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatNodesRequestBuilder<'a> {
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
pub struct CatPendingTasksRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatPendingTasksRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatPendingTasksRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatPendingTasksRequestBuilder<'a> {
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
pub struct CatPluginsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatPluginsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatPluginsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatPluginsRequestBuilder<'a> {
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
pub struct CatRecoveryRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    bytes: Option<&'a i32>,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatRecoveryRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatRecoveryRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatRecoveryRequestBuilder<'a> {
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
pub struct CatRepositoriesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatRepositoriesRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatRepositoriesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatRepositoriesRequestBuilder<'a> {
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
pub struct CatSegmentsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    bytes: Option<&'a i32>,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatSegmentsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatSegmentsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatSegmentsRequestBuilder<'a> {
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
pub struct CatShardsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    bytes: Option<&'a i32>,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatShardsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatShardsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatShardsRequestBuilder<'a> {
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
pub struct CatSnapshotsRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatSnapshotsRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatSnapshotsRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatSnapshotsRequestBuilder<'a> {
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
pub struct CatTasksRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    actions: &'a Vec<String>,
    detailed: Option<&'a bool>,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    node_id: &'a Vec<String>,
    parent_task: Option<&'a i64>,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatTasksRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatTasksRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatTasksRequestBuilder<'a> {
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
pub struct CatTemplatesRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatTemplatesRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatTemplatesRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatTemplatesRequestBuilder<'a> {
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
pub struct CatThreadPoolRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    format: &'a str,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a str,
    s: &'a Vec<String>,
    size: Option<&'a i32>,
    v: Option<&'a bool>,
}
impl<'a> CatThreadPoolRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatThreadPoolRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for CatThreadPoolRequestBuilder<'a> {
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
#[doc = "Cat APIs"]
pub struct CatNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> CatNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        CatNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-alias.html"]
    pub fn aliases(&self) -> CatAliasesRequestBuilder {
        CatAliasesRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-allocation.html"]
    pub fn allocation(&self) -> CatAllocationRequestBuilder {
        CatAllocationRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-count.html"]
    pub fn count(&self) -> CatCountRequestBuilder {
        CatCountRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-fielddata.html"]
    pub fn fielddata(&self) -> CatFielddataRequestBuilder {
        CatFielddataRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-health.html"]
    pub fn health(&self) -> CatHealthRequestBuilder {
        CatHealthRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat.html"]
    pub fn help(&self) -> CatHelpRequestBuilder {
        CatHelpRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-indices.html"]
    pub fn indices(&self) -> CatIndicesRequestBuilder {
        CatIndicesRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-master.html"]
    pub fn master(&self) -> CatMasterRequestBuilder {
        CatMasterRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-nodeattrs.html"]
    pub fn nodeattrs(&self) -> CatNodeattrsRequestBuilder {
        CatNodeattrsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-nodes.html"]
    pub fn nodes(&self) -> CatNodesRequestBuilder {
        CatNodesRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-pending-tasks.html"]
    pub fn pending_tasks(&self) -> CatPendingTasksRequestBuilder {
        CatPendingTasksRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-plugins.html"]
    pub fn plugins(&self) -> CatPluginsRequestBuilder {
        CatPluginsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-recovery.html"]
    pub fn recovery(&self) -> CatRecoveryRequestBuilder {
        CatRecoveryRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-repositories.html"]
    pub fn repositories(&self) -> CatRepositoriesRequestBuilder {
        CatRepositoriesRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-segments.html"]
    pub fn segments(&self) -> CatSegmentsRequestBuilder {
        CatSegmentsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-shards.html"]
    pub fn shards(&self) -> CatShardsRequestBuilder {
        CatShardsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-snapshots.html"]
    pub fn snapshots(&self) -> CatSnapshotsRequestBuilder {
        CatSnapshotsRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn tasks(&self) -> CatTasksRequestBuilder {
        CatTasksRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-templates.html"]
    pub fn templates(&self) -> CatTemplatesRequestBuilder {
        CatTemplatesRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-thread-pool.html"]
    pub fn thread_pool(&self) -> CatThreadPoolRequestBuilder {
        CatThreadPoolRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Cat APIs"]
    pub fn cat(&self) -> CatNamespaceClient {
        CatNamespaceClient::new(self)
    }
}
