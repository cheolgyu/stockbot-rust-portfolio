use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/*
    https://github.com/rust-lang/socket2/issues/268

*/
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ModelViewVolume {
    // cp code
    pub id: i64,
    pub short_code: String,
    pub short_name: String,
    pub country: String,
    pub exchange: String,
    //
    pub data: Value,
    pub obs_type: String,
    pub obs_val: String,
    pub freq_val: String,
    pub percent: String,
}
