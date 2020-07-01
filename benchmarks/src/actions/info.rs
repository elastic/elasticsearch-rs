use crate::{Action, Config};
use elasticsearch::{http::response::Response, Error};
use tokio::runtime::Runtime;

pub fn info() -> Action {
    Action {
        action: "info".to_string(),
        category: Some("core".to_string()),
        warmups: 0,
        environment: None,
        repetitions: 10000,
        operations: Some(1),
        setup: None,
        run,
    }
}

fn run(_i: i32, config: &Config, runtime: &mut Runtime) -> Result<Response, Error> {
    let client = config.runner_client();
    runtime.block_on(async { client.info().send().await })
}
