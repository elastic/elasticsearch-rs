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
#[doc = "API parts for the Tasks Cancel API"]
pub enum TasksCancelParts<'a> {
    #[doc = "No parts"]
    None,
    #[doc = "TaskId"]
    TaskId(&'a str),
}
impl<'a> TasksCancelParts<'a> {
    #[doc = "Builds a relative URL path to the Tasks Cancel API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TasksCancelParts::None => "/_tasks/_cancel".into(),
            TasksCancelParts::TaskId(ref task_id) => {
                let mut p = String::with_capacity(16usize + task_id.len());
                p.push_str("/_tasks/");
                p.push_str(task_id.as_ref());
                p.push_str("/_cancel");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Tasks Cancel API](https://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html). Cancels a task, if it can be cancelled through an API."]
pub struct TasksCancel<'a, B> {
    client: Elasticsearch,
    parts: TasksCancelParts<'a>,
    actions: Option<&'a [&'a str]>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    nodes: Option<&'a [&'a str]>,
    parent_task_id: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> TasksCancel<'a, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TasksCancel] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: TasksCancelParts<'a>) -> Self {
        TasksCancel {
            client,
            parts,
            headers: HeaderMap::new(),
            actions: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            nodes: None,
            parent_task_id: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "A comma-separated list of actions that should be cancelled. Leave empty to cancel all."]
    pub fn actions(mut self, actions: &'a [&'a str]) -> Self {
        self.actions = Some(actions);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TasksCancel<'a, JsonBody<T>>
    where
        T: Serialize,
    {
        TasksCancel {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            actions: self.actions,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            nodes: self.nodes,
            parent_task_id: self.parent_task_id,
            pretty: self.pretty,
            source: self.source,
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
    #[doc = "A comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes"]
    pub fn nodes(mut self, nodes: &'a [&'a str]) -> Self {
        self.nodes = Some(nodes);
        self
    }
    #[doc = "Cancel tasks with specified parent task id (node_id:task_number). Set to -1 to cancel all."]
    pub fn parent_task_id(mut self, parent_task_id: &'a str) -> Self {
        self.parent_task_id = Some(parent_task_id);
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
    #[doc = "Creates an asynchronous call to the Tasks Cancel API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(
                    rename = "actions",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                actions: Option<&'a [&'a str]>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "nodes", serialize_with = "crate::client::serialize_coll_qs")]
                nodes: Option<&'a [&'a str]>,
                #[serde(rename = "parent_task_id")]
                parent_task_id: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                actions: self.actions,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                nodes: self.nodes,
                parent_task_id: self.parent_task_id,
                pretty: self.pretty,
                source: self.source,
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
#[doc = "API parts for the Tasks Get API"]
pub enum TasksGetParts<'a> {
    #[doc = "TaskId"]
    TaskId(&'a str),
}
impl<'a> TasksGetParts<'a> {
    #[doc = "Builds a relative URL path to the Tasks Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TasksGetParts::TaskId(ref task_id) => {
                let mut p = String::with_capacity(8usize + task_id.len());
                p.push_str("/_tasks/");
                p.push_str(task_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Tasks Get API](https://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html). Returns information about a task."]
pub struct TasksGet<'a> {
    client: Elasticsearch,
    parts: TasksGetParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_completion: Option<bool>,
}
impl<'a> TasksGet<'a> {
    #[doc = "Creates a new instance of [TasksGet] with the specified API parts"]
    pub fn new(client: Elasticsearch, parts: TasksGetParts<'a>) -> Self {
        TasksGet {
            client,
            parts,
            headers: HeaderMap::new(),
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_completion: None,
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
    #[doc = "Wait for the matching tasks to complete (default: false)"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Tasks Get API that can be awaited"]
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
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                wait_for_completion: self.wait_for_completion,
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
#[doc = "API parts for the Tasks List API"]
pub enum TasksListParts {
    #[doc = "No parts"]
    None,
}
impl TasksListParts {
    #[doc = "Builds a relative URL path to the Tasks List API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TasksListParts::None => "/_tasks".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Tasks List API](https://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html). Returns a list of tasks."]
pub struct TasksList<'a> {
    client: Elasticsearch,
    parts: TasksListParts,
    actions: Option<&'a [&'a str]>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    group_by: Option<GroupBy>,
    headers: HeaderMap,
    human: Option<bool>,
    nodes: Option<&'a [&'a str]>,
    parent_task_id: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_completion: Option<bool>,
}
impl<'a> TasksList<'a> {
    #[doc = "Creates a new instance of [TasksList]"]
    pub fn new(client: Elasticsearch) -> Self {
        TasksList {
            client,
            parts: TasksListParts::None,
            headers: HeaderMap::new(),
            actions: None,
            detailed: None,
            error_trace: None,
            filter_path: None,
            group_by: None,
            human: None,
            nodes: None,
            parent_task_id: None,
            pretty: None,
            source: None,
            timeout: None,
            wait_for_completion: None,
        }
    }
    #[doc = "A comma-separated list of actions that should be returned. Leave empty to return all."]
    pub fn actions(mut self, actions: &'a [&'a str]) -> Self {
        self.actions = Some(actions);
        self
    }
    #[doc = "Return detailed task information (default: false)"]
    pub fn detailed(mut self, detailed: bool) -> Self {
        self.detailed = Some(detailed);
        self
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
    #[doc = "Group tasks by nodes or parent/child relationships"]
    pub fn group_by(mut self, group_by: GroupBy) -> Self {
        self.group_by = Some(group_by);
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
    #[doc = "A comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes"]
    pub fn nodes(mut self, nodes: &'a [&'a str]) -> Self {
        self.nodes = Some(nodes);
        self
    }
    #[doc = "Return tasks with specified parent task id (node_id:task_number). Set to -1 to return all."]
    pub fn parent_task_id(mut self, parent_task_id: &'a str) -> Self {
        self.parent_task_id = Some(parent_task_id);
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
    #[doc = "Wait for the matching tasks to complete (default: false)"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Tasks List API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(
                    rename = "actions",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                actions: Option<&'a [&'a str]>,
                #[serde(rename = "detailed")]
                detailed: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "group_by")]
                group_by: Option<GroupBy>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "nodes", serialize_with = "crate::client::serialize_coll_qs")]
                nodes: Option<&'a [&'a str]>,
                #[serde(rename = "parent_task_id")]
                parent_task_id: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "timeout")]
                timeout: Option<&'a str>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                actions: self.actions,
                detailed: self.detailed,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                group_by: self.group_by,
                human: self.human,
                nodes: self.nodes,
                parent_task_id: self.parent_task_id,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                wait_for_completion: self.wait_for_completion,
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
#[doc = "Namespace client for Tasks APIs"]
pub struct Tasks {
    client: Elasticsearch,
}
impl Tasks {
    #[doc = "Creates a new instance of [Tasks]"]
    pub fn new(client: Elasticsearch) -> Self {
        Self { client }
    }
    #[doc = "Cancels a task, if it can be cancelled through an API."]
    pub fn cancel<'a>(&self, parts: TasksCancelParts<'a>) -> TasksCancel<'a, ()> {
        TasksCancel::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about a task."]
    pub fn get<'a>(&self, parts: TasksGetParts<'a>) -> TasksGet<'a> {
        TasksGet::new(self.client.clone(), parts)
    }
    #[doc = "Returns a list of tasks."]
    pub fn list<'a>(&self) -> TasksList<'a> {
        TasksList::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Tasks APIs"]
    pub fn tasks(&self) -> Tasks {
        Tasks::new(self.clone())
    }
}