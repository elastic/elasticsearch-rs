use elasticsearch::http::{response::Response, Method, StatusCode};
use serde_json::Value;

/// Reads the response from Elasticsearch, returning the method, status code, text response,
/// and the response parsed from json or yaml
pub async fn read_response(
    response: Response,
) -> Result<(Method, StatusCode, String, Value), failure::Error> {
    let is_json = response.content_type().starts_with("application/json");
    let is_yaml = response.content_type().starts_with("application/yaml");
    let method = response.method();
    let status_code = response.status_code();
    let text = response.text().await?;
    let json = if is_json && !text.is_empty() {
        serde_json::from_slice::<Value>(text.as_ref())?
    } else if is_yaml && !text.is_empty() {
        serde_yaml::from_slice::<Value>(text.as_ref())?
    }
     else {
        Value::Null
    };

    Ok((method, status_code, text, json))
}
