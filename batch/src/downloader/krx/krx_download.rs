use reqwest::{header, Client};

use crate::{
    downloader::krx::{
        krx_param_code_detail::KrxParamCodeDetail, krx_param_code_state::KrxParamCodeState,
        krx_param_price::KrxParamPrice, KrxToParam,
    },
    job::input_data::file::{fm, FileInfo},
};

#[derive(Debug, Clone)]
pub enum KrxDownloadType {
    CodeDetail,
    CodeState,
    Price,
}

#[derive(Debug, Clone)]
pub struct KrxDownload {
    pub(crate) download_type: KrxDownloadType,
}

impl KrxDownload {
    const KRX_CODE: &str = "https://data.krx.co.kr/comm/fileDn/GenerateOTP/generate.cmd";
    const KRX_FILE: &str = "https://data.krx.co.kr/comm/fileDn/download_excel/download.cmd";

    pub async fn download_code_detail() -> Result<(), Box<dyn std::error::Error>> {
        let item = KrxDownload {
            download_type: KrxDownloadType::CodeDetail,
        };
        let params = KrxParamCodeDetail {};
        item.request(&params).await?;

        Ok(())
    }
    pub async fn download_code_state() -> Result<(), Box<dyn std::error::Error>> {
        let item = KrxDownload {
            download_type: KrxDownloadType::CodeState,
        };
        let params = KrxParamCodeState {};
        item.request(&params).await?;
        Ok(())
    }
    pub async fn download_price(params: KrxParamPrice) -> Result<(), Box<dyn std::error::Error>> {
        let item = KrxDownload {
            download_type: KrxDownloadType::Price,
        };
        item.request(&params).await?;

        Ok(())
    }

    async fn request(&self, params: &impl KrxToParam) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<(header::HeaderName, header::HeaderValue)> = vec![
            (header::CONTENT_TYPE, header::HeaderValue::from_static("application/x-www-form-urlencoded"),),
            (header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36")),
        ];
        let fi = FileInfo::new();
        let save_file_name = match self.download_type {
            KrxDownloadType::CodeDetail => fi.kr_company_detail_full,
            KrxDownloadType::CodeState => fi.kr_company_state_full,
            KrxDownloadType::Price => {
                let item: &KrxParamPrice = params.as_any().downcast_ref().unwrap();
                item.get_save_file_name(fi.kr_price_dir)
            }
        };

        let headers: header::HeaderMap = data.into_iter().collect();

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let code = self.request_code(&client, params).await?;
        self.request_file(&client, save_file_name, code).await?;
        Ok(())
    }

    async fn request_code(
        &self,
        client: &Client,
        params: &impl KrxToParam,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // let params2 = params.to_param();
        // println!("==>parms==>{}", params2);
        let res = client
            .post(Self::KRX_CODE)
            .body(params.to_param())
            .send()
            .await?;
        let body = res.bytes().await?;
        let code = str::from_utf8(&body)?;

        Ok(code.to_string())
    }

    async fn request_file(
        &self,
        client: &Client,
        save_file_name: String,
        code: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let req_body = format!("code={}", code);
        let res = client.post(Self::KRX_FILE).body(req_body).send().await?;

        //println!("Response: {:?}", res);
        let body = res.bytes().await?;
        fm::write_all(&save_file_name, body.as_ref());

        Ok(())
    }
}

mod tests {

    use crate::{
        downloader::krx::{
            krx_download::{KrxDownload, KrxDownloadType},
            krx_param_code_detail::KrxParamCodeDetail,
            krx_param_code_state::KrxParamCodeState,
            krx_param_price::KrxParamPrice,
        },
        job::input_data::file::fm,
    };

    #[tokio::test]
    async fn it_works_download_mk_dir() {
        fm::mkdir();
        /////////////////////////////////
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn it_works_download_code_detail() {
        let item = KrxDownload {
            download_type: KrxDownloadType::CodeDetail,
        };
        let params = KrxParamCodeDetail {};
        item.request(&params).await.unwrap();

        /////////////////////////////////
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn it_works_download_code_state() {
        let item = KrxDownload {
            download_type: KrxDownloadType::CodeState,
        };
        let params = KrxParamCodeState {};
        item.request(&params).await.unwrap();

        /////////////////////////////////
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn it_works_download_price() {
        let item = KrxDownload {
            download_type: KrxDownloadType::Price,
        };
        let params = KrxParamPrice {
            batch_id: 999,
            std_code: "KR7001041003".to_string(),
            short_code: "001045".to_string(),
            short_name: "CJ우".to_string(),
            start: "20250824".to_string(),
            end: "20250924".to_string(),
        };
        item.request(&params).await.unwrap();

        /////////////////////////////////
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
