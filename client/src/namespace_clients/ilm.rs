

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct IlmDeleteLifecycleBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmDeleteLifecycleBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IlmDeleteLifecycleBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IlmDeleteLifecycleBuilder {
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
pub struct IlmExplainLifecycleBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmExplainLifecycleBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IlmExplainLifecycleBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IlmExplainLifecycleBuilder {
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
pub struct IlmGetLifecycleBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmGetLifecycleBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IlmGetLifecycleBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IlmGetLifecycleBuilder {
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
pub struct IlmGetStatusBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmGetStatusBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IlmGetStatusBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IlmGetStatusBuilder {
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
pub struct IlmMoveToStepBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmMoveToStepBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IlmMoveToStepBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IlmMoveToStepBuilder {
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
pub struct IlmPutLifecycleBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmPutLifecycleBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IlmPutLifecycleBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IlmPutLifecycleBuilder {
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
pub struct IlmRemovePolicyBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmRemovePolicyBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IlmRemovePolicyBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IlmRemovePolicyBuilder {
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
pub struct IlmRetryBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmRetryBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IlmRetryBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IlmRetryBuilder {
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
pub struct IlmStartBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmStartBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IlmStartBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IlmStartBuilder {
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
pub struct IlmStopBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmStopBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        IlmStopBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for IlmStopBuilder {
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
#[doc = "Ilm APIs"]
pub struct IlmClient {
    client: ElasticsearchClient,
}
impl IlmClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        IlmClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-delete-lifecycle.html"]
    pub fn delete_lifecycle(&self) -> IlmDeleteLifecycleBuilder {
        IlmDeleteLifecycleBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-explain-lifecycle.html"]
    pub fn explain_lifecycle(&self) -> IlmExplainLifecycleBuilder {
        IlmExplainLifecycleBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-lifecycle.html"]
    pub fn get_lifecycle(&self) -> IlmGetLifecycleBuilder {
        IlmGetLifecycleBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-status.html"]
    pub fn get_status(&self) -> IlmGetStatusBuilder {
        IlmGetStatusBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-move-to-step.html"]
    pub fn move_to_step(&self) -> IlmMoveToStepBuilder {
        IlmMoveToStepBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-put-lifecycle.html"]
    pub fn put_lifecycle(&self) -> IlmPutLifecycleBuilder {
        IlmPutLifecycleBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-remove-policy.html"]
    pub fn remove_policy(&self) -> IlmRemovePolicyBuilder {
        IlmRemovePolicyBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-retry-policy.html"]
    pub fn retry(&self) -> IlmRetryBuilder {
        IlmRetryBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-start.html"]
    pub fn start(&self) -> IlmStartBuilder {
        IlmStartBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-stop.html"]
    pub fn stop(&self) -> IlmStopBuilder {
        IlmStopBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Ilm APIs"]
    pub fn ilm(&self) -> IlmClient {
        IlmClient::new(self.clone())
    }
}
