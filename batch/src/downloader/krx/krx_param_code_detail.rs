use crate::downloader::krx::KrxToParam;

#[derive(Debug, Clone)]
pub struct KrxParamCodeDetail {}
impl KrxToParam for KrxParamCodeDetail {
    fn to_param(&self) -> String {
        "mktId=ALL&share=1&csvxls_isNo=false&name=fileDown&url=dbms/MDC/STAT/standard/MDCSTAT01901"
            .to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
