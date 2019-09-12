extern crate reqwest;

use crate::{response::EsResponse, http_method::HttpMethod};
use reqwest::Method;
use url::Url;

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

    pub fn send(&self, method: HttpMethod, path: &str) -> reqwest::Result<reqwest::Response> {
        let url = self.url.join(path).expect("Not a valid URL");
        let reqwest_method = self.method(method);
        self.client.request(reqwest_method, url).send()
    }
}
