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
pub struct TasksCancel<B> {
    client: Elasticsearch,
    actions: Option<Vec<String>>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    nodes: Option<Vec<String>>,
    parent_task_id: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    task_id: Option<String>,
}
impl<B> TasksCancel<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch) -> Self {
        TasksCancel {
            client,
            actions: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            nodes: None,
            parent_task_id: None,
            pretty: None,
            source: None,
            task_id: None,
        }
    }
    #[doc = "A comma-separated list of actions that should be cancelled. Leave empty to cancel all."]
    pub fn actions(mut self, actions: Option<Vec<String>>) -> Self {
        self.actions = actions;
        self
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
    #[doc = "A comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes"]
    pub fn nodes(mut self, nodes: Option<Vec<String>>) -> Self {
        self.nodes = nodes;
        self
    }
    #[doc = "Cancel tasks with specified parent task id (node_id:task_number). Set to -1 to cancel all."]
    pub fn parent_task_id(mut self, parent_task_id: Option<String>) -> Self {
        self.parent_task_id = parent_task_id;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl<B> Sender for TasksCancel<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_tasks/_cancel";
        let method = HttpMethod::Post;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "actions")]
                actions: Option<Vec<String>>,
                #[serde(rename = "nodes")]
                nodes: Option<Vec<String>>,
                #[serde(rename = "parent_task_id")]
                parent_task_id: Option<String>,
            }
            let query_params = QueryParamsStruct {
                actions: self.actions,
                nodes: self.nodes,
                parent_task_id: self.parent_task_id,
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
pub struct TasksGet {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    task_id: String,
    timeout: Option<String>,
    wait_for_completion: Option<bool>,
}
impl TasksGet {
    pub fn new(client: Elasticsearch, task_id: String) -> Self {
        TasksGet {
            client,
            task_id: task_id,
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
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
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
    #[doc = "Wait for the matching tasks to complete (default: false)"]
    pub fn wait_for_completion(mut self, wait_for_completion: Option<bool>) -> Self {
        self.wait_for_completion = wait_for_completion;
        self
    }
}
impl Sender for TasksGet {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_tasks/{task_id}";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                timeout: self.timeout,
                wait_for_completion: self.wait_for_completion,
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
pub struct TasksList {
    client: Elasticsearch,
    actions: Option<Vec<String>>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    group_by: Option<GroupBy>,
    human: Option<bool>,
    nodes: Option<Vec<String>>,
    parent_task_id: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_completion: Option<bool>,
}
impl TasksList {
    pub fn new(client: Elasticsearch) -> Self {
        TasksList {
            client,
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
    pub fn actions(mut self, actions: Option<Vec<String>>) -> Self {
        self.actions = actions;
        self
    }
    #[doc = "Return detailed task information (default: false)"]
    pub fn detailed(mut self, detailed: Option<bool>) -> Self {
        self.detailed = detailed;
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
    #[doc = "Group tasks by nodes or parent/child relationships"]
    pub fn group_by(mut self, group_by: Option<GroupBy>) -> Self {
        self.group_by = group_by;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "A comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes"]
    pub fn nodes(mut self, nodes: Option<Vec<String>>) -> Self {
        self.nodes = nodes;
        self
    }
    #[doc = "Return tasks with specified parent task id (node_id:task_number). Set to -1 to return all."]
    pub fn parent_task_id(mut self, parent_task_id: Option<String>) -> Self {
        self.parent_task_id = parent_task_id;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
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
    #[doc = "Wait for the matching tasks to complete (default: false)"]
    pub fn wait_for_completion(mut self, wait_for_completion: Option<bool>) -> Self {
        self.wait_for_completion = wait_for_completion;
        self
    }
}
impl Sender for TasksList {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = "/_tasks";
        let method = HttpMethod::Get;
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "actions")]
                actions: Option<Vec<String>>,
                #[serde(rename = "detailed")]
                detailed: Option<bool>,
                #[serde(rename = "group_by")]
                group_by: Option<GroupBy>,
                #[serde(rename = "nodes")]
                nodes: Option<Vec<String>>,
                #[serde(rename = "parent_task_id")]
                parent_task_id: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
                #[serde(rename = "wait_for_completion")]
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParamsStruct {
                actions: self.actions,
                detailed: self.detailed,
                group_by: self.group_by,
                nodes: self.nodes,
                parent_task_id: self.parent_task_id,
                timeout: self.timeout,
                wait_for_completion: self.wait_for_completion,
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
#[doc = "Tasks APIs"]
pub struct Tasks {
    client: Elasticsearch,
}
impl Tasks {
    pub fn new(client: Elasticsearch) -> Self {
        Tasks { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn cancel<B>(&self) -> TasksCancel<B>
    where
        B: Serialize,
    {
        TasksCancel::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn get(&self, task_id: String) -> TasksGet {
        TasksGet::new(self.client.clone(), task_id)
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn list(&self) -> TasksList {
        TasksList::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Tasks APIs"]
    pub fn tasks(&self) -> Tasks {
        Tasks::new(self.clone())
    }
}
