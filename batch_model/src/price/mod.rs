use chrono::NaiveDate;

pub use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
pub use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

use crate::graph;

#[derive(Debug, Clone, Default)]
pub struct AddPrice {
    pub code_id: i64,
    pub dt: NaiveDate,
    pub op: Decimal,
    pub hp: Decimal,
    pub lp: Decimal,
    pub cp: Decimal,
    // 거래량
    pub vol: i64,
    // 거래 대금
    pub t_val: i64,
    // 상장 주식수
    pub stock_cnt: i64,
    // 시가총액
    pub capital: i64,
}

#[derive(Debug, Clone, Default)]
pub struct AddPriceXlsx {
    pub code_id: i64,
    pub dt: String,
    pub op: String,
    pub hp: String,
    pub lp: String,
    pub cp: String,
    // 거래량
    pub vol: String,
    // 거래 대금
    pub t_val: String,
    // 상장 주식수
    pub stock_cnt: String,
    // 시가총액
    pub capital: String,
}
impl AddPriceXlsx {
    const INVALID_FORMAT: &str = "%Y/%m/%d";
    const COMMA: &str = ",";
    const EMPTY: &str = "";
    pub fn to_add_price(&self) -> AddPrice {
        AddPrice {
            code_id: self.code_id,
            dt: NaiveDate::parse_from_str(&self.dt, Self::INVALID_FORMAT).unwrap(),
            op: Decimal::from_str(&self.op.replace(Self::COMMA, Self::EMPTY)).unwrap(),
            hp: Decimal::from_str(&self.hp.replace(Self::COMMA, Self::EMPTY)).unwrap(),
            lp: Decimal::from_str(&self.lp.replace(Self::COMMA, Self::EMPTY)).unwrap(),
            cp: Decimal::from_str(&self.cp.replace(Self::COMMA, Self::EMPTY)).unwrap(),
            vol: self
                .vol
                .replace(Self::COMMA, Self::EMPTY)
                .parse::<i64>()
                .unwrap(),
            t_val: self
                .t_val
                .replace(Self::COMMA, Self::EMPTY)
                .parse::<i64>()
                .unwrap(),
            stock_cnt: self
                .stock_cnt
                .replace(Self::COMMA, Self::EMPTY)
                .parse::<i64>()
                .unwrap(),
            capital: self
                .capital
                .replace(Self::COMMA, Self::EMPTY)
                .parse::<i64>()
                .unwrap(),
        }
    }
}

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Price {
    pub code_id: i64,
    pub dt: NaiveDate,
    pub op: Decimal,
    pub hp: Decimal,
    pub lp: Decimal,
    pub cp: Decimal,
    // 거래량
    pub vol: i64,
    // 거래 대금
    pub t_val: i64,
    // 상장 주식수
    pub stock_cnt: i64,
    // 시가총액
    pub capital: i64,
}

impl Price {
    pub fn dt_to_i32(dt: NaiveDate) -> i32 {
        let sd: String = dt.format(graph::DT_FORMAT).to_string();
        match sd.parse::<i32>() {
            Ok(v) => v,
            Err(e) => {
                println!("self: {:?}", dt);
                panic!("Error: {}", e);
            }
        }
    }
}

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Default)]
pub struct PriceWithConvertDate {
    pub code_id: i64,
    pub dt: NaiveDate,
    pub x_tick: i64,
    pub op: Decimal,
    pub hp: Decimal,
    pub lp: Decimal,
    pub cp: Decimal,
    //pub dt_point_id: i64,
    // pub op_point: Decimal,
    // pub hp_point: Decimal,
    // pub lp_point: Decimal,
    // pub cp_point: Decimal,
    // pub vol: i64,
    // pub fbr: String,
    // pub convert_x_val: Decimal,
}

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PriceView {
    pub dt: NaiveDate,
    pub op: String,
    pub hp: String,
    pub lp: String,
    pub cp: String,
    pub vol: String,
}

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PriceStartDate {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub std_code: String,
    pub start_dt: NaiveDate,
}

mod tests {
    use std::str::FromStr;

    use rust_decimal::Decimal;

    #[test]
    fn it_works() {
        let decimal_string = "123.45";

        // Attempt to parse the string into a Decimal
        let result: Result<Decimal, rust_decimal::Error> = Decimal::from_str(decimal_string);

        match result {
            Ok(decimal_value) => {
                println!(
                    "Successfully converted '{}' to Decimal: {}",
                    decimal_string, decimal_value
                );
            }
            Err(e) => {
                eprintln!("Error converting '{}' to Decimal: {}", decimal_string, e);
            }
        }

        // You can also use unwrap() if you are sure the string is valid and want to panic on error
        let another_decimal_string = "1,789.01";
        let another_decimal = Decimal::from_str(another_decimal_string).unwrap();
        println!("Another Decimal from string: {}", another_decimal);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
