

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct RollupDeleteJobRequest<'a> {}
pub struct RollupDeleteJobRequestBuilder<'a> {}
impl<'a> RollupDeleteJobRequestBuilder<'a> {
    pub fn build(&self) -> RollupDeleteJobRequest<'a> {
        RollupDeleteJobRequest {}
    }
}
pub struct RollupGetJobsRequest<'a> {}
pub struct RollupGetJobsRequestBuilder<'a> {}
impl<'a> RollupGetJobsRequestBuilder<'a> {
    pub fn build(&self) -> RollupGetJobsRequest<'a> {
        RollupGetJobsRequest {}
    }
}
pub struct RollupGetRollupCapsRequest<'a> {}
pub struct RollupGetRollupCapsRequestBuilder<'a> {}
impl<'a> RollupGetRollupCapsRequestBuilder<'a> {
    pub fn build(&self) -> RollupGetRollupCapsRequest<'a> {
        RollupGetRollupCapsRequest {}
    }
}
pub struct RollupGetRollupIndexCapsRequest<'a> {}
pub struct RollupGetRollupIndexCapsRequestBuilder<'a> {}
impl<'a> RollupGetRollupIndexCapsRequestBuilder<'a> {
    pub fn build(&self) -> RollupGetRollupIndexCapsRequest<'a> {
        RollupGetRollupIndexCapsRequest {}
    }
}
pub struct RollupPutJobRequest<'a> {}
pub struct RollupPutJobRequestBuilder<'a> {}
impl<'a> RollupPutJobRequestBuilder<'a> {
    pub fn build(&self) -> RollupPutJobRequest<'a> {
        RollupPutJobRequest {}
    }
}
pub struct RollupRollupSearchRequest<'a> {
    rest_total_hits_as_int: Option<&'a bool>,
    typed_keys: Option<&'a bool>,
}
pub struct RollupRollupSearchRequestBuilder<'a> {
    rest_total_hits_as_int: Option<&'a bool>,
    typed_keys: Option<&'a bool>,
}
impl<'a> RollupRollupSearchRequestBuilder<'a> {
    pub fn build(&self) -> RollupRollupSearchRequest<'a> {
        RollupRollupSearchRequest {
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            typed_keys: self.typed_keys,
        }
    }
}
pub struct RollupStartJobRequest<'a> {}
pub struct RollupStartJobRequestBuilder<'a> {}
impl<'a> RollupStartJobRequestBuilder<'a> {
    pub fn build(&self) -> RollupStartJobRequest<'a> {
        RollupStartJobRequest {}
    }
}
pub struct RollupStopJobRequest<'a> {
    timeout: &'a String,
    wait_for_completion: Option<&'a bool>,
}
pub struct RollupStopJobRequestBuilder<'a> {
    timeout: &'a String,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> RollupStopJobRequestBuilder<'a> {
    pub fn build(&self) -> RollupStopJobRequest<'a> {
        RollupStopJobRequest {
            timeout: self.timeout,
            wait_for_completion: self.wait_for_completion,
        }
    }
}
#[doc = "Rollup APIs"]
pub struct RollupNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> RollupNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        RollupNamespaceClient { client }
    }
    #[doc = ""]
    pub fn delete_job(&self, request: &RollupDeleteJobRequest) -> Result<Response> {
        self.client.send(HttpMethod::Delete, "/_rollup/job/{id}")
    }
    #[doc = ""]
    pub fn get_jobs(&self, request: &RollupGetJobsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_rollup/job/{id}")
    }
    #[doc = ""]
    pub fn get_rollup_caps(&self, request: &RollupGetRollupCapsRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_rollup/data/{id}")
    }
    #[doc = ""]
    pub fn get_rollup_index_caps(
        &self,
        request: &RollupGetRollupIndexCapsRequest,
    ) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/{index}/_rollup/data")
    }
    #[doc = ""]
    pub fn put_job(&self, request: &RollupPutJobRequest) -> Result<Response> {
        self.client.send(HttpMethod::Put, "/_rollup/job/{id}")
    }
    #[doc = ""]
    pub fn rollup_search(&self, request: &RollupRollupSearchRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "{index}/_rollup_search")
    }
    #[doc = ""]
    pub fn start_job(&self, request: &RollupStartJobRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_rollup/job/{id}/_start")
    }
    #[doc = ""]
    pub fn stop_job(&self, request: &RollupStopJobRequest) -> Result<Response> {
        self.client
            .send(HttpMethod::Post, "/_rollup/job/{id}/_stop")
    }
}
impl ElasticsearchClient {
    #[doc = "Rollup APIs"]
    pub fn rollup(&self) -> RollupNamespaceClient {
        RollupNamespaceClient::new(self)
    }
}
