use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct CatAliasesRequest<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatAliasesRequestBuilder<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatAliasesRequestBuilder<'a> {
    pub fn build(&self) -> CatAliasesRequest<'a> {
        CatAliasesRequest {
            format: self.format,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatAllocationRequest<'a> {
    bytes: Option<&'a i32>,
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatAllocationRequestBuilder<'a> {
    bytes: Option<&'a i32>,
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatAllocationRequestBuilder<'a> {
    pub fn build(&self) -> CatAllocationRequest<'a> {
        CatAllocationRequest {
            bytes: self.bytes,
            format: self.format,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatCountRequest<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatCountRequestBuilder<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatCountRequestBuilder<'a> {
    pub fn build(&self) -> CatCountRequest<'a> {
        CatCountRequest {
            format: self.format,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatFielddataRequest<'a> {
    bytes: Option<&'a i32>,
    fields: &'a Vec<String>,
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatFielddataRequestBuilder<'a> {
    bytes: Option<&'a i32>,
    fields: &'a Vec<String>,
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatFielddataRequestBuilder<'a> {
    pub fn build(&self) -> CatFielddataRequest<'a> {
        CatFielddataRequest {
            bytes: self.bytes,
            fields: self.fields,
            format: self.format,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatHealthRequest<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    ts: Option<&'a bool>,
    v: Option<&'a bool>,
}
pub struct CatHealthRequestBuilder<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    ts: Option<&'a bool>,
    v: Option<&'a bool>,
}
impl<'a> CatHealthRequestBuilder<'a> {
    pub fn build(&self) -> CatHealthRequest<'a> {
        CatHealthRequest {
            format: self.format,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            ts: self.ts,
            v: self.v,
        }
    }
}
pub struct CatHelpRequest<'a> {
    help: Option<&'a bool>,
    s: &'a Vec<String>,
}
pub struct CatHelpRequestBuilder<'a> {
    help: Option<&'a bool>,
    s: &'a Vec<String>,
}
impl<'a> CatHelpRequestBuilder<'a> {
    pub fn build(&self) -> CatHelpRequest<'a> {
        CatHelpRequest {
            help: self.help,
            s: self.s,
        }
    }
}
pub struct CatIndicesRequest<'a> {
    bytes: Option<&'a i32>,
    format: &'a String,
    h: &'a Vec<String>,
    health: Option<&'a i32>,
    help: Option<&'a bool>,
    include_unloaded_segments: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    pri: Option<&'a bool>,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatIndicesRequestBuilder<'a> {
    bytes: Option<&'a i32>,
    format: &'a String,
    h: &'a Vec<String>,
    health: Option<&'a i32>,
    help: Option<&'a bool>,
    include_unloaded_segments: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    pri: Option<&'a bool>,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatIndicesRequestBuilder<'a> {
    pub fn build(&self) -> CatIndicesRequest<'a> {
        CatIndicesRequest {
            bytes: self.bytes,
            format: self.format,
            h: self.h,
            health: self.health,
            help: self.help,
            include_unloaded_segments: self.include_unloaded_segments,
            local: self.local,
            master_timeout: self.master_timeout,
            pri: self.pri,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatMasterRequest<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatMasterRequestBuilder<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatMasterRequestBuilder<'a> {
    pub fn build(&self) -> CatMasterRequest<'a> {
        CatMasterRequest {
            format: self.format,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatNodeattrsRequest<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatNodeattrsRequestBuilder<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatNodeattrsRequestBuilder<'a> {
    pub fn build(&self) -> CatNodeattrsRequest<'a> {
        CatNodeattrsRequest {
            format: self.format,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatNodesRequest<'a> {
    format: &'a String,
    full_id: Option<&'a bool>,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatNodesRequestBuilder<'a> {
    format: &'a String,
    full_id: Option<&'a bool>,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatNodesRequestBuilder<'a> {
    pub fn build(&self) -> CatNodesRequest<'a> {
        CatNodesRequest {
            format: self.format,
            full_id: self.full_id,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatPendingTasksRequest<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatPendingTasksRequestBuilder<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatPendingTasksRequestBuilder<'a> {
    pub fn build(&self) -> CatPendingTasksRequest<'a> {
        CatPendingTasksRequest {
            format: self.format,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatPluginsRequest<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatPluginsRequestBuilder<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatPluginsRequestBuilder<'a> {
    pub fn build(&self) -> CatPluginsRequest<'a> {
        CatPluginsRequest {
            format: self.format,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatRecoveryRequest<'a> {
    bytes: Option<&'a i32>,
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatRecoveryRequestBuilder<'a> {
    bytes: Option<&'a i32>,
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatRecoveryRequestBuilder<'a> {
    pub fn build(&self) -> CatRecoveryRequest<'a> {
        CatRecoveryRequest {
            bytes: self.bytes,
            format: self.format,
            h: self.h,
            help: self.help,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatRepositoriesRequest<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatRepositoriesRequestBuilder<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatRepositoriesRequestBuilder<'a> {
    pub fn build(&self) -> CatRepositoriesRequest<'a> {
        CatRepositoriesRequest {
            format: self.format,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatSegmentsRequest<'a> {
    bytes: Option<&'a i32>,
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatSegmentsRequestBuilder<'a> {
    bytes: Option<&'a i32>,
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatSegmentsRequestBuilder<'a> {
    pub fn build(&self) -> CatSegmentsRequest<'a> {
        CatSegmentsRequest {
            bytes: self.bytes,
            format: self.format,
            h: self.h,
            help: self.help,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatShardsRequest<'a> {
    bytes: Option<&'a i32>,
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatShardsRequestBuilder<'a> {
    bytes: Option<&'a i32>,
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatShardsRequestBuilder<'a> {
    pub fn build(&self) -> CatShardsRequest<'a> {
        CatShardsRequest {
            bytes: self.bytes,
            format: self.format,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatSnapshotsRequest<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatSnapshotsRequestBuilder<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatSnapshotsRequestBuilder<'a> {
    pub fn build(&self) -> CatSnapshotsRequest<'a> {
        CatSnapshotsRequest {
            format: self.format,
            h: self.h,
            help: self.help,
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatTasksRequest<'a> {
    actions: &'a Vec<String>,
    detailed: Option<&'a bool>,
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    node_id: &'a Vec<String>,
    parent_task: Option<&'a i64>,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatTasksRequestBuilder<'a> {
    actions: &'a Vec<String>,
    detailed: Option<&'a bool>,
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    node_id: &'a Vec<String>,
    parent_task: Option<&'a i64>,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatTasksRequestBuilder<'a> {
    pub fn build(&self) -> CatTasksRequest<'a> {
        CatTasksRequest {
            actions: self.actions,
            detailed: self.detailed,
            format: self.format,
            h: self.h,
            help: self.help,
            node_id: self.node_id,
            parent_task: self.parent_task,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatTemplatesRequest<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
pub struct CatTemplatesRequestBuilder<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    v: Option<&'a bool>,
}
impl<'a> CatTemplatesRequestBuilder<'a> {
    pub fn build(&self) -> CatTemplatesRequest<'a> {
        CatTemplatesRequest {
            format: self.format,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            v: self.v,
        }
    }
}
pub struct CatThreadPoolRequest<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    size: Option<&'a i32>,
    v: Option<&'a bool>,
}
pub struct CatThreadPoolRequestBuilder<'a> {
    format: &'a String,
    h: &'a Vec<String>,
    help: Option<&'a bool>,
    local: Option<&'a bool>,
    master_timeout: &'a String,
    s: &'a Vec<String>,
    size: Option<&'a i32>,
    v: Option<&'a bool>,
}
impl<'a> CatThreadPoolRequestBuilder<'a> {
    pub fn build(&self) -> CatThreadPoolRequest<'a> {
        CatThreadPoolRequest {
            format: self.format,
            h: self.h,
            help: self.help,
            local: self.local,
            master_timeout: self.master_timeout,
            s: self.s,
            size: self.size,
            v: self.v,
        }
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
    pub fn aliases(&self, request: &CatAliasesRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/aliases")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-allocation.html"]
    pub fn allocation(&self, request: &CatAllocationRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/allocation")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-count.html"]
    pub fn count(&self, request: &CatCountRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/count")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-fielddata.html"]
    pub fn fielddata(&self, request: &CatFielddataRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/fielddata")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-health.html"]
    pub fn health(&self, request: &CatHealthRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/health")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat.html"]
    pub fn help(&self, request: &CatHelpRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-indices.html"]
    pub fn indices(&self, request: &CatIndicesRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/indices")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-master.html"]
    pub fn master(&self, request: &CatMasterRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/master")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-nodeattrs.html"]
    pub fn nodeattrs(&self, request: &CatNodeattrsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/nodeattrs")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-nodes.html"]
    pub fn nodes(&self, request: &CatNodesRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/nodes")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-pending-tasks.html"]
    pub fn pending_tasks(&self, request: &CatPendingTasksRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/pending_tasks")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-plugins.html"]
    pub fn plugins(&self, request: &CatPluginsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/plugins")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-recovery.html"]
    pub fn recovery(&self, request: &CatRecoveryRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/recovery")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-repositories.html"]
    pub fn repositories(&self, request: &CatRepositoriesRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/repositories")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-segments.html"]
    pub fn segments(&self, request: &CatSegmentsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/segments")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-shards.html"]
    pub fn shards(&self, request: &CatShardsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/shards")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-snapshots.html"]
    pub fn snapshots(&self, request: &CatSnapshotsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/snapshots")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn tasks(&self, request: &CatTasksRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/tasks")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-templates.html"]
    pub fn templates(&self, request: &CatTemplatesRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/templates")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-thread-pool.html"]
    pub fn thread_pool(&self, request: &CatThreadPoolRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_cat/thread_pool")
    }
}
impl ElasticsearchClient {
    #[doc = "Cat APIs"]
    pub fn cat(&self) -> CatNamespaceClient {
        CatNamespaceClient::new(self)
    }
}
