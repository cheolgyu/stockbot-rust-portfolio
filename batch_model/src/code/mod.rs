use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::app_type::{country_type::CountryType, exchange_type::ExchangeType};
pub mod view;
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Code {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub std_code: String,
    pub country: CountryType,
    pub exchange: ExchangeType,
}
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CodeSimple {
    pub id: i64,
    pub std_code: String,
    pub short_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddCode {
    pub std_code: String,
    pub country: CountryType,
    pub exchange: ExchangeType,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CodeDetail {
    pub code_id: i64,
    pub std_code: String,
    pub short_code: String,
    pub std_name: String,
    pub short_name: String,
    pub eng_name: String,
    pub listing_date: String,
    pub exchange: ExchangeType,
    pub securities_type: String,
    pub company_type: String,
    pub stock_type: String,
    pub face_value: String,
    pub stock_cnt: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AddCodeDetail {
    //pub code_id: i64,
    pub std_code: String,
    pub short_code: String,
    pub std_name: String,
    pub short_name: String,
    pub eng_name: String,
    pub listing_date: String,
    pub exchange: ExchangeType,
    pub securities_type: String,
    pub company_type: String,
    pub stock_type: String,
    pub face_value: String,
    pub stock_cnt: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CodeState {
    pub code_id: i64,
    pub short_code: String,
    pub stop: bool,
    pub clear: bool,
    pub manger: bool,
    pub alert: bool,
    pub unfaithful: bool,
    pub low_liquidity: bool,
    pub shortage: bool,
    pub overheated: bool,
    pub caution: bool,
    pub warning: bool,
    pub risk: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AddCodeState {
    pub code_id: Option<i64>,
    pub short_code: String,
    pub stop: String,
    pub clear: String,
    pub manger: String,
    pub alert: String,
    pub unfaithful: String,
    pub low_liquidity: String,
    pub shortage: String,
    pub overheated: String,
    pub caution: String,
    pub warning: String,
    pub risk: String,
}
impl AddCodeState {
    fn parse_ox(inp: &str) -> Result<bool, String> {
        if inp == "X" {
            Ok(false)
        } else if inp == "O" {
            Ok(true)
        } else {
            Err("알수 없는 기호 들어옴".to_string())
        }
    }

    pub fn into_code_state(&self, code_id: i64) -> CodeState {
        CodeState {
            code_id: code_id,
            short_code: self.short_code.to_string(),
            stop: AddCodeState::parse_ox(&self.stop).unwrap(),
            clear: AddCodeState::parse_ox(&self.clear).unwrap(),
            manger: AddCodeState::parse_ox(&self.manger).unwrap(),
            alert: AddCodeState::parse_ox(&self.alert).unwrap(),
            unfaithful: AddCodeState::parse_ox(&self.unfaithful).unwrap(),
            low_liquidity: AddCodeState::parse_ox(&self.low_liquidity).unwrap(),
            shortage: AddCodeState::parse_ox(&self.shortage).unwrap(),
            overheated: AddCodeState::parse_ox(&self.overheated).unwrap(),
            caution: AddCodeState::parse_ox(&self.caution).unwrap(),
            warning: AddCodeState::parse_ox(&self.warning).unwrap(),
            risk: AddCodeState::parse_ox(&self.risk).unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MappingInvesting {
    pub code: String,
    pub investing_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateInfo {
    pub dt: u32,
    pub y: i32,
    pub m: u32,
    pub d: u32,
    pub q: u8,
    pub w: u8,
}
impl DateInfo {
    // dt,y,m,d
    pub fn dt_string2u32(dt: String) -> (u32, i32, u32, u32) {
        let rm_dt = dt.replace("-", "");
        let y: &str = &rm_dt[..4];
        let m: &str = &rm_dt[4..6];
        let d: &str = &rm_dt[6..8];
        let mut m_one: &str = &m;
        let mut d_one: &str = &d;

        if m[0..1].eq("0") {
            m_one = &m[1..2];
        }
        if d[0..1].eq("0") {
            d_one = &d[1..2];
        }

        let yi = y.parse::<i32>().unwrap();
        let mi = m_one.parse::<u32>().unwrap();
        let di = d_one.parse::<u32>().unwrap();

        (rm_dt.parse::<u32>().unwrap(), yi, mi, di)
    }
    pub fn get_quarter(m: u32) -> u8 {
        if m < 4 {
            1
        } else if m < 7 {
            2
        } else if m < 10 {
            3
        } else {
            4
        }
    }

    // 20220818:String -> 2022818
    pub fn new(dt: String) -> Self {
        let (dti, yi, mi, di) = Self::dt_string2u32(dt);

        let q: u8 = if mi < 4 {
            1
        } else if mi < 7 {
            2
        } else if mi < 10 {
            3
        } else {
            4
        };

        let n = NaiveDate::from_ymd_opt(yi.into(), mi.into(), di.into()).unwrap();
        let w = n.iso_week().week() as u8;

        //println!("ymd={},{},{},  q,w={},{}", y, m, d, q, w);

        //let dt  =  chrono::Local::now();
        Self {
            dt: dti,
            y: yi,
            m: mi,
            d: di,
            q: q,
            w: w,
        }
    }
}
