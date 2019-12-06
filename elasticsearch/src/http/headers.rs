/// The default user-agent header value sent by the client
pub static DEFAULT_USER_AGENT: &str = concat!("elasticsearch-rs/", env!("CARGO_PKG_VERSION"));

/// The default content-type header value of `application/json`
pub static DEFAULT_CONTENT_TYPE: &str = "application/json";

/// The default accept header value of `application/json`
pub static DEFAULT_ACCEPT: &str = "application/json";
