use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::site::param::param_paging_two_sort::ParamPagingTwoSort;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Param {
    #[param(inline)]
    #[serde(flatten)]
    pub paging: ParamPagingTwoSort,
    pub id: Option<i64>,
}
impl Param {
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
impl Default for Param {
    fn default() -> Self {
        Self {
            paging: ParamPagingTwoSort::default_sort("step_start_time", true),
            id: None,
        }
    }
}
