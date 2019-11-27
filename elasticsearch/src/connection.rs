use crate::{error::ElasticsearchError, http_method::HttpMethod, response::ElasticsearchResponse};

use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT},
    Client, Method,
};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

/// A connection to an Elasticsearch node, used to send an API request
#[derive(Debug, Clone)]
pub struct Connection {
    client: reqwest::Client,
    url: Url,
}

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

    /// Sends a synchronous API request to the Elasticsearch node
    pub async fn send<B, Q>(
        &self,
        method: HttpMethod,
        path: &str,
        query_string: Option<&Q>,
        body: Option<B>,
    ) -> Result<ElasticsearchResponse, ElasticsearchError>
    where
        B: Serialize,
        Q: Serialize + ?Sized,
    {
        let url = self.url.join(path)?;
        let reqwest_method = self.method(method);
        let mut request_builder = self.client.request(reqwest_method, &url.to_string());
        let mut headers = HeaderMap::new();

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        // TODO: autogenerate user agent with version, based on version in Cargo.toml, which will align with the REST spec version of Elasticsearch
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("elasticsearch-rs/0.1.0"),
        );

        request_builder = request_builder.headers(headers);

        if let Some(b) = body {
            request_builder = request_builder.json(&b);
        };

        if let Some(q) = query_string {
            request_builder = request_builder.query(q);
        }

        let response = request_builder.send().await?;
        Ok(ElasticsearchResponse::new(response))
    }
}

impl Default for Connection {
    fn default() -> Self {
        Connection::new(Url::parse("http://localhost:9200").unwrap())
    }
}
