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
#[allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
    params::*,
};
use serde::Serialize;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Snapshot Cleanup Repository API"]
pub enum SnapshotCleanupRepositoryParts<'b> {
    #[doc = "Repository"]
    Repository(&'b str),
}
impl<'b> SnapshotCleanupRepositoryParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Cleanup Repository API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotCleanupRepositoryParts::Repository(ref repository) => {
                let mut p = String::with_capacity(20usize + repository.len());
                p.push_str("/_snapshot/");
                p.push_str(repository.as_ref());
                p.push_str("/_cleanup");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Snapshot Cleanup Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nRemoves stale data from repository."]
pub struct SnapshotCleanupRepository<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SnapshotCleanupRepositoryParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> SnapshotCleanupRepository<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotCleanupRepository] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SnapshotCleanupRepositoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotCleanupRepository {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> SnapshotCleanupRepository<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SnapshotCleanupRepository {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Cleanup Repository API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Snapshot Create API"]
pub enum SnapshotCreateParts<'b> {
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'b str, &'b str),
}
impl<'b> SnapshotCreateParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Create API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotCreateParts::RepositorySnapshot(ref repository, ref snapshot) => {
                let mut p = String::with_capacity(12usize + repository.len() + snapshot.len());
                p.push_str("/_snapshot/");
                p.push_str(repository.as_ref());
                p.push_str("/");
                p.push_str(snapshot.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Snapshot Create API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nCreates a snapshot in a repository."]
pub struct SnapshotCreate<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SnapshotCreateParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> SnapshotCreate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotCreate] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SnapshotCreateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotCreate {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> SnapshotCreate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SnapshotCreate {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            wait_for_completion: self.wait_for_completion,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Should this request wait until the operation has completed before returning"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Create API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Snapshot Create Repository API"]
pub enum SnapshotCreateRepositoryParts<'b> {
    #[doc = "Repository"]
    Repository(&'b str),
}
impl<'b> SnapshotCreateRepositoryParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Create Repository API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotCreateRepositoryParts::Repository(ref repository) => {
                let mut p = String::with_capacity(11usize + repository.len());
                p.push_str("/_snapshot/");
                p.push_str(repository.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Snapshot Create Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nCreates a repository."]
pub struct SnapshotCreateRepository<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SnapshotCreateRepositoryParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    verify: Option<bool>,
}
impl<'a, 'b, B> SnapshotCreateRepository<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotCreateRepository] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SnapshotCreateRepositoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotCreateRepository {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> SnapshotCreateRepository<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SnapshotCreateRepository {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
            verify: self.verify,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Whether to verify the repository after creation"]
    pub fn verify(mut self, verify: bool) -> Self {
        self.verify = Some(verify);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Create Repository API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
                #[serde(rename = "verify")]
                verify: Option<bool>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Snapshot Delete API"]
pub enum SnapshotDeleteParts<'b> {
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'b str, &'b str),
}
impl<'b> SnapshotDeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotDeleteParts::RepositorySnapshot(ref repository, ref snapshot) => {
                let mut p = String::with_capacity(12usize + repository.len() + snapshot.len());
                p.push_str("/_snapshot/");
                p.push_str(repository.as_ref());
                p.push_str("/");
                p.push_str(snapshot.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Snapshot Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nDeletes a snapshot."]
pub struct SnapshotDelete<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SnapshotDeleteParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SnapshotDelete<'a, 'b> {
    #[doc = "Creates a new instance of [SnapshotDelete] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SnapshotDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotDelete {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Delete API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Snapshot Delete Repository API"]
pub enum SnapshotDeleteRepositoryParts<'b> {
    #[doc = "Repository"]
    Repository(&'b [&'b str]),
}
impl<'b> SnapshotDeleteRepositoryParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Delete Repository API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotDeleteRepositoryParts::Repository(ref repository) => {
                let repository_str = repository.join(",");
                let mut p = String::with_capacity(11usize + repository_str.len());
                p.push_str("/_snapshot/");
                p.push_str(repository_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Snapshot Delete Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nDeletes a repository."]
pub struct SnapshotDeleteRepository<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SnapshotDeleteRepositoryParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> SnapshotDeleteRepository<'a, 'b> {
    #[doc = "Creates a new instance of [SnapshotDeleteRepository] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SnapshotDeleteRepositoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotDeleteRepository {
            client,
            parts,
            headers,
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
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Delete Repository API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Snapshot Get API"]
pub enum SnapshotGetParts<'b> {
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'b str, &'b [&'b str]),
}
impl<'b> SnapshotGetParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotGetParts::RepositorySnapshot(ref repository, ref snapshot) => {
                let snapshot_str = snapshot.join(",");
                let mut p = String::with_capacity(12usize + repository.len() + snapshot_str.len());
                p.push_str("/_snapshot/");
                p.push_str(repository.as_ref());
                p.push_str("/");
                p.push_str(snapshot_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Snapshot Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nReturns information about a snapshot."]
pub struct SnapshotGet<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SnapshotGetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    verbose: Option<bool>,
}
impl<'a, 'b> SnapshotGet<'a, 'b> {
    #[doc = "Creates a new instance of [SnapshotGet] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SnapshotGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotGet {
            client,
            parts,
            headers,
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
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether to ignore unavailable snapshots, defaults to false which means a SnapshotMissingException is thrown"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Whether to show verbose snapshot info or only show the basic info found in the repository index blob"]
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = Some(verbose);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Get API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "verbose")]
                verbose: Option<bool>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Snapshot Get Repository API"]
pub enum SnapshotGetRepositoryParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Repository"]
    Repository(&'b [&'b str]),
}
impl<'b> SnapshotGetRepositoryParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Get Repository API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotGetRepositoryParts::None => "/_snapshot".into(),
            SnapshotGetRepositoryParts::Repository(ref repository) => {
                let repository_str = repository.join(",");
                let mut p = String::with_capacity(11usize + repository_str.len());
                p.push_str("/_snapshot/");
                p.push_str(repository_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Snapshot Get Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nReturns information about a repository."]
pub struct SnapshotGetRepository<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SnapshotGetRepositoryParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SnapshotGetRepository<'a, 'b> {
    #[doc = "Creates a new instance of [SnapshotGetRepository] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SnapshotGetRepositoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotGetRepository {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Get Repository API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Snapshot Restore API"]
pub enum SnapshotRestoreParts<'b> {
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'b str, &'b str),
}
impl<'b> SnapshotRestoreParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Restore API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotRestoreParts::RepositorySnapshot(ref repository, ref snapshot) => {
                let mut p = String::with_capacity(21usize + repository.len() + snapshot.len());
                p.push_str("/_snapshot/");
                p.push_str(repository.as_ref());
                p.push_str("/");
                p.push_str(snapshot.as_ref());
                p.push_str("/_restore");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Snapshot Restore API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nRestores a snapshot."]
pub struct SnapshotRestore<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SnapshotRestoreParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> SnapshotRestore<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotRestore] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SnapshotRestoreParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotRestore {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> SnapshotRestore<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SnapshotRestore {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            wait_for_completion: self.wait_for_completion,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Should this request wait until the operation has completed before returning"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Restore API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Snapshot Status API"]
pub enum SnapshotStatusParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Repository"]
    Repository(&'b str),
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'b str, &'b [&'b str]),
}
impl<'b> SnapshotStatusParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotStatusParts::None => "/_snapshot/_status".into(),
            SnapshotStatusParts::Repository(ref repository) => {
                let mut p = String::with_capacity(19usize + repository.len());
                p.push_str("/_snapshot/");
                p.push_str(repository.as_ref());
                p.push_str("/_status");
                p.into()
            }
            SnapshotStatusParts::RepositorySnapshot(ref repository, ref snapshot) => {
                let snapshot_str = snapshot.join(",");
                let mut p = String::with_capacity(20usize + repository.len() + snapshot_str.len());
                p.push_str("/_snapshot/");
                p.push_str(repository.as_ref());
                p.push_str("/");
                p.push_str(snapshot_str.as_ref());
                p.push_str("/_status");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Snapshot Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nReturns information about the status of a snapshot."]
pub struct SnapshotStatus<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SnapshotStatusParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SnapshotStatus<'a, 'b> {
    #[doc = "Creates a new instance of [SnapshotStatus] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SnapshotStatusParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotStatus {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether to ignore unavailable snapshots, defaults to false which means a SnapshotMissingException is thrown"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Status API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Snapshot Verify Repository API"]
pub enum SnapshotVerifyRepositoryParts<'b> {
    #[doc = "Repository"]
    Repository(&'b str),
}
impl<'b> SnapshotVerifyRepositoryParts<'b> {
    #[doc = "Builds a relative URL path to the Snapshot Verify Repository API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SnapshotVerifyRepositoryParts::Repository(ref repository) => {
                let mut p = String::with_capacity(19usize + repository.len());
                p.push_str("/_snapshot/");
                p.push_str(repository.as_ref());
                p.push_str("/_verify");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Snapshot Verify Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nVerifies a repository."]
pub struct SnapshotVerifyRepository<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SnapshotVerifyRepositoryParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> SnapshotVerifyRepository<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotVerifyRepository] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SnapshotVerifyRepositoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SnapshotVerifyRepository {
            client,
            parts,
            headers,
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
    pub fn body<T>(self, body: T) -> SnapshotVerifyRepository<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SnapshotVerifyRepository {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Snapshot Verify Repository API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'b str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
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
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Snapshot APIs"]
pub struct Snapshot<'a> {
    client: &'a Elasticsearch,
}
impl<'a> Snapshot<'a> {
    #[doc = "Creates a new instance of [Snapshot]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        Self { client }
    }
    #[doc = "[Snapshot Cleanup Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nRemoves stale data from repository."]
    pub fn cleanup_repository<'b>(
        &'a self,
        parts: SnapshotCleanupRepositoryParts<'b>,
    ) -> SnapshotCleanupRepository<'a, 'b, ()> {
        SnapshotCleanupRepository::new(&self.client, parts)
    }
    #[doc = "[Snapshot Create API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nCreates a snapshot in a repository."]
    pub fn create<'b>(&'a self, parts: SnapshotCreateParts<'b>) -> SnapshotCreate<'a, 'b, ()> {
        SnapshotCreate::new(&self.client, parts)
    }
    #[doc = "[Snapshot Create Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nCreates a repository."]
    pub fn create_repository<'b>(
        &'a self,
        parts: SnapshotCreateRepositoryParts<'b>,
    ) -> SnapshotCreateRepository<'a, 'b, ()> {
        SnapshotCreateRepository::new(&self.client, parts)
    }
    #[doc = "[Snapshot Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nDeletes a snapshot."]
    pub fn delete<'b>(&'a self, parts: SnapshotDeleteParts<'b>) -> SnapshotDelete<'a, 'b> {
        SnapshotDelete::new(&self.client, parts)
    }
    #[doc = "[Snapshot Delete Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nDeletes a repository."]
    pub fn delete_repository<'b>(
        &'a self,
        parts: SnapshotDeleteRepositoryParts<'b>,
    ) -> SnapshotDeleteRepository<'a, 'b> {
        SnapshotDeleteRepository::new(&self.client, parts)
    }
    #[doc = "[Snapshot Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nReturns information about a snapshot."]
    pub fn get<'b>(&'a self, parts: SnapshotGetParts<'b>) -> SnapshotGet<'a, 'b> {
        SnapshotGet::new(&self.client, parts)
    }
    #[doc = "[Snapshot Get Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nReturns information about a repository."]
    pub fn get_repository<'b>(
        &'a self,
        parts: SnapshotGetRepositoryParts<'b>,
    ) -> SnapshotGetRepository<'a, 'b> {
        SnapshotGetRepository::new(&self.client, parts)
    }
    #[doc = "[Snapshot Restore API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nRestores a snapshot."]
    pub fn restore<'b>(&'a self, parts: SnapshotRestoreParts<'b>) -> SnapshotRestore<'a, 'b, ()> {
        SnapshotRestore::new(&self.client, parts)
    }
    #[doc = "[Snapshot Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nReturns information about the status of a snapshot."]
    pub fn status<'b>(&'a self, parts: SnapshotStatusParts<'b>) -> SnapshotStatus<'a, 'b> {
        SnapshotStatus::new(&self.client, parts)
    }
    #[doc = "[Snapshot Verify Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/modules-snapshots.html)\n\nVerifies a repository."]
    pub fn verify_repository<'b>(
        &'a self,
        parts: SnapshotVerifyRepositoryParts<'b>,
    ) -> SnapshotVerifyRepository<'a, 'b, ()> {
        SnapshotVerifyRepository::new(&self.client, parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Snapshot APIs"]
    pub fn snapshot(&self) -> Snapshot {
        Snapshot::new(&self)
    }
}
