extern crate reqwest;

use reqwest::Method;
use std::boxed::Box;

use crate::{es_response::EsResponse, http_method::HttpMethod, node_pool::NodePool};

pub trait Connection {
    type Pool: NodePool;

    fn new<T>(pool: &Self::Pool) -> Self;

    fn send(&self, method: HttpMethod, path: &str) -> EsResponse;
}

pub struct ReqwestConnection {
    client: reqwest::Client,
    pool: Box<NodePool>,
}

impl ReqwestConnection {
    fn method(&self, method: HttpMethod) -> Method {
        match method {
            Get => Method::GET,
            Put => Method::PUT,
            Post => Method::POST,
            Delete => Method::DELETE,
            Head => Method::HEAD,
        }
    }
}

impl Connection for ReqwestConnection {
    fn new<T>(pool: T) -> ReqwestConnection<T> {
        ReqwestConnection {
            client: reqwest::Client::new(),
            pool,
        }
    }

    fn send(&self, method: HttpMethod, path: &str) -> EsResponse {
        let node = self.pool.next();
        let url = node.url.join(path).expect("Not a valid URL");
        let reqwest_method = self.method(method);
        self.client.request(reqwest_method, url).send();
    }
}
