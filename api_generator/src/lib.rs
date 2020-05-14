// needed for quote!
#![recursion_limit="256"]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate quote;

pub mod error;
pub mod generator;
pub mod rest_spec;
