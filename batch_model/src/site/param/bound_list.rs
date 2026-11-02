use crate::site::param::{
    param_exchange::ParamExchange, param_line::ParamLine,
    param_paging_two_sort::ParamPagingTwoSort, param_search::ParamSearch,
};

use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

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
    pub line: ParamLine,
    #[param(inline)]
    #[serde(flatten)]
    pub search: ParamSearch,
}

impl Default for Param {
    fn default() -> Self {
        Self {
            paging: ParamPagingTwoSort::default_sort("change_rate",true),
            line: ParamLine::default(),
            exchange: ParamExchange::default(),
            search: ParamSearch::default(),
        }
    }
}
