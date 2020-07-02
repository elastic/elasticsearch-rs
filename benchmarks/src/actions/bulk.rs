use crate::{Action, Config};
use elasticsearch::{
    cluster::ClusterHealthParts,
    http::response::Response,
    indices::{IndicesCreateParts, IndicesDeleteParts},
    BulkOperation, BulkParts, Error,
};
use serde_json::Value;
use tokio::runtime::Runtime;

pub fn bulk() -> Action {
    Action {
        action: "bulk".to_string(),
        category: Some("core".to_string()),
        warmups: 10,
        environment: None,
        repetitions: 1000,
        operations: Some(OPERATIONS),
        setup: Some(setup),
        run,
    }
}

static OPERATIONS: i32 = 10000;

static INDEX: &str = "test-bench-bulk";

static mut BODY: Vec<BulkOperation<Value>> = Vec::new();

unsafe fn push_to_body(op: BulkOperation<Value>) {
    BODY.push(op);
}

fn setup(config: &Config, runtime: &mut Runtime) -> Result<Response, Error> {
    let client = config.runner_client();

    for _ in 0..OPERATIONS {
        unsafe {
            push_to_body(
                BulkOperation::index(config.data_sources("small").unwrap().clone()).into(),
            );
        }
    }

    runtime.block_on(async {
        let _response = client
            .indices()
            .delete(IndicesDeleteParts::Index(&[INDEX]))
            .send()
            .await?;

        let _response = client
            .indices()
            .create(IndicesCreateParts::Index(INDEX))
            .body(json!({
                "settings": {
                    "number_of_shards": 3,
                    "refresh_interval": "5s"
                }
            }))
            .send()
            .await?;

        client
            .cluster()
            .health(ClusterHealthParts::Index(&[INDEX]))
            .send()
            .await
    })
}

fn run(_i: i32, config: &Config, runtime: &mut Runtime) -> Result<Response, Error> {
    let client = config.runner_client();
    runtime.block_on(async {
        unsafe {
            client
                .bulk(BulkParts::Index(INDEX))
                .body(BODY.to_vec())
                .send()
                .await
        }
    })
}
