---
mapped_pages:
  - https://www.elastic.co/guide/en/elasticsearch/client/rust-api/current/overview.html
  - https://www.elastic.co/guide/en/elasticsearch/client/rust-api/current/index.html
navigation_title: Rust
---

# Elasticsearch Rust Client [overview]

This is the official Rust client for {{es}}. Full documentation is hosted on [docs.rs](https://docs.rs/elasticsearch) — this page provides *only* an overview.

Further resources:

* [Source code](https://github.com/elastic/elasticsearch-rs)
* [API documentation](https://docs.rs/elasticsearch)


## Features [features]

* Fluent builders for all {{es}} REST API endpoints
* Persistent keep-alive connections
* TLS support with system or custom certificates
* Proxy support with authentication
* Async support with Tokio


## {{es}} Version compatibility [_es_version_compatibility]

The Elasticsearch Rust client is forward compatible; meaning that the client supports communicating with greater minor versions of Elasticsearch. Elasticsearch language clients are also backwards compatible with lesser supported minor Elasticsearch versions.


## Create a client [_create_a_client]

To create a client to make API calls to Elasticsearch running on `https://localhost:9200`

```rust
let client = Elasticsearch::default();
```

Alternatively, you can create a client to make API calls against Elasticsearch running on a specific `url::Url`

```rust
let transport = Transport::single_node("https://example.com")?;
let client = Elasticsearch::new(transport);
```

If you’re running against an Elasticsearch deployment in [Elastic Cloud](https://www.elastic.co/cloud/), a client can be created using a [Cloud ID](docs-content://deploy-manage/deploy/elastic-cloud/find-cloud-id.md) and credentials retrieved from the Cloud web console

```rust
let cloud_id = "<cloud id from cloud web console>";
let credentials = Credentials::Basic("<username>".into(), "<password>".into());
let transport = Transport::cloud(cloud_id, credentials)?;
let client = Elasticsearch::new(transport);
```


## Making API calls [_making_api_calls]

The following makes an API call to `tweets/_search` with the json body `{"query":{"match":{"message":"Elasticsearch"}}}`

```rust
let response = client
    .search(SearchParts::Index(&["tweets"]))
    .from(0)
    .size(10)
    .body(json!({
        "query": {
            "match": {
                "message": "Elasticsearch rust"
            }
        }
    }))
    .send()
    .await?;

let response_body = response.json::<Value>().await?;
let took = response_body["took"].as_i64().unwrap();
for hit in response_body["hits"]["hits"].as_array().unwrap() {
    // print the source document
    println!("{:?}", hit["_source"]);
}
```

