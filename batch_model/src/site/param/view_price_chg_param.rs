pub mod list {
    use crate::site::param::{
        param_exchange::ParamExchange, param_paging_two_sort::ParamPagingTwoSort,
        param_search::ParamSearch, SqlObsVal,
    };
    use chrono::NaiveDate;
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
        pub search: ParamSearch,
        #[param(inline)]
        pub dt: Option<NaiveDate>,
    }
    impl SqlObsVal for Param {}
    impl Default for Param {
        fn default() -> Self {
            Self {
                paging: ParamPagingTwoSort::default_sort("vol", true),
                exchange: ParamExchange::default(),
                search: ParamSearch::default(),
                dt: None,
            }
        }
    }
}
pub mod code_by {
    use crate::site::param::{
        param_exchange::ParamExchange, param_paging_two_sort::ParamPagingTwoSort,
    };
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};
    use utoipa::IntoParams;

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
        pub exchange: ParamExchange,

        #[param(inline)]
        #[serde(flatten)]
        pub dt: Option<NaiveDate>,
    }
    impl Default for Param {
        fn default() -> Self {
            Self {
                code_id: Default::default(),
                paging: ParamPagingTwoSort::default_sort("dt", true),
                exchange: ParamExchange::default(),
                dt: None,
            }
        }
    }
}
