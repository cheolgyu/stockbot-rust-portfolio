use ::serde::{Deserialize, Serialize};
use chrono::serde;
use rust_decimal::Decimal;

pub mod frequency_analysis;
pub mod standard_deviation;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct ObsValue(pub Decimal);
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct Year(pub i32);
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, Hash, PartialEq)]
pub struct AggVolFreqDataItem {
    pub val: Decimal,
    pub cnt: Decimal,
    pub rate: Decimal,
}
