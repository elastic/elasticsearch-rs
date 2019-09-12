

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct LicenseDeleteRequest<'a> {}
pub struct LicenseDeleteRequestBuilder<'a> {}
impl<'a> LicenseDeleteRequestBuilder<'a> {
    pub fn build(&self) -> LicenseDeleteRequest<'a> {
        LicenseDeleteRequest {}
    }
}
pub struct LicenseGetRequest<'a> {
    local: Option<&'a bool>,
}
pub struct LicenseGetRequestBuilder<'a> {
    local: Option<&'a bool>,
}
impl<'a> LicenseGetRequestBuilder<'a> {
    pub fn build(&self) -> LicenseGetRequest<'a> {
        LicenseGetRequest { local: self.local }
    }
}
pub struct LicenseGetBasicStatusRequest<'a> {}
pub struct LicenseGetBasicStatusRequestBuilder<'a> {}
impl<'a> LicenseGetBasicStatusRequestBuilder<'a> {
    pub fn build(&self) -> LicenseGetBasicStatusRequest<'a> {
        LicenseGetBasicStatusRequest {}
    }
}
pub struct LicenseGetTrialStatusRequest<'a> {}
pub struct LicenseGetTrialStatusRequestBuilder<'a> {}
impl<'a> LicenseGetTrialStatusRequestBuilder<'a> {
    pub fn build(&self) -> LicenseGetTrialStatusRequest<'a> {
        LicenseGetTrialStatusRequest {}
    }
}
pub struct LicensePostRequest<'a> {
    acknowledge: Option<&'a bool>,
}
pub struct LicensePostRequestBuilder<'a> {
    acknowledge: Option<&'a bool>,
}
impl<'a> LicensePostRequestBuilder<'a> {
    pub fn build(&self) -> LicensePostRequest<'a> {
        LicensePostRequest {
            acknowledge: self.acknowledge,
        }
    }
}
pub struct LicensePostStartBasicRequest<'a> {
    acknowledge: Option<&'a bool>,
}
pub struct LicensePostStartBasicRequestBuilder<'a> {
    acknowledge: Option<&'a bool>,
}
impl<'a> LicensePostStartBasicRequestBuilder<'a> {
    pub fn build(&self) -> LicensePostStartBasicRequest<'a> {
        LicensePostStartBasicRequest {
            acknowledge: self.acknowledge,
        }
    }
}
pub struct LicensePostStartTrialRequest<'a> {
    acknowledge: Option<&'a bool>,
    ty: &'a String,
}
pub struct LicensePostStartTrialRequestBuilder<'a> {
    acknowledge: Option<&'a bool>,
    ty: &'a String,
}
impl<'a> LicensePostStartTrialRequestBuilder<'a> {
    pub fn build(&self) -> LicensePostStartTrialRequest<'a> {
        LicensePostStartTrialRequest {
            acknowledge: self.acknowledge,
            ty: self.ty,
        }
    }
}
#[doc = "License APIs"]
pub struct LicenseNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> LicenseNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        LicenseNamespaceClient { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/delete-license.html"]
    pub fn delete(&self, request: &LicenseDeleteRequest) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_license")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-license.html"]
    pub fn get(&self, request: &LicenseGetRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_license")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-basic-status.html"]
    pub fn get_basic_status(&self, request: &LicenseGetBasicStatusRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_license/basic_status")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/get-trial-status.html"]
    pub fn get_trial_status(&self, request: &LicenseGetTrialStatusRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_license/trial_status")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/update-license.html"]
    pub fn post(&self, request: &LicensePostRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_license")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/start-basic.html"]
    pub fn post_start_basic(&self, request: &LicensePostStartBasicRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_license/start_basic")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/master/start-trial.html"]
    pub fn post_start_trial(&self, request: &LicensePostStartTrialRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_license/start_trial")
    }
}
impl ElasticsearchClient {
    #[doc = "License APIs"]
    pub fn license(&self) -> LicenseNamespaceClient {
        LicenseNamespaceClient::new(self)
    }
}
