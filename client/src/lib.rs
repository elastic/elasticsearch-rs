// TODO: whilst developing
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate serde;

mod client;
mod connection;
mod settings;
mod enums;
mod response;
mod http_method;
mod namespace_clients;
//mod node_pool;

pub use crate::{
    client::ElasticsearchClient,
    connection::Connection,
    settings::ConnectionSettings,
    response::ElasticsearchResponse,
    http_method::HttpMethod,
    //node_pool::{Node, NodePool, SingleNodePool},
};
