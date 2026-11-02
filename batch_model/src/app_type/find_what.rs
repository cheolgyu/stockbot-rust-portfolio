use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use utoipa::ToSchema;
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "find_what"))]
#[derive(Deserialize, Serialize, Clone, Debug, Copy, Default, PartialEq, Eq, ToSchema)]
#[schema(example = "F")]
pub enum What {
    #[default]
    F,
    K,
    S,
    H,
}
impl What {}
impl FromStr for What {
    type Err = ();

    fn from_str(input: &str) -> Result<What, Self::Err> {
        match input {
            "F" => Ok(What::F),
            "K" => Ok(What::K),
            "S" => Ok(What::S),
            "H" => Ok(What::H),
            _ => Err(()),
        }
    }
}
impl fmt::Display for What {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            What::F => write!(f, "{}", "F"),
            What::K => write!(f, "{}", "K"),
            What::S => write!(f, "{}", "S"),
            What::H => write!(f, "{}", "H"),
        }
    }
}
