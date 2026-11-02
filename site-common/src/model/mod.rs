// use self::{
//     board::{view_bound::ModelViewBound, view_volume::ModelViewVolume},
//     // find::{ModelViewFind, Why},
//     paging::info::Info,
// };
// use serde::{Deserialize, Serialize};
// use serde_json::Map;
// use std::fmt::{self, Display};
// use utoipa::ToSchema;

// pub mod board;
// pub mod by_code;
// pub mod compoent;
// pub mod find;
// pub mod paging;

// pub mod batch_timer_plan;
// pub mod system;
// pub mod utils;
// pub mod volume;

// pub const DEFAULT_OBS_TPYE_VOLUME: &str = "MonthMax";
// pub const DEFAULT_OBS_VAL_VOLUME: i32 = 0;

// #[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, ToSchema)]
// pub struct PageResult {
//     pub info: Info,
//     pub data: Vec<ModelViewBound>,
// }
// // #[derive(Debug, Clone, Serialize, Deserialize, Default)]
// // pub struct PageResultVolume {
// //     pub info: Info,
// //     pub data: Vec<ModelViewVolume>,
// // }

// #[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
// pub struct PageResultFind {
//     pub data: Vec<ModelViewFind>,
//     pub select_box: Vec<(Why, i64)>,
//     pub info: Info,
// }

// #[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
// pub enum PriceType {
//     CLOSE,
//     OPEN,
//     LOW,
//     HIGH,
// }
// impl PriceType {
//     pub fn to_short(&self) -> String {
//         match self {
//             PriceType::CLOSE => "cp".to_string(),
//             PriceType::OPEN => "op".to_string(),
//             PriceType::LOW => "lp".to_string(),
//             PriceType::HIGH => "hp".to_string(),
//         }
//     }
// }
// impl Display for PriceType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             PriceType::CLOSE => write!(f, "CLOSE"),
//             PriceType::OPEN => write!(f, "OPEN"),
//             PriceType::LOW => write!(f, "LOW"),
//             PriceType::HIGH => write!(f, "HIGH"),
//         }
//     }
// }
