use crate::{Action, Config};
use elasticsearch::{
    cluster::ClusterHealthParts,
    http::response::Response,
    indices::{IndicesCreateParts, IndicesDeleteParts},
    Error, IndexParts,
};
use tokio::runtime::Runtime;

pub fn index() -> Action {
    Action {
        action: "index".to_string(),
        category: Some("core".to_string()),
        warmups: 100,
        environment: None,
        repetitions: 10000,
        operations: Some(1),
        setup: Some(setup),
        run,
    }
}

static INDEX: &str = "test-bench-index";

fn setup(config: &Config, runtime: &mut Runtime) -> Result<Response, Error> {
    let client = config.runner_client();
    runtime.block_on(async {
        let _response = client
            .indices()
            .delete(IndicesDeleteParts::Index(&[INDEX]))
            .send()
            .await?;

        let _response = client
            .indices()
            .create(IndicesCreateParts::Index(INDEX))
            .send()
            .await?;

        client
            .cluster()
            .health(ClusterHealthParts::Index(&[INDEX]))
            .send()
            .await
    })
}

fn run(i: i32, config: &Config, runtime: &mut Runtime) -> Result<Response, Error> {
    let client = config.runner_client();
    let json = config.data_sources("small").unwrap();
    runtime.block_on(async {
        client
            .index(IndexParts::IndexId(INDEX, i.to_string().as_str()))
            .body(json)
            .send()
            .await
    })
}
