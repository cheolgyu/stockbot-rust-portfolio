use crate::{
    downloader::krx::krx_download::KrxDownload,
    job::input_data::file::FileInfo,
    utils::batch::{
        err::{ItemProcessorError, ItemReaderError, ItemWriterError},
        step::Step,
    },
};
use async_trait::async_trait;
use batch_model::{
    app_type::{country_type::CountryType, exchange_type::ExchangeType},
    code::AddCodeDetail,
};
use calamine::{open_workbook, Reader, Xlsx};
use sqlx::{Pool, Postgres, Row};
use std::{fs::File, io::Read, str::FromStr};
use tracing::info;

pub struct StepCodeDetail {
    pub data: Option<Vec<AddCodeDetail>>,
}

const UPDATE_QUERY: &str = "
UPDATE public.code_detail
SET std_code=$1, short_code=$2, std_name=$3, short_name=$4, eng_name=$5, listing_date=$6, exchange=$7, securities_type=$8, company_type=$9, stock_type=$10, face_value=$11, stock_cnt=$12
WHERE std_code=$1;";

const INSERT_CODE_DETAIL: &str = "
INSERT INTO public.code_detail(
code_id, std_code, short_code, std_name, short_name, 
eng_name, listing_date, exchange, securities_type, company_type, 
stock_type, face_value, stock_cnt)
VALUES (
    $1, $2, $3, $4, $5, 
    $6, $7, $8, $9, $10, 
    $11, $12, $13);";

const INSERT_CODE: &str = "
INSERT INTO public.code(
	 std_code, country, exchange, name, code)
	VALUES ($1, $2, $3, $4, $5) RETURNING id";

#[async_trait]
impl Step for StepCodeDetail {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        let download_code_detail: &str = &dotenvy::var("DOWNLOAD_ALLOW_CODE_DETAIL").unwrap();
        if download_code_detail == "true" {
            info!("다운로드 시작.StepCodeDetail = {} ", download_code_detail);
            KrxDownload::download_code_detail().await.unwrap();
        } else {
            info!(
                "다운로드 안해요. StepCodeDetail = {} ",
                download_code_detail
            );
        }

        Ok(())
    }

    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        self.read_excel();
        Ok(())
    }

    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        let mut insert_new_list: Vec<AddCodeDetail> = vec![];

        for v in self.data.as_mut().unwrap().iter() {
            //SET std_code=$1, short_code=$2, std_name=$3, short_name=$4, eng_name=$5,
            //listing_date=$6, exchange=$7, securities_type=$8, company_type=$9, stock_type=$10,
            //face_value=$11, stock_cnt=$12
            let rows_affected = sqlx::query(UPDATE_QUERY)
                .bind(&v.std_code)
                .bind(&v.short_code)
                .bind(&v.std_name)
                .bind(&v.short_name)
                .bind(&v.eng_name)
                //
                .bind(&v.listing_date)
                .bind(&v.exchange)
                .bind(&v.securities_type)
                .bind(&v.company_type)
                .bind(&v.stock_type)
                //
                .bind(&v.face_value)
                .bind(&v.stock_cnt)
                .execute(pool)
                .await
                .unwrap()
                .rows_affected();
            if rows_affected == 0 {
                insert_new_list.push(v.clone());
            }
        }

        for v in insert_new_list.iter() {
            let row = sqlx::query(INSERT_CODE)
                .bind(&v.std_code)
                .bind(CountryType::KR)
                .bind(&v.exchange)
                .bind(&v.short_name)
                .bind(&v.short_code)
                .fetch_one(pool)
                .await
                .unwrap();
            let id: i64 = row.try_get("id").unwrap();
            info!("새로운 CODE 생성 {:?},{:?}", id, &v.std_code);
            sqlx::query(INSERT_CODE_DETAIL)
                .bind(id)
                .bind(&v.std_code)
                .bind(&v.short_code)
                .bind(&v.std_name)
                .bind(&v.short_name)
                //
                .bind(&v.eng_name)
                .bind(&v.listing_date)
                .bind(&v.exchange)
                .bind(&v.securities_type)
                .bind(&v.company_type)
                //
                .bind(&v.stock_type)
                .bind(&v.face_value)
                .bind(&v.stock_cnt)
                .execute(pool)
                .await
                .map_err(|e| {
                    info!(" 오류발생 !! {:?}", e);
                    info!(" 오류발생 !! {:?}", v);
                })
                .unwrap();
        }

        Ok(())
    }
}
impl StepCodeDetail {
    fn read_excel(&mut self) {
        let mut parse_companys: Vec<AddCodeDetail> = vec![];
        let fi = FileInfo::new();
        let fnm = fi.kr_company_detail_full;
        let path = std::path::Path::new(&fnm);

        println!("fnm={:#?}", path);

        let mut excel: Xlsx<_> = open_workbook(path).expect("엑셀파일 열기 실패..;");
        if let Ok(r) = excel.worksheet_range("Sheet1") {
            let mut i = 0;
            for row in r.rows() {
                let std_code = row[0].to_string();
                //println!("row={:?}, row[0]={:?}", row, row[0]);
                if i != 0 {
                    let cmp = AddCodeDetail {
                        //code_id: (),
                        std_code: row[0].to_string(),
                        short_code: row[1].to_string(),
                        std_name: row[2].to_string(),
                        short_name: row[3].to_string(),
                        eng_name: row[4].to_string(),
                        listing_date: row[5].to_string(),
                        exchange: ExchangeType::from_str(&row[6].to_string()).unwrap(),
                        securities_type: row[7].to_string(),
                        company_type: row[8].to_string(),
                        stock_type: row[9].to_string(),
                        face_value: row[10].to_string(),
                        stock_cnt: row[11].to_string(),
                    };

                    parse_companys.push(cmp);
                }

                i = i + 1;
            }
        }
        self.data = Some(parse_companys);
    }
}

