// TODO: whilst developing
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate serde;

mod client;
mod connection;
mod enums;
mod http_method;
mod namespace_clients;
mod response;
mod settings;
//mod node_pool;

pub use crate::{
    client::ElasticsearchClient,
    connection::Connection,
    http_method::HttpMethod,
    //node_pool::{Node, NodePool, SingleNodePool},
    response::ElasticsearchResponse,
    settings::ConnectionSettings,
};
