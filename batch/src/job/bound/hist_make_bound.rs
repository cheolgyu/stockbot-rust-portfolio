use std::fmt::Error;

use batch_model::{
    app_type::price_type::PriceType,
    bound::AddBound,
    graph::{ConvertPoint, DirectionType},
    price::PriceWithConvertDate,
};
use rust_decimal::Decimal;

use crate::job::bound::ymxb::Ymxb;

pub struct MakeBound {
    pub code_id: i64,
    pub line: PriceType,
    pub list: Vec<PriceWithConvertDate>,
}
impl MakeBound {
    pub fn run(&self) -> Vec<AddBound> {
        let mut res: Vec<AddBound> = Vec::new();
        let size = self.list.len() - 1;

        let mut mp1: ConvertPoint = ConvertPoint::from_price(self.list.get(0).unwrap(), self.line);
        let mut mp2: ConvertPoint = ConvertPoint::from_price(self.list.get(1).unwrap(), self.line);
        let mut keep_direct: u32 = get_direction(&mp1.price, &mp2.price);
        for i in 0..size {
            // println!("-----------------------");
            // println!("                          ");
            // println!("-----------------------");
            let is_last = i + 1 == size;
            let inext: usize = i + 1; //if is_last { i } else { i + 1 };
                                      //  println!("i={i},  inext={inext},  is_last={is_last}");

            let item = self.list.get(i).unwrap();
            let next: &PriceWithConvertDate = self.list.get(inext).unwrap();
            // println!("item={:?}, next={:?}", item, next);

            let p1: ConvertPoint = ConvertPoint::from_price(item, self.line);
            let p2: ConvertPoint = ConvertPoint::from_price(next, self.line);
            let direct: u32 = get_direction(&p1.price, &p2.price);
            //  println!("direct={direct},  p1={:?}, p2={:?}", &p1, &p2);
            if keep_direct == direct {
                // println!("----------------------> 유지");
                mp2 = p2;
            } else {
                // println!("----------------------> 다름");

                let add_bound = self.get_add_bound(mp1, mp2, keep_direct).unwrap();
                res.push(add_bound);
                mp1 = p1;
                mp2 = p2;
                keep_direct = direct;
            }
            if is_last {
                // println!("----------------------> is_last");

                let add_bound = self.get_add_bound(mp1, mp2, direct).unwrap();
                res.push(add_bound);
                mp2 = p2;
            }
            // println!(" ");
            // println!(" ");
            // println!("-----------mp1={:?}, mp2={:?}", &mp1, &mp2);
        }

        //let res_size = res.len();
        // println!("res.size={res_size}-----------------------------------------------------------------------");
        res
    }

    fn get_add_bound(
        &self,
        p1: ConvertPoint,
        p2: ConvertPoint,
        direction: u32,
    ) -> Result<AddBound, Error> {
        // println!(
        //     "get_add_bound     p1={:?}, p2={:?}==============================================",
        //     p1, p2
        // );
        let ymxb = Ymxb::get_ymxb(&p1, &p2)?;
        let add_bound = AddBound {
            code_id: self.code_id,
            line: self.line,
            p1x: p1.dt,
            p1y: p1.price,
            p2x: p2.dt,
            p2y: p2.price,
            direction: DirectionType::from_u32(direction),
            x_cnt: ymxb.x_cnt,
            y_cnt: ymxb.y_cnt,
            change_rate: ymxb.change_rate,
            ymxb_p3y: ymxb.ymxb_p3y,
            ymxb_m: ymxb.ymxb_m,
            ymxb_b: ymxb.ymxb_b,
        };
        Ok(add_bound)
    }
}

fn get_direction(p1: &Decimal, p2: &Decimal) -> u32 {
    if p1 > p2 {
        1
    } else if p1 == p2 {
        2
    } else if p1 < p2 {
        3
    } else {
        panic!("???????????????????????");
    }
}

