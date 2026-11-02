use super::GetURI;
use crate::request::addr;
use batch_model::site::param;

impl GetURI for param::rate_by_date_list::Param {
    fn get_uri(&self) -> &str {
        addr::URL_VIEW_RATE_BY_DATE
    }
}
impl GetURI for param::rate_by_date_by_code::Param {
    fn get_uri(&self) -> &str {
        addr::URL_VIEW_RATE_BY_DATE
    }
    fn is_code_by(&self) -> bool {
        true
    }
    fn get_code_by(&self) -> String {
        self.code_id.to_string()
    }
}
