use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const TEXT_KOSPI: &str = "코스피";
pub const TEXT_KOSDAQ: &str = "코스닥";
pub const TEXT_KONEX: &str = "코넥스";

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "exchange_type"))]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Copy, Serialize, ToSchema, Default)]
#[warn(non_camel_case_types)]
pub enum ExchangeType {
    /*
        한국
    */
    //코스피`
    #[default]
    KOSPI,
    //코스닥
    KOSDAQ,
    #[cfg_attr(feature = "sqlx", sqlx(rename = "KOSDAQ_GLOBAL"))]
    KosdaqGlobal,
    //코넥스
    KONEX,
    /*
        미국
    */
    //나스닥
    NASDAQ,
    //뉴욕증권거래소
    NYSE,
    //아멕스(아메리칸 익스프레스)
    AMEX,
}
// https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html#manual-implementation
// https://docs.rs/sqlx/latest/sqlx/prelude/trait.Type.html

// #[cfg(feature = "postgres")]
// impl sqlx::Type<Postgres> for ExchangeType {
//     fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
//         todo!()
//     }

//     fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
//         *ty == Self::type_info()
//     }
// }

impl ExchangeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExchangeType::KOSPI => "KOSPI",
            ExchangeType::KOSDAQ => "KOSDAQ",
            ExchangeType::KosdaqGlobal => "KOSDAQ_GLOBAL",
            ExchangeType::KONEX => "KONEX",
            ExchangeType::NASDAQ => "NASDAQ",
            ExchangeType::NYSE => "NYSE",
            ExchangeType::AMEX => "AMEX",
        }
    }

    pub fn text(&self) -> String {
        match self {
            ExchangeType::KOSPI => TEXT_KOSPI.to_owned(),
            ExchangeType::KOSDAQ => TEXT_KOSDAQ.to_owned(),
            ExchangeType::KONEX => TEXT_KONEX.to_owned(),
            _ => "준비중입니다.".to_string(),
        }
    }
}
impl fmt::Display for ExchangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExchangeType::KOSPI => write!(f, "KOSPI"),
            ExchangeType::KOSDAQ => write!(f, "KOSDAQ"),
            ExchangeType::KosdaqGlobal => write!(f, "KOSDAQ_GLOBAL"),
            ExchangeType::KONEX => write!(f, "KONEX"),
            ExchangeType::NASDAQ => write!(f, "NASDAQ"),
            ExchangeType::NYSE => write!(f, "NYSE"),
            ExchangeType::AMEX => write!(f, "AMEX"),
        }
    }
}
use std::error::Error;
use std::fmt;
use std::str::FromStr;
impl FromStr for ExchangeType {
    type Err = ParseExchageError;

    fn from_str(input: &str) -> Result<ExchangeType, ParseExchageError> {
        match input.to_uppercase().as_str() {
            "KOSPI" => Ok(ExchangeType::KOSPI),
            "KOSDAQ" => Ok(ExchangeType::KOSDAQ),
            "KOSDAQ GLOBAL" => Ok(ExchangeType::KosdaqGlobal),

            "KONEX" => Ok(ExchangeType::KONEX),

            "NASDAQ" => Ok(ExchangeType::NASDAQ),
            "NYSE" => Ok(ExchangeType::NYSE),
            "AMEX" => Ok(ExchangeType::AMEX),
            _ => Err(ParseExchageError::new(
                format!("거래소  선택하세요; input:{:?}", input).as_str(),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseExchageError {
    details: String,
}
impl ParseExchageError {
    fn new(msg: &str) -> ParseExchageError {
        ParseExchageError {
            details: msg.to_string(),
        }
    }
}
impl fmt::Display for ParseExchageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "거래소 파서 오류;")
    }
}

impl Error for ParseExchageError {
    fn description(&self) -> &str {
        &self.details
    }
}
