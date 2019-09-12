# elasticsearch-rs: Elasticsearch Rust Client

A spacetime project to build a low level Rust client.

## Design principles

1. Generate as much of the client as feasible from the REST API specs
2. Get it working, then refine/refactor. Start simple and iterate

The general usage of the client is envisioned as:

```rust
let settings = ConnectionSettings::new();
let connection = Connection::new(Url::parse("http://localhost:9200").unwrap());
let client = ElasticsearchClient::new(settings, connection);
let response = client.cat().indices();
```

### Running MSVC debugger in VS Code

From [Bryce Van Dyk's blog post](https://www.brycevandyk.com/debug-rust-on-windows-with-visual-studio-code-and-the-msvc-debugger/)

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
