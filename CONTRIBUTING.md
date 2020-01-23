# Contributing

Contributing to the client is very much appreciated, no pull request (PR) is too big or too small!
We ask that _before_ opening a PR however, you check to see if there is an issue that discusses the change that you
wish to make. If there isn't, it's best to open a new issue first to discuss it, to save you time in future
and help us further ascertain the crux of the issue. If an issue already exists, please add to the discussion there.

Once an issue has been discussed and agreement that it should be acted upon, if you wish to work on a PR
to address it, please assign the issue to yourself, so that others know that it is being worked on.

## Sign the Contributor License Agreement

We do ask that you sign the [Contiributor License Agreement](https://www.elastic.co/contributor-agreement) 
before we can accept pull requests from you.

## Development

The following information may help with contributing to this project. The workspace contains two packages:

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

### Design principles

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

### Current development setup

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

### Coding style guide

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