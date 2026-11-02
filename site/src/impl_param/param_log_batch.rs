use super::GetURI;
use crate::request::addr;
use batch_model::site::param::log_batch::Param;

impl GetURI for Param {
    fn get_uri(&self) -> &str {
        addr::URL_SYSTEM_LOG_BATCH
    }
}
