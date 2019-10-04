use super::super::client::Elasticsearch;
use super::super::enums::*;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct TasksCancel {
    client: Elasticsearch,
    actions: Option<Vec<String>>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    nodes: Option<Vec<String>>,
    parent_task_id: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    task_id: Option<String>,
}
impl TasksCancel {
    pub fn new(client: Elasticsearch) -> Self {
        TasksCancel {
            client,
            ..Default::default()
        }
    }
    #[doc = "A comma-separated list of actions that should be cancelled. Leave empty to cancel all."]
    pub fn actions(mut self, actions: Option<Vec<String>>) -> Self {
        self.actions = actions;
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
}
impl Sender for TasksCancel {
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
pub struct TasksGet {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    task_id: Option<String>,
    timeout: Option<String>,
    wait_for_completion: Option<bool>,
}
impl TasksGet {
    pub fn new(client: Elasticsearch) -> Self {
        TasksGet {
            client,
            ..Default::default()
        }
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
}
impl Sender for TasksGet {
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
            ..Default::default()
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
    #[doc = "Group tasks by nodes or parent/child relationships"]
    pub fn group_by(mut self, group_by: Option<GroupBy>) -> Self {
        self.group_by = group_by;
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
}
impl Sender for TasksList {
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
#[doc = "Tasks APIs"]
pub struct Tasks {
    client: Elasticsearch,
}
impl Tasks {
    pub fn new(client: Elasticsearch) -> Self {
        Tasks { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn cancel(&self) -> TasksCancel {
        TasksCancel::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn get(&self) -> TasksGet {
        TasksGet::new(self.client.clone())
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