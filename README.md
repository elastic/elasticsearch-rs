# elasticsearch &emsp; [![Latest Version]][crates.io] [![Docs]][docs.rs] [![Apache-2 licensed]][license]

[Latest Version]: https://img.shields.io/crates/v/elasticsearch.svg
[crates.io]: https://crates.io/crates/elasticsearch
[Docs]: https://docs.rs/elasticsearch/badge.svg
[docs.rs]: https://docs.rs/elasticsearch
[Apache-2 licensed]: https://img.shields.io/crates/l/elasticsearch.svg
[license]: ./LICENSE

Official Rust Client for [Elasticsearch](https://github.com/elastic/elasticsearch).

Full documentation is available at https://docs.rs/elasticsearch

The project is still very much a _work in progress_ and in an _alpha_ state; 
input and contributions welcome!

## Versions and Compatibility

| Rust client | Elasticsearch | Status |
|-------------|---------------|--------|
| 7.x         | 7.x           | alpha  |

A major version of the client is compatible with the same major version of Elasticsearch.
Since Elasticsearch is developed following [Semantic Versioning](https://semver.org/) principles,
Any minor/patch version of the client can be used against any minor/patch version of Elasticsearch
**within the same major version lineage**. For example,

- A `7.5.0` client can be used against `7.0.0` Elasticsearch
- A `7.5.0` client can be used against `7.6.0` Elasticsearch

In the former case, a 7.5.0 client may contain additional API functions that are not available
in 7.0.0 Elasticsearch. In this case, these APIs cannot be used, but for any APIs available in
Elasticsearch, the respective API functions on the client will be compatible.

In the latter case, a 7.5.0 client won't contain API functions for APIs that are introduced in
Elasticsearch 7.6.0+, but for all other APIs available in Elasticsearch, the respective API
functions on the client will be compatible.

**No compatibility assurances are given between different major versions of the client and
Elasticsearch**. Major differences likely exist between major versions of Elasticsearch, particularly
around request and response object formats, but also around API urls and behaviour.

## Features

The following are a list of Cargo features that can be enabled or disabled:

- **native-tls** *(enabled by default)*: Enables TLS functionality provided by `native-tls`.
- **rustls-tls**: Enables TLS functionality provided by `rustls`.

## Getting started

The client exposes all Elasticsearch APIs as associated functions, either on
the root client, `Elasticsearch`, or on one of the _namespaced clients_, such as `Cat`, `Indices`, etc. The _namespaced clients_
are based on the grouping of APIs within the [Elasticsearch](https://github.com/elastic/elasticsearch/tree/master/rest-api-spec) and [X-Pack](https://github.com/elastic/elasticsearch/tree/master/x-pack/plugin/src/test/resources/rest-api-spec/api) REST API specs from which much of the client is generated.
All API functions are `async` only, and can be `await`ed.

### Installing

Add `elasticsearch` crate and version to Cargo.toml. Choose the version
that is compatible with the version of Elasticsearch you're using

```toml
[dependencies]
elasticsearch = "7.6.1-alpha.1"
```

The following _optional_ dependencies may also be useful to create requests and read responses

```toml
serde = "~1"
serde_json = "~1"
```

### Create a client

Build a transport to make API requests to Elasticsearch using the `TransportBuilder`, 
which allows setting of proxies, authentication schemes, certificate validation, and
other transport related settings.

To create a client to make API calls to Elasticsearch running on `http://localhost:9200`

```rust,no_run
use elasticsearch::{Error, Elasticsearch};

fn run() {
    let client = Elasticsearch::default();
}
```
Alternatively, you can create a client to make API calls against Elasticsearch running on a specific url

```rust,no_run
use elasticsearch::{
    Error, Elasticsearch, 
    http::transport::{Transport, SingleNodeConnectionPool}
};

fn run() -> Result<(), Error> {
    let transport = Transport::single_node("https://example.com")?;
    let client = Elasticsearch::new(transport);
    Ok(())
}
```

 If you're running against an Elasticsearch deployment in [Elastic Cloud](https://www.elastic.co/cloud/),
 a client can be created using a [Cloud ID](https://www.elastic.co/guide/en/cloud/current/ec-cloud-id.html)
 and credentials retrieved from the Cloud web console

```rust,no_run
use elasticsearch::{
    auth::Credentials,
    Error, Elasticsearch, 
    http::transport::Transport,
};
use url::Url;

fn run() -> Result<(), Error> {
    let cloud_id = "cluster_name:Y2xvdWQtZW5kcG9pbnQuZXhhbXBsZSQzZGFkZjgyM2YwNTM4ODQ5N2VhNjg0MjM2ZDkxOGExYQ==";
    // can use other types of Credentials too, like Bearer or ApiKey
    let credentials = Credentials::Basic("<username>".into(), "<password>".into());
    let transport = Transport::cloud(cloud_id, credentials)?;
    let client = Elasticsearch::new(transport);
    Ok(())
}
```

 More control over how a `Transport` is built can be
 achieved using `TransportBuilder` to build a transport, and
 passing it to `Elasticsearch::new()` create a new instance of `Elasticsearch`

```rust,no_run
use elasticsearch::{
    auth::Credentials,
    Error, Elasticsearch, 
    http::transport::{TransportBuilder,SingleNodeConnectionPool},
};
use url::Url;

fn run() -> Result<(), Error> {
    let url = Url::parse("https://example.com")?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
    let client = Elasticsearch::new(transport);
    Ok(())
}
```

### Making API calls

The following will execute a `POST` request to `/_search?allow_no_indices=true` with
a JSON body of `{"query":{"match_all":{}}}`

```rust,no_run
use elasticsearch::{Elasticsearch, Error, SearchParts};
use serde_json::{json, Value};

async fn run() -> Result<(), Error> {

    let client = Elasticsearch::default();
    
    // make a search API call
    let search_response = client
        .search(SearchParts::None)
        .body(json!({
            "query": {
                "match_all": {}
            }
        }))
        .allow_no_indices(true)
        .send()
        .await?;
    
    // get the HTTP response status code
    let status_code = search_response.status_code();
    
    // read the response body. Consumes search_response
    let response_body = search_response.read_body::<Value>().await?; 
    
    // read fields from the response body         
    let took = response_body["took"].as_i64().unwrap();

    Ok(())
}
```

The client provides functions on each API builder struct
for all query string parameters available for that API. APIs with multiple
URI path variants, where some can contain parts parameters, are modelled as enums.

`Elasticsearch` also has an async `send` function on the root that allows sending an
API call to an endpoint not represented as an API function, for example, experimental
and beta APIs

```rust,no_run
use elasticsearch::{http::Method, Elasticsearch, Error, SearchParts};
use http::HeaderMap;
use serde_json::{json, Value};
use url::Url;

async fn run() -> Result<(), Error> {
    let client = Elasticsearch::default();
    let body = b"{\"query\":{\"match_all\":{}}}";
    let response = client
        .send(Method::Post,
            SearchParts::Index(&["tweets"]).url().as_ref(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body.as_ref())
        )
        .await?;
    Ok(())
}
```

## License

This is free software, licensed under [The Apache License Version 2.0.](LICENSE).
