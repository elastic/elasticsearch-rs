// TODO: whilst developing
#![allow(unused_imports)]
#![allow(dead_code)]

mod client;
mod connection;
mod connection_settings;
mod enums;
mod es_response;
mod http_method;
mod namespace_clients;
//mod node_pool;

pub use crate::{
    client::ElasticsearchClient,
    connection::Connection,
    connection_settings::ConnectionSettings,
    es_response::EsResponse,
    http_method::HttpMethod,
    namespace_clients,
    //node_pool::{Node, NodePool, SingleNodePool},
};
