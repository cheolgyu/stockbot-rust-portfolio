use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::app_type::{country_type::CountryType, exchange_type::ExchangeType};

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct ViewPriceChg {
    pub code_id: i64,
    pub std_code: String,
    pub code: String,
    pub name: String,
    pub country: CountryType,
    pub exchange: ExchangeType,
    //
    pub dt: NaiveDate,
    pub lh: Decimal,
    pub oc: Decimal,
    pub vol: Decimal,
}
impl ViewPriceChg {}
