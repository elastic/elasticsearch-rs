

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[Default]
pub struct SnapshotCreateRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    master_timeout: &'a str,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> SnapshotCreateRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SnapshotCreateRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SnapshotCreateRequestBuilder<'a> {
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
pub struct SnapshotCreateRepositoryRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    master_timeout: &'a str,
    timeout: &'a str,
    verify: Option<&'a bool>,
}
impl<'a> SnapshotCreateRepositoryRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SnapshotCreateRepositoryRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SnapshotCreateRepositoryRequestBuilder<'a> {
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
pub struct SnapshotDeleteRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    master_timeout: &'a str,
}
impl<'a> SnapshotDeleteRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SnapshotDeleteRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SnapshotDeleteRequestBuilder<'a> {
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
pub struct SnapshotDeleteRepositoryRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    master_timeout: &'a str,
    timeout: &'a str,
}
impl<'a> SnapshotDeleteRepositoryRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SnapshotDeleteRepositoryRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SnapshotDeleteRepositoryRequestBuilder<'a> {
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
pub struct SnapshotGetRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a str,
    verbose: Option<&'a bool>,
}
impl<'a> SnapshotGetRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SnapshotGetRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SnapshotGetRequestBuilder<'a> {
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
pub struct SnapshotGetRepositoryRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    local: Option<&'a bool>,
    master_timeout: &'a str,
}
impl<'a> SnapshotGetRepositoryRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SnapshotGetRepositoryRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SnapshotGetRepositoryRequestBuilder<'a> {
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
pub struct SnapshotRestoreRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    master_timeout: &'a str,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> SnapshotRestoreRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SnapshotRestoreRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SnapshotRestoreRequestBuilder<'a> {
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
pub struct SnapshotStatusRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    ignore_unavailable: Option<&'a bool>,
    master_timeout: &'a str,
}
impl<'a> SnapshotStatusRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SnapshotStatusRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SnapshotStatusRequestBuilder<'a> {
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
pub struct SnapshotVerifyRepositoryRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    master_timeout: &'a str,
    timeout: &'a str,
}
impl<'a> SnapshotVerifyRepositoryRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SnapshotVerifyRepositoryRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for SnapshotVerifyRepositoryRequestBuilder<'a> {
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
#[doc = "Snapshot APIs"]
pub struct SnapshotNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> SnapshotNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        SnapshotNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn create(&self) -> SnapshotCreateRequestBuilder {
        SnapshotCreateRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn create_repository(&self) -> SnapshotCreateRepositoryRequestBuilder {
        SnapshotCreateRepositoryRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn delete(&self) -> SnapshotDeleteRequestBuilder {
        SnapshotDeleteRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn delete_repository(&self) -> SnapshotDeleteRepositoryRequestBuilder {
        SnapshotDeleteRepositoryRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn get(&self) -> SnapshotGetRequestBuilder {
        SnapshotGetRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn get_repository(&self) -> SnapshotGetRepositoryRequestBuilder {
        SnapshotGetRepositoryRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn restore(&self) -> SnapshotRestoreRequestBuilder {
        SnapshotRestoreRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn status(&self) -> SnapshotStatusRequestBuilder {
        SnapshotStatusRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn verify_repository(&self) -> SnapshotVerifyRepositoryRequestBuilder {
        SnapshotVerifyRepositoryRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Snapshot APIs"]
    pub fn snapshot(&self) -> SnapshotNamespaceClient {
        SnapshotNamespaceClient::new(self)
    }
}
