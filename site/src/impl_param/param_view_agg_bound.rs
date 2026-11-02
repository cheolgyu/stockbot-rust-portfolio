use super::GetURI;
use crate::request::addr;
use batch_model::site::param::agg_bound_param::list::Param;

impl GetURI for Param {
    fn get_uri(&self) -> &str {
        addr::URL_VIEW_AGG_BOUND
    }
}
pub mod code_by {
    use super::GetURI;
    use crate::request::addr;
    use batch_model::site::param::agg_bound_param::code_by::Param;

    impl GetURI for Param {
        fn get_uri(&self) -> &str {
            addr::URL_VIEW_AGG_BOUND
        }
        fn is_code_by(&self) -> bool {
            true
        }
        fn get_code_by(&self) -> String {
            self.code_id.to_string()
        }
    }
}
