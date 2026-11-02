// https://m.stock.naver.com/front-api/external/chart/domestic/info?symbol=454910&requestType=1&startTime=20240827&endTime=20251104&timeframe=day
#[derive(Debug, Clone)]
pub struct NaverParamPrice {
    pub batch_id: i64,
    pub code_id: i64,
    pub short_code: String,
    pub start_time: String,
    pub end_time: String,
}

impl NaverParamPrice {
    
}
