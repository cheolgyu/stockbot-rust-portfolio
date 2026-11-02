use serde::Deserialize;
use utoipa::IntoParams;

use crate::app_type::exchange_type::ExchangeType;

#[derive(Deserialize, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Param {
    #[param(inline)]
    pub exchange: ExchangeType,
}
