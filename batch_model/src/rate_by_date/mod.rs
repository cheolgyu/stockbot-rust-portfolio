use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
pub mod sum_by_date;
pub mod view;

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RateByDate {
    pub code_id: i64,
    pub dt: NaiveDate,
    pub vol_rate: Decimal,
    pub t_val_rate: Decimal,
}
