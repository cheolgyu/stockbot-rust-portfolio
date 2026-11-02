use serde::{Deserialize, Serialize};

pub mod agg_bound_type;
pub mod batch_run_type;
pub mod country_type;
pub mod exchange_type;
pub mod find_what;
pub mod find_why;
pub mod obs_epoch;
pub mod obs_target;
pub mod obs_type;
pub mod price_type;

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "obs_period"))]
#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ObsPeriod {
    //주
    #[default]
    Week,
    //월
    Month,
    //분기
    Quarter,
}
