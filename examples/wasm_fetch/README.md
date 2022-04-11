# Using elasticsearch-rs in the browser

Uses the Rust client in a browser using WebAssembly. The [`index.html`](index.html) web page asks for Elasticsearch URL, login and password and returns the server's version and build date.

The Elasticsearch server must have [CORS enabled](https://www.elastic.co/guide/en/elasticsearch/reference/8.1/modules-network.html#http-settings) with the following configuration in `elasticsearch.yml`:

```yaml
http.cors.enabled: true
http.cors.allow-origin: "*"  # avoid this in production
http.cors.allow-credentials: true
http.cors.allow-headers: Content-Type, Content-Length, Authorization, Accept, User-Agent, X-Elastic-Client-Meta
```

After installing [the `wasm-pack` CLI](https://rustwasm.github.io/docs/wasm-pack/), run: 

```
# Build
wasm-pack build --target web

# Serve the files
python3 -m http.server

# Open http://localhost:8000
```
