use crate::{
    connection::Connection, connection_settings::ConnectionSettings, es_response::EsResponse,
    http_method::HttpMethod,
};

pub struct Client<T> {
    settings: ConnectionSettings<T>,
}

impl<T> Client<T>
where
    T: Connection,
{
    pub fn new<T>(settings: ConnectionSettings<T>) -> Client<T> {
        Client { settings }
    }

    pub fn send(&self, method: HttpMethod, path: &str) -> EsResponse {
        self.settings.connection().send(method, path)
    }
}
