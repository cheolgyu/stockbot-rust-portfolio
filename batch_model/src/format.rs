use chrono::{NaiveDate, ParseError};
pub const DT_FORMAT2: &str = "%Y%m%d";

pub fn format2(x: i32) -> Result<NaiveDate, ParseError> {
    let res = NaiveDate::parse_from_str(&format!("{}", x), DT_FORMAT2);

    res
}
