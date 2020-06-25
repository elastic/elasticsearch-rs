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
