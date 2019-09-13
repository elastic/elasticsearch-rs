

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct TasksCancelRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    actions: Option<&'a Vec<String>>,
    nodes: Option<&'a Vec<String>>,
    parent_task_id: &'a str,
}
impl<'a> TasksCancelRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        TasksCancelRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for TasksCancelRequestBuilder<'a> {
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
pub struct TasksGetRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    timeout: &'a str,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> TasksGetRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        TasksGetRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for TasksGetRequestBuilder<'a> {
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
pub struct TasksListRequestBuilder<'a> {
    client: &'a ElasticsearchClient,
    error_trace: Option<&'a bool>,
    filter_path: Option<&'a Vec<String>>,
    human: Option<&'a bool>,
    pretty: Option<&'a bool>,
    source: &'a str,
    actions: Option<&'a Vec<String>>,
    detailed: Option<&'a bool>,
    group_by: Option<&'a i32>,
    nodes: Option<&'a Vec<String>>,
    parent_task_id: &'a str,
    timeout: &'a str,
    wait_for_completion: Option<&'a bool>,
}
impl<'a> TasksListRequestBuilder<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        TasksListRequestBuilder {
            client,
            ..Default::default()
        }
    }
}
impl<'a> Sender for TasksListRequestBuilder<'a> {
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
pub struct TasksNamespaceClient<'a> {
    client: &'a ElasticsearchClient,
}
impl<'a> TasksNamespaceClient<'a> {
    pub fn new(client: &'a ElasticsearchClient) -> Self {
        TasksNamespaceClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn cancel(&self) -> TasksCancelRequestBuilder {
        TasksCancelRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn get(&self) -> TasksGetRequestBuilder {
        TasksGetRequestBuilder::default()
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn list(&self) -> TasksListRequestBuilder {
        TasksListRequestBuilder::default()
    }
}
impl ElasticsearchClient {
    #[doc = "Tasks APIs"]
    pub fn tasks(&self) -> TasksNamespaceClient {
        TasksNamespaceClient::new(self)
    }
}
