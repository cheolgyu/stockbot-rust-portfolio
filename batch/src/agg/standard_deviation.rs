use std::collections::HashMap;

use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

use crate::agg::{ObsValue, Year};

#[derive(Default, Clone, Debug)]
pub struct StandardDeviation {
    // k: 연도 ,v: 관측값: 거래량 최대인 월
    pub data: HashMap<Year, ObsValue>,
    // data 수
    pub count: usize,
    // 평균
    pub average: Decimal,
    // 편차
    pub deviation: Vec<Decimal>,
    // 분산
    pub variance: Decimal,
    //편차 제곱
    pub square_root_deviation: Vec<Decimal>,
    // 표준편차
    pub sd: Decimal,
    // 합
    pub sum: Decimal,
}
impl StandardDeviation {
    const DP: u32 = 2;

    pub fn calculate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.count = self.data.len();
        // 합
        for (_, v) in &self.data {
            self.sum = self.sum + v.0
        }
        // 평균
        self.average = self
            .sum
            .checked_div(self.count.into())
            .unwrap()
            .round_dp(Self::DP);
        // 편차, 편차 제곱
        for (_, v) in &self.data {
            let deviation = v.0.checked_sub(self.average).unwrap();
            self.deviation.push(deviation);
            self.square_root_deviation.push(deviation * deviation);
        }

        // 분산
        let mut sum = dec!(0);
        for v in &self.square_root_deviation {
            sum = sum + v;
        }
        self.variance = sum
            .checked_div(self.count.into())
            .unwrap()
            .round_dp(Self::DP);
        // 표준편차
        self.sd = self.variance.sqrt().unwrap().round_dp(Self::DP);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    use crate::agg::{standard_deviation::StandardDeviation, ObsValue, Year};

    #[test]
    fn it_works() {
        let mut sd = StandardDeviation::default();
        let mut data: HashMap<Year, ObsValue> = HashMap::new();
        data.insert(Year(1), ObsValue(dec!(-10)));
        data.insert(Year(2), ObsValue(dec!(-5)));
        data.insert(Year(3), ObsValue(dec!(0)));
        data.insert(Year(4), ObsValue(dec!(5)));
        data.insert(Year(5), ObsValue(dec!(10)));

        sd.data = data;
        sd.calculate().unwrap();

        println!("{:#?}", sd);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
