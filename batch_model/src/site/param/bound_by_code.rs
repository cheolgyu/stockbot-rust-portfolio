use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::{
    app_type::price_type::PriceType, site::param::param_paging_two_sort::ParamPagingTwoSort,
};

#[derive(Deserialize, Serialize, Clone, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Param {
    #[param(example = 123)]
    pub code_id: i64,
    #[param(example = "CLOSE")]
    pub line: PriceType,

    #[param(inline)]
    #[serde(flatten)]
    pub paging: ParamPagingTwoSort,
}

impl Default for Param {
    fn default() -> Self {
        Self {
            code_id: Default::default(),
            line: Default::default(),
            paging: ParamPagingTwoSort::default_sort("p1x", true),
        }
    }
}
