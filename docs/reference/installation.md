---
mapped_pages:
  - https://www.elastic.co/guide/en/elasticsearch/client/rust-api/current/installation.html
---

# Installation [installation]

Add `elasticsearch` crate and version to Cargo.toml. Choose the version that is compatible with the version of {{es}} you are using:

```toml subs=true
[dependencies]
elasticsearch = "{{version}}"
```

The following *optional* dependencies may also be useful to create requests and read responses:

```toml
serde = "1"
serde_json = "1"
```

