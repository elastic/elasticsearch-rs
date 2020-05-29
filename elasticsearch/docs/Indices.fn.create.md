# Examples

Create an index with a mapping

```rust,no_run
# use elasticsearch::{Elasticsearch, Error, indices::IndicesCreateParts};
# use serde_json::{json, Value};
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
let client = Elasticsearch::default();
let response = client
    .indices()
    .create(IndicesCreateParts::Index("test_index"))
    .body(json!({
        "mappings" : {
            "properties" : {
                "field1" : { "type" : "text" }
            }
        }
    }))
    .send()
    .await?;
    
# Ok(())
# }
```