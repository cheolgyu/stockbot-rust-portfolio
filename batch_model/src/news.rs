use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct News {
    pub code_id: i64,
    pub dt: NaiveDate,
    pub title: String,
    pub link: String,
    pub media_type: String,
}
