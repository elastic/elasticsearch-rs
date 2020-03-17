pub mod common;
use common::*;

use elasticsearch::{
    http::headers::{HeaderValue, X_OPAQUE_ID},
    params::TrackTotalHits,
    SearchParts,
};

use crate::common::client::index_documents;
use elasticsearch::http::headers::{
    HeaderMap, HeaderName, ACCEPT, CONTENT_TYPE, DEFAULT_ACCEPT, DEFAULT_CONTENT_TYPE,
};
use hyper::Method;
use reqwest::StatusCode;
use serde_json::{json, Value};

#[tokio::test]
async fn default_user_agent_content_type_accept_headers() -> Result<(), failure::Error> {
    let server = server::http(move |req| async move {
        assert_eq!(req.headers()["user-agent"], DEFAULT_USER_AGENT);
        assert_eq!(req.headers()["content-type"], "application/json");
        assert_eq!(req.headers()["accept"], "application/json");
        http::Response::default()
    });

    let client = client::create_for_url(format!("http://{}", server.addr()).as_ref());
    let _response = client.ping().send().await?;

    Ok(())
}

#[tokio::test]
async fn x_opaque_id_header() -> Result<(), failure::Error> {
    let server = server::http(move |req| async move {
        assert_eq!(req.headers()["x-opaque-id"], "foo");
        http::Response::default()
    });

    let client = client::create_for_url(format!("http://{}", server.addr()).as_ref());
    let _response = client
        .ping()
        .header(
            HeaderName::from_static(X_OPAQUE_ID),
            HeaderValue::from_static("foo"),
        )
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn serialize_querystring() -> Result<(), failure::Error> {
    let server = server::http(move |req| async move {
        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri().path(), "/_search");
        assert_eq!(
                req.uri().query(),
                Some("filter_path=took%2C_shards&pretty=true&q=title%3AElasticsearch&track_total_hits=100000")
            );
        http::Response::default()
    });

    let client = client::create_for_url(format!("http://{}", server.addr()).as_ref());
    let _response = client
        .search(SearchParts::None)
        .pretty(true)
        .filter_path(&["took", "_shards"])
        .track_total_hits(TrackTotalHits::Count(100_000))
        .q("title:Elasticsearch")
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn search_with_body() -> Result<(), failure::Error> {
    let client = client::create_default();
    let _ = index_documents(&client).await?;
    let response = client
        .search(SearchParts::None)
        .body(json!({
            "query": {
                "match_all": {}
            }
        }))
        .allow_no_indices(true)
        .send()
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    let response_body = response.read_body::<Value>().await?;
    assert!(response_body["took"].as_i64().is_some());

    Ok(())
}

#[tokio::test]
async fn search_with_no_body() -> Result<(), failure::Error> {
    let client = client::create_default();
    let _ = index_documents(&client).await?;
    let response = client
        .search(SearchParts::None)
        .pretty(true)
        .q("title:Elasticsearch")
        .send()
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    let response_body = response.read_body::<Value>().await?;
    assert!(response_body["took"].as_i64().is_some());

    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        assert!(hit["_source"]["title"].as_str().is_some());
    }

    Ok(())
}

#[tokio::test]
async fn cat_health_format_json() -> Result<(), failure::Error> {
    let client = client::create_default();
    let response = client
        .cat()
        .health()
        .format("json")
        .pretty(true)
        .send()
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert!(response
        .headers()
        .get(CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap()
        .starts_with(DEFAULT_CONTENT_TYPE));
    let _response_body = response.read_body::<Value>().await?;

    Ok(())
}

#[tokio::test]
async fn cat_health_header_json() -> Result<(), failure::Error> {
    let client = client::create_default();
    let response = client
        .cat()
        .health()
        .header(ACCEPT, HeaderValue::from_static(DEFAULT_ACCEPT))
        .pretty(true)
        .send()
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert!(response
        .headers()
        .get(CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap()
        .starts_with(DEFAULT_CONTENT_TYPE));
    let _response_body = response.read_body::<Value>().await?;

    Ok(())
}

#[tokio::test]
async fn cat_health_text() -> Result<(), failure::Error> {
    let client = client::create_default();
    let response = client.cat().health().pretty(true).send().await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert!(response
        .headers()
        .get(CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap()
        .starts_with("text/plain"));
    let _response_body = response.read_body_as_text().await?;

    Ok(())
}

#[tokio::test]
async fn clone_search_with_body() -> Result<(), failure::Error> {
    let client = client::create_default();
    let _ = index_documents(&client).await?;
    let base_request = client.search(SearchParts::None);

    let request_clone = base_request.clone().q("title:Elasticsearch").size(1);

    let _request = base_request
        .body(json!({
            "query": {
                "match_all": {}
            }
        }))
        .allow_no_indices(true);

    let response = request_clone.send().await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    let response_body = response.read_body::<Value>().await?;

    assert_eq!(response_body["hits"]["hits"].as_array().unwrap().len(), 1);

    Ok(())
}

#[tokio::test]
async fn byte_slice_body() -> Result<(), failure::Error> {
    let client = client::create_default();
    let body = b"{\"query\":{\"match_all\":{}}}";

    let response = client
        .send(
            elasticsearch::http::Method::Post,
            SearchParts::None.url().as_ref(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body.as_ref()),
        )
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    let _response_body = response.read_body::<Value>().await?;

    Ok(())
}
