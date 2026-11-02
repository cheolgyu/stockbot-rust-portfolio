use crate::{
    downloader::krx::krx_download::KrxDownload,
    utils::batch::{
        err::{ItemProcessorError, ItemReaderError, ItemWriterError},
        step::Step,
    },
};
use async_trait::async_trait;
use batch_model::code::{AddCodeState, CodeState};
use calamine::{open_workbook, Reader, Xlsx};
use sqlx::{Pool, Postgres, Row};
use std::collections::HashMap;
use tracing::info;

use super::file::FileInfo;
pub struct StepCodeState {
    pub data: Option<Vec<AddCodeState>>,
}

const UPSERT_CODE_STATE: &str = "
INSERT INTO public.code_state(
	code_id, short_code, stop, clear, manger, 
    alert, unfaithful,  low_liquidity, shortage,  overheated, 
    caution, warning, risk)
	VALUES (
        $1, $2, $3, $4, $5, 
        $6, $7, $8, $9, $10, 
        $11, $12, $13
    ) ON CONFLICT (code_id) 
    DO UPDATE SET 
    -- code_id=$1, 
              short_code=$2, stop=$3, clear=$4, manger=$5, 
    alert=$6, unfaithful=$7, low_liquidity=$8, shortage=$9, overheated=$10, 
    caution=$11, warning=$12, risk=$13
    ;
";

const SELECT_CODE: &str = "select c.id, c.std_code, d.short_code  from code c left join code_detail d on c.id = d.code_id where c. country='KR';";

#[async_trait]
impl Step for StepCodeState {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        let download_code_state: &str = &dotenvy::var("DOWNLOAD_ALLOW_CODE_STATE").unwrap();
        if download_code_state == "true" {
            info!("code_state-다운로드 시작. = {} ", download_code_state);
            KrxDownload::download_code_state().await.unwrap();
        } else {
            info!("code_state-다운로드 안해요. = {} ", download_code_state);
        }

        Ok(())
    }

    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        let mut parse_companys: Vec<AddCodeState> = vec![];
        let fi = FileInfo::new();
        let fnm = fi.kr_company_state_full;
        let path = std::path::Path::new(&fnm);
        println!("fnm={:#?}", path);

        let mut excel: Xlsx<_> = open_workbook(path).unwrap();
        if let Ok(r) = excel.worksheet_range("Sheet1") {
            let mut i = 0;
            for row in r.rows() {
                let std_code: String = row[0].to_string();
                //println!("row={:?}, row[0]={:?}", row, row[0]);
                if i != 0 {
                    let cmp = AddCodeState {
                        code_id: None,
                        short_code: row[0].to_string(),
                        //code_nm : : row[1].to_string(),
                        stop: row[2].to_string(),
                        clear: row[3].to_string(),
                        manger: row[4].to_string(),
                        alert: row[5].to_string(),
                        unfaithful: row[6].to_string(),
                        low_liquidity: row[7].to_string(),
                        shortage: row[8].to_string(),
                        overheated: row[9].to_string(),
                        caution: row[10].to_string(),
                        warning: row[11].to_string(),
                        risk: row[12].to_string(),
                    };

                    parse_companys.push(cmp);
                }

                i = i + 1;
            }
        }
        self.data = Some(parse_companys);
        Ok(())
    }

    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        let mut upsert_items: Vec<CodeState> = vec![];

        let res = sqlx::query(SELECT_CODE).fetch_all(pool).await.unwrap();

        let mut code_map: HashMap<String, i64> = HashMap::new();
        for r in res {
            let id = r.try_get("id").unwrap();
            let short_code = r.try_get("short_code").unwrap();
            code_map.insert(short_code, id);
        }

        let list = self.data.clone().unwrap();
        for item in list.iter() {
            let k = item.short_code.clone();
            let cs = item.into_code_state(*code_map.get(&k).unwrap());
            upsert_items.push(cs);
        }

        for v in upsert_items.iter() {
            sqlx::query(UPSERT_CODE_STATE)
                .bind(v.code_id)
                .bind(v.short_code.clone())
                .bind(v.stop)
                .bind(v.clear)
                .bind(v.manger)
                .bind(v.alert)
                .bind(v.unfaithful)
                .bind(v.low_liquidity)
                .bind(v.shortage)
                .bind(v.overheated)
                .bind(v.caution)
                .bind(v.warning)
                .bind(v.risk)
                .execute(pool)
                .await
                .unwrap();
        }

        Ok(())
    }
}
