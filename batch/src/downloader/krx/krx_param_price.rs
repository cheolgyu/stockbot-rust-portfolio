use crate::downloader::krx::KrxToParam;

#[derive(Debug, Clone)]
pub struct KrxParamPrice {
    pub batch_id: i64,
    pub std_code: String,
    pub short_code: String,
    pub short_name: String,
    pub start: String,
    pub end: String,
}
impl KrxParamPrice {
    pub fn get_save_file_name(&self, kr_price_dir: String) -> String {
        let batch_id = self.batch_id;
        let short_code = &self.short_code;
        let fnm = format!("{}~{}.xlsx", self.start, self.end);
        format!("{}/{batch_id}/{short_code}/{fnm}", kr_price_dir,)
    }
}
impl KrxToParam for KrxParamPrice {
    //locale=ko_KR&tboxisuCd_finder_stkisu0_1=37550L/DL이앤씨2우(전환)&isuCd=KR737550K029&isuCd2=KR7005930003&codeNmisuCd_finder_stkisu0_1=DL이앤씨2우(전환)&param1isuCd_finder_stkisu0_1=ALL&strtDd=20250916&endDd=20250924&adjStkPrc_check=Y&adjStkPrc=2&share=1&money=1&csvxls_isNo=false&name=fileDown&url=dbms/MDC/STAT/standard/MDCSTAT01701
    //locale=ko_KR&tboxisuCd_finder_stkisu0_1={}&isuCd={}&isuCd2=KR7005930003&codeNmisuCd_finder_stkisu0_1={}&param1isuCd_finder_stkisu0_1=ALL&strtDd={}&endDd={}&adjStkPrc_check=Y&adjStkPrc=2&share=1&money=1&csvxls_isNo=false&name=fileDown&url=dbms/MDC/STAT/standard/MDCSTAT01701
    fn to_param(&self) -> String {
        let p1 = format!("{}/{}", self.short_code, self.short_name);
        let p2 = self.std_code.clone();
        let p3 = self.short_name.clone();
        let p4 = self.start.clone();
        let p5 = self.end.clone();

        format!("locale=ko_KR&tboxisuCd_finder_stkisu0_1={p1}&isuCd={p2}&isuCd2=KR7005930003&codeNmisuCd_finder_stkisu0_1={p3}&param1isuCd_finder_stkisu0_1=ALL&strtDd={p4}&endDd={p5}&adjStkPrc_check=Y&adjStkPrc=2&share=1&money=1&csvxls_isNo=false&name=fileDown&url=dbms/MDC/STAT/standard/MDCSTAT01701").to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
