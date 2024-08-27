# Using elasticsearch-rs in Web Assembly

`elasticsearch-rs` runs in Web Assembly runtimes where [the Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) is available. This includes web browsers, node.js and any system based on node.js like Cloudflare workers.

Using the Fetch API is automatically enabled when the target architecture is `wasm`. Note that in this context, the `rustls-tls` and `native-tls` features should not be used (and will cause compilation errors) as TLS encryption is handled by the Fetch API.

**The example**

This is a browser-based demonstration. The [`index.html`](index.html) web page asks for Elasticsearch URL, login and password and returns the server's version and build date.

The Elasticsearch server must have [CORS enabled](https://www.elastic.co/guide/en/elasticsearch/reference/8.1/modules-network.html#http-settings) with the following configuration in `elasticsearch.yml` (on [Elastic Cloud](https://cloud.elastic.co/), you can add this in "Manage user settings and extensions"):

```yaml
http.cors.enabled: true
http.cors.allow-origin: "*"  # avoid this in production
http.cors.allow-credentials: true
```

If using Elastisearch older than version 8.8.0, also add this configuration:
```yaml
http.cors.allow-headers: Content-Type, Content-Length, Authorization, Accept, User-Agent, X-Elastic-Client-Meta
```


After [installing the `wasm-pack` CLI](https://rustwasm.github.io/docs/wasm-pack/), run: 

```
# Build
wasm-pack build --target web

# Serve the files
python3 -m http.server

# Open http://localhost:8000
```
