use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::model::{
    compoent::{exchange::ParamsExchange, search::ParamsSearch},
    paging::params::ParamsPaging,
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct BoundParams {
    #[param(inline)]
    #[serde(flatten)]
    pub paging: ParamsPaging,
    #[param(inline)]
    #[serde(flatten)]
    pub exchange: ParamsExchange,
    #[param(inline)]
    #[serde(flatten)]
    pub search: ParamsSearch,
}

impl Default for BoundParams {
    fn default() -> Self {
        let mut p = ParamsPaging::default();
        p.sort = "cp_change_rate".to_string();
        p.desc = true;
        BoundParams {
            paging: p,
            exchange: ParamsExchange::new(),
            search: ParamsSearch::new(),
        }
    }
}

// impl From<BoundParams> for BoundParamsOption {
//     fn from(opt: BoundParams) -> Self {

//         if opt.page.is_some() {
//             pq.paging.page = opt.page;
//         }
//         if opt.size.is_some() {
//             pq.paging.size = opt.size;
//         }
//         if opt.sort.is_some() {
//             pq.paging.sort = opt.sort;
//         }
//         if opt.desc.is_some() {
//             pq.paging.desc = opt.desc;
//         }
//         if opt.exchange.is_some() {
//             let v = CheckboxExchange::valid_str(opt.exchange.unwrap());
//             pq.exchange = CheckboxExchange::query_format(v);
//         }
//         if opt.search.is_some() {
//             pq.search = opt.search.unwrap();
//         }
//         if opt.obs_type.is_some() {
//             pq.obs_type = opt.obs_type.unwrap();
//         }
//         if opt.obs_val.is_some() {
//             pq.obs_val = opt.obs_val.unwrap();
//         }
//         pq
//     }
// }

// impl BoundParams {
//     pub fn format_get(&self) -> String {
//         let p = serde_urlencoded::to_string(self).unwrap();
//         let url: String = format!("?{}", p);

//         url
//     }

//     pub fn set_exchange(&mut self, v: Vec<String>) {
//         self.paging.page(1);
//         self.exchange = CheckboxExchange::query_format(v);
//     }

//     pub fn set_search(&mut self, v: String) {
//         self.paging.page(1);
//         self.search = v;
//     }

//     pub fn init_desc(&mut self) {
//         self.paging.desc = DEFAULT_DESC_VAL;
//     }

//     pub fn set_page_1(&mut self) {
//         self.paging.page(1)
//     }

//     pub fn set_page(&mut self, v: usize) {
//         self.paging.page(v.into())
//     }

//     pub fn set_sort(&mut self, v: String) {
//         self.paging.page(1);
//         if self.paging.sort == v {
//             self.paging.desc = !self.paging.desc;
//         } else {
//             self.paging.sort = v;
//             self.paging.desc = DEFAULT_DESC_VAL;
//         }
//     }
// }
