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
    client::Elasticsearch, enums::*, error::ElasticsearchError, http_method::HttpMethod,
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Tasks Cancel API"]
pub enum TasksCancelUrlParts<'a> {
    None,
    TaskId(&'a str),
}
impl<'a> TasksCancelUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            TasksCancelUrlParts::None => "/_tasks/_cancel".into(),
            TasksCancelUrlParts::TaskId(ref task_id) => {
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
#[doc = "Request builder for the Tasks Cancel API"]
pub struct TasksCancel<'a, B> {
    client: Elasticsearch,
    parts: TasksCancelUrlParts<'a>,
    actions: Option<&'a [&'a str]>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    nodes: Option<&'a [&'a str]>,
    parent_task_id: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
}
impl<'a, B> TasksCancel<'a, B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, parts: TasksCancelUrlParts<'a>) -> Self {
        TasksCancel {
            client,
            parts,
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
    pub fn body<T>(self, body: T) -> TasksCancel<'a, T>
    where
        T: Serialize,
    {
        TasksCancel {
            client: self.client,
            parts: self.parts,
            body: Some(body),
            actions: self.actions,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
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
    #[doc = "Creates an asynchronous request to the Tasks Cancel API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "actions",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                actions: Option<&'a [&'a str]>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(
                    rename = "nodes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                nodes: Option<&'a [&'a str]>,
                #[serde(rename = "parent_task_id", skip_serializing_if = "Option::is_none")]
                parent_task_id: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Tasks Get API"]
pub enum TasksGetUrlParts<'a> {
    TaskId(&'a str),
}
impl<'a> TasksGetUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            TasksGetUrlParts::TaskId(ref task_id) => {
                let mut p = String::with_capacity(8usize + task_id.len());
                p.push_str("/_tasks/");
                p.push_str(task_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Tasks Get API"]
pub struct TasksGet<'a> {
    client: Elasticsearch,
    parts: TasksGetUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_completion: Option<bool>,
}
impl<'a> TasksGet<'a> {
    pub fn new(client: Elasticsearch, parts: TasksGetUrlParts<'a>) -> Self {
        TasksGet {
            client,
            parts,
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
    #[doc = "Creates an asynchronous request to the Tasks Get API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
                #[serde(
                    rename = "wait_for_completion",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Tasks List API"]
pub enum TasksListUrlParts {
    None,
}
impl TasksListUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            TasksListUrlParts::None => "/_tasks".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Tasks List API"]
pub struct TasksList<'a> {
    client: Elasticsearch,
    parts: TasksListUrlParts,
    actions: Option<&'a [&'a str]>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    group_by: Option<GroupBy>,
    human: Option<bool>,
    nodes: Option<&'a [&'a str]>,
    parent_task_id: Option<&'a str>,
    pretty: Option<bool>,
    source: Option<&'a str>,
    timeout: Option<&'a str>,
    wait_for_completion: Option<bool>,
}
impl<'a> TasksList<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        TasksList {
            client,
            parts: TasksListUrlParts::None,
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
    #[doc = "Creates an asynchronous request to the Tasks List API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct<'a> {
                #[serde(
                    rename = "actions",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                actions: Option<&'a [&'a str]>,
                #[serde(rename = "detailed", skip_serializing_if = "Option::is_none")]
                detailed: Option<bool>,
                #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
                group_by: Option<GroupBy>,
                #[serde(rename = "human", skip_serializing_if = "Option::is_none")]
                human: Option<bool>,
                #[serde(
                    rename = "nodes",
                    serialize_with = "crate::client::serialize_coll_qs",
                    skip_serializing_if = "Option::is_none"
                )]
                nodes: Option<&'a [&'a str]>,
                #[serde(rename = "parent_task_id", skip_serializing_if = "Option::is_none")]
                parent_task_id: Option<&'a str>,
                #[serde(rename = "pretty", skip_serializing_if = "Option::is_none")]
                pretty: Option<bool>,
                #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
                source: Option<&'a str>,
                #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
                timeout: Option<&'a str>,
                #[serde(
                    rename = "wait_for_completion",
                    skip_serializing_if = "Option::is_none"
                )]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
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
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Tasks APIs"]
pub struct Tasks {
    client: Elasticsearch,
}
impl Tasks {
    pub fn new(client: Elasticsearch) -> Self {
        Tasks { client }
    }
    #[doc = "Cancels a task, if it can be cancelled through an API."]
    pub fn cancel<'a>(&self, parts: TasksCancelUrlParts<'a>) -> TasksCancel<'a, ()> {
        TasksCancel::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about a task."]
    pub fn get<'a>(&self, parts: TasksGetUrlParts<'a>) -> TasksGet<'a> {
        TasksGet::new(self.client.clone(), parts)
    }
    #[doc = "Returns a list of tasks."]
    pub fn list<'a>(&self) -> TasksList<'a> {
        TasksList::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Tasks APIs"]
    pub fn tasks(&self) -> Tasks {
        Tasks::new(self.clone())
    }
}