use super::GetURI;
use crate::request::addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Default)]
pub struct Param {
    pub short_code: String,
}

impl GetURI for Param {
    fn get_uri(&self) -> &str {
        addr::URL_CODE
    }
    fn is_code_by(&self) -> bool {
        true
    }
    fn get_code_by(&self) -> String {
        self.short_code.to_string()
    }
}
