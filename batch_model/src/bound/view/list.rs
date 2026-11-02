use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::app_type::{
    country_type::CountryType, exchange_type::ExchangeType, price_type::PriceType,
};

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Clone, Deserialize, Default, Serialize)]
pub struct BoundViewList {
    pub id: i64,

    pub std_code: String,
    pub code: String,
    pub name: String,
    pub country: CountryType,
    pub exchange: ExchangeType,
    pub code_id: i64,
    pub line: PriceType,
    pub p1x: NaiveDate,
    pub p1y: String,
    pub p2x: NaiveDate,
    pub p2y: String,
    pub direction: String,
    pub x_cnt: String,
    pub y_cnt: String,
    pub change_rate: Decimal,
    pub ymxb_p3y: String,
    pub ymxb_m: String,
    pub ymxb_b: String,
    pub created_at: DateTime<Utc>,
}

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Clone, Deserialize, Default, Serialize)]
pub struct BoundCodeDetail {
    pub code_id: i64,
    pub p1x: NaiveDate,
    pub p1y: String,
    pub p2x: NaiveDate,
    pub p2y: String,
    pub direction: String,
    pub x_cnt: String,
    pub y_cnt: String,
    pub change_rate: Decimal,
    pub ymxb_p3y: String,
    pub ymxb_m: String,
    pub ymxb_b: String,
}
