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
pub mod common;
use common::*;

use elasticsearch::ExplainParts;
use reqwest::StatusCode;
use serde_json::{json, Value};

/// Responses in the range 400-599 return Response body
#[tokio::test]
async fn bad_request_returns_response() -> Result<(), failure::Error> {
    let client = client::create_default();
    let response = client
        .explain(ExplainParts::IndexId("non_existent_index", "id"))
        .body(json!({}))
        .send()
        .await?;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

    let error = response.json::<Value>().await?;
    assert_eq!(error["status"].as_i64(), Some(400));
    assert_eq!(
        error["error"]["type"].as_str(),
        Some("action_request_validation_exception")
    );
    assert_eq!(
        error["error"]["reason"].as_str(),
        Some("Validation Failed: 1: query is missing;")
    );

    Ok(())
}

#[tokio::test]
async fn deserialize_exception() -> Result<(), failure::Error> {
    let client = client::create_default();
    let response = client
        .explain(ExplainParts::IndexId("non_existent_index", "id"))
        .error_trace(true)
        .body(json!({}))
        .send()
        .await?;

    let status_code = response.status_code();
    assert_eq!(status_code, StatusCode::BAD_REQUEST);

    let ex = response.exception().await?.unwrap();
    let error = ex.error();

    assert_eq!(ex.status().unwrap(), status_code.as_u16());
    assert_eq!(error.ty(), Some("action_request_validation_exception"));
    assert!(error.stack_trace().is_some());
    assert_eq!(
        error.reason(),
        Some("Validation Failed: 1: query is missing;")
    );
    assert!(!error.root_cause().is_empty());
    assert_eq!(error.reason(), error.root_cause().first().unwrap().reason());
    Ok(())
}
