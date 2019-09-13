

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct LicenseDeleteRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> LicenseDeleteRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        LicenseDeleteRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for LicenseDeleteRequestBuilder<'a> {
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
pub struct LicenseGetRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    local: Option<&'a bool>,
}
impl<'a> LicenseGetRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        LicenseGetRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for LicenseGetRequestBuilder<'a> {
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
pub struct LicenseGetBasicStatusRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> LicenseGetBasicStatusRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        LicenseGetBasicStatusRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for LicenseGetBasicStatusRequestBuilder<'a> {
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
pub struct LicenseGetTrialStatusRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
}
impl<'a> LicenseGetTrialStatusRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        LicenseGetTrialStatusRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for LicenseGetTrialStatusRequestBuilder<'a> {
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
pub struct LicensePostRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    acknowledge: Option<&'a bool>,
}
impl<'a> LicensePostRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        LicensePostRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for LicensePostRequestBuilder<'a> {
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
pub struct LicensePostStartBasicRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    acknowledge: Option<&'a bool>,
}
impl<'a> LicensePostStartBasicRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        LicensePostStartBasicRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for LicensePostStartBasicRequestBuilder<'a> {
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
pub struct LicensePostStartTrialRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    acknowledge: Option<&'a bool>,
    ty: &'a str,
}
impl<'a> LicensePostStartTrialRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        LicensePostStartTrialRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for LicensePostStartTrialRequestBuilder<'a> {
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
#[doc = "License APIs"]
pub struct LicenseNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> LicenseNamespaceClient<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        LicenseNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/delete-license.html"]
    pub fn delete(&self) -> LicenseDeleteRequestBuilder {
        LicenseDeleteRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-license.html"]
    pub fn get(&self) -> LicenseGetRequestBuilder {
        LicenseGetRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-basic-status.html"]
    pub fn get_basic_status(&self) -> LicenseGetBasicStatusRequestBuilder {
        LicenseGetBasicStatusRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-trial-status.html"]
    pub fn get_trial_status(&self) -> LicenseGetTrialStatusRequestBuilder {
        LicenseGetTrialStatusRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/update-license.html"]
    pub fn post(&self) -> LicensePostRequestBuilder {
        LicensePostRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/start-basic.html"]
    pub fn post_start_basic(&self) -> LicensePostStartBasicRequestBuilder {
        LicensePostStartBasicRequestBuilder::default()
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/start-trial.html"]
    pub fn post_start_trial(&self) -> LicensePostStartTrialRequestBuilder {
        LicensePostStartTrialRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "License APIs"]
    pub fn license(&self) -> LicenseNamespaceClient {
        LicenseNamespaceClient::new(self)
    }
}
