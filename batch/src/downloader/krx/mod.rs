use std::any::Any;

pub mod krx_download;
pub mod krx_param_code_detail;
pub mod krx_param_code_state;
pub mod krx_param_price;

pub trait KrxToParam {
    fn to_param(&self) -> String;

    fn as_any(&self) -> &dyn Any;
}
