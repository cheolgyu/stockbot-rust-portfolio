use serde::{Deserialize, Serialize};

use crate::app_type::{country_type::CountryType, exchange_type::ExchangeType};

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Deserialize, Clone, Default, Serialize)]
pub struct CodeViewList {
    //#[order(0)]
    pub id: i64,
    //#[order(1)]
    pub std_code: String,
    //#[order(2)]
    pub short_code: String,
    //#[order(3)]
    pub country: CountryType,
    //#[order(4)]
    pub exchange: ExchangeType,
    //#[order(5)]
    pub code_id: i64,
    //#[order(6)]
    pub std_name: String,
    //#[order(7)]
    pub short_name: String,
    //#[order(8)]
    pub eng_name: String,
    //#[order(9)]
    pub listing_date: String,
    //#[order(10)]
    pub securities_type: String,
    //#[order(11)]
    pub company_type: String,
    //#[order(12)]
    pub stock_type: String,
    //#[order(13)]
    pub face_value: String,
    //#[order(14)]
    pub stock_cnt: String,
}
