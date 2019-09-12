use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct SnapshotCreateRequest<'a> {
    master_timeout: &'a String,
    wait_for_completion: Option<&'a bool>,
}
pub struct SnapshotCreateRequestBuilder<'a> {
    master_timeout: &'a String,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> SnapshotCreateRequestBuilder<'a> {
    pub fn build(&self) -> SnapshotCreateRequest<'a> {
        SnapshotCreateRequest {
            master_timeout: self.master_timeout,
            wait_for_completion: self.wait_for_completion,
        }
    }
}
pub struct SnapshotCreateRepositoryRequest<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
    verify: Option<&'a bool>,
}
pub struct SnapshotCreateRepositoryRequestBuilder<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
    verify: Option<&'a bool>,
}
impl<'a> SnapshotCreateRepositoryRequestBuilder<'a> {
    pub fn build(&self) -> SnapshotCreateRepositoryRequest<'a> {
        SnapshotCreateRepositoryRequest {
            master_timeout: self.master_timeout,
            timeout: self.timeout,
            verify: self.verify,
        }
    }
}
pub struct SnapshotDeleteRequest<'a> {
    master_timeout: &'a String,
}
pub struct SnapshotDeleteRequestBuilder<'a> {
    master_timeout: &'a String,
}
impl<'a> SnapshotDeleteRequestBuilder<'a> {
    pub fn build(&self) -> SnapshotDeleteRequest<'a> {
        SnapshotDeleteRequest {
            master_timeout: self.master_timeout,
        }
    }
}
pub struct SnapshotDeleteRepositoryRequest<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct SnapshotDeleteRepositoryRequestBuilder<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> SnapshotDeleteRepositoryRequestBuilder<'a> {
    pub fn build(&self) -> SnapshotDeleteRepositoryRequest<'a> {
        SnapshotDeleteRepositoryRequest {
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
pub struct SnapshotGetRequest<'a> {
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    verbose: Option<&'a bool>,
}
pub struct SnapshotGetRequestBuilder<'a> {
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
    verbose: Option<&'a bool>,
}
impl<'a> SnapshotGetRequestBuilder<'a> {
    pub fn build(&self) -> SnapshotGetRequest<'a> {
        SnapshotGetRequest {
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
            verbose: self.verbose,
        }
    }
}
pub struct SnapshotGetRepositoryRequest<'a> {
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
pub struct SnapshotGetRepositoryRequestBuilder<'a> {
    local: Option<&'a bool>,
    master_timeout: &'a String,
}
impl<'a> SnapshotGetRepositoryRequestBuilder<'a> {
    pub fn build(&self) -> SnapshotGetRepositoryRequest<'a> {
        SnapshotGetRepositoryRequest {
            local: self.local,
            master_timeout: self.master_timeout,
        }
    }
}
pub struct SnapshotRestoreRequest<'a> {
    master_timeout: &'a String,
    wait_for_completion: Option<&'a bool>,
}
pub struct SnapshotRestoreRequestBuilder<'a> {
    master_timeout: &'a String,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> SnapshotRestoreRequestBuilder<'a> {
    pub fn build(&self) -> SnapshotRestoreRequest<'a> {
        SnapshotRestoreRequest {
            master_timeout: self.master_timeout,
            wait_for_completion: self.wait_for_completion,
        }
    }
}
pub struct SnapshotStatusRequest<'a> {
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
}
pub struct SnapshotStatusRequestBuilder<'a> {
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a String,
}
impl<'a> SnapshotStatusRequestBuilder<'a> {
    pub fn build(&self) -> SnapshotStatusRequest<'a> {
        SnapshotStatusRequest {
            ignore_unavailable: self.ignore_unavailable,
            master_timeout: self.master_timeout,
        }
    }
}
pub struct SnapshotVerifyRepositoryRequest<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
pub struct SnapshotVerifyRepositoryRequestBuilder<'a> {
    master_timeout: &'a String,
    timeout: &'a String,
}
impl<'a> SnapshotVerifyRepositoryRequestBuilder<'a> {
    pub fn build(&self) -> SnapshotVerifyRepositoryRequest<'a> {
        SnapshotVerifyRepositoryRequest {
            master_timeout: self.master_timeout,
            timeout: self.timeout,
        }
    }
}
#[doc = "Snapshot APIs"]
pub struct SnapshotNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> SnapshotNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SnapshotNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn create(&self, request: &SnapshotCreateRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Put, "/_snapshot/{repository}/{snapshot}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn create_repository(&self, request: &SnapshotCreateRepositoryRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_snapshot/{repository}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn delete(&self, request: &SnapshotDeleteRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_snapshot/{repository}/{snapshot}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn delete_repository(&self, request: &SnapshotDeleteRepositoryRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_snapshot/{repository}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn get(&self, request: &SnapshotGetRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Get, "/_snapshot/{repository}/{snapshot}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn get_repository(&self, request: &SnapshotGetRepositoryRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_snapshot")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn restore(&self, request: &SnapshotRestoreRequest) -> Result<Response> {
        self.client.send(
            HttpMethod::Post,
            "/_snapshot/{repository}/{snapshot}/_restore",
        )
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn status(&self, request: &SnapshotStatusRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_snapshot/_status")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn verify_repository(&self, request: &SnapshotVerifyRepositoryRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_snapshot/{repository}/_verify")
    }
}
impl ElasticsearchClient {
    #[doc = "Snapshot APIs"]
    pub fn snapshot(&self) -> SnapshotNamespaceClient {
        SnapshotNamespaceClient::new(self)
    }
}
