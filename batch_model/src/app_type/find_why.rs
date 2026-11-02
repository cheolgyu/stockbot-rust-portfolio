use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "find_why"))]
#[derive(Deserialize, Serialize, Clone, Debug, Copy, Default, PartialEq, Eq, ToSchema)]
#[schema(example = "B00")]
pub enum Why {
    #[default]
    B00,
    B10,
    B11,
    B12,
    V00,
}
impl FromStr for Why {
    type Err = ();

    fn from_str(input: &str) -> Result<Why, Self::Err> {
        match input {
            "B00" => Ok(Why::B00),
            "B10" => Ok(Why::B10),
            "B11" => Ok(Why::B11),
            "B12" => Ok(Why::B12),
            "V00" => Ok(Why::V00),
            _ => Err(()),
        }
    }
}

impl Why {
    pub fn text(&self) -> String {
        match self {
            Why::B00 => format!("{}", "바운드 순"),
            Why::B10 => format!("{}", "바운드-오늘 같은거"),
            Why::B11 => format!("{}", "바운드-오늘은 오른것"),
            Why::B12 => format!("{}", "바운드-오늘은 떨어진것"),
            Why::V00 => format!("{}", "거래량 순"),
        }
    }
}
impl fmt::Display for Why {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Why::B00 => write!(f, "{}", "B00"),
            Why::B10 => write!(f, "{}", "B10"),
            Why::B11 => write!(f, "{}", "B11"),
            Why::B12 => write!(f, "{}", "B12"),
            Why::V00 => write!(f, "{}", "V00"),
        }
    }
}
