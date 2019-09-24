

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct LicenseDeleteBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl LicenseDeleteBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        LicenseDeleteBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for LicenseDeleteBuilder {
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
pub struct LicenseGetBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    local: Option<bool>,
}
impl LicenseGetBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        LicenseGetBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for LicenseGetBuilder {
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
pub struct LicenseGetBasicStatusBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl LicenseGetBasicStatusBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        LicenseGetBasicStatusBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for LicenseGetBasicStatusBuilder {
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
pub struct LicenseGetTrialStatusBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl LicenseGetTrialStatusBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        LicenseGetTrialStatusBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for LicenseGetTrialStatusBuilder {
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
pub struct LicensePostBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    acknowledge: Option<bool>,
}
impl LicensePostBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        LicensePostBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for LicensePostBuilder {
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
pub struct LicensePostStartBasicBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    acknowledge: Option<bool>,
}
impl LicensePostStartBasicBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        LicensePostStartBasicBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for LicensePostStartBasicBuilder {
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
pub struct LicensePostStartTrialBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    acknowledge: Option<bool>,
    ty: Option<String>,
}
impl LicensePostStartTrialBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        LicensePostStartTrialBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for LicensePostStartTrialBuilder {
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
pub struct LicenseClient {
    client: ElasticsearchClient,
}
impl LicenseClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        LicenseClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/delete-license.html"]
    pub fn delete(&self) -> LicenseDeleteBuilder {
        LicenseDeleteBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-license.html"]
    pub fn get(&self) -> LicenseGetBuilder {
        LicenseGetBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-basic-status.html"]
    pub fn get_basic_status(&self) -> LicenseGetBasicStatusBuilder {
        LicenseGetBasicStatusBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-trial-status.html"]
    pub fn get_trial_status(&self) -> LicenseGetTrialStatusBuilder {
        LicenseGetTrialStatusBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/update-license.html"]
    pub fn post(&self) -> LicensePostBuilder {
        LicensePostBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/start-basic.html"]
    pub fn post_start_basic(&self) -> LicensePostStartBasicBuilder {
        LicensePostStartBasicBuilder::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/start-trial.html"]
    pub fn post_start_trial(&self) -> LicensePostStartTrialBuilder {
        LicensePostStartTrialBuilder::new(self.client.clone())
    }
}
impl ElasticsearchClient {
    #[doc = "License APIs"]
    pub fn license(&self) -> LicenseClient {
        LicenseClient::new(self.clone())
    }
}
