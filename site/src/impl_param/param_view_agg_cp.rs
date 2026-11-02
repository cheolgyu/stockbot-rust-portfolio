use super::GetURI;
use crate::request::addr;
use batch_model::site::param::agg_cp_list::Param;

impl GetURI for Param {
    fn get_uri(&self) -> &str {
        addr::URL_VIEW_AGG_CP
    }
}
