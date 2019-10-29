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
use super::super::client::Elasticsearch;
use super::super::enums::*;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::error::ElasticsearchError;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
#[derive(Default)]
pub struct GraphExplore {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Vec<String>,
    pretty: Option<bool>,
    routing: Option<String>,
    source: Option<String>,
    timeout: Option<String>,
    ty: Option<Vec<String>>,
}
impl GraphExplore {
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        GraphExplore {
            client,
            index: index,
            ..Default::default()
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
    #[doc = "Specific routing value"]
    pub fn routing(mut self, routing: Option<String>) -> Self {
        self.routing = routing;
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
impl Sender for GraphExplore {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let query_params = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                routing: self.routing,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body: Option<()> = None;
        let response = self
            .client
            .send(HttpMethod::Post, "/", query_params.as_ref(), body)?;
        Ok(response)
    }
}
#[doc = "Graph APIs"]
pub struct Graph {
    client: Elasticsearch,
}
impl Graph {
    pub fn new(client: Elasticsearch) -> Self {
        Graph { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/graph-explore-api.html"]
    pub fn explore(&self, index: Vec<String>) -> GraphExplore {
        GraphExplore::new(self.client.clone(), index)
    }
}
impl Elasticsearch {
    #[doc = "Graph APIs"]
    pub fn graph(&self) -> Graph {
        Graph::new(self.clone())
    }
}