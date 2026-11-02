use serde_json::Value;

pub fn format_data(v: &Value) -> String {
    match v {
        Value::Null => "-".to_string(),
        Value::Bool(v) => format!("{}", v),
        Value::Number(v) => format!("{}", v),
        Value::String(v) => v.to_string(),
        Value::Array(v) => format!("{:?}", v),
        Value::Object(v) => format!("{:?}", v),
    }
}