mod tests {
    use std::{fs::File, io::Read, str::FromStr};

    use batch_model::{app_type::exchange_type::ExchangeType, code::AddCodeDetail};
    use calamine::{open_workbook, Reader, Xlsx};

    use crate::job::input_data::{file::FileInfo, setp_code_detail::StepCodeDetail};

    #[tokio::test]
    async fn check_dir() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let workspace_root = std::path::Path::new(manifest_dir)
            .parent()
            .expect("Failed to get parent directory");
        println!("Workspace root: {:?}", workspace_root);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn read_file_test() {
        /*
        vscode 로 batch/main.rs 열어 main 함수 위에 run 버튼 생기고
        run 버튼 누르면 오류가 나옴.

        run test에서는 잘됨.
        파일경로가 절대 경로가 아니여서 그런듯.

         */
        let fi = FileInfo::new();
        let fnm2 = format!("{}/test.txt", fi.kr_company_dir);
        let mut f = File::open(fnm2).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Something went wrong reading the file");
        println!("contents={}", contents);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn read_excel_test() {
        let mut scd = StepCodeDetail { data: None };

        scd.read_excel();
        let mut i = 0;
        for f in scd.data.unwrap() {
            if i > 1 {
                break;
            }
            println!("f={:#?}", f);
            i += 1;
        }

        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn excel_test() {
        let mut parse_companys: Vec<AddCodeDetail> = vec![];
        let fi = FileInfo::new();

        let fnm = fi.kr_company_detail_full;
        let mut excel: Xlsx<_> = open_workbook(fnm).unwrap();
        if let Ok(r) = excel.worksheet_range("Sheet1") {
            let mut i = 0;
            for row in r.rows() {
                let std_code = row[0].to_string();
                //println!("row={:?}, row[0]={:?}", row, row[0]);
                if i != 0 {
                    let cmp = AddCodeDetail {
                        //code_id: (),
                        std_code: row[0].to_string(),
                        short_code: row[1].to_string(),
                        std_name: row[2].to_string(),
                        short_name: row[3].to_string(),
                        eng_name: row[4].to_string(),
                        listing_date: row[5].to_string(),
                        exchange: ExchangeType::from_str(&row[6].to_string()).unwrap(),
                        securities_type: row[7].to_string(),
                        company_type: row[8].to_string(),
                        stock_type: row[9].to_string(),
                        face_value: row[10].to_string(),
                        stock_cnt: row[11].to_string(),
                    };

                    parse_companys.push(cmp);
                }

                i = i + 1;
            }
        }

        let mut i = 0;
        for f in parse_companys {
            if i > 1 {
                break;
            }
            println!("f={:#?}", f);
            i += 1;
        }

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
