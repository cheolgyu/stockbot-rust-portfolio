use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::site::param::param_paging_two_sort::ParamPagingTwoSort;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Param {
    #[param(inline)]
    #[serde(flatten)]
    pub paging: ParamPagingTwoSort,
}
impl Default for Param {
    fn default() -> Self {
        Self {
            paging: ParamPagingTwoSort::default_sort("crated_dt", true),
        }
    }
    //
}
