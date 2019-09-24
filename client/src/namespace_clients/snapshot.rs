

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct SnapshotCreateBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
    wait_for_completion: Option<bool>,
}
impl SnapshotCreateBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SnapshotCreateBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SnapshotCreateBuilder {
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
pub struct SnapshotCreateRepositoryBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
    timeout: Option<String>,
    verify: Option<bool>,
}
impl SnapshotCreateRepositoryBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SnapshotCreateRepositoryBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SnapshotCreateRepositoryBuilder {
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
pub struct SnapshotDeleteBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
}
impl SnapshotDeleteBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SnapshotDeleteBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SnapshotDeleteBuilder {
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
pub struct SnapshotDeleteRepositoryBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
    timeout: Option<String>,
}
impl SnapshotDeleteRepositoryBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SnapshotDeleteRepositoryBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SnapshotDeleteRepositoryBuilder {
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
pub struct SnapshotGetBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<String>,
    verbose: Option<bool>,
}
impl SnapshotGetBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SnapshotGetBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SnapshotGetBuilder {
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
pub struct SnapshotGetRepositoryBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    local: Option<bool>,
    master_timeout: Option<String>,
}
impl SnapshotGetRepositoryBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SnapshotGetRepositoryBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SnapshotGetRepositoryBuilder {
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
pub struct SnapshotRestoreBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
    wait_for_completion: Option<bool>,
}
impl SnapshotRestoreBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SnapshotRestoreBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SnapshotRestoreBuilder {
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
pub struct SnapshotStatusBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<String>,
}
impl SnapshotStatusBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SnapshotStatusBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SnapshotStatusBuilder {
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
pub struct SnapshotVerifyRepositoryBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    master_timeout: Option<String>,
    timeout: Option<String>,
}
impl SnapshotVerifyRepositoryBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        SnapshotVerifyRepositoryBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for SnapshotVerifyRepositoryBuilder {
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
#[doc = "Snapshot APIs"]
pub struct SnapshotClient {
    client: ElasticsearchClient,
}
impl SnapshotClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        SnapshotClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn create(&self) -> SnapshotCreateBuilder {
        SnapshotCreateBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn create_repository(&self) -> SnapshotCreateRepositoryBuilder {
        SnapshotCreateRepositoryBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn delete(&self) -> SnapshotDeleteBuilder {
        SnapshotDeleteBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn delete_repository(&self) -> SnapshotDeleteRepositoryBuilder {
        SnapshotDeleteRepositoryBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn get(&self) -> SnapshotGetBuilder {
        SnapshotGetBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn get_repository(&self) -> SnapshotGetRepositoryBuilder {
        SnapshotGetRepositoryBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn restore(&self) -> SnapshotRestoreBuilder {
        SnapshotRestoreBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn status(&self) -> SnapshotStatusBuilder {
        SnapshotStatusBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn verify_repository(&self) -> SnapshotVerifyRepositoryBuilder {
        SnapshotVerifyRepositoryBuilder::new(self.client.clone())
    }
}
impl ElasticsearchClient {
    #[doc = "Snapshot APIs"]
    pub fn snapshot(&self) -> SnapshotClient {
        SnapshotClient::new(self.clone())
    }
}
