use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::app_type::{find_what::What, find_why::Why};

use super::param_paging::ParamPaging;

#[derive(Deserialize, Serialize, Clone, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Param {
    #[param(example = 123)]
    pub code_id: i64,

    #[param(inline)]
    pub what: What,

    #[param(inline)]
    pub why: Why,

    #[param(inline)]
    #[serde(flatten)]
    pub paging: ParamPaging,
}

impl Default for Param {
    fn default() -> Self {
        let mut paging: ParamPaging = ParamPaging::default();
        paging.sort = "change_rate".to_string();

        Self {
            code_id: Default::default(),
            what: Default::default(),
            why: Default::default(),
            paging: paging,
        }
    }
}

impl Param {}
