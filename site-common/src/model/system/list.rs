use std::fmt::format;

use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::model::paging::{info::Info, params::ParamsPaging};

use super::Model;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Result {
    pub data: Vec<Model>,
    pub info: Info,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, IntoParams, Default)]
#[into_params(parameter_in = Query)]
pub struct Params {
    #[param(inline)]
    #[serde(flatten)]
    pub paging: ParamsPaging,
    pub id: Option<i64>,
}
impl Params {
    pub fn format_sql_batch_id(&self) -> String {
        let chk = match self.id {
            Some(id) => {
                if id > 0 {
                    true
                } else {
                    false
                }
            }
            None => false,
        };
        if chk {
            let id: i64 = self.id.map(|id| id).unwrap();
            format!(" AND  batch_id = {}", id)
        } else {
            format!("")
        }
    }
}
