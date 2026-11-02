use std::str::FromStr;

use batch_model::{code::DateInfo, price::AddPrice};
use chrono::NaiveDate;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde_json::Value;
use tokio::io::AsyncReadExt;
use tracing::info;

#[derive(Debug)]
pub struct ParserFile {
    pub code_id: i64,
    pub fnm: String,
}
impl ParserFile {
    pub async fn run(&self) -> Vec<AddPrice> {
        let mut list: Vec<AddPrice> = Vec::new();
        // let fnm2 = fnm.clone();

        // replace ' => \""
        // println!("Parse Run fnm={}", fnm);
        let mut f = tokio::fs::File::open(&self.fnm).await.unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).await.unwrap();

        let data_string = match String::from_utf8(buffer) {
            Ok(data) => data,
            Err(err) => {
                info!("파일 데이터를 string으로 불러오기 실패, 신규상장시 오류발생함. 데이터가 없음으로로");
                info!("파일이름: {}", self.fnm);
                panic!("{:?}", err);
            }
        };

        let mut new_string = data_string.replace("'", "\"");
        new_string = new_string.replace(", ],", ", 0 ],");

        //println!("new_string, [0]={}", new_string);
        //            let new_string_json_vec:Vec<serde_json::Value> ;
        let j_vec: Vec<serde_json::Value> = match serde_json::from_str(&new_string) {
            Ok(res) => res,
            Err(err) => {
                info!("오류발생:{:?}", err);
                info!("오류발생: self={:?}", self);
                panic!("{:?}", err);
            }
        };
        // println!("j_vec, len()={}", j_vec.len());
        // println!("j_vec, [0]={}", j_vec[0]);
        let mut i = 0;
        //let jvec_len = j_vec.len();
        for v in j_vec.into_iter() {
            if i != 0 {
                let dt = v[0].as_str().expect("날짜파싱 실패.");
                let di = DateInfo::new(dt.to_string());
                let dt = NaiveDate::from_ymd_opt(di.y, di.m, di.d).unwrap();

                let pi = AddPrice {
                    code_id: self.code_id,
                    dt: NaiveDate::from_ymd_opt(di.y, di.m, di.d).unwrap(),
                    op: Self::parse_json2_price(v[1].clone()),
                    hp: Self::parse_json2_price(v[2].clone()),
                    lp: Self::parse_json2_price(v[3].clone()),
                    cp: Self::parse_json2_price(v[4].clone()),
                    vol: v[5].as_i64().unwrap(),
                    t_val: 0,
                    stock_cnt: 0,
                    capital: 0,
                };

                list.push(pi);
            }
            i = i + 1;
        }
        // println!(
        //     "parse:naver_chart code={:?}, file_len={} , len()={}, fnm={}",
        //     self.info.code.clone(),
        //     jvec_len,
        //     list.len(),
        //     fnm.to_string()
        // );
        list
    }

    fn parse_json2_price(inp: Value) -> Decimal {
        if inp.is_number() {
            let n: u64 = match inp.as_u64() {
                Some(inp_some) => inp_some,
                None => {
                    println!("None :inp.as_u64: inp={},inp={:?}", inp, inp);
                    panic!();
                }
            };
            Decimal::from_u64(n).unwrap()
        } else if inp.is_f64() {
            let n: f64 = match inp.as_f64() {
                Some(inp_some) => inp_some,
                None => {
                    println!("None :inp.as_f64: inp={},inp={:?}", inp, inp);
                    panic!();
                }
            };
            Decimal::from_f64(n).unwrap()
        } else {
            let n: &str = match inp.as_str() {
                Some(inp_some) => inp_some,
                None => {
                    println!("None :inp.as_str: inp={},inp={:?}", inp, inp);
                    panic!();
                }
            };
            Decimal::from_str(n).unwrap()
        }
    }
}
