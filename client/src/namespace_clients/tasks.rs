

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct TasksCancelBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    actions: Option<Vec<String>>,
    nodes: Option<Vec<String>>,
    parent_task_id: Option<String>,
}
impl TasksCancelBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        TasksCancelBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for TasksCancelBuilder {
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
pub struct TasksGetBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_completion: Option<bool>,
}
impl TasksGetBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        TasksGetBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for TasksGetBuilder {
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
pub struct TasksListBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    actions: Option<Vec<String>>,
    detailed: Option<bool>,
    group_by: Option<i32>,
    nodes: Option<Vec<String>>,
    parent_task_id: Option<String>,
    timeout: Option<String>,
    wait_for_completion: Option<bool>,
}
impl TasksListBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        TasksListBuilder {
            client,
            ..Default::default()
        }
    }
}
impl Sender for TasksListBuilder {
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
pub struct TasksClient {
    client: ElasticsearchClient,
}
impl TasksClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        TasksClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn cancel(&self) -> TasksCancelBuilder {
        TasksCancelBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn get(&self) -> TasksGetBuilder {
        TasksGetBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn list(&self) -> TasksListBuilder {
        TasksListBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Tasks APIs"]
    pub fn tasks(&self) -> TasksClient {
        TasksClient::new(self.clone())
    }
}
