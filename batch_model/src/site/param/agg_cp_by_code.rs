use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::{
    app_type::{obs_epoch::ObsEpoch, obs_target::ObsTarget},
    site::param::{param_paging_two_sort::ParamPagingTwoSort, SqlObsVal},
};

#[derive(Deserialize, Serialize, Clone, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Param {
    #[param(example = 123)]
    pub code_id: i64,
    #[param(inline)]
    #[serde(flatten)]
    pub paging: ParamPagingTwoSort,
    #[param(example = "Month")]
    pub obs_epoch: ObsEpoch,
    #[param(example = "Min")]
    pub obs_target: ObsTarget,
    #[param(example = "None")]
    pub obs_val: Option<i32>,
}
impl SqlObsVal for Param {}
impl Default for Param {
    fn default() -> Self {
        Self {
            code_id: Default::default(),
            obs_epoch: ObsEpoch::default(),
            obs_target: ObsTarget::default(),
            paging: ParamPagingTwoSort::default_sort("rate_epoch", true),
            obs_val: None,
        }
    }
}
