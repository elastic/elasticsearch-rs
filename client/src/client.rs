use crate::{
    connection::Connection, settings::ConnectionSettings, response::ElasticsearchResponse,
    http_method::HttpMethod,
};

use reqwest::{header::HeaderMap, Response, Result, StatusCode};
use serde::de::DeserializeOwned;

/// Sender trait for terminal method to send the request to Elasticsearch
pub trait Sender {
    fn send<T>(self) -> Result<(HeaderMap, StatusCode, Option<T>)>
    where
        T: DeserializeOwned;
}

pub struct ElasticsearchClient {
    settings: ConnectionSettings,
    connection: Connection,
}

impl ElasticsearchClient {
    pub fn new<T>(settings: ConnectionSettings, connection: Connection) -> Self {
        ElasticsearchClient {
            settings,
            connection,
        }
    }

    pub fn send(&self, method: HttpMethod, path: &str) -> Result<Response> {
        self.connection.send(method, path)
    }
}
