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
        headers::{HeaderMap, HeaderName, HeaderValue},
        request::{Body, JsonBody, NdBody},
        response::Response,
        Method,
    },
    params::*,
};
use serde::Serialize;
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Snapshot Cleanup Repository API"]
pub enum SnapshotCleanupRepositoryParts<'a> {
    #[doc = "Repository"]
    Repository(&'a str),
}
impl<'a> SnapshotCleanupRepositoryParts<'a> {
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
#[doc = "Builder for the [Snapshot Cleanup Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html). Removes stale data from repository."]
pub struct SnapshotCleanupRepository<'a, B> {
    client: Elasticsearch,
    parts: SnapshotCleanupRepositoryParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> SnapshotCleanupRepository<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotCleanupRepository] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SnapshotCleanupRepositoryParts<'a>) -> Self {
        SnapshotCleanupRepository {
            client,
            parts,
            headers: HeaderMap::new(),
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
    pub fn body<T>(self, body: T) -> SnapshotCleanupRepository<'a, JsonBody<T>>
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
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
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
pub enum SnapshotCreateParts<'a> {
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'a str, &'a str),
}
impl<'a> SnapshotCreateParts<'a> {
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
#[doc = "Builder for the [Snapshot Create API](https://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html). Creates a snapshot in a repository."]
pub struct SnapshotCreate<'a, B> {
    client: Elasticsearch,
    parts: SnapshotCreateParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    wait_for_completion: Option<bool>,
}
impl<'a, B> SnapshotCreate<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotCreate] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SnapshotCreateParts<'a>) -> Self {
        SnapshotCreate {
            client,
            parts,
            headers: HeaderMap::new(),
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
    pub fn body<T>(self, body: T) -> SnapshotCreate<'a, JsonBody<T>>
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
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
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
pub enum SnapshotCreateRepositoryParts<'a> {
    #[doc = "Repository"]
    Repository(&'a str),
}
impl<'a> SnapshotCreateRepositoryParts<'a> {
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
#[doc = "Builder for the [Snapshot Create Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html). Creates a repository."]
pub struct SnapshotCreateRepository<'a, B> {
    client: Elasticsearch,
    parts: SnapshotCreateRepositoryParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    verify: Option<bool>,
}
impl<'a, B> SnapshotCreateRepository<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotCreateRepository] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SnapshotCreateRepositoryParts<'a>) -> Self {
        SnapshotCreateRepository {
            client,
            parts,
            headers: HeaderMap::new(),
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
    pub fn body<T>(self, body: T) -> SnapshotCreateRepository<'a, JsonBody<T>>
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
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
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
pub enum SnapshotDeleteParts<'a> {
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'a str, &'a str),
}
impl<'a> SnapshotDeleteParts<'a> {
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
#[doc = "Builder for the [Snapshot Delete API](https://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html). Deletes a snapshot."]
pub struct SnapshotDelete<'a> {
    client: Elasticsearch,
    parts: SnapshotDeleteParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SnapshotDelete<'a> {
    #[doc = "Creates a new instance of [SnapshotDelete] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SnapshotDeleteParts<'a>) -> Self {
        SnapshotDelete {
            client,
            parts,
            headers: HeaderMap::new(),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
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
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
pub enum SnapshotDeleteRepositoryParts<'a> {
    #[doc = "Repository"]
    Repository(&'a [&'a str]),
}
impl<'a> SnapshotDeleteRepositoryParts<'a> {
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
#[doc = "Builder for the [Snapshot Delete Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html). Deletes a repository."]
pub struct SnapshotDeleteRepository<'a> {
    client: Elasticsearch,
    parts: SnapshotDeleteRepositoryParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a> SnapshotDeleteRepository<'a> {
    #[doc = "Creates a new instance of [SnapshotDeleteRepository] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SnapshotDeleteRepositoryParts<'a>) -> Self {
        SnapshotDeleteRepository {
            client,
            parts,
            headers: HeaderMap::new(),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
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
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
pub enum SnapshotGetParts<'a> {
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'a str, &'a [&'a str]),
}
impl<'a> SnapshotGetParts<'a> {
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
#[doc = "Builder for the [Snapshot Get API](https://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html). Returns information about a snapshot."]
pub struct SnapshotGet<'a> {
    client: Elasticsearch,
    parts: SnapshotGetParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    verbose: Option<bool>,
}
impl<'a> SnapshotGet<'a> {
    #[doc = "Creates a new instance of [SnapshotGet] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SnapshotGetParts<'a>) -> Self {
        SnapshotGet {
            client,
            parts,
            headers: HeaderMap::new(),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
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
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
pub enum SnapshotGetRepositoryParts<'a> {
    #[doc = "No parts"]
    None,
    #[doc = "Repository"]
    Repository(&'a [&'a str]),
}
impl<'a> SnapshotGetRepositoryParts<'a> {
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
#[doc = "Builder for the [Snapshot Get Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html). Returns information about a repository."]
pub struct SnapshotGetRepository<'a> {
    client: Elasticsearch,
    parts: SnapshotGetRepositoryParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SnapshotGetRepository<'a> {
    #[doc = "Creates a new instance of [SnapshotGetRepository] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SnapshotGetRepositoryParts<'a>) -> Self {
        SnapshotGetRepository {
            client,
            parts,
            headers: HeaderMap::new(),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
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
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
pub enum SnapshotRestoreParts<'a> {
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'a str, &'a str),
}
impl<'a> SnapshotRestoreParts<'a> {
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
#[doc = "Builder for the [Snapshot Restore API](https://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html). Restores a snapshot."]
pub struct SnapshotRestore<'a, B> {
    client: Elasticsearch,
    parts: SnapshotRestoreParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    wait_for_completion: Option<bool>,
}
impl<'a, B> SnapshotRestore<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotRestore] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SnapshotRestoreParts<'a>) -> Self {
        SnapshotRestore {
            client,
            parts,
            headers: HeaderMap::new(),
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
    pub fn body<T>(self, body: T) -> SnapshotRestore<'a, JsonBody<T>>
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
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
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
pub enum SnapshotStatusParts<'a> {
    #[doc = "No parts"]
    None,
    #[doc = "Repository"]
    Repository(&'a str),
    #[doc = "Repository and Snapshot"]
    RepositorySnapshot(&'a str, &'a [&'a str]),
}
impl<'a> SnapshotStatusParts<'a> {
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
#[doc = "Builder for the [Snapshot Status API](https://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html). Returns information about the status of a snapshot."]
pub struct SnapshotStatus<'a> {
    client: Elasticsearch,
    parts: SnapshotStatusParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a> SnapshotStatus<'a> {
    #[doc = "Creates a new instance of [SnapshotStatus] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SnapshotStatusParts<'a>) -> Self {
        SnapshotStatus {
            client,
            parts,
            headers: HeaderMap::new(),
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
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
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
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
pub enum SnapshotVerifyRepositoryParts<'a> {
    #[doc = "Repository"]
    Repository(&'a str),
}
impl<'a> SnapshotVerifyRepositoryParts<'a> {
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
#[doc = "Builder for the [Snapshot Verify Repository API](https://www.elastic.co/guide/en/elasticsearch/reference/master/modules-snapshots.html). Verifies a repository."]
pub struct SnapshotVerifyRepository<'a, B> {
    client: Elasticsearch,
    parts: SnapshotVerifyRepositoryParts<'a>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
}
impl<'a, B> SnapshotVerifyRepository<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SnapshotVerifyRepository] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: SnapshotVerifyRepositoryParts<'a>) -> Self {
        SnapshotVerifyRepository {
            client,
            parts,
            headers: HeaderMap::new(),
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
    pub fn body<T>(self, body: T) -> SnapshotVerifyRepository<'a, JsonBody<T>>
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
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
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
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'a str) -> Self {
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
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
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
pub struct Snapshot {
    client: Elasticsearch,
}
impl Snapshot {
    #[doc = "Creates a new instance of [Snapshot]"]
    pub fn new(client: Elasticsearch) -> Self {
        Self { client }
    }
    #[doc = "Removes stale data from repository."]
    pub fn cleanup_repository<'a>(
        &self,
        parts: SnapshotCleanupRepositoryParts<'a>,
    ) -> SnapshotCleanupRepository<'a, ()> {
        SnapshotCleanupRepository::new(self.client.clone(), parts)
    }
    #[doc = "Creates a snapshot in a repository."]
    pub fn create<'a>(&self, parts: SnapshotCreateParts<'a>) -> SnapshotCreate<'a, ()> {
        SnapshotCreate::new(self.client.clone(), parts)
    }
    #[doc = "Creates a repository."]
    pub fn create_repository<'a>(
        &self,
        parts: SnapshotCreateRepositoryParts<'a>,
    ) -> SnapshotCreateRepository<'a, ()> {
        SnapshotCreateRepository::new(self.client.clone(), parts)
    }
    #[doc = "Deletes a snapshot."]
    pub fn delete<'a>(&self, parts: SnapshotDeleteParts<'a>) -> SnapshotDelete<'a> {
        SnapshotDelete::new(self.client.clone(), parts)
    }
    #[doc = "Deletes a repository."]
    pub fn delete_repository<'a>(
        &self,
        parts: SnapshotDeleteRepositoryParts<'a>,
    ) -> SnapshotDeleteRepository<'a> {
        SnapshotDeleteRepository::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about a snapshot."]
    pub fn get<'a>(&self, parts: SnapshotGetParts<'a>) -> SnapshotGet<'a> {
        SnapshotGet::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about a repository."]
    pub fn get_repository<'a>(
        &self,
        parts: SnapshotGetRepositoryParts<'a>,
    ) -> SnapshotGetRepository<'a> {
        SnapshotGetRepository::new(self.client.clone(), parts)
    }
    #[doc = "Restores a snapshot."]
    pub fn restore<'a>(&self, parts: SnapshotRestoreParts<'a>) -> SnapshotRestore<'a, ()> {
        SnapshotRestore::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about the status of a snapshot."]
    pub fn status<'a>(&self, parts: SnapshotStatusParts<'a>) -> SnapshotStatus<'a> {
        SnapshotStatus::new(self.client.clone(), parts)
    }
    #[doc = "Verifies a repository."]
    pub fn verify_repository<'a>(
        &self,
        parts: SnapshotVerifyRepositoryParts<'a>,
    ) -> SnapshotVerifyRepository<'a, ()> {
        SnapshotVerifyRepository::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Snapshot APIs"]
    pub fn snapshot(&self) -> Snapshot {
        Snapshot::new(self.clone())
    }
}