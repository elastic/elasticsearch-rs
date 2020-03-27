use serde_json::Value;

pub fn len_from_value(value: &Value) -> Result<usize, failure::Error> {
    match value {
        Value::Number(n) => {
            Ok(n.as_i64().unwrap() as usize)
        },
        Value::String(s) => {
            Ok(s.len())
        },
        Value::Array(a) => {
            Ok(a.len())
        },
        Value::Object(o) => {
            Ok(o.len())
        },
        v => Err(failure::err_msg(format!("Cannot get length from {:?}", v)))
    }
}
