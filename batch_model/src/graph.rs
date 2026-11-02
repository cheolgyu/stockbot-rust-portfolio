use crate::{
    app_type::{exchange_type::ExchangeType, price_type::PriceType},
    price::PriceWithConvertDate,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

pub const DT_FORMAT: &str = "%Y%m%d";
// 좌표 평면
/*
 필요이유:  y에 해당하는 좌표값이 몇인지몰름. (x는 안구함.)
 p1과 p2사이의 y차이에 해당하는 점들의 수는?: 마켓마다 다름;
 p1과 p2사이의 x차이에 해당하는 점들의 수는?: 마켓오픈일 마다 다름;
그럼으로 p1과 p2를 좌표좀으로 변화 후에 p3을 구하고 p3을 가격으로 변환하여 사용해야됨.

*/
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct YTick {
    pub exchange: ExchangeType,
    #[serde(with = "rust_decimal::serde::float")]
    pub y_tick: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub y: Decimal,
}

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "direction_type"))]
#[derive(Debug, Display, PartialEq, Clone, Serialize, Deserialize, Default, Copy)]
pub enum DirectionType {
    #[default]
    //감소
    DECREASE,
    //유지
    CONSTANT,
    //증가
    INCREASE,
}
impl DirectionType {
    pub fn from_u32(value: u32) -> Self {
        match value {
            1 => Self::DECREASE,
            2 => Self::CONSTANT,
            3 => Self::INCREASE,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    #[serde(with = "rust_decimal::serde::float")]
    pub y: Decimal,
}

impl Point {
    /*
    20220115 -> 2022-01-15
    */
    // pub fn x_to_dt(x: i32) -> NaiveDate {
    //     if x == 0 {
    //         NaiveDate::parse_from_str(
    //             &format!("{}", KR_PRICE_DEFAULT_START_DATE),
    //             format::DT_FORMAT,
    //         )
    //         .unwrap()
    //     } else {
    //         NaiveDate::parse_from_str(&format!("{}", x), format::DT_FORMAT)
    //             .expect(&format!("오류난 x={}", x))
    //     }
    // }
    /*
    2022-01-15-> 20220115
    */
    pub fn dt_to_x(dt: NaiveDate) -> i32 {
        let sd: String = dt.format(DT_FORMAT).to_string();
        match sd.parse::<i32>() {
            Ok(v) => v,
            Err(e) => {
                println!("dt: {:?}", dt);
                panic!("Error: {}", e);
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default, Copy)]
pub struct ConvertPoint {
    pub dt: NaiveDate,
    pub price: Decimal,
    pub x_tick: i64,
}

impl ConvertPoint {
    pub fn from_price(value: &PriceWithConvertDate, line: PriceType) -> Self {
        let price_info: (Decimal, Decimal) = match line {
            PriceType::CLOSE => (value.cp, value.cp),
            PriceType::OPEN => (value.op, value.op),
            PriceType::LOW => (value.lp, value.lp),
            PriceType::HIGH => (value.hp, value.hp),
        };
        Self {
            dt: value.dt,
            price: price_info.0,
            x_tick: value.x_tick,
        }
    }
}
