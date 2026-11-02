use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::app_type::exchange_type::ExchangeType;

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDate {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub exchange: ExchangeType,
    pub std_code: String,
    pub start_dt: NaiveDate,
}
