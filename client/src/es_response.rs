extern crate reqwest;

use self::reqwest::{Error, Response};

pub struct EsResponse {
    // TODO: figure out what to expose
    response: Result<Response, Error>,
}
