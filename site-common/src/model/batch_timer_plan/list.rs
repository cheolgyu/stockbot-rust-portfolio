use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::model::paging::{info::Info, params::ParamsPaging};

use super::Model;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Result {
    pub data: Vec<Model>,
    pub info: Info,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Params {
    #[param(inline)]
    #[serde(flatten)]
    pub paging: ParamsPaging,
}
impl Default for Params {
    fn default() -> Self {
        let mut paging = ParamsPaging::default();
        paging.sort = "id".to_string();
        paging.desc = true;
        Self { paging }
    }
}
