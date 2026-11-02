use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::{
    agg::{AggVolFreqDataItem, ObsValue, Year},
    job::utils::calculate_rate,
};

#[derive(Default, Clone, Debug)]
pub struct FrequencyAnalysis {
    // k: 연도 ,v: 관측값: 거래량 최대인 월
    pub data: HashMap<Year, ObsValue>,
    pub all_cnt: usize,
    pub best: AggVolFreqDataItem,
    pub res: Vec<AggVolFreqDataItem>,
}
impl FrequencyAnalysis {
    pub fn calculate(&mut self) {
        let size: usize = self.data.len();
        self.all_cnt = size;

        let mut tmp: HashMap<ObsValue, i32> = HashMap::new();

        let mut res: Vec<AggVolFreqDataItem> = vec![];
        for v in &self.data {
            let ov = v.1.clone();
            *tmp.entry(ov).or_insert(0) += 1;
        }

        for v in tmp {
            let rate = calculate_rate(size.into(), v.1.into()).unwrap();
            res.push(AggVolFreqDataItem {
                val: v.0 .0,
                cnt: v.1.into(),
                rate: rate.round_dp(2),
            });
        }

        res.sort_by_key(|k| k.rate);
        res.reverse();
        self.best = res[0].clone();
        self.res = res;
    }
}
#[derive(Default, Clone, Debug)]
pub struct FrequencyItem {
    // 키워드
    pub keyword: ObsValue,
    // 빈도수
    pub freq_cnt: usize,
    // 빈도수 비중
    pub rate: Decimal,
}

mod tests {
    use crate::agg::{AggVolFreqDataItem, ObsValue, Year};
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use std::collections::HashMap;

    use crate::agg::{
        frequency_analysis::FrequencyAnalysis, standard_deviation::StandardDeviation,
    };

    #[test]
    fn it_works() {
        let mut fa = FrequencyAnalysis::default();
        let mut data: HashMap<Year, ObsValue> = HashMap::new();
        data.insert(Year(2021), ObsValue(dec!(1)));
        data.insert(Year(2022), ObsValue(dec!(1)));
        data.insert(Year(2023), ObsValue(dec!(1)));
        data.insert(Year(2024), ObsValue(dec!(2)));
        data.insert(Year(2025), ObsValue(dec!(2)));
        data.insert(Year(2026), ObsValue(dec!(3)));
        //data.insert(Year(6), ObsValue(dec!(1)));

        fa.data = data;
        fa.calculate();

        println!("{:#?}", fa);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
