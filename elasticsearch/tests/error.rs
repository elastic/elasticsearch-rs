pub mod common;
use common::*;

use reqwest::StatusCode;
use serde_json::{json, Value};
use elasticsearch::ExplainParts;

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

    let error = response.read_body::<Value>().await?;
    assert_eq!(error["status"].as_i64(), Some(400));
    assert_eq!(error["error"]["type"].as_str(), Some("action_request_validation_exception"));
    assert_eq!(error["error"]["reason"].as_str(), Some("Validation Failed: 1: query is missing;"));

    Ok(())
}