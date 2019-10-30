extern crate reqwest;

use crate::{error::ElasticsearchError, http_method::HttpMethod, response::ElasticsearchResponse};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT},
    Method,
};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

#[derive(Debug, Clone)]
pub struct Connection {
    client: reqwest::Client,
    url: Url,
}

/// A connection to an Elasticsearch node, used to send a request
impl Connection {
    fn method(&self, method: HttpMethod) -> Method {
        match method {
            HttpMethod::Get => Method::GET,
            HttpMethod::Put => Method::PUT,
            HttpMethod::Post => Method::POST,
            HttpMethod::Delete => Method::DELETE,
            HttpMethod::Head => Method::HEAD,
        }
    }

    pub fn new(url: Url) -> Connection {
        Connection {
            // client: reqwest::Client::builder().proxy(reqwest::Proxy::http("http://localhost:8888").unwrap()).build().unwrap(),
            client: reqwest::Client::new(),
            url,
        }
    }

    /// Sends a request to the Elasticsearch node
    pub fn send<S, Q>(
        &self,
        method: HttpMethod,
        path: &str,
        query_string: Option<&Q>,
        body: Option<S>,
    ) -> Result<ElasticsearchResponse, ElasticsearchError>
    where
        S: Serialize,
        Q: Serialize + ?Sized,
    {
        let url = self.url.join(path).unwrap();
        let reqwest_method = self.method(method);

        let mut headers = HeaderMap::new();

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        // TODO: autogenerate user agent with version
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("elasticsearch-rs/7.3.1"),
        );

        let mut request_builder = self.client.request(reqwest_method, url).headers(headers);

        if let Some(b) = body {
            request_builder = request_builder.json(&b);
        };

        if let Some(q) = query_string {
            request_builder = request_builder.query(q);
        }

        let response = request_builder.send()?;
        Ok(ElasticsearchResponse::new(response))
    }
}

impl Default for Connection {
    fn default() -> Self {
        Connection::new(Url::parse("http://localhost:9200").unwrap())
    }
}
