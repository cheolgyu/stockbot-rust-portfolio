#[cfg(feature = "site")]
pub mod view;
use crate::{app_type::price_type::PriceType, graph::DirectionType};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
pub use serde_repr::{Deserialize_repr, Serialize_repr};

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Clone, Deserialize, Default)]
pub struct Bound {
    pub code_id: i64,
    pub line: PriceType,

    pub p1x: NaiveDate,
    #[serde(with = "rust_decimal::serde::float")]
    pub p1y: Decimal,
    pub p2x: NaiveDate,
    #[serde(with = "rust_decimal::serde::float")]
    pub p2y: Decimal,
    pub change_rate: Decimal,

    pub direction: DirectionType,
    pub x_cnt: i32,
    pub y_cnt: Decimal,

    pub ymxb_p3y: Decimal,
    pub ymxb_m: Decimal,
    pub ymxb_b: Decimal,
}

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddBound {
    pub code_id: i64,
    pub line: PriceType,

    pub p1x: NaiveDate,
    #[serde(with = "rust_decimal::serde::float")]
    pub p1y: Decimal,
    pub p2x: NaiveDate,
    #[serde(with = "rust_decimal::serde::float")]
    pub p2y: Decimal,

    pub direction: DirectionType,
    pub x_cnt: Decimal,
    pub y_cnt: Decimal,
    pub change_rate: Decimal,

    pub ymxb_p3y: Decimal,
    pub ymxb_m: Decimal,
    pub ymxb_b: Decimal,
}

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "bound_custom_line_type"))]
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum BoundCustomLineType {
    // P1: 저가기존 뒤에서2번째 반등, P2: 1번째 반등
    K2,
    // P1: 고가기존 뒤에서2번째 반등, P2: 1번째 반등
    K3,
}
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct BoundDaily {
    pub code_id: i64,
    pub line: PriceType,
    // cp
    pub p1x: NaiveDate,
    pub p1y: Decimal,
    pub p2x: NaiveDate,
    pub p2y: Decimal,
    pub direction: DirectionType,
    pub x_cnt: i32,
    pub y_cnt: Decimal,
    pub change_rate: Decimal,
    pub ymxb_p3y: Decimal,
    pub ymxb_m: Decimal,
    pub ymxb_b: Decimal,
}

/*
#[serde(flatten)] query_as not working
ViewBound 복사해서 site 에서 사용함.
*/
