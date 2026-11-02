use super::GetURI;
use crate::request::addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Default)]
pub struct Param {}

impl GetURI for Param {
    fn get_uri(&self) -> &str {
        addr::URL_SYSTEM_HEALTH_BATCH_TIMER
    }
}
