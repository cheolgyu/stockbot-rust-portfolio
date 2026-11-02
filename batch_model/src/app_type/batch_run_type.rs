use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use utoipa::{IntoParams, ToSchema};

use crate::app_type::country_type::CountryType;

#[derive(Debug, Clone, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct BatchParams {
    pub batch_type: BatchType,
    pub country_type: CountryType,
    pub run_type: RunType,
}

impl BatchParams {
    pub fn to_args(&self) -> Vec<String> {
        vec![
            self.batch_type.to_string(),
            self.country_type.to_string(),
            self.run_type.to_string(),
        ]
    }
}

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "batch_type"))]
#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, ToSchema, EnumString, Display, Default,
)]
pub enum BatchType {
    #[default]
    BATCH,
    NEWS,
}

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "run_type"))]
#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, ToSchema, EnumString, Display, Default,
)]
pub enum RunType {
    #[default]
    START,
    EXIT,
}
