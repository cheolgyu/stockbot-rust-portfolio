use std::str::FromStr;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use utoipa::IntoParams;

use crate::model::{
    find::{What, Why},
    paging::{info::Info, params::ParamsPaging},
};

#[derive(Deserialize, Serialize, Clone, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Params {
    #[param(example = 123)]
    pub code_id: i64,

    #[param(inline)]
    pub what: What,

    #[param(inline)]
    pub why: Why,

    #[param(inline)]
    #[serde(flatten)]
    pub paging: ParamsPaging,
}
impl Params {
    pub fn default() -> Self {
        let mut paging = ParamsPaging::default();
        paging.sort = "dt".to_string();
        paging.desc = true;
        Self {
            paging: paging,
            //dt: NaiveDate::from_str("2022-09-16").unwrap(),
            what: What::H,
            why: Why::default(),
            code_id: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Result {
    pub data: Vec<Model>,
    pub info: Info,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Model {
    pub dt: NaiveDate,
    pub what: What,
    pub why: Why,
    pub order_num: i32,
    //
    pub line: String,
    pub p1x: NaiveDate,
    pub change_rate: Decimal,
}
impl Model {
    pub fn get_td(&self) -> Option<Vec<Value>> {
        let mut res: Vec<Value> = vec![];
        let j: Value = json!(self);
        match j.as_object() {
            Some(m) => {
                for v in Self::get_table_order().iter() {
                    let k: String = v.to_string();
                    let val = m.get(&k).unwrap().clone();
                    res.push(val);
                }
                Some(res)
            }
            None => None,
        }
    }

    pub fn get_table_order() -> Vec<&'static str> {
        let th: Vec<&str> = vec![
            "dt",
            "what",
            "why",
            "order_num",
            "line",
            "p1x",
            "change_rate",
        ];
        th
    }

    pub fn get_th() -> Vec<(&'static str, &'static str)> {
        let th: Vec<(&str, &str)> = vec![
            ("dt", "일자"),
            ("what", "위치"),
            ("why", "이유"),
            ("order_num", "순서"),
            ("line", "가격종류"),
            ("p1x", "시작일"),
            ("change_rate", "퍼센트"),
        ];
        th
    }
}
