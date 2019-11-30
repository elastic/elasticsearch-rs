mod support;
use support::*;

use elasticsearch::{Elasticsearch, SearchUrlParts, Connection, ConnectionBuilder, Credentials};

use reqwest::StatusCode;
use serde_json::json;
use serde_json::Value;
use url::Url;
use sysinfo::{SystemExt};

fn create_client(url: Url) -> Elasticsearch {
    let mut connection_builder = ConnectionBuilder::new(url);

    // check if the Fiddler process is running, and hook it up as a proxy if so.
    let system = sysinfo::System::new();
    if !system.get_process_by_name("Fiddler").is_empty() {
        let proxy_url = Url::parse("http://localhost:8888").unwrap();
        connection_builder = connection_builder.proxy(proxy_url);
    }

    let connection = connection_builder.build().unwrap();
    Elasticsearch::new(connection)
}

#[tokio::test]
async fn sends_default_user_agent_content_type_accept_headers() -> Result<(), failure::Error> {
    let server = server::http(move |req| {
        async move {
            assert_eq!(req.headers()["user-agent"], DEFAULT_USER_AGENT);
            assert_eq!(req.headers()["content-type"], "application/json");
            assert_eq!(req.headers()["accept"], "application/json");
            http::Response::default()
        }
    });

    let url = Url::parse(format!("http://{}", server.addr()).as_ref())?;
    let client = create_client(url);
    let response = client
        .search(SearchUrlParts::None)
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn search_with_body() -> Result<(), failure::Error> {
    let client = create_client(Url::parse("http://localhost:9200").unwrap());
    let response = client
        .search(SearchUrlParts::None)
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
    assert!(response_body["took"].as_i64().unwrap() > 0);

    Ok(())
}

#[tokio::test]
async fn search_with_no_body() -> Result<(), failure::Error> {
    let client = create_client(Url::parse("http://localhost:9200").unwrap());
    let response = client
        .search(SearchUrlParts::None)
        .pretty(true)
        .q("title:Elasticsearch")
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
async fn cat_count() -> Result<(), failure::Error> {
    let client = create_client(Url::parse("http://localhost:9200").unwrap());
    let response = client
        .cat()
        .health()
        .format("json")
        .pretty(true)
        .send()
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    let _response_body = response.read_body::<Value>().await?;

    Ok(())
}

#[tokio::test]
async fn serialize_slice_collection_on_querystring() -> Result<(), failure::Error> {
    let client = create_client(Url::parse("http://localhost:9200").unwrap());
    let response = client
        .search(SearchUrlParts::None)
        .pretty(true)
        .filter_path(&["took"])
        .q("title:Elasticsearch")
        .send()
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    let response_body = response.read_body::<Value>().await?;

    assert!(response_body["took"].as_i64().unwrap() > 0);
    assert!(response_body.get("hits").is_none());

    Ok(())
}

#[tokio::test]
async fn clone_search_with_body() -> Result<(), failure::Error> {
    let client = create_client(Url::parse("http://localhost:9200").unwrap());

    let base_request = client
        .search(SearchUrlParts::None);

    let request_clone = base_request
        .clone()
        .q("title:Elasticsearch")
        .size(1);

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
