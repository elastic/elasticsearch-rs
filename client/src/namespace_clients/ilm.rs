

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[Default]
pub struct IlmDeleteLifecycleRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IlmDeleteLifecycleRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IlmDeleteLifecycleRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IlmDeleteLifecycleRequestBuilder<'a> {
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
pub struct IlmExplainLifecycleRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IlmExplainLifecycleRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IlmExplainLifecycleRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IlmExplainLifecycleRequestBuilder<'a> {
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
pub struct IlmGetLifecycleRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IlmGetLifecycleRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IlmGetLifecycleRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IlmGetLifecycleRequestBuilder<'a> {
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
pub struct IlmGetStatusRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IlmGetStatusRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IlmGetStatusRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IlmGetStatusRequestBuilder<'a> {
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
pub struct IlmMoveToStepRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IlmMoveToStepRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IlmMoveToStepRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IlmMoveToStepRequestBuilder<'a> {
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
pub struct IlmPutLifecycleRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IlmPutLifecycleRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IlmPutLifecycleRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IlmPutLifecycleRequestBuilder<'a> {
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
pub struct IlmRemovePolicyRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IlmRemovePolicyRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IlmRemovePolicyRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IlmRemovePolicyRequestBuilder<'a> {
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
pub struct IlmRetryRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IlmRetryRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IlmRetryRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IlmRetryRequestBuilder<'a> {
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
pub struct IlmStartRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IlmStartRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IlmStartRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IlmStartRequestBuilder<'a> {
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
pub struct IlmStopRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IlmStopRequestBuilder<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IlmStopRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for IlmStopRequestBuilder<'a> {
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
#[doc = "Ilm APIs"]
pub struct IlmNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> IlmNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        IlmNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-delete-lifecycle.html"]
    pub fn delete_lifecycle(&self) -> IlmDeleteLifecycleRequestBuilder {
        IlmDeleteLifecycleRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-explain-lifecycle.html"]
    pub fn explain_lifecycle(&self) -> IlmExplainLifecycleRequestBuilder {
        IlmExplainLifecycleRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-lifecycle.html"]
    pub fn get_lifecycle(&self) -> IlmGetLifecycleRequestBuilder {
        IlmGetLifecycleRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-status.html"]
    pub fn get_status(&self) -> IlmGetStatusRequestBuilder {
        IlmGetStatusRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-move-to-step.html"]
    pub fn move_to_step(&self) -> IlmMoveToStepRequestBuilder {
        IlmMoveToStepRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-put-lifecycle.html"]
    pub fn put_lifecycle(&self) -> IlmPutLifecycleRequestBuilder {
        IlmPutLifecycleRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-remove-policy.html"]
    pub fn remove_policy(&self) -> IlmRemovePolicyRequestBuilder {
        IlmRemovePolicyRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-retry-policy.html"]
    pub fn retry(&self) -> IlmRetryRequestBuilder {
        IlmRetryRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-start.html"]
    pub fn start(&self) -> IlmStartRequestBuilder {
        IlmStartRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-stop.html"]
    pub fn stop(&self) -> IlmStopRequestBuilder {
        IlmStopRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Ilm APIs"]
    pub fn ilm(&self) -> IlmNamespaceClient {
        IlmNamespaceClient::new(self)
    }
}
