use serde_json::Value;
use elasticsearch::http::{Method, StatusCode, response::Response};

pub fn len_from_value(value: &Value) -> Result<usize, failure::Error> {
    match value {
        Value::Number(n) => Ok(n.as_i64().unwrap() as usize),
        Value::String(s) => Ok(s.len()),
        Value::Array(a) => Ok(a.len()),
        Value::Object(o) => Ok(o.len()),
        v => Err(failure::err_msg(format!("Cannot get length from {:?}", v))),
    }
}

pub async fn read_response(response: Response) -> Result<(Method, StatusCode, String, Value), failure::Error> {
    let is_json = response.content_type().starts_with("application/json");
    let method = response.method();
    let status_code = response.status_code();
    let text = response.text().await?;
    let json = if is_json && !text.is_empty() {
        serde_json::from_slice::<Value>(text.as_ref())?
    } else {
        Value::Null
    };

    Ok((method, status_code, text, json))
}
