extern crate reqwest;

use self::reqwest::{
    Response,
    Error
};

pub struct EsResponse {
    // TODO: figure out what to expose
    response: Result<Response, Error>
}