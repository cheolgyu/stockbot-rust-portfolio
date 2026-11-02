use serde::{Deserialize, Serialize};

pub mod param_batch_timer_health;
pub mod param_batch_timer_plan_list;
pub mod param_by_code_agg_cp;
pub mod param_by_code_agg_vol;
mod param_by_code_bound;
pub mod param_by_code_code;
pub mod param_by_code_find;
pub mod param_by_code_price;
pub mod param_exchange_list;
pub mod param_log_batch;
pub mod param_view_agg_bound;
pub mod param_view_agg_cp;
pub mod param_view_agg_vol;
pub mod param_view_bound;
pub mod param_view_find;
pub mod param_view_price_chg;
pub mod param_view_rate_by_date;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Default)]
pub struct ParamPaging {
    pub page: usize,
    pub size: usize,
    pub sort: String,
    pub desc: bool,
}

// #[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Default)]
// pub struct Exchange {
//     pub exchange: String,
// }
pub fn vec_to_comma<T: ToString>(items: Vec<T>) -> String {
    let mut v_exchange = "".to_string();
    if !items.is_empty() {
        v_exchange = items.first().unwrap().to_string();
        for x in 1..items.len() {
            let v = items.get(x).unwrap().to_string();
            v_exchange = v_exchange + "," + &v;
        }
    }

    v_exchange
}

// #[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Default)]
// pub struct PriceType {
//     pub price_type: String,
// }

// #[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Default)]
// pub struct Search {
//     pub search: String,
// }

// #[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Default)]
// pub struct ObsType {
//     pub obs_type: String,
// }
// #[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Default)]
// pub struct What {
//     pub what: String,
// }
// #[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Default)]
// pub struct Why {
//     pub why: String,
// }

pub trait GetURI
where
    Self: Serialize,
{
    fn get_uri(&self) -> &str;
    fn get_param(&self) -> String {
        serde_urlencoded::to_string(self).unwrap()
    }
    fn is_code_by(&self) -> bool {
        false
    }

    fn get_code_by(&self) -> String {
        "".to_string()
    }

    fn uri(&self) -> String {
        if self.is_code_by() {
            format!(
                "{}/{}?{}",
                &self.get_uri(),
                &self.get_code_by(),
                &self.get_param()
            )
        } else {
            format!("{}?{}", &self.get_uri(), &self.get_param())
        }
    }
}
