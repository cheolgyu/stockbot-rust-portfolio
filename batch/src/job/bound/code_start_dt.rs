use batch_model::app_type::{exchange_type::ExchangeType, price_type::PriceType};
use chrono::NaiveDate;
use sqlx::FromRow;
#[derive(Debug, Clone, FromRow)]
pub struct CodeStartDt {
    pub id: i64,
    pub exchange: ExchangeType,
    pub close: NaiveDate,
    pub open: NaiveDate,
    pub high: NaiveDate,
    pub low: NaiveDate,
}

impl CodeStartDt {
    pub fn get_dt(&self, t: PriceType) -> NaiveDate {
        match t {
            PriceType::CLOSE => self.close,
            PriceType::OPEN => self.open,
            PriceType::LOW => self.low,
            PriceType::HIGH => self.high,
        }
    }
}
