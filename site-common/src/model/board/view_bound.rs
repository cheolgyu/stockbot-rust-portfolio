use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::PriceType;

/*
    https://github.com/rust-lang/socket2/issues/268

*/
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModelViewBound {
    // cp code
    pub id: i64,
    pub short_code: String,
    pub short_name: String,
    pub country: String,
    pub exchange: String,
    // cp BoundLast
    pub code_id: i64,
    // cp
    #[schema(value_type = String)]
    pub cp_p1x: NaiveDate,
    #[schema(value_type = String)]
    pub cp_p1y: Decimal,
    #[schema(value_type = String)]
    pub cp_p2x: NaiveDate,
    #[schema(value_type = String)]
    pub cp_p2y: Decimal,
    pub cp_direction: String,
    pub cp_x_cnt: i32,
    #[schema(value_type = String)]
    pub cp_y_cnt: Decimal,
    #[schema(value_type = String)]
    pub cp_change_rate: Decimal,
    #[schema(value_type = String)]
    pub cp_ymxb_p3y: Decimal,
    #[schema(value_type = String)]
    pub cp_ymxb_m: Decimal,
    #[schema(value_type = String)]
    pub cp_ymxb_b: Decimal,
    //// op
    #[schema(value_type = String)]
    pub op_p1x: NaiveDate,
    #[schema(value_type = String)]
    pub op_p1y: Decimal,
    #[schema(value_type = String)]
    pub op_p2x: NaiveDate,
    #[schema(value_type = String)]
    pub op_p2y: Decimal,
    pub op_direction: String,
    pub op_x_cnt: i32,
    #[schema(value_type = String)]
    pub op_y_cnt: Decimal,
    #[schema(value_type = String)]
    pub op_change_rate: Decimal,
    #[schema(value_type = String)]
    pub op_ymxb_p3y: Decimal,
    #[schema(value_type = String)]
    pub op_ymxb_m: Decimal,
    #[schema(value_type = String)]
    pub op_ymxb_b: Decimal,

    //// lp
    #[schema(value_type = String)]
    pub lp_p1x: NaiveDate,
    #[schema(value_type = String)]
    pub lp_p1y: Decimal,
    #[schema(value_type = String)]
    pub lp_p2x: NaiveDate,
    #[schema(value_type = String)]
    pub lp_p2y: Decimal,
    pub lp_direction: String,
    pub lp_x_cnt: i32,
    #[schema(value_type = String)]
    pub lp_y_cnt: Decimal,
    #[schema(value_type = String)]
    pub lp_change_rate: Decimal,
    #[schema(value_type = String)]
    pub lp_ymxb_p3y: Decimal,
    #[schema(value_type = String)]
    pub lp_ymxb_m: Decimal,
    #[schema(value_type = String)]
    pub lp_ymxb_b: Decimal,
    //// hp
    #[schema(value_type = String)]
    pub hp_p1x: NaiveDate,
    #[schema(value_type = String)]
    pub hp_p1y: Decimal,
    #[schema(value_type = String)]
    pub hp_p2x: NaiveDate,
    #[schema(value_type = String)]
    pub hp_p2y: Decimal,
    pub hp_direction: String,
    pub hp_x_cnt: i32,
    #[schema(value_type = String)]
    pub hp_y_cnt: Decimal,
    #[schema(value_type = String)]
    pub hp_change_rate: Decimal,
    #[schema(value_type = String)]
    pub hp_ymxb_p3y: Decimal,
    #[schema(value_type = String)]
    pub hp_ymxb_m: Decimal,
    #[schema(value_type = String)]
    pub hp_ymxb_b: Decimal,
}

impl ModelViewBound {
    pub fn get(&self, price_type: &PriceType) -> Vec<String> {
        match price_type {
            PriceType::CLOSE => vec![
                self.cp_p1x.to_string(),
                self.cp_p1y.to_string(),
                self.cp_p2x.to_string(),
                self.cp_p2y.to_string(),
                self.cp_direction.to_string(),
                self.cp_x_cnt.to_string(),
                self.cp_y_cnt.to_string(),
                self.cp_change_rate.to_string(),
                self.cp_ymxb_p3y.to_string(),
                self.cp_ymxb_m.to_string(),
                self.cp_ymxb_b.to_string(),
            ],
            PriceType::OPEN => vec![
                self.op_p1x.to_string(),
                self.op_p1y.to_string(),
                self.op_p2x.to_string(),
                self.op_p2y.to_string(),
                self.op_direction.to_string(),
                self.op_x_cnt.to_string(),
                self.op_y_cnt.to_string(),
                self.op_change_rate.to_string(),
                self.op_ymxb_p3y.to_string(),
                self.op_ymxb_m.to_string(),
                self.op_ymxb_b.to_string(),
            ],
            PriceType::LOW => vec![
                self.lp_p1x.to_string(),
                self.lp_p1y.to_string(),
                self.lp_p2x.to_string(),
                self.lp_p2y.to_string(),
                self.lp_direction.to_string(),
                self.lp_x_cnt.to_string(),
                self.lp_y_cnt.to_string(),
                self.lp_change_rate.to_string(),
                self.lp_ymxb_p3y.to_string(),
                self.lp_ymxb_m.to_string(),
                self.lp_ymxb_b.to_string(),
            ],
            PriceType::HIGH => vec![
                self.hp_p1x.to_string(),
                self.hp_p1y.to_string(),
                self.hp_p2x.to_string(),
                self.hp_p2y.to_string(),
                self.hp_direction.to_string(),
                self.hp_x_cnt.to_string(),
                self.hp_y_cnt.to_string(),
                self.hp_change_rate.to_string(),
                self.hp_ymxb_p3y.to_string(),
                self.hp_ymxb_m.to_string(),
                self.hp_ymxb_b.to_string(),
            ],
        }
    }
}
