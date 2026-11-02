use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::app_type::{find_what::What, find_why::Why, price_type::PriceType};

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Clone, Deserialize, Default, utoipa::ToSchema, Serialize)]
pub struct FindViewByCode {
    // #[order(0)]
    #[schema(value_type = String)]
    pub dt: NaiveDate,
    // #[order(1)]
    pub what: What,
    // #[order(2)]
    pub why: Why,
    // #[order(3)]
    pub order_num: i32,

    // #[order(4)]
    pub line: PriceType,

    #[schema(value_type = String)]
    // #[order(5)]
    pub p1x: NaiveDate,
    // #[order(6)]
    pub change_rate: Decimal,
}
