use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::app_type::{
    agg_bound_type::AggBoundType, country_type::CountryType, exchange_type::ExchangeType, obs_epoch::ObsEpoch, obs_target::ObsTarget
};

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct ViewAggFreq {
    pub code_id: i64,
    pub std_code: String,
    pub code: String,
    pub name: String,
    pub country: CountryType,
    pub exchange: ExchangeType,
    //
    pub obs_epoch: ObsEpoch,
    pub obs_target: ObsTarget,
    //
    pub v_epoch_cnt: Decimal,
    pub sum_v_epoch_cnt: Decimal,
    pub rate_all_epoch: Decimal,
    pub rate_epoch: Decimal,
    pub v_obs_agg: Vec<Decimal>,
}
impl ViewAggFreq {
    pub fn obs_to_label(&self) -> String {
        format!(
            "{} {}",
            self.obs_epoch.to_label(),
            self.obs_target.to_label()
        )
    }
}

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct ViewAggBoundFreq {
    pub code_id: i64,
    pub std_code: String,
    pub code: String,
    pub name: String,
    pub country: CountryType,
    pub exchange: ExchangeType,
    //
    pub agg_bound_type:AggBoundType,
    pub obs_epoch: ObsEpoch,
    pub obs_target: ObsTarget,
    //
    pub v_epoch_cnt: Decimal,
    pub sum_v_epoch_cnt: Decimal,
    pub rate_all_epoch: Decimal,
    pub rate_epoch: Decimal,
    pub v_obs_agg: Vec<Decimal>,
}
impl ViewAggBoundFreq {
    pub fn obs_to_label(&self) -> String {
        format!(
            "{} {}",
            self.obs_epoch.to_label(),
            self.obs_target.to_label()
        )
    }
}
