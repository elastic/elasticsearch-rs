#[macro_use]
extern crate log;
extern crate simple_logger;

#[macro_use]
extern crate lazy_static;

extern crate serde_json;

#[macro_use]
extern crate quote;
extern crate api_generator;

pub mod generator;
pub mod github;
pub mod regex;
pub mod step;
