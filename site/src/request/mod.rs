use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

use gloo::console::error;
use gloo_net::http::Request;
use serde::Serialize;
use serde_json::Value;

use crate::impl_param::GetURI;

pub mod addr {
    pub const API_VERSION: &str = "/api/v1";
    pub const URL_EXCHANGE: &str = "/utils/exchange_list";
    pub const URL_UTILS_OPENDT: &str = "/utils/open_dt_list?exchange=KOSPI";
    pub const URL_VIEW_BOUND: &str = "/bound";
    pub const URL_VIEW_AGG_VOL: &str = "/agg/vol";
    pub const URL_VIEW_AGG_BOUND: &str = "/agg/bound";
    pub const URL_VIEW_PRICE_CHG: &str = "/price_chg";
    pub const URL_VIEW_AGG_CP: &str = "/agg/cp";
    pub const URL_VIEW_RATE_BY_DATE: &str = "/rate_by_dt";
    pub const URL_VIEW_FIND: &str = "/find";
    pub const URL_CODE: &str = "/code";
    pub const URL_SWAGGER: &str = "/swagger-ui/";
    pub const URL_SYSTEM_LOG_BATCH: &str = "/system/logs/batch";
    pub const URL_SYSTEM_HEALTH_BATCH_TIMER: &str = "/system/health/batch_timer";
    pub const URL_SYSTEM_BATCH_TIMER_PLAN: &str = "/batch_timer_plan";
    pub const URL_PRICE: &str = "/price";
}

//https://docs.rs/pulldown-cmark/latest/pulldown_cmark/
/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: Value,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<Value> for FetchError {
    fn from(value: Value) -> Self {
        Self { err: value }
    }
}

/// The possible states a fetch request can be in.
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

// https://rustwasm.github.io/docs/wasm-bindgen/examples/fetch.html
// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
/// /*
///  set head 안됨. set header 하려면 goo-net 쓰면됨. 하지만 goo-net으로 jsvalue로 리턴이 안됨.
///  */
pub async fn fetch<T: GetURI>(param: T) -> Result<Value, FetchError>
where
    T: Serialize,
{
    let url = format!(
        "{}{}{}",
        dotenvy_macro::dotenv!("BACKEND_ADDR"),
        addr::API_VERSION,
        param.uri()
    );
    let resp = Request::get(&url).send().await;
    match resp {
        Ok(resp) => match resp.json::<Value>().await {
            Ok(res) => Ok(res),
            Err(err) => {
                error!("err1:{}", &err.to_string());
                panic!("{:#?}", err);
            }
        },
        Err(err) => {
            error!("err1:{}", &err.to_string());
            panic!("{:#?}", err);
        }
    }
}
