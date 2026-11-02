use std::error::Error;

use chrono::NaiveDate;
use dotenvy::var;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

const DT_FORMAT: &str = "%Y-%m-%d";
pub fn get_default_start_dt() -> Result<NaiveDate, Box<dyn Error>> {
    let kr_price_default_start_date: String = var("KR_PRICE_DEFAULT_START_DATE")?;
    let search_start_dt_init =
        NaiveDate::parse_from_str(&format!("{}", kr_price_default_start_date), DT_FORMAT)?;
    Ok(search_start_dt_init)
}
pub fn calculate_change_rate(
    old_price: Decimal,
    new_price: Decimal,
) -> Result<Decimal, std::fmt::Error> {
    //let res = ((new_price - old_price) / old_price) * Decimal::new(100, 0).round_dp(2);
    let res = (match (new_price - old_price).checked_div(old_price) {
        Some(v) => v,
        None => {
            // 1/0 오류 발생.
            Decimal::ZERO
        }
    }) * Decimal::new(100, 0).round_dp(2);
    Ok(res)
}
/// 전체값의 일정 값에 해당하는 비율을 계산합니다.
pub fn calculate_rate(all: Decimal, part: Decimal) -> Result<Decimal, std::fmt::Error> {
    let res = match part.checked_div(all) {
        Some(v) => v * Decimal::new(100, 0).round_dp(3),
        None => Decimal::ZERO,
    };
    Ok(res)
}
