

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use reqwest::{Error, Request, Response, Result};
pub struct TasksCancelRequest<'a> {
    actions: &'a Vec<String>,
    nodes: &'a Vec<String>,
    parent_task_id: &'a String,
}
pub struct TasksCancelRequestBuilder<'a> {
    actions: &'a Vec<String>,
    nodes: &'a Vec<String>,
    parent_task_id: &'a String,
}
impl<'a> TasksCancelRequestBuilder<'a> {
    pub fn build(&self) -> TasksCancelRequest<'a> {
        TasksCancelRequest {
            actions: self.actions,
            nodes: self.nodes,
            parent_task_id: self.parent_task_id,
        }
    }
}
pub struct TasksGetRequest<'a> {
    timeout: &'a String,
    wait_for_completion: Option<&'a bool>,
}
pub struct TasksGetRequestBuilder<'a> {
    timeout: &'a String,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> TasksGetRequestBuilder<'a> {
    pub fn build(&self) -> TasksGetRequest<'a> {
        TasksGetRequest {
            timeout: self.timeout,
            wait_for_completion: self.wait_for_completion,
        }
    }
}
pub struct TasksListRequest<'a> {
    actions: &'a Vec<String>,
    detailed: Option<&'a bool>,
    group_by: Option<&'a i32>,
    nodes: &'a Vec<String>,
    parent_task_id: &'a String,
    timeout: &'a String,
    wait_for_completion: Option<&'a bool>,
}
pub struct TasksListRequestBuilder<'a> {
    actions: &'a Vec<String>,
    detailed: Option<&'a bool>,
    group_by: Option<&'a i32>,
    nodes: &'a Vec<String>,
    parent_task_id: &'a String,
    timeout: &'a String,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> TasksListRequestBuilder<'a> {
    pub fn build(&self) -> TasksListRequest<'a> {
        TasksListRequest {
            actions: self.actions,
            detailed: self.detailed,
            group_by: self.group_by,
            nodes: self.nodes,
            parent_task_id: self.parent_task_id,
            timeout: self.timeout,
            wait_for_completion: self.wait_for_completion,
        }
    }
}
#[doc = "Tasks APIs"]
pub struct TasksNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> TasksNamespaceClient<'a> {
    pub fn new(client: &ElasticsearchClient) -> Self {
        TasksNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn cancel(&self, request: &TasksCancelRequest) -> Result<Response> {
        self.client.send(HttpMethod::Post, "/_tasks/_cancel")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn get(&self, request: &TasksGetRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_tasks/{task_id}")
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn list(&self, request: &TasksListRequest) -> Result<Response> {
        self.client.send(HttpMethod::Get, "/_tasks")
    }
}
impl ElasticsearchClient {
    #[doc = "Tasks APIs"]
    pub fn tasks(&self) -> TasksNamespaceClient {
        TasksNamespaceClient::new(self)
    }
}
