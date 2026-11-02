use std::io::Error;

use bytes::Bytes;
use reqwest::{Client, StatusCode};
use tracing::info;

use crate::{downloader::naver::naver_param_price::NaverParamPrice, job::input_data::file::fm};

#[derive(Debug, Clone)]
pub struct NaverDownlad {
    pub param: NaverParamPrice,
    pub fnm: String,
}
impl NaverDownlad {
    pub async fn request_save_file(&self) -> Result<(), Error> {
        let fnm = &self.fnm;
        let url = self.to_url();
        Self::request(fnm, url).await;

        Ok(())
    }

    async fn request(fnm: &str, url: String) {
        let client: Client = Client::new();

        let mut res = client
            .get(&url) // <- Create request builder
            .send() // <- Send http request
            .await
            .unwrap();

        if res.status() == StatusCode::OK {
            let body: Bytes = res.bytes().await.unwrap();
            fm::write_all(fnm, body.as_ref());
        } else {
            info!("네이버 차트 실패!!: url=>{:?}", url);
            info!("네이버 차트 실패!!: resp=>{:?}", res);
            panic!("네이버 차트 실패!!: resp.status() = {}", res.status())
        }
    }
    pub fn to_url(&self) -> String {
        let short_code = &self.param.short_code;
        let start_time = &self.param.start_time;
        let end_time = &self.param.end_time;

        format!("https://m.stock.naver.com/front-api/external/chart/domestic/info?symbol={short_code}&requestType=1&startTime={start_time}&endTime={end_time}&timeframe=day")
    }
}
