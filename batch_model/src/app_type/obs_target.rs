use std::{
    fmt::{self},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use strum_macros::Display;

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "obs_target"))]
#[derive(
    Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Display, utoipa::ToSchema,
)]
pub enum ObsTarget {
    #[default]
    MIN,
    MAX,
    AVG,
}

impl ObsTarget {
    pub fn to_vec() -> Vec<Self> {
        vec![Self::MIN, Self::MAX, Self::AVG]
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::MIN => "MIN".to_string(),
            Self::MAX => "MAX".to_string(),
            Self::AVG => "AVG".to_string(),
        }
    }
    pub fn text(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MIN => write!(f, "{}", "최소"),
            Self::MAX => write!(f, "{}", "최대"),
            Self::AVG => write!(f, "{}", "평균"),
        }
    }

    pub fn to_label(&self) -> String {
        match self {
            Self::MIN => "최소".to_string(),
            Self::MAX => "최대".to_string(),
            Self::AVG => "평균".to_string(),
        }
    }
}
impl FromStr for ObsTarget {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "MIN" => Ok(Self::MIN),
            "MAX" => Ok(Self::MAX),
            "AVG" => Ok(Self::AVG),
            _ => Err(()),
        }
    }
}
