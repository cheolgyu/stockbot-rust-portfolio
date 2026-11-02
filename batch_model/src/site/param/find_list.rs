use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use super::{param_exchange::ParamExchange, param_paging::ParamPaging, param_search::ParamSearch};
use crate::app_type::{find_what::What, find_why::Why};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Param {
    #[param(inline)]
    #[serde(flatten)]
    pub paging: ParamPaging,
    #[param(inline)]
    #[serde(flatten)]
    pub exchange: ParamExchange,
    #[param(inline)]
    #[serde(flatten)]
    pub search: ParamSearch,
    //pub dt: NaiveDate,
    #[param(inline)]
    pub what: What,
    #[param(inline)]
    pub why: Why,
}

impl Default for Param {
    fn default() -> Self {
        let mut paging: ParamPaging = ParamPaging::default();
        paging.sort = "change_rate".to_string();
        Self {
            paging: paging,
            exchange: Default::default(),
            search: Default::default(),
            what: Default::default(),
            why: Default::default(),
        }
    }
}
