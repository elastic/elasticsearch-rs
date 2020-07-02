/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
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
