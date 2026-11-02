use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::model::paging::params::ParamsPaging;

#[derive(Deserialize, Serialize, Clone, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Params {
    #[param(example = 123)]
    pub code_id: i64,
    #[param(example = "CLOSE")]
    pub line: String,

    #[param(inline)]
    #[serde(flatten)]
    pub paging: ParamsPaging,
}
impl Params {
    pub fn default() -> Self {
        let mut paging = ParamsPaging::default();
        paging.sort = "p1x".to_string();
        paging.desc = true;
        Self {
            paging: paging,
            line: "CLOSE".to_string(),
            code_id: 0,
        }
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// pub struct Result {
//     pub data: Vec<Model>,
//     pub info: Info,
// }

// #[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
// pub struct Model {
//     pub code_id: i64,
//     pub line: String,

//     pub p1x: NaiveDate,
//     #[serde(with = "rust_decimal::serde::float")]
//     pub p1y: Decimal,
//     pub p2x: NaiveDate,
//     #[serde(with = "rust_decimal::serde::float")]
//     pub p2y: Decimal,

//     pub direction: String,
//     pub x_cnt: i32,
//     pub y_cnt: Decimal,
//     pub change_rate: Decimal,

//     pub ymxb_p3y: Decimal,
//     pub ymxb_m: Decimal,
//     pub ymxb_b: Decimal,
// }

// impl Model {
//     pub fn get_td(&self) -> Option<Vec<Value>> {
//         let mut res: Vec<Value> = vec![];
//         let j: Value = json!(self);
//         match j.as_object() {
//             Some(m) => {
//                 for v in Self::get_table_order().iter() {
//                     let k: String = v.to_string();
//                     let val = m.get(&k).unwrap().clone();
//                     res.push(val);
//                 }
//                 Some(res)
//             }
//             None => None,
//         }
//     }

//     pub fn get_table_order() -> Vec<&'static str> {
//         let th: Vec<&str> = vec![
//             "code_id",
//             "line",
//             "p1x",
//             "p1y",
//             "p2x",
//             "p2y",
//             "direction",
//             "x_cnt",
//             "y_cnt",
//             "ymxb_p3y",
//             "ymxb_m",
//             "ymxb_b",
//         ];
//         th
//     }

//     pub fn get_th() -> Vec<(&'static str, &'static str)> {
//         let th: Vec<(&str, &str)> = vec![
//             ("code_id", "code_id"),
//             ("line", "가격종류"),
//             ("p1x", "시작일자"),
//             ("p1y", "시작가격"),
//             ("p2x", "종료일자"),
//             ("p2y", "종료가격"),
//             ("direction", "방향"),
//             ("x_cnt", "변화 일수"),
//             ("y_cnt", "변화 가격"),
//             ("ymxb_p3y", "직선 가격"),
//             ("ymxb_m", "직선 기울기"),
//             ("ymxb_b", "직선 y절편"),
//         ];
//         th
//     }
// }
