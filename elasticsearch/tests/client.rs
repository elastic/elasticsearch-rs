use elasticsearch::{Elasticsearch, SearchUrlParts, Connection, ConnectionBuilder, Credentials};

use reqwest::StatusCode;
use serde_json::json;
use serde_json::Value;
use url::Url;
use sysinfo::{SystemExt};

fn create_client() -> Elasticsearch {
    let url = Url::parse("http://localhost:9200").unwrap();
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
async fn search_with_body() -> Result<(), failure::Error> {
    let client = create_client();
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
    let client = create_client();
    let response = client
        .search(SearchUrlParts::None)
        .pretty(true)
        .q("title:Elasticsearch".into())
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
async fn serialize_vec_string_on_querystring() -> Result<(), failure::Error> {
    let client = create_client();
    let response = client
        .search(SearchUrlParts::None)
        .pretty(true)
        .filter_path(vec!["took".into()])
        .q("title:Elasticsearch".into())
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
    let client = create_client();

    let base_request = client
        .search(SearchUrlParts::None);

    let request_clone = base_request
        .clone()
        .q("title:Elasticsearch".into())
        .size(1);

    let request = base_request
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
