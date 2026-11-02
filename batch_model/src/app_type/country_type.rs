use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "country_type"))]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Copy, ToSchema, Default)]
pub enum CountryType {
    #[default]
    KR,
    US,
}

pub const EXCAHGE_US: [ExchangeType; 3] =
    [ExchangeType::NASDAQ, ExchangeType::NYSE, ExchangeType::AMEX];

pub const EXCAHGE_KR: [ExchangeType; 3] = [
    ExchangeType::KOSPI,
    ExchangeType::KOSDAQ,
    ExchangeType::KONEX,
];
impl fmt::Display for CountryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CountryType::KR => write!(f, "KR"),
            CountryType::US => write!(f, "US"),
        }
    }
}

use std::error::Error;
use std::str::FromStr;

use crate::app_type::exchange_type::ExchangeType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCountryError {
    details: String,
}
impl ParseCountryError {
    fn new(msg: &str) -> ParseCountryError {
        ParseCountryError {
            details: msg.to_string(),
        }
    }
}
impl fmt::Display for ParseCountryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "나라 파서 오류;")
    }
}

impl Error for ParseCountryError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl FromStr for CountryType {
    type Err = ParseCountryError;

    fn from_str(input: &str) -> Result<CountryType, ParseCountryError> {
        match input.to_uppercase().as_str() {
            "KR" => Ok(CountryType::KR),
            "US" => Ok(CountryType::US),
            _ => Err(ParseCountryError::new(
                format!("나라를  선택하세요; input:{:?}", input).as_str(),
            )),
        }
    }
}
