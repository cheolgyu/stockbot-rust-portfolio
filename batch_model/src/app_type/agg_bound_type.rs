use std::{
    fmt::{self},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use strum_macros::Display;

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "agg_bound_type"))]
#[derive(
    Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Display, utoipa::ToSchema,
)]
pub enum AggBoundType {
    #[default]
    CHG, //가격 변화율
    DAY, // x_cnt
}

impl AggBoundType {
    pub fn to_vec() -> Vec<Self> {
        vec![Self::CHG, Self::DAY]
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::CHG => "CHG".to_string(),
            Self::DAY => "DAY".to_string(),
        }
    }
    pub fn text(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CHG => write!(f, "{}", "가격 변화율"),
            Self::DAY => write!(f, "{}", "바운드 지속일"),
        }
    }

    pub fn to_label(&self) -> String {
        match self {
            Self::CHG => "가격 변화율".to_string(),
            Self::DAY => "바운드 지속일".to_string(),
        }
    }
}
impl FromStr for AggBoundType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "CHG" => Ok(Self::CHG),
            "DAY" => Ok(Self::DAY),
            _ => Err(()),
        }
    }
}
