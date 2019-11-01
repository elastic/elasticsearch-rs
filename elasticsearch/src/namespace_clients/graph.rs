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
pub struct GraphExplore<B> {
    client: Elasticsearch,
    body: Option<B>,
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
impl<B> GraphExplore<B>
where
    B: Serialize,
{
    pub fn new(client: Elasticsearch, index: Vec<String>) -> Self {
        GraphExplore {
            client,
            index: index,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            routing: None,
            source: None,
            timeout: None,
            ty: None,
        }
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
    #[doc = "A comma-separated list of index names to search; use `_all` or empty string to perform the operation on all indices"]
    pub fn index(mut self, index: Vec<String>) -> Self {
        self.index = index;
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
    #[doc = "A comma-separated list of document types to search; leave empty to perform the operation on all types"]
    pub fn ty(mut self, ty: Option<Vec<String>>) -> Self {
        self.ty = ty;
        self
    }
}
impl<B> Sender for GraphExplore<B>
where
    B: Serialize,
{
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = match &self.ty {
            Some(ty) => {
                let index_str = self.index.join(",");
                let ty_str = ty.join(",");
                let mut p = String::with_capacity(17usize + index_str.len() + ty_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/");
                p.push_str(ty_str.as_ref());
                p.push_str("/_graph/explore");
                std::borrow::Cow::Owned(p)
            }
            None => {
                let index_str = self.index.join(",");
                let mut p = String::with_capacity(16usize + index_str.len());
                p.push_str("/");
                p.push_str(index_str.as_ref());
                p.push_str("/_graph/explore");
                std::borrow::Cow::Owned(p)
            }
        };
        let method = match self.body {
            Some(_) => HttpMethod::Post,
            None => HttpMethod::Get,
        };
        let query_string = {
            #[derive(Serialize)]
            struct QueryParamsStruct {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "filter_path")]
                filter_path: Option<Vec<String>>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "routing")]
                routing: Option<String>,
                #[serde(rename = "source")]
                source: Option<String>,
                #[serde(rename = "timeout")]
                timeout: Option<String>,
            }
            let query_params = QueryParamsStruct {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)?;
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
    pub fn explore<B>(&self, index: Vec<String>) -> GraphExplore<B>
    where
        B: Serialize,
    {
        GraphExplore::new(self.client.clone(), index)
    }
}
impl Elasticsearch {
    #[doc = "Graph APIs"]
    pub fn graph(&self) -> Graph {
        Graph::new(self.clone())
    }
}