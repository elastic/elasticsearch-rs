

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct IlmDeleteLifecycleRequest<'a> {}
pub struct IlmDeleteLifecycleRequestBuilder<'a> {}
impl<'a> IlmDeleteLifecycleRequestBuilder<'a> {
    pub fn build(&self) -> IlmDeleteLifecycleRequest<'a> {
        IlmDeleteLifecycleRequest {}
    }
}
pub struct IlmExplainLifecycleRequest<'a> {}
pub struct IlmExplainLifecycleRequestBuilder<'a> {}
impl<'a> IlmExplainLifecycleRequestBuilder<'a> {
    pub fn build(&self) -> IlmExplainLifecycleRequest<'a> {
        IlmExplainLifecycleRequest {}
    }
}
pub struct IlmGetLifecycleRequest<'a> {}
pub struct IlmGetLifecycleRequestBuilder<'a> {}
impl<'a> IlmGetLifecycleRequestBuilder<'a> {
    pub fn build(&self) -> IlmGetLifecycleRequest<'a> {
        IlmGetLifecycleRequest {}
    }
}
pub struct IlmGetStatusRequest<'a> {}
pub struct IlmGetStatusRequestBuilder<'a> {}
impl<'a> IlmGetStatusRequestBuilder<'a> {
    pub fn build(&self) -> IlmGetStatusRequest<'a> {
        IlmGetStatusRequest {}
    }
}
pub struct IlmMoveToStepRequest<'a> {}
pub struct IlmMoveToStepRequestBuilder<'a> {}
impl<'a> IlmMoveToStepRequestBuilder<'a> {
    pub fn build(&self) -> IlmMoveToStepRequest<'a> {
        IlmMoveToStepRequest {}
    }
}
pub struct IlmPutLifecycleRequest<'a> {}
pub struct IlmPutLifecycleRequestBuilder<'a> {}
impl<'a> IlmPutLifecycleRequestBuilder<'a> {
    pub fn build(&self) -> IlmPutLifecycleRequest<'a> {
        IlmPutLifecycleRequest {}
    }
}
pub struct IlmRemovePolicyRequest<'a> {}
pub struct IlmRemovePolicyRequestBuilder<'a> {}
impl<'a> IlmRemovePolicyRequestBuilder<'a> {
    pub fn build(&self) -> IlmRemovePolicyRequest<'a> {
        IlmRemovePolicyRequest {}
    }
}
pub struct IlmRetryRequest<'a> {}
pub struct IlmRetryRequestBuilder<'a> {}
impl<'a> IlmRetryRequestBuilder<'a> {
    pub fn build(&self) -> IlmRetryRequest<'a> {
        IlmRetryRequest {}
    }
}
pub struct IlmStartRequest<'a> {}
pub struct IlmStartRequestBuilder<'a> {}
impl<'a> IlmStartRequestBuilder<'a> {
    pub fn build(&self) -> IlmStartRequest<'a> {
        IlmStartRequest {}
    }
}
pub struct IlmStopRequest<'a> {}
pub struct IlmStopRequestBuilder<'a> {}
impl<'a> IlmStopRequestBuilder<'a> {
    pub fn build(&self) -> IlmStopRequest<'a> {
        IlmStopRequest {}
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
    pub fn delete_lifecycle(&self, request: &IlmDeleteLifecycleRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Delete, "/_ilm/policy/{policy}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-explain-lifecycle.html"]
    pub fn explain_lifecycle(&self, request: &IlmExplainLifecycleRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_ilm/explain")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-lifecycle.html"]
    pub fn get_lifecycle(&self, request: &IlmGetLifecycleRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ilm/policy/{policy}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-status.html"]
    pub fn get_status(&self, request: &IlmGetStatusRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_ilm/status")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-move-to-step.html"]
    pub fn move_to_step(&self, request: &IlmMoveToStepRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_ilm/move/{index}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-put-lifecycle.html"]
    pub fn put_lifecycle(&self, request: &IlmPutLifecycleRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_ilm/policy/{policy}")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-remove-policy.html"]
    pub fn remove_policy(&self, request: &IlmRemovePolicyRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_ilm/remove")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-retry-policy.html"]
    pub fn retry(&self, request: &IlmRetryRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/{index}/_ilm/retry")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-start.html"]
    pub fn start(&self, request: &IlmStartRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_ilm/start")
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-stop.html"]
    pub fn stop(&self, request: &IlmStopRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_ilm/stop")
    }
}
impl ElasticsearchClient {
    #[doc = "Ilm APIs"]
    pub fn ilm(&self) -> IlmNamespaceClient {
        IlmNamespaceClient::new(self)
    }
}