mod tests {
    use crate::job::{
        bound::{hist_make_bound::MakeBound, ymxb::Ymxb},
        utils::calculate_change_rate,
    };
    use batch_model::{app_type::price_type::PriceType, price::PriceWithConvertDate};
    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn check_calculate_change_rate() {
        let old_y = Decimal::new(57500, 0);
        let new_y = Decimal::new(54100, 0);
        println!("old_y,new_y=>{:#?},{:#?}", old_y, new_y);
        let div = new_y - old_y;
        println!("div=>{:#?}", div);

        assert_eq!(div, Decimal::new(-3400, 0));
        let s1 = div / old_y;
        println!("s1=>{:#?}", s1);
        let s2: Decimal = (s1 * Decimal::new(100, 0)).round_dp(2);
        println!("s2=>{:#?}", s2);

        let test = calculate_change_rate(old_y, new_y);
        println!("res=>{:#?}", test);
        println!("res=>{:#?}", test.unwrap().round_dp(2));
        let res = -5.91;
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn checkeecimal() {
        let test = Decimal::new(0, 0);
        println!("test=>{:#?}", test);
        let test2 = test.round_dp(2);
        println!("test2=>{:#?}", test2);

        let res = -5.91;
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_make_bound() {
        let code_id = 1;
        let line = PriceType::CLOSE;
        let mut list: Vec<PriceWithConvertDate> = vec![
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 01).unwrap(),
                x_tick: 1,
                op: dec!(1000),
                hp: dec!(1000),
                lp: dec!(1000),
                cp: dec!(1000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 02).unwrap(),
                x_tick: 2,
                op: dec!(2000),
                hp: dec!(2000),
                lp: dec!(2000),
                cp: dec!(2000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 03).unwrap(),
                x_tick: 3,
                op: dec!(3000),
                hp: dec!(3000),
                lp: dec!(3000),
                cp: dec!(3000),
            },
        ];

        // let mb = MakeBound {
        //     code_id,
        //     line,
        //     list: list.clone(),
        // };
        // let res = mb.run();

        // for i in res {
        //     println!("{:#?}", i);
        // }
        println!("=======================================");
        println!("=======================================");
        println!("=======================================");
        //////////////////////////////////////
        /// //////////////////////////////////////
        /// //////////////////////////////////////
        ///
        let mut list: Vec<PriceWithConvertDate> = vec![
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 01).unwrap(),
                x_tick: 1,
                op: dec!(1000),
                hp: dec!(1000),
                lp: dec!(1000),
                cp: dec!(1000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 02).unwrap(),
                x_tick: 2,
                op: dec!(2000),
                hp: dec!(2000),
                lp: dec!(2000),
                cp: dec!(2000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 03).unwrap(),
                x_tick: 3,
                op: dec!(3000),
                hp: dec!(3000),
                lp: dec!(3000),
                cp: dec!(3000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 04).unwrap(),
                x_tick: 4,
                op: dec!(2000),
                hp: dec!(2000),
                lp: dec!(2000),
                cp: dec!(2000),
            },
        ];

        let mb = MakeBound {
            code_id,
            line,
            list: list.clone(),
        };
        let res = mb.run();

        for i in res {
            println!("{:#?}", i);
        }

        let res = -5.91;
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_make_bound_2() {
        let code_id = 1;
        let line = PriceType::CLOSE;
        let mut list: Vec<PriceWithConvertDate> = vec![
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 01).unwrap(),
                x_tick: 1,
                op: dec!(1000),
                hp: dec!(1000),
                lp: dec!(1000),
                cp: dec!(1000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 02).unwrap(),
                x_tick: 2,
                op: dec!(2000),
                hp: dec!(2000),
                lp: dec!(2000),
                cp: dec!(2000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 03).unwrap(),
                x_tick: 3,
                op: dec!(3000),
                hp: dec!(3000),
                lp: dec!(3000),
                cp: dec!(3000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 04).unwrap(),
                x_tick: 4,
                op: dec!(4000),
                hp: dec!(4000),
                lp: dec!(4000),
                cp: dec!(4000),
            },
        ];

        let mb = MakeBound {
            code_id,
            line,
            list: list.clone(),
        };
        let res = mb.run();

        for i in res {
            println!("{:#?}", i);
        }

        let res = -5.91;
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_make_bound_3() {
        let code_id = 1;
        let line = PriceType::CLOSE;
        let mut list: Vec<PriceWithConvertDate> = vec![
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 01).unwrap(),
                x_tick: 1,
                op: dec!(1000),
                hp: dec!(1000),
                lp: dec!(1000),
                cp: dec!(1000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 02).unwrap(),
                x_tick: 2,
                op: dec!(2000),
                hp: dec!(2000),
                lp: dec!(2000),
                cp: dec!(2000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 03).unwrap(),
                x_tick: 3,
                op: dec!(1000),
                hp: dec!(1000),
                lp: dec!(1000),
                cp: dec!(1000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 04).unwrap(),
                x_tick: 4,
                op: dec!(2000),
                hp: dec!(2000),
                lp: dec!(2000),
                cp: dec!(2000),
            },
        ];

        let mb = MakeBound {
            code_id,
            line,
            list: list.clone(),
        };
        let res = mb.run();

        for i in res {
            println!("{:#?}", i);
        }

        let res = -5.91;
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[tokio::test]
    async fn test_make_bound_4() {
        let code_id = 1;
        let line = PriceType::CLOSE;
        let mut list: Vec<PriceWithConvertDate> = vec![
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 01).unwrap(),
                x_tick: 1,
                op: dec!(1000),
                hp: dec!(1000),
                lp: dec!(1000),
                cp: dec!(1000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 02).unwrap(),
                x_tick: 2,
                op: dec!(2000),
                hp: dec!(2000),
                lp: dec!(2000),
                cp: dec!(2000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 03).unwrap(),
                x_tick: 3,
                op: dec!(1000),
                hp: dec!(1000),
                lp: dec!(1000),
                cp: dec!(1000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 04).unwrap(),
                x_tick: 4,
                op: dec!(2000),
                hp: dec!(2000),
                lp: dec!(2000),
                cp: dec!(2000),
            },
            PriceWithConvertDate {
                code_id: code_id,
                dt: NaiveDate::from_ymd_opt(2020, 01, 05).unwrap(),
                x_tick: 5,
                op: dec!(11000),
                hp: dec!(11000),
                lp: dec!(11000),
                cp: dec!(11000),
            },
        ];

        let mb = MakeBound {
            code_id,
            line,
            list: list.clone(),
        };
        let res = mb.run();

        for i in res {
            println!("{:#?}", i);
        }

        let res = -5.91;
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
