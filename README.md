# elasticsearch

An official Rust Client for Elasticsearch.

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
- A `7.4.0` client can be used against `7.5.1` Elasticsearch

In the former case, a 7.5.0 client may contain additional API functions that are not available
in 7.0.0 Elasticsearch. In this case, these APIs cannot be used, but for any APIs available in
Elasticsearch, the respective API functions on the client will be compatible.

In the latter case, a 7.4.0 client won't contain API functions for APIs that are introduced in
Elasticsearch 7.5.0+, but for all other APIs available in Elasticsearch, the respective API
functions on the client will be compatible.

**No compatibility assurances are given between different major versions of the client and
Elasticsearch**. Major differences likely exist between major versions of Elasticsearch, particularly
around request and response object formats, but also around API urls and behaviour.

## Overview

The workspace contains two packages

### api_generator

A small executable to download REST API specs from GitHub and generate much of the client from the specs. Run with

```sh
cargo run -p api_generator
```

from the repository root directory, and follow the prompts. The minimum REST API spec version compatible with the 
generator is `v7.4.0`.

The api_generator makes heavy use of the [`syn`](https://docs.rs/syn/1.0.5/syn/) and [`quote`](https://docs.rs/quote/1.0.2/quote/) crates to generate Rust code from the REST API specs.
The `quote!` macro is particularly useful as it accepts Rust code that can include placeholder tokens (prefixed with `#`)
that will be interpolated during expansion. Unlike procedural macros, the token stream returned by the `quote!` macro
can be `to_string()`'ed and written to disk, and this is used to create much of the client scaffolding.

### elasticsearch

The client package crate. The client exposes all Elasticsearch APIs as associated functions, either on
the root client, `Elasticsearch`, or on one of the _namespaced clients_, such as `Cat`, `Indices`, etc. The _namespaced clients_
are based on the grouping of APIs within the [Elasticsearch](https://github.com/elastic/elasticsearch/tree/master/rest-api-spec) and [X-Pack](https://github.com/elastic/elasticsearch/tree/master/x-pack/plugin/src/test/resources/rest-api-spec/api) REST API specs from which much of the client is generated.
All API functions are `async` only, and can be `await`ed.

#### Installing

Add `elasticsearch` crate and version to Cargo.toml. Choose the version
that is compatible with the version of Elasticsearch you're using

```toml
[dependencies]
elasticsearch = "7.5.1-alpha1"
```

The following _optional_ dependencies may also be useful to create requests and read responses

```toml
serde = "~1"
serde_json = "~1"
```

#### Create a client

Build a transport to make API requests to Elasticsearch using the `TransportBuilder`, 
which allows setting of proxies and authentication schemes

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
    let credentials = Credentials::Basic("<username>".into(), "<password>".into());
    let transport = Transport::cloud(cloud_id, credentials)?;
    let client = Elasticsearch::new(transport);
    Ok(())
}
```

 More control over how a [Transport](http::transport::Transport) is built can be
 achieved using [TransportBuilder](http::transport::TransportBuilder) to build a transport, and
 passing it to [Elasticsearch::new] create a new instance of [Elasticsearch]

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

#### Making API calls

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
API call to an endpoint not represented as an API function.

## Design principles

1. Generate as much of the client as feasible from the REST API specs

    The REST API specs contain information about
    - the URL parts e.g. `{index}/{type}/_search` and variants
    - accepted HTTP methods e.g. `GET`, `POST`
    - the URL query string parameters
    - whether the API accepts a body
    
2. Prefer generation methods that produce ASTs and token streams over strings. 
The `quote` and `syn` crates help

3. Get it working, then refine/refactor

    - Start simple and iterate
    - Design of the API is conducive to ease of use
    - Asynchronous only
    - Control API invariants through arguments on API function. For example
    
      ```norun
      client.delete_script(DeleteScriptParts::Id("script_id"))
          .send()
          .await?;
      ```
    
      An id must always be provided for a delete script API call, so the `delete_script()` function 
      must accept it as a value.

## Contributing

Contributing to the client is very much appreciated, no pull request (PR) is too big or too small!
We ask that _before_ opening a PR however, you check to see if there is an issue that discusses the change that you
wish to make. If there isn't, it's best to open a new issue first to discuss it, to save you time in future
and help us further ascertain the crux of the issue. If an issue already exists, please add to the discussion there.

Once an issue has been discussed and agreement that it should be acted upon, if you wish to work on a PR
to address it, please assign the issue to yourself, so that others know that it is being worked on.

### Sign the Contributor License Agreement

We do ask that you sign the [Contiributor License Agreement](https://www.elastic.co/contributor-agreement) 
before we can accept pull requests from you.

### Current Setup

Use Rust nightly for development for now:

```sh
> rustup show
...

active toolchain
----------------

nightly-x86_64-pc-windows-msvc (default)
rustc 1.41.0-nightly (a44774c3a 2019-11-25)
```

It is expected to move to rust stable once dependencies compile on stable.

### Coding styleguide

The repository adheres to the styling enforced by `rustfmt`.

#### Formatting

Rust code can be formatted using [`rustfmt`](https://github.com/rust-lang/rustfmt). Follow the instructions to install.

To format all packages in a workspace, from the workspace root

```sh
cargo fmt
```

It is strongly recommended to run this before opening a PR.

#### Clippy

[Clippy](https://github.com/rust-lang/rust-clippy) is a bunch of lints to catch common mistakes and improve your Rust code! Follow the instructions to install.

Run clippy before opening a PR

```sh
cargo clippy
```

## Development

### Running MSVC debugger in VS Code

From [Bryce Van Dyk's blog post](https://www.brycevandyk.com/debug-rust-on-windows-with-visual-studio-code-and-the-msvc-debugger/), 
if wishing to use the MSVC debugger with Rust in VS code, which may be preferred on Windows

1. Install [C/C++ VS Code extensions](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools)

2. Place the following in `.vscode/launch.json` in the project root

    ```json
    {
        "version": "0.2.0",
        "configurations": [   
            {
                "name": "Debug api_generator",
                "type": "cppvsdbg",
                "request": "launch",
                "program": "${workspaceFolder}/target/debug/api_generator.exe",
                "args": [],
                "stopAtEntry": false,
                "cwd": "${workspaceFolder}",
                "environment": [],
                "externalConsole": false
            }
        ]
    }
    ```
    
3. Add `"debug.allowBreakpointsEverywhere": true` to VS code settings.json

## License

This is free software, licensed under [The Apache License Version 2.0.](LICENSE).