pub use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
pub use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{slice::Iter, str::FromStr};
use strum_macros::Display;

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "price_type"))]
#[derive(
    Debug, PartialEq, Display, Clone, Copy, Serialize, Deserialize, Default, Eq, utoipa::ToSchema,
)]
pub enum PriceType {
    #[default]
    CLOSE,
    OPEN,
    LOW,
    HIGH,
}

impl FromStr for PriceType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "CLOSE" => Ok(PriceType::CLOSE),
            "OPEN" => Ok(PriceType::OPEN),
            "LOW" => Ok(PriceType::LOW),
            "HIGH" => Ok(PriceType::HIGH),
            _ => Err(()),
        }
    }
}

impl PriceType {
    pub fn to_text(&self) -> &str {
        match &self {
            PriceType::CLOSE => "종가",
            PriceType::OPEN => "시가",
            PriceType::LOW => "저가",
            PriceType::HIGH => "고가",
        }
    }
    pub fn iterator() -> Iter<'static, PriceType> {
        static PRICE_KIND: [PriceType; 4] = [
            PriceType::CLOSE,
            PriceType::OPEN,
            PriceType::LOW,
            PriceType::HIGH,
        ];
        PRICE_KIND.iter()
    }
    pub fn from_u32(value: u32) -> Self {
        match value {
            1 => Self::CLOSE,
            2 => Self::OPEN,
            3 => Self::LOW,
            4 => Self::HIGH,
            _ => panic!("Unknown value: {}", value),
        }
    }
    pub fn short_key(&self) -> &str {
        match &self {
            PriceType::CLOSE => "cp",
            PriceType::OPEN => "op",
            PriceType::LOW => "lp",
            PriceType::HIGH => "hp",
        }
    }

    pub fn short_key_list() -> Vec<String> {
        vec![
            "cp".to_string(),
            "op".to_string(),
            "lp".to_string(),
            "hp".to_string(),
        ]
    }
}
