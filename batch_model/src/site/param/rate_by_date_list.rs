use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::site::param::{param_paging_two_sort::ParamPagingTwoSort, SqlObsVal};

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
    #[param(inline)]
    pub dt: Option<NaiveDate>,
}
impl SqlObsVal for Param {}

impl Default for Param {
    fn default() -> Self {
        Self {
            paging: ParamPagingTwoSort::default_sort2("dt", true, "vol_rate", true),
            exchange: ParamExchange::default(),
            search: ParamSearch::default(),
            dt: None,
        }
    }
}
