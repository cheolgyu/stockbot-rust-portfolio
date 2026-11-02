use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Eq, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct ParamsSearch {
    #[schema(required = false, example = "")]
    pub search: Option<String>,
}
impl ParamsSearch {
    pub fn new() -> Self {
        Self {
            search: Some("".to_string()),
        }
    }
    ///  AND name like '%{search}%'  
    pub fn sql_query(&self, query: String) -> String {
        match &self.search {
            Some(v) => {
                if v.ne("") {
                    format!(
                        " {query} AND  name like '%{search}%'  ",
                        query = query,
                        search = v
                    )
                } else {
                    query
                }
            }
            None => query,
        }
    }
}
