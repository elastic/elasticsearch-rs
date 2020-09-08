# Examples

To initiate a scroll, make search API call with a specified `scroll` timeout,
then fetch the next set of hits using the `_scroll_id` returned in
the response. Once no more hits are returned, clear the scroll.

```rust,no_run
# use elasticsearch::{Elasticsearch, Error, SearchParts, ScrollParts, ClearScrollParts};
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

let scroll = "1m";
let mut response = client
    .search(SearchParts::Index(&["tweets"]))
    .scroll(scroll)
    .body(json!({
        "query": {
            "match": {
                "body": {
                    "query": "Elasticsearch rust",
                    "operator": "AND"
                }
            }
        }
    }))
    .send()
    .await?;

let mut response_body = response.json::<Value>().await?;
let mut scroll_id = response_body["_scroll_id"].as_str().unwrap();
let mut hits = response_body["hits"]["hits"].as_array().unwrap();

print_hits(hits);

while hits.len() > 0 {
    response = client
        .scroll(ScrollParts::None)
        .body(json!({
            "scroll": scroll,
            "scroll_id": scroll_id
        }))
        .send()
        .await?;

    response_body = response.json::<Value>().await?;
    scroll_id = response_body["_scroll_id"].as_str().unwrap();
    hits = response_body["hits"]["hits"].as_array().unwrap();
    print_hits(hits);
}

response = client
    .clear_scroll(ClearScrollParts::None)
    .body(json!({
        "scroll_id": scroll_id
    }))
    .send()
    .await?;
    
# Ok(())
# }
```