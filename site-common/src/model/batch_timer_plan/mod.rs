use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};

pub mod list;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct Model {
    pub id: i64,
    pub batch_type: String,
    pub country_type: String,
    pub run_type: String,
    pub updated_dt: Option<DateTime<Local>>,
    pub crated_dt: DateTime<Local>,
}
