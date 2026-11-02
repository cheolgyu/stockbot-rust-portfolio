use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::{
    app_type::{obs_epoch::ObsEpoch, obs_target::ObsTarget},
    site::param::{param_paging_two_sort::ParamPagingTwoSort, SqlObsVal},
};

use super::{param_exchange::ParamExchange, param_search::ParamSearch};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Param {
    #[param(inline)]
    #[serde(flatten)]
    pub paging: ParamPagingTwoSort,
    #[param(inline)]
    #[serde(flatten)]
    pub exchange: ParamExchange,
    #[param(inline)]
    #[serde(flatten)]
    pub search: ParamSearch,
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
            paging: ParamPagingTwoSort::default_sort2("v_epoch_cnt", true, "rate_epoch", true),
            exchange: ParamExchange::default(),
            search: ParamSearch::default(),
            obs_epoch: ObsEpoch::default(),
            obs_target: ObsTarget::default(),
            obs_val: None,
        }
    }
}
