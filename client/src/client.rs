use crate::{
    connection::Connection, connection_settings::ConnectionSettings, es_response::EsResponse,
    http_method::HttpMethod,
};

use reqwest::{Response, Result};

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
