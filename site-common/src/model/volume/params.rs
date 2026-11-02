// use std::str::FromStr;

// use serde::{Deserialize, Serialize};
// use utoipa::IntoParams;

// use crate::model::{
//     board::ObsType,
//     compoent::{exchange::ParamsExchange, search::ParamsSearch},
//     paging::params::ParamsPaging,
//     DEFAULT_OBS_TPYE_VOLUME, DEFAULT_OBS_VAL_VOLUME,
// };

// impl Default for VolumeParams {
//     fn default() -> Self {
//         let mut p = ParamsPaging::default();
//         p.sort = "percent".to_string();
//         p.desc = true;
//         VolumeParams {
//             paging: p,
//             exchange: ParamsExchange::new(),
//             search: ParamsSearch::new(),
//             obs_type: DEFAULT_OBS_TPYE_VOLUME.to_string(),
//             obs_val: DEFAULT_OBS_VAL_VOLUME.to_string(),
//         }
//     }
// }
// impl VolumeParams {
//     pub fn obs_type(&self) -> String {
//         let s = self.obs_type.clone();
//         let chk = ObsType::from_str(&s);
//         match chk {
//             Ok(_) => s,
//             Err(_) => DEFAULT_OBS_TPYE_VOLUME.to_string(),
//         }
//     }
//     pub fn obs_val(&self) -> i32 {
//         let s = self.obs_val.clone();

//         let chk = s.parse::<i32>();
//         match chk {
//             Ok(v) => v,
//             Err(_) => DEFAULT_OBS_VAL_VOLUME,
//         }
//     }
// }
