use std::str::FromStr;

pub fn number<T: std::fmt::Display>(value: T) -> Result<serde_json::Number, std::fmt::Error> {
    serde_json::Number::from_str(&value.to_string()).map_err(|e| {
        dbg!("input:{}", value.to_string());
        dbg!("error:{}", e);
        return std::fmt::Error;
    })
}
