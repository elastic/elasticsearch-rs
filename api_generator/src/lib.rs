// needed for quote!
#![recursion_limit = "512"]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate quote;

pub mod error;
pub mod generator;
pub mod rest_spec;
