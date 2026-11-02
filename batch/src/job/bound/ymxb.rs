use std::error::Error;

use batch_model::graph::ConvertPoint;
use rust_decimal::Decimal;
use tracing::info;

use crate::job::utils::calculate_change_rate;

pub struct Ymxb {
    pub x_cnt: Decimal,
    pub y_cnt: Decimal,
    pub change_rate: Decimal,

    pub ymxb_p3y: Decimal,
    pub ymxb_m: Decimal,
    pub ymxb_b: Decimal,
}

impl Ymxb {
    pub fn get_ymxb(p1: &ConvertPoint, p2: &ConvertPoint) -> Result<Self, std::fmt::Error> {
        /*

            m: 기울기 구하기
                t2.y - t1.y
            m = ------------
                t2.x - t1.x
        */
        let p1_x_tick = Decimal::new(p1.x_tick, 0);
        let p2_x_tick = Decimal::new(p2.x_tick, 0);

        let cp1: (Decimal, Decimal) = (p1_x_tick, p1.price);
        let cp2: (Decimal, Decimal) = (p2_x_tick, p2.price);

        let m: Decimal = Self::calculate_gradient(cp1, cp2)?;

        /*
            b: y절편 구하기
            y=mx+b
            y1 =m*x1+b
            b= y1-m*x1
        */
        let b = p1.price - (m * p1_x_tick);

        /*

            p3_y 구하기
            y = m*(t2+1).x + b
        */
        let next_y = m * (p1_x_tick + Decimal::ONE) + b;

        let change_rate = calculate_change_rate(p1.price, p2.price)?;

        Ok(Ymxb {
            x_cnt: p2_x_tick - p1_x_tick,
            y_cnt: p2.price - p1.price,
            change_rate: change_rate,
            ymxb_p3y: next_y,
            ymxb_m: m,
            ymxb_b: b,
        })
    }
    fn calculate_gradient(
        p1: (Decimal, Decimal),
        p2: (Decimal, Decimal),
    ) -> Result<Decimal, std::fmt::Error> {
        let change_y = p2.1 - p1.1;
        let change_x = p2.0 - p1.0;
        let res = change_y / change_x;
        Ok(res)
    }
}

mod tests {
    use rust_decimal::Decimal;

    use crate::job::bound::ymxb::Ymxb;

    #[tokio::test]
    async fn check_calculate_gradient() {
        let p1: (Decimal, Decimal) = (2.into(), (-9).into());
        let p2: (Decimal, Decimal) = ((-1).into(), 6.into());

        let res = Ymxb::calculate_gradient(p1, p2);
        println!("res=>{:#?}", res);
    }
}
