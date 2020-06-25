use crate::Action;
use elasticsearch::{http::response::Response, Elasticsearch, Error};
use tokio::runtime::Runtime;

pub fn action() -> Action {
    Action {
        action: "ping".to_string(),
        category: Some("core".to_string()),
        warmups: 0,
        environment: None,
        repetitions: 10000,
        operations: Some(0),
        setup: None,
        run: ping,
    }
}

fn ping(i: i32, client: &Elasticsearch, runtime: &mut Runtime) -> Result<Response, Error> {
    runtime.block_on(async { client.ping().send().await })
}
