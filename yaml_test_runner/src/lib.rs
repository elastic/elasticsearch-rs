#[macro_use]
extern crate log;
extern crate simple_logger;

#[macro_use]
mod macros;

#[macro_use]
extern crate lazy_static;

extern crate serde_json;

#[macro_use]
extern crate quote;
extern crate api_generator;

pub mod generator;
pub mod step;
pub mod github;
pub mod client;
pub mod regex;
pub mod transform;
pub mod util;