# elasticsearch-rs: 

Repository for the official Elasticsearch Rust Client.

The project is still very much a _work in progress_ and in an _alpha_ state; 
input and contributions welcome!

## Versions and Compatibility

<table>
    <tr>
        <th><b>Rust client<b></th>
        <th><b>Elasticsearch<b></th>
        <th><b>Status</b></th>
    </tr>
    <tr>
    	<td><code>7.x</code></td>
    	<td><code>7.x</code></td>
    	<td><code>alpha</code></td>
    </tr>
</table>

The Rust client is largely generated from the REST API specs of Elasticsearch. 
As such, the API functions available align with the version of the specs from which
they're generated. Elasticsearch strives to adhere to [Semantic Versioning](https://semver.org/),
which is reflected in the specs and thus this client.

What this means in practice is that a 7.x Rust client is compatible with Elasticsearch 7.x. Where
the client minor version is greater than Elasticsearch minor version, it may contain API functions
for APIs that are not available in the version of Elasticsearch, but all other API functions will
be compatible. Where the client minor version is less than Elasticsearch minor version, all API
functions will be compatible. 

## Overview

The workspace contains two packages

### api_generator

A small executable to download REST API specs from GitHub and generate much of the client from the specs. Run with

```
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

_(Once client is released to crates.io!)_

Add crate name and version to Cargo.toml. Choose the version
that is comaptible with the version of Elasticsearch you're using

```sh
[dependencies]
elasticsearch = "*"
```

#### Connecting

Build a connection to Elasticsearch using the `ConnectionBuilder`, which allows
setting of proxies and authentication schemes

```rust
let url = Url::parse("http://localhost:9200").unwrap();

let conn = ConnectionBuilder::new(url)
    .auth(Credentials::Basic("elastic".into(), "password".into()))
    .build()?;
```

The connection can then be passed to the root client

```rust
let client = Elasticsearch::new(conn);
```

#### Calling an API endpoint

The following will execute a `POST` request to `/_search?allow_no_indices=true` with
a JSON body of `{"query":{"match_all":{}}}`

```rust
let url = Url::parse("http://localhost:9200").unwrap();
let conn = ConnectionBuilder::new(url).build().unwrap();
let client = Elasticsearch::new(conn);

// make a search API call
let search_response = client
    .search(SearchUrlParts::None)
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
let took = response_body["took"].as_i64()?;
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
    - Control API invariants through arguments on API function
    
      e.g. `client.delete_script("script_id".into()).send()?;`
      
      An id must always be provided for a script, so the `delete_script()` function must accept
      it as a value.

## Contributing

Contributing to the client is very much appreciated, no pull request (PR) is too big or too small!
We ask that _before_ opening a PR however, you check to see if there is an issue that discusses the change that you
wish to make. If there isn't, it's best to open a new issue first to discuss it, to save you time in future
and help us further ascertain the crux of the issue. If an issue already exists, please add to the discussion there.

Once an issue has been discussed and agreement that it should be acted upon, if you wish to work on a PR
to address it, please assign the issue to yourself, so that others know that it is being worked on.

### Sign the Contributor License Agreement

We do ask that you sign the [Contiributor License Agreement](https://www.elastic.co/contributor-agreement) before we can accept pull requests from you.

### Current Setup

Using Rust nightly for development, whilst reqwest crate, the HTTP client used, has support for `async`
in an alpha release and has a dependency which uses features that only work on nightly:

```
> rustup show
...

active toolchain
----------------

nightly-x86_64-pc-windows-msvc (default)
rustc 1.41.0-nightly (a44774c3a 2019-11-25)
```

It is expected to move to stable once dependencies compile on stable.

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