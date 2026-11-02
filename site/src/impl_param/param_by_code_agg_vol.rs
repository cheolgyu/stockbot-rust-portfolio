use super::GetURI;
use crate::request::addr;
use batch_model::site::param::agg_vol_by_code::Param;

impl GetURI for Param {
    fn get_uri(&self) -> &str {
        addr::URL_VIEW_AGG_VOL
    }
    fn is_code_by(&self) -> bool {
        true
    }
    fn get_code_by(&self) -> String {
        self.code_id.to_string()
    }
}
