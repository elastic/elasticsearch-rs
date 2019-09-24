extern crate reqwest;

use crate::{http_method::HttpMethod, response::ElasticsearchResponse};
use reqwest::{Method, Result};
use serde::de::DeserializeOwned;
use url::Url;
use self::reqwest::header::{HeaderMap, CONTENT_TYPE, HeaderValue, USER_AGENT};

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
            client: reqwest::Client::new(),
            url,
        }
    }

    pub fn send<T>(&self, method: HttpMethod, path: &str, query: Option<&[(String, String)]>, body: Option<Vec<u8>>) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        let url = self.url.join(path).expect("Not a valid URL");
        let reqwest_method = self.method(method);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let mut request_builder = self.client
            .request(reqwest_method, url)
            .headers(headers);

        request_builder = match body {
            Some(b) => request_builder.body(b),
            None => request_builder,
        };

        request_builder = match query {
            Some(q) => request_builder.query(q),
            None => request_builder,
        };

        let mut response = request_builder.send()?;
        let response_body = response.json::<T>()?;

        Ok(ElasticsearchResponse {
            headers: response.headers().clone(),
            status_code: response.status(),
            body: Some(response_body),
        })
    }
}

impl Default for Connection {
    fn default() -> Self {
        Connection::new(Url::parse("http://localhost:9200").unwrap())
    }
}
