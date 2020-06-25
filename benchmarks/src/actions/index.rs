use crate::Action;
use elasticsearch::{
    cluster::ClusterHealthParts,
    http::response::Response,
    indices::{IndicesCreateParts, IndicesDeleteParts},
    Elasticsearch, Error, IndexParts,
};
use tokio::runtime::Runtime;

pub fn action() -> Action {
    Action {
        action: "index".to_string(),
        category: Some("core".to_string()),
        warmups: 0,
        environment: None,
        repetitions: 10000,
        operations: Some(0),
        setup: Some(setup),
        run: index,
    }
}

static INDEX: &'static str = "test-bench-index";

fn setup(client: &Elasticsearch, runtime: &mut Runtime) -> Result<Response, Error> {
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

fn index(i: i32, client: &Elasticsearch, runtime: &mut Runtime) -> Result<Response, Error> {
    runtime.block_on(async {
        client
            .index(IndexParts::IndexId(INDEX, i.to_string().as_str()))
            .body(json!(
                {
                    "small": "document here"
                }
            ))
            .send()
            .await
    })
}
