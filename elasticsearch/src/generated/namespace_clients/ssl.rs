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
#[doc = "API parts for the Ssl Certificates API"]
pub enum SslCertificatesParts {
    #[doc = "No parts"]
    None,
}
impl SslCertificatesParts {
    #[doc = "Builds a relative URL path to the Ssl Certificates API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SslCertificatesParts::None => "/_ssl/certificates".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Ssl Certificates API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-ssl.html)"]
pub struct SslCertificates<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SslCertificatesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SslCertificates<'a, 'b> {
    #[doc = "Creates a new instance of [SslCertificates]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        SslCertificates {
            client,
            parts: SslCertificatesParts::None,
            headers: HeaderMap::new(),
            error_trace: None,
            filter_path: None,
            human: None,
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
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
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
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ssl Certificates API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
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
#[doc = "Namespace client for Ssl APIs"]
pub struct Ssl<'a> {
    client: &'a Elasticsearch,
}
impl<'a> Ssl<'a> {
    #[doc = "Creates a new instance of [Ssl]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        Self { client }
    }
    #[doc = "[Ssl Certificates API](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-ssl.html)"]
    pub fn certificates<'b>(&'a self) -> SslCertificates<'a, 'b> {
        SslCertificates::new(&self.client)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Ssl APIs"]
    pub fn ssl(&self) -> Ssl {
        Ssl::new(&self)
    }
}
