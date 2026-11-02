use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::app_type::exchange_type::ExchangeType;

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SumByDate {
    pub dt: NaiveDate,
    pub exchange: ExchangeType,
    pub sum_vol: Decimal,
    pub sum_t_val: Decimal,
}

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SumByDateItem {
    pub code_id: i64,
    pub dt: NaiveDate,
    pub vol: Decimal,
    pub t_val: Decimal,
    pub sum_vol: Decimal,
    pub sum_t_val: Decimal,
}
