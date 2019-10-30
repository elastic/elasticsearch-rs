// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
use crate::{
    client::{Elasticsearch, Sender},
    enums::*,
    error::ElasticsearchError,
    http_method::HttpMethod,
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
pub struct SnapshotCreate<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    repository: String,
    snapshot: String,
    source: Option<String>,
    wait_for_completion: Option<bool>,
}
impl<B> SnapshotCreate<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, repository: String, snapshot: String) -> Self {
        SnapshotCreate {
            client,
            repository: repository,
            snapshot: snapshot,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            wait_for_completion: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "A repository name"]
    pub fn repository(mut self, repository: String) -> Self {
        self.repository = repository;
        self
    }
    #[doc = "A snapshot name"]
    pub fn snapshot(mut self, snapshot: String) -> Self {
        self.snapshot = snapshot;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Should this request wait until the operation has completed before returning"]
    pub fn wait_for_completion(mut self, wait_for_completion: Option<bool>) -> Self {
        self.wait_for_completion = wait_for_completion;
        self
    }
}
impl<B> Sender for SnapshotCreate<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_snapshot/{repository}/{snapshot}";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SnapshotCreateRepository<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    repository: String,
    source: Option<String>,
    timeout: Option<String>,
    verify: Option<bool>,
}
impl<B> SnapshotCreateRepository<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, repository: String) -> Self {
        SnapshotCreateRepository {
            client,
            repository: repository,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
            verify: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "A repository name"]
    pub fn repository(mut self, repository: String) -> Self {
        self.repository = repository;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Whether to verify the repository after creation"]
    pub fn verify(mut self, verify: Option<bool>) -> Self {
        self.verify = verify;
        self
    }
}
impl<B> Sender for SnapshotCreateRepository<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_snapshot/{repository}";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "verify")]
                verify: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                verify: self.verify,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SnapshotDelete {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    repository: String,
    snapshot: String,
    source: Option<String>,
}
impl SnapshotDelete {
    pub fn new(client: Elasticsearch, repository: String, snapshot: String) -> Self {
        SnapshotDelete {
            client,
            repository: repository,
            snapshot: snapshot,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "A repository name"]
    pub fn repository(mut self, repository: String) -> Self {
        self.repository = repository;
        self
    }
    #[doc = "A snapshot name"]
    pub fn snapshot(mut self, snapshot: String) -> Self {
        self.snapshot = snapshot;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SnapshotDelete {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_snapshot/{repository}/{snapshot}";
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SnapshotDeleteRepository {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    repository: Vec<String>,
    source: Option<String>,
    timeout: Option<String>,
}
impl SnapshotDeleteRepository {
    pub fn new(client: Elasticsearch, repository: Vec<String>) -> Self {
        SnapshotDeleteRepository {
            client,
            repository: repository,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "A comma-separated list of repository names"]
    pub fn repository(mut self, repository: Vec<String>) -> Self {
        self.repository = repository;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for SnapshotDeleteRepository {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_snapshot/{repository}";
        let method = HttpMethod::Delete;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SnapshotGet {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    repository: String,
    snapshot: Vec<String>,
    source: Option<String>,
    verbose: Option<bool>,
}
impl SnapshotGet {
    pub fn new(client: Elasticsearch, repository: String, snapshot: Vec<String>) -> Self {
        SnapshotGet {
            client,
            repository: repository,
            snapshot: snapshot,
            error_trace: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            source: None,
            verbose: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether to ignore unavailable snapshots, defaults to false which means a SnapshotMissingException is thrown"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "A repository name"]
    pub fn repository(mut self, repository: String) -> Self {
        self.repository = repository;
        self
    }
    #[doc = "A comma-separated list of snapshot names"]
    pub fn snapshot(mut self, snapshot: Vec<String>) -> Self {
        self.snapshot = snapshot;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Whether to show verbose snapshot info or only show the basic info found in the repository index blob"]
    pub fn verbose(mut self, verbose: Option<bool>) -> Self {
        self.verbose = verbose;
        self
    }
}
impl Sender for SnapshotGet {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_snapshot/{repository}/{snapshot}";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "verbose")]
                verbose: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                verbose: self.verbose,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SnapshotGetRepository {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    repository: Option<Vec<String>>,
    source: Option<String>,
}
impl SnapshotGetRepository {
    pub fn new(client: Elasticsearch) -> Self {
        SnapshotGetRepository {
            client,
            error_trace: None,
            filter_path: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            repository: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "A comma-separated list of repository names"]
    pub fn repository(mut self, repository: Option<Vec<String>>) -> Self {
        self.repository = repository;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SnapshotGetRepository {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_snapshot";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SnapshotRestore<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    repository: String,
    snapshot: String,
    source: Option<String>,
    wait_for_completion: Option<bool>,
}
impl<B> SnapshotRestore<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, repository: String, snapshot: String) -> Self {
        SnapshotRestore {
            client,
            repository: repository,
            snapshot: snapshot,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            wait_for_completion: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "A repository name"]
    pub fn repository(mut self, repository: String) -> Self {
        self.repository = repository;
        self
    }
    #[doc = "A snapshot name"]
    pub fn snapshot(mut self, snapshot: String) -> Self {
        self.snapshot = snapshot;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Should this request wait until the operation has completed before returning"]
    pub fn wait_for_completion(mut self, wait_for_completion: Option<bool>) -> Self {
        self.wait_for_completion = wait_for_completion;
        self
    }
}
impl<B> Sender for SnapshotRestore<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_snapshot/{repository}/{snapshot}/_restore";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SnapshotStatus {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    repository: Option<String>,
    snapshot: Option<Vec<String>>,
    source: Option<String>,
}
impl SnapshotStatus {
    pub fn new(client: Elasticsearch) -> Self {
        SnapshotStatus {
            client,
            error_trace: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            repository: None,
            snapshot: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether to ignore unavailable snapshots, defaults to false which means a SnapshotMissingException is thrown"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "A repository name"]
    pub fn repository(mut self, repository: Option<String>) -> Self {
        self.repository = repository;
        self
    }
    #[doc = "A comma-separated list of snapshot names"]
    pub fn snapshot(mut self, snapshot: Option<Vec<String>>) -> Self {
        self.snapshot = snapshot;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for SnapshotStatus {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_snapshot/_status";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
pub struct SnapshotVerifyRepository<B> {
    client: Elasticsearch,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    repository: String,
    source: Option<String>,
    timeout: Option<String>,
}
impl<B> SnapshotVerifyRepository<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, repository: String) -> Self {
        SnapshotVerifyRepository {
            client,
            repository: repository,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body(mut self, body: Option<B>) -> Self {
        self.body = body;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "A repository name"]
    pub fn repository(mut self, repository: String) -> Self {
        self.repository = repository;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl<B> Sender for SnapshotVerifyRepository<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_snapshot/{repository}/_verify";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<String>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, path, query_string.as_ref(), body)?;
        Ok(response)
    }
}
#[doc = "Snapshot APIs"]
pub struct Snapshot {
    client: Elasticsearch,
}
impl Snapshot {
    pub fn new(client: Elasticsearch) -> Self {
        Snapshot { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn create<B>(&self, repository: String, snapshot: String) -> SnapshotCreate<B>
    where
        B: Serialize,
    {
        SnapshotCreate::new(self.client.clone(), repository, snapshot)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn create_repository<B>(&self, repository: String) -> SnapshotCreateRepository<B>
    where
        B: Serialize,
    {
        SnapshotCreateRepository::new(self.client.clone(), repository)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn delete(&self, repository: String, snapshot: String) -> SnapshotDelete {
        SnapshotDelete::new(self.client.clone(), repository, snapshot)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn delete_repository(&self, repository: Vec<String>) -> SnapshotDeleteRepository {
        SnapshotDeleteRepository::new(self.client.clone(), repository)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn get(&self, repository: String, snapshot: Vec<String>) -> SnapshotGet {
        SnapshotGet::new(self.client.clone(), repository, snapshot)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn get_repository(&self) -> SnapshotGetRepository {
        SnapshotGetRepository::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn restore<B>(&self, repository: String, snapshot: String) -> SnapshotRestore<B>
    where
        B: Serialize,
    {
        SnapshotRestore::new(self.client.clone(), repository, snapshot)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn status(&self) -> SnapshotStatus {
        SnapshotStatus::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html"]
    pub fn verify_repository<B>(&self, repository: String) -> SnapshotVerifyRepository<B>
    where
        B: Serialize,
    {
        SnapshotVerifyRepository::new(self.client.clone(), repository)
    }
}
impl Elasticsearch {
    #[doc = "Snapshot APIs"]
    pub fn snapshot(&self) -> Snapshot {
        Snapshot::new(self.clone())
    }
}