# Examples

To make a multi-search request, specify the headers and bodies
for the search requests in the body. The body accepts a
`Vec<T>` where `T` implements the [Body] trait.

```rust,no_run
# use elasticsearch::{Elasticsearch, Error, MsearchParts};
# use elasticsearch::http::request::JsonBody;
# use serde_json::{json, Value};
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
let client = Elasticsearch::default();

fn print_hits(hits: &[Value]) {
    for hit in hits {
        println!(
            "id: '{}', source: '{}', score: '{}'",
            hit["_id"].as_str().unwrap(),
            hit["_source"],
            hit["_score"].as_f64().unwrap()
        );
    }
}

let msearch_response = client
    .msearch(MsearchParts::None)
    .body::<JsonBody<Value>>(vec![
        json!({"index":"cat_food"}).into(),
        json!({"query":{"term":{"name":{"term":"Whiskers"}}}}).into(),
        json!({"index":"cat_food"}).into(),
        json!({"query":{"term":{"name":{"term":"Chicken"}}}}).into(),
        json!({"index":"cat_food"}).into(),
        json!({"query":{"term":{"name":{"term":"Turkey"}}}}).into(),
    ])
    .send()
    .await?;

let json: Value = msearch_response.json().await?;

// iterate over the responses
for response in json["responses"].as_array().unwrap()
{
    print_hits(response["hits"]["hits"].as_array().unwrap());
}
    
# Ok(())
# }
```
