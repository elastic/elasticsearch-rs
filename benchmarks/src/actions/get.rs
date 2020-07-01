use crate::{Action, Config};
use elasticsearch::{
    http::response::Response, indices::IndicesDeleteParts, params::Refresh, Error, GetParts,
    IndexParts,
};
use tokio::runtime::Runtime;

pub fn get() -> Action {
    Action {
        action: "get".to_string(),
        category: Some("core".to_string()),
        warmups: 100,
        environment: None,
        repetitions: 10000,
        operations: Some(1),
        setup: Some(setup),
        run,
    }
}

static INDEX: &str = "test-bench-get";

fn setup(config: &Config, runtime: &mut Runtime) -> Result<Response, Error> {
    let client = config.runner_client();
    runtime.block_on(async {
        let _response = client
            .indices()
            .delete(IndicesDeleteParts::Index(&[INDEX]))
            .send()
            .await?;

        client
            .index(IndexParts::IndexId(INDEX, "1"))
            .body(json!({"title":"Test"}))
            .refresh(Refresh::WaitFor)
            .send()
            .await
    })
}

fn run(_i: i32, config: &Config, runtime: &mut Runtime) -> Result<Response, Error> {
    let client = config.runner_client();
    runtime.block_on(async { client.get(GetParts::IndexId(INDEX, "1")).send().await })
}
