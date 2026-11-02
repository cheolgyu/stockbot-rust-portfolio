use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::app_type::{
    country_type::CountryType, exchange_type::ExchangeType, price_type::PriceType,
};
use crate::app_type::{find_what::What, find_why::Why};

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Default, ToSchema)]
pub struct FindViewList {
    // cp code
    //#[order(0)]
    pub id: i64,
    //#[order(1)]
    pub short_code: String,
    //#[order(2)]
    pub short_name: String,
    //#[order(3)]
    pub country: CountryType,
    //#[order(4)]
    pub exchange: ExchangeType,
    //
    #[schema(value_type = String)]
    //#[order(5)]
    pub dt: NaiveDate,
    //#[order(6)]
    pub what: What,
    //#[order(7)]
    pub why: Why,
    //#[order(8)]
    pub order_num: i32,

    //#[order(9)]
    pub line: PriceType,
    #[schema(value_type = String)]
    //#[order(10)]
    pub p1x: NaiveDate,
    //#[order(11)]
    pub change_rate: Decimal,
}
