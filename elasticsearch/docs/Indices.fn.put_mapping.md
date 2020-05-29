# Examples

Put a mapping into an existing index, assuming the index does not have a mapping, 
or that any properties specified do not conflict with existing properties

```rust,no_run
# use elasticsearch::{Elasticsearch, Error, indices::IndicesPutMappingParts};
# use serde_json::{json, Value};
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
let client = Elasticsearch::default();
let response = client
    .indices()
    .put_mapping(IndicesPutMappingParts::Index(&["test_index"]))
    .body(json!({
        "properties" : {
            "field1" : { "type" : "text" }
        }
    }))
    .send()
    .await?;
    
# Ok(())
# }
```