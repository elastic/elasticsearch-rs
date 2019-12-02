// From reqwest crate
// Licensed under Apache License, Version 2.0
// https://github.com/seanmonstar/reqwest/blob/master/LICENSE-APACHE

pub mod client;
pub mod server;

#[allow(unused)]
pub static DEFAULT_USER_AGENT: &'static str =
    concat!("elasticsearch-rs/", env!("CARGO_PKG_VERSION"));
