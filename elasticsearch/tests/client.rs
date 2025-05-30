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

use elasticsearch::{
    http::{
        headers::{
            HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, DEFAULT_ACCEPT,
            DEFAULT_CONTENT_TYPE, X_OPAQUE_ID,
        },
        StatusCode,
    },
    indices::{IndicesCreateParts, IndicesDeleteParts},
    params::TrackTotalHits,
    SearchParts,
};

use crate::common::client::index_documents;
use bytes::Bytes;
use http::Method;
use serde_json::{json, Value};
use std::time::Duration;

#[tokio::test]
async fn default_user_agent_content_type_accept_headers() -> anyhow::Result<()> {
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
async fn default_header() -> anyhow::Result<()> {
    let server = server::http(move |req| async move {
        assert_eq!(req.headers()["x-opaque-id"], "foo");
        http::Response::default()
    });

    let builder = client::create_builder(format!("http://{}", server.addr()).as_ref()).header(
        HeaderName::from_static(X_OPAQUE_ID),
        HeaderValue::from_static("foo"),
    );

    let client = client::create(builder);
    let _response = client.ping().send().await?;

    Ok(())
}

#[tokio::test]
async fn override_default_header() -> anyhow::Result<()> {
    let server = server::http(move |req| async move {
        assert_eq!(req.headers()["x-opaque-id"], "bar");
        http::Response::default()
    });

    let builder = client::create_builder(format!("http://{}", server.addr()).as_ref()).header(
        HeaderName::from_static(X_OPAQUE_ID),
        HeaderValue::from_static("foo"),
    );

    let client = client::create(builder);
    let _response = client
        .ping()
        .header(
            HeaderName::from_static(X_OPAQUE_ID),
            HeaderValue::from_static("bar"),
        )
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn x_opaque_id_header() -> anyhow::Result<()> {
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
async fn uses_global_request_timeout() {
    let server = server::http(move |_| async move {
        std::thread::sleep(Duration::from_secs(1));
        http::Response::default()
    });

    let builder = client::create_builder(format!("http://{}", server.addr()).as_ref())
        .timeout(std::time::Duration::from_millis(500));

    let client = client::create(builder);
    let response = client.ping().send().await;

    match response {
        Ok(_) => panic!("Expected timeout error, but response received"),
        Err(e) => assert!(e.is_timeout(), "Expected timeout error, but was {:?}", e),
    }
}

#[tokio::test]
async fn uses_call_request_timeout() {
    let server = server::http(move |_| async move {
        std::thread::sleep(Duration::from_secs(1));
        http::Response::default()
    });

    let builder = client::create_builder(format!("http://{}", server.addr()).as_ref())
        .timeout(std::time::Duration::from_secs(2));

    let client = client::create(builder);
    let response = client
        .ping()
        .request_timeout(Duration::from_millis(500))
        .send()
        .await;

    match response {
        Ok(r) => panic!("Expected timeout error, but response received: {:?}", r),
        Err(e) => assert!(e.is_timeout(), "Expected timeout error, but was {:?}", e),
    }
}

#[tokio::test]
async fn call_request_timeout_supersedes_global_timeout() {
    let server = server::http(move |_| async move {
        std::thread::sleep(Duration::from_secs(1));
        http::Response::default()
    });

    let builder = client::create_builder(format!("http://{}", server.addr()).as_ref())
        .timeout(std::time::Duration::from_millis(500));

    let client = client::create(builder);
    let response = client
        .ping()
        .request_timeout(Duration::from_secs(2))
        .send()
        .await;

    match response {
        Ok(_) => (),
        Err(e) => assert!(e.is_timeout(), "Did not expect error, but was {:?}", e),
    }
}

#[tokio::test]
async fn warning_headers() -> anyhow::Result<()> {
    let client = client::create_default();
    let index = "warnings-test";
    let _delete_response = client
        .indices()
        .delete(IndicesDeleteParts::Index(&[index]))
        .send()
        .await?;

    let _create_response = client
        .indices()
        .create(IndicesCreateParts::Index(index))
        .send()
        .await?;

    // ES|QL produces a warning when no limit is defined
    let response = client
        .esql()
        .query()
        .body(json!({ "query": "from warnings-test | keep *" }))
        .send()
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    let warnings = response.warning_headers().collect::<Vec<&str>>();
    assert!(!warnings.is_empty());
    assert!(
        warnings.iter().any(|&w| w.contains("No limit defined")),
        "warnings= {:?}",
        &warnings
    );

    let _delete_response = client
        .indices()
        .delete(IndicesDeleteParts::Index(&[index]))
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn serialize_querystring() -> anyhow::Result<()> {
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
async fn search_with_body() -> anyhow::Result<()> {
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

    let expected_url = {
        let mut addr = client::cluster_addr();
        if !addr.ends_with('/') {
            addr.push('/');
        }
        let mut url = url::Url::parse(addr.as_str())?;
        url.set_username("").unwrap();
        url.set_password(None).unwrap();
        url.join("_search?allow_no_indices=true")?
    };

    if let Some(c) = response.content_length() {
        assert!(c > 0);
    }

    assert_eq!(response.url(), &expected_url);
    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.method(), elasticsearch::http::Method::Post);
    let debug = format!("{:?}", &response);
    assert!(debug.contains("method"));
    assert!(debug.contains("status_code"));
    assert!(debug.contains("headers"));
    let response_body = response.json::<Value>().await?;
    assert!(response_body["took"].as_i64().is_some());

    Ok(())
}

#[tokio::test]
async fn search_with_no_body() -> anyhow::Result<()> {
    let client = client::create_default();
    let _ = index_documents(&client).await?;
    let response = client
        .search(SearchParts::None)
        .pretty(true)
        .q("title:Elasticsearch")
        .send()
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.method(), elasticsearch::http::Method::Get);
    let response_body = response.json::<Value>().await?;
    assert!(response_body["took"].as_i64().is_some());

    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        assert!(hit["_source"]["title"].as_str().is_some());
    }

    Ok(())
}

#[tokio::test]
async fn read_response_as_bytes() -> anyhow::Result<()> {
    let client = client::create_default();
    let _ = index_documents(&client).await?;
    let response = client
        .search(SearchParts::None)
        .pretty(true)
        .q("title:Elasticsearch")
        .send()
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.method(), elasticsearch::http::Method::Get);

    let response: Bytes = response.bytes().await?;
    let json: Value = serde_json::from_slice(&response).unwrap();

    assert!(json["took"].as_i64().is_some());

    for hit in json["hits"]["hits"].as_array().unwrap() {
        assert!(hit["_source"]["title"].as_str().is_some());
    }

    Ok(())
}

#[tokio::test]
async fn cat_health_format_json() -> anyhow::Result<()> {
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
    let _response_body = response.json::<Value>().await?;

    Ok(())
}

#[tokio::test]
async fn cat_health_header_json() -> anyhow::Result<()> {
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
    let _response_body = response.json::<Value>().await?;

    Ok(())
}

#[tokio::test]
async fn cat_health_text() -> anyhow::Result<()> {
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
    let _response_body = response.text().await?;

    Ok(())
}

#[tokio::test]
async fn clone_search_with_body() -> anyhow::Result<()> {
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
    let response_body = response.json::<Value>().await?;

    assert_eq!(response_body["hits"]["hits"].as_array().unwrap().len(), 1);

    Ok(())
}

#[tokio::test]
async fn byte_slice_body() -> anyhow::Result<()> {
    let client = client::create_default();
    let body = b"{\"query\":{\"match_all\":{}}}";

    let response = client
        .send(
            elasticsearch::http::Method::Post,
            SearchParts::None.url().as_ref(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body.as_ref()),
            None,
        )
        .await?;

    assert_eq!(response.status_code(), StatusCode::OK);
    let _response_body = response.json::<Value>().await?;

    Ok(())
}
