use super::GetURI;
use crate::request::addr;
use batch_model::site::param::bound_list::Param;

impl GetURI for Param {
    fn get_uri(&self) -> &str {
        addr::URL_VIEW_BOUND
    }
}
