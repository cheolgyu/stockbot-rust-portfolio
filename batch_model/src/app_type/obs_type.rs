use std::{
    fmt::{self},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use strum_macros::Display;

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "obs_type"))]
#[derive(
    Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Display, utoipa::ToSchema,
)]
pub enum ObsType {
    //주
    WeekMin,

    WeekMax,
    //월
    MonthMin,
    #[default]
    MonthMax,
    //분기
    QuarterMin,
    QuarterMax,
}
impl FromStr for ObsType {
    type Err = ();

    fn from_str(input: &str) -> Result<ObsType, Self::Err> {
        match input {
            "WeekMin" => Ok(ObsType::WeekMin),
            "WeekMax" => Ok(ObsType::WeekMax),
            "MonthMin" => Ok(ObsType::MonthMin),
            "MonthMax" => Ok(ObsType::MonthMax),
            "QuarterMin" => Ok(ObsType::QuarterMin),
            "QuarterMax" => Ok(ObsType::QuarterMax),
            _ => Err(()),
        }
    }
}

impl ObsType {
    pub fn to_vec() -> Vec<ObsType> {
        vec![
            ObsType::WeekMin,
            ObsType::WeekMax,
            ObsType::MonthMin,
            ObsType::MonthMax,
            ObsType::QuarterMin,
            ObsType::QuarterMax,
        ]
    }

    pub fn to_string(&self) -> String {
        match self {
            ObsType::WeekMin => "WeekMin".to_string(),
            ObsType::WeekMax => "WeekMax".to_string(),
            ObsType::MonthMin => "MonthMin".to_string(),
            ObsType::MonthMax => "MonthMax".to_string(),
            ObsType::QuarterMin => "QuarterMin".to_string(),
            ObsType::QuarterMax => "QuarterMax".to_string(),
        }
    }
    pub fn text(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObsType::WeekMin => write!(f, "{}", "주 최소"),
            ObsType::WeekMax => write!(f, "{}", "주 최대"),
            ObsType::MonthMin => write!(f, "{}", "월 최소"),
            ObsType::MonthMax => write!(f, "{}", "월 최대"),
            ObsType::QuarterMin => write!(f, "{}", "분기 최소"),
            ObsType::QuarterMax => write!(f, "{}", "분기 최대"),
        }
    }

    pub fn to_label(&self) -> String {
        match self {
            ObsType::WeekMin => "주 최소".to_string(),
            ObsType::WeekMax => "주 최대".to_string(),
            ObsType::MonthMin => "월 최소".to_string(),
            ObsType::MonthMax => "월 최대".to_string(),
            ObsType::QuarterMin => "분기 최소".to_string(),
            ObsType::QuarterMax => "분기 최대".to_string(),
        }
    }
}
