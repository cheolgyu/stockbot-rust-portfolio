use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::app_type::price_type::PriceType;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct ParamLine {
    #[schema(example = "CLOSE")]
    pub line: PriceType,
}

impl Default for ParamLine {
    fn default() -> Self {
        Self {
            line: PriceType::CLOSE,
        }
    }
}
impl ParamLine {
    pub fn format_sql(&self) -> String {
        format!(" and line='{}'", self.line.to_string())
    }

    // pub fn valid(&self) -> (bool, String) {
    //     self.check_required()
    // }

    // fn check_required(&self) -> (bool, String) {
    //     if self.exchange.len() == 0 {
    //         (false, "거래소 1개는 필수 선택입니다.".to_string())
    //     } else {
    //         (true, "검사성공".to_string())
    //     }
    // }
}
