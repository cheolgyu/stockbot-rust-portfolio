use super::GetURI;
use crate::request::addr;
use batch_model::site::param::view_price_chg_param::list::Param;

impl GetURI for Param {
    fn get_uri(&self) -> &str {
        addr::URL_VIEW_PRICE_CHG
    }
}
pub mod code_by {
    use super::GetURI;
    use crate::request::addr;
    use batch_model::site::param::view_price_chg_param::code_by::Param;

    impl GetURI for Param {
        fn get_uri(&self) -> &str {
            addr::URL_VIEW_PRICE_CHG
        }
        fn is_code_by(&self) -> bool {
            true
        }
        fn get_code_by(&self) -> String {
            self.code_id.to_string()
        }
    }
}
