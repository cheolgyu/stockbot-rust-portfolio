use serde::{Deserialize, Serialize};
use serde_with::formats::CommaSeparator;
use serde_with::serde_as;
use serde_with::StringWithSeparator;
use utoipa::{IntoParams, ToSchema};

use crate::app_type::exchange_type::ExchangeType;

#[serde_as]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct ParamExchange {
    #[schema(example = "KOSPI,KOSDAQ")]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, ExchangeType>")]
    pub exchange: Vec<ExchangeType>,
}

impl Default for ParamExchange {
    fn default() -> Self {
        Self {
            exchange: vec![ExchangeType::KOSPI, ExchangeType::KOSDAQ],
        }
    }
}
impl ParamExchange {
    pub fn format_sql_in(&self) -> String {
        // let list: Vec<String> = CheckboxExchange::valid_str(self.exchange.clone());
        let mut v = "".to_string();
        let mut i = 0;
        let s = self.exchange.len();
        for x in &self.exchange {
            if v == "" {
                v = format!("'{}'", x);
            } else {
                if i < s {
                    v = format!("{},'{}'", v, x);
                }
            }
            i = i + 1;
        }
        v.to_string()
    }

    pub fn valid(&self) -> (bool, String) {
        self.check_required()
    }

    fn check_required(&self) -> (bool, String) {
        if self.exchange.len() == 0 {
            (false, "거래소 1개는 필수 선택입니다.".to_string())
        } else {
            (true, "검사성공".to_string())
        }
    }
}

// #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
// #[schema(example = "KOSPI")]
// pub enum Key {
//     #[default]
//     KOSPI,
//     KOSDAQ,
//     KONEX,
// }

// impl FromStr for Key {
//     type Err = ();

//     fn from_str(input: &str) -> Result<Self, Self::Err> {
//         match input {
//             "KOSPI" => Ok(Key::KOSPI),
//             "KOSDAQ" => Ok(Key::KOSDAQ),
//             "KONEX" => Ok(Key::KONEX),
//             _ => Err(()),
//         }
//     }
// }
// impl Key {
//     pub fn text(self) -> String {
//         match self {
//             Key::KOSPI => TEXT_KOSPI.to_owned(),
//             Key::KOSDAQ => TEXT_KOSDAQ.to_owned(),
//             Key::KONEX => TEXT_KONEX.to_owned(),
//         }
//     }
// }
// pub const TEXT_KOSPI: &str = "코스피";
// pub const TEXT_KOSDAQ: &str = "코스닥";
// pub const TEXT_KONEX: &str = "코넥스";

// impl fmt::Display for Key {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Key::KOSPI => write!(f, "KOSPI"),
//             Key::KOSDAQ => write!(f, "KOSDAQ"),
//             Key::KONEX => write!(f, "KONEX"),
//         }
//     }
// }

// #[derive(Debug, Clone, Default)]
// pub struct Checkbox {
//     pub key: Key,
//     pub text: String,
//     pub checked: bool,
// }

// #[derive(Debug, Clone)]
// pub struct CheckboxExchange(pub Vec<Checkbox>);

// impl CheckboxExchange {
//     pub fn new() -> Self {
//         CheckboxExchange(vec![
//             Checkbox {
//                 key: Key::KOSPI,
//                 text: Key::KOSPI.text(),
//                 checked: true,
//             },
//             Checkbox {
//                 key: Key::KOSDAQ,
//                 text: Key::KOSDAQ.text(),
//                 checked: true,
//             },
//             Checkbox {
//                 key: Key::KONEX,
//                 text: Key::KONEX.text(),
//                 checked: false,
//             },
//         ])
//     }

//     pub fn new_checked(v: Vec<String>) -> Self {
//         let mut res: Self = Self::new();

//         for i in 0..res.0.len() {
//             let item = res.0[i].key.to_string();
//             let mut is_true = false;
//             for j in 0..v.len() {
//                 if item == v[j] {
//                     is_true = true;
//                 }
//             }
//             res.0[i].checked = is_true;
//         }

//         res
//     }

//     pub fn update_value(&mut self, v: Checkbox) {
//         let list = &mut self.0;
//         let mut checked: i32 = 0;
//         for checkbox in list {
//             if checkbox.checked {
//                 checked = checked + 1;
//             }
//         }

//         if checked != 0 {
//             let list = &mut self.0;

//             for checkbox in list {
//                 if checkbox.key == v.key {
//                     checkbox.checked = v.checked;
//                 }
//             }
//         }
//     }

//     pub fn query_format(list: Vec<String>) -> String {
//         let mut v_exchange = "".to_string();
//         if list.len() > 0 {
//             v_exchange = list.get(0).unwrap().to_string();
//             for x in 1..list.len() {
//                 let v = list.get(x).unwrap().to_string();
//                 v_exchange = v_exchange + "," + &v;
//             }
//         }

//         v_exchange
//     }

//     pub fn checked_to_vec(&self) -> Vec<String> {
//         let mut exchange_res: Vec<String> = vec![];

//         for v in &self.0 {
//             if v.checked {
//                 exchange_res.push(v.key.to_string())
//             }
//         }
//         exchange_res
//     }

//     pub fn valid_str(v: String) -> Vec<String> {
//         let exchange_vec: Vec<&str> = v.split(",").collect();
//         let mut exchange_res: Vec<String> = vec![];
//         for x in exchange_vec {
//             if Key::from_str(x).is_ok() {
//                 exchange_res.push(x.to_string())
//             }
//         }
//         exchange_res
//     }
// }
