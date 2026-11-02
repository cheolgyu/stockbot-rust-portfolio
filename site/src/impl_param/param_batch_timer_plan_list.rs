use super::GetURI;
use crate::request::addr;
use batch_model::site::param::batch_timer_plan::Param;

impl GetURI for Param {
    fn get_uri(&self) -> &str {
        addr::URL_SYSTEM_BATCH_TIMER_PLAN
    }
}
