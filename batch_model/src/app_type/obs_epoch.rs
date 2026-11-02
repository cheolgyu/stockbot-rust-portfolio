use std::{
    fmt::{self},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use strum_macros::Display;

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "obs_epoch"))]
#[derive(
    Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Display, utoipa::ToSchema,
)]
pub enum ObsEpoch {
    WEEK,
    #[default]
    MONTH,
    QUARTER,
}

impl ObsEpoch {
    pub fn to_vec() -> Vec<Self> {
        vec![Self::WEEK, Self::MONTH, Self::QUARTER]
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::WEEK => "WEEK".to_string(),
            Self::MONTH => "MONTH".to_string(),
            Self::QUARTER => "QUARTER".to_string(),
        }
    }
    pub fn text(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::WEEK => write!(f, "{}", "주 "),
            Self::MONTH => write!(f, "{}", "월 "),
            Self::QUARTER => write!(f, "{}", "분기 "),
        }
    }

    pub fn to_label(&self) -> String {
        match self {
            Self::WEEK => "주".to_string(),
            Self::MONTH => "월".to_string(),
            Self::QUARTER => "분기".to_string(),
        }
    }
}
impl FromStr for ObsEpoch {
    type Err = ();

    fn from_str(input: &str) -> Result<ObsEpoch, Self::Err> {
        match input {
            "WEEK" => Ok(Self::WEEK),
            "MONTH" => Ok(Self::MONTH),
            "QUARTER" => Ok(Self::QUARTER),
            _ => Err(()),
        }
    }
}
