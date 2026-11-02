use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::app_type::{
    batch_run_type::{BatchType, RunType},
    country_type::CountryType,
};

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Clone, Deserialize, Default, Serialize)]
pub struct BatchTimerPlan {
    //#[order(1)]
    pub id: i64,
    //#[order(2)]
    pub batch_type: BatchType,
    //#[order(3)]
    pub country_type: CountryType,
    //#[order(4)]
    pub run_type: RunType,
    //#[order(5)]
    pub updated_dt: Option<DateTime<Local>>,
    //#[order(6)]
    pub crated_dt: DateTime<Local>,
}
