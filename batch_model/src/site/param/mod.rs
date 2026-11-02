pub mod agg_bound_param;
pub mod agg_cp_by_code;
pub mod agg_cp_list;
pub mod agg_vol_by_code;
pub mod agg_vol_list;
pub mod batch_timer_plan;
pub mod bound_by_code;
pub mod bound_list;
pub mod find_by_code;
pub mod find_list;
pub mod log_batch;
pub mod open_dt;
pub mod param_exchange;
pub mod param_line;
pub mod param_paging;
pub mod param_paging_two_sort;
pub mod param_search;
pub mod price_by_code;
pub mod rate_by_date_by_code;
pub mod rate_by_date_list;
pub mod view_price_chg_param;

pub trait SqlObsVal {
    fn obs_val_to_query(&self, obs_val: Option<i32>) -> String {
        match obs_val {
            Some(v) => format!(" AND    {v} = ANY(v_obs_agg)   "),
            None => "".to_string(),
        }
    }
}
