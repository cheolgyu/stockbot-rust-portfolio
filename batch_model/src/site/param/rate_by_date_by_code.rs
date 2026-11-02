use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::site::param::{param_paging_two_sort::ParamPagingTwoSort, SqlObsVal};

#[derive(Deserialize, Serialize, Clone, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Param {
    #[param(example = 123)]
    pub code_id: i64,
    #[param(inline)]
    #[serde(flatten)]
    pub paging: ParamPagingTwoSort,
    #[param(inline)]
    #[serde(flatten)]
    pub dt: Option<NaiveDate>,
}

impl SqlObsVal for Param {}

impl Default for Param {
    fn default() -> Self {
        Self {
            code_id: Default::default(),
            paging: ParamPagingTwoSort::default_sort2("dt", true, "vol_rate", true),
            dt: None,
        }
    }
}
