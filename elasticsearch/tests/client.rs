use elasticsearch::{Elasticsearch, SearchUrlParts};

use reqwest::StatusCode;
use serde_json::json;
use serde_json::Value;

#[tokio::test]
async fn test_search_with_body() -> Result<(), failure::Error> {
    let client = Elasticsearch::default();
    let response = client
        .search(SearchUrlParts::None)
        .body(Some(json!({
            "query": {
                "match_all": {}
            }
        })))
        .allow_no_indices(Some(true))
        .send()
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    let response_body = response.read_body::<Value>().await?;
    assert!(response_body["took"].as_i64().unwrap() > 0);

    Ok(())
}

#[tokio::test]
async fn test_search_no_body() -> Result<(), failure::Error> {
    let client = Elasticsearch::default();
    let response = client
        .search(SearchUrlParts::None)
        .pretty(Some(true))
        .q(Some("title:Elasticsearch".into()))
        .send()
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    let response_body = response.read_body::<Value>().await?;
    assert!(response_body["took"].as_i64().unwrap() > 0);

    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        assert!(hit["_source"]["title"].as_str().is_some());
    }

    Ok(())
}

#[tokio::test]
async fn test_serialize_vec_string_on_querystring() -> Result<(), failure::Error> {
    let client = Elasticsearch::default();
    let response = client
        .search(SearchUrlParts::None)
        .pretty(Some(true))
        .filter_path(Some(vec!["took".into()]))
        .q(Some("title:Elasticsearch".into()))
        .send()
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    let response_body = response.read_body::<Value>().await?;

    assert!(response_body["took"].as_i64().unwrap() > 0);
    assert!(response_body.get("hits").is_none());

    Ok(())
}

#[tokio::test]
async fn test_clone_search_with_body() -> Result<(), failure::Error> {
    let client = Elasticsearch::default();

    let request = client
        .search(SearchUrlParts::None)
        .body(Some(json!({
            "query": {
                "match_all": {}
            }
        })))
        .allow_no_indices(Some(true));

    let request_clone = request
        .clone()
        .q(Some("title:Elasticsearch".into()))
        .size(Some(1))
        .body(Option::<()>::None)
        .allow_no_indices(None);

    let response = request_clone.send().await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    let response_body = response.read_body::<Value>().await?;

    assert_eq!(response_body["hits"]["hits"].as_array().unwrap().len(), 1);

    Ok(())
}
