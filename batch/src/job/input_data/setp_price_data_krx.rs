use std::vec;

use crate::{
    downloader::krx::{krx_download::KrxDownload, krx_param_price::KrxParamPrice},
    job::utils::get_default_start_dt,
    utils::batch::{
        err::{ItemProcessorError, ItemReaderError, ItemWriterError},
        step::Step,
    },
};
use async_channel::Receiver;
use async_trait::async_trait;
use batch_model::{
    app_type::country_type::CountryType,
    price::{AddPrice, AddPriceXlsx, PriceStartDate},
};
use calamine::{open_workbook, Reader, Xlsx};
use chrono::{DateTime, Days, NaiveDate, Utc};
use chrono_tz::{Asia::Seoul, Tz};

use reqwest::Client;
use sqlx::{query, Executor, Pool, Postgres};
use tokio::task::JoinHandle;
use tracing::info;

use super::file::FileInfo;

pub struct StepPirce {
    pub batch_id: i64,
    pub country: CountryType,
    pub code_start_dt: Vec<PriceStartDate>,
    pub end_dt: Option<NaiveDate>,
}
const SELECT_START_DT: &str = "
select c.id, c.code, c.name, c.std_code , coalesce(max(p.dt), $1) as start_dt 
from code c  left join hist.price p on c.id = p.code_id 
where c.country='KR' group by c.id  ";

#[derive(Debug, Clone)]
pub enum Command {
    Set {
        code_id: i64,
        save_file: Vec<String>,
    },
}

#[async_trait]
impl Step for StepPirce {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        println!("종목의 가격 조회하기- 시작일, 종료일, 종목코드 구하기");

        let search_start_dt_init = get_default_start_dt().unwrap();
        let seoul_now: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
        let search_end_dt = seoul_now.naive_utc().date();
        let rows = sqlx::query_as::<_, PriceStartDate>(SELECT_START_DT)
            .bind(search_start_dt_init)
            .fetch_all(pool)
            .await
            .unwrap();

        //info!("start_dt={:?}", start_dt);
        self.code_start_dt = rows;
        self.end_dt = Some(search_end_dt);
        println!(
            "종목의 가격 조회하기- 시작일, 종료일, 종목코드 구하기 완료 수{:#?}",
            &self.code_start_dt.len()
        );
        Ok(())
    }

    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        pool.execute(" call hist.insert_price_chg(); ")
            .await
            .unwrap();

        Ok(())
    }

    /*
        make codes_insert
    */
    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        let allow_download = if dotenvy::var("DOWNLOAD_ALLOW_PRICE").unwrap() == "true" {
            true
        } else {
            false
        };
        //////////////////////////////////////////////////////
        let cap = 16;
        let (tx, rx) = async_channel::bounded::<Command>(cap);
        let loop_size = self.code_start_dt.len();
        //////////////////
        let mut i = 0;
        let items = self.code_start_dt.clone();
        let end_dt = self.end_dt.unwrap();
        let end = end_dt.format("%Y%m%d").to_string();
        println!("setp price count :{:#?}", items.len());
        let client: Client = Client::new();
        // 최대 다운로드 기간
        let max_search_period = chrono::TimeDelta::days(365 * 2 - 1).num_days();
        let batch_id = self.batch_id;

        let s1 = tokio::spawn(async move {
            let fi = FileInfo::new();

            for item in items {
                let start_dt = item.start_dt;
                if start_dt == end_dt {
                    continue;
                }
                let mut save_file: Vec<String> = Vec::new();
                let period = search_between(max_search_period, start_dt, end_dt);
                let mut save_file: Vec<String> = Vec::new();
                for v in period {
                    let krx_param_price = KrxParamPrice {
                        batch_id: batch_id,
                        std_code: item.std_code.clone(),
                        short_code: item.code.clone(),
                        short_name: item.name.clone(),
                        start: v.0.format("%Y%m%d").to_string(),
                        end: v.1.format("%Y%m%d").to_string(),
                    };
                    let fnm = krx_param_price.get_save_file_name(fi.kr_price_dir.clone());

                    if allow_download {
                        KrxDownload::download_price(krx_param_price).await.unwrap();
                    }
                    save_file.push(fnm);
                }

                println!(" save_file.len()={}", save_file.len());

                let cmd = Command::Set {
                    code_id: item.id.clone(),
                    save_file: save_file,
                };

                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    match tx_clone.send(cmd).await {
                        Ok(_) => {
                            // println!("sender: i={}, send-{:#?}", i, &item)
                        }
                        Err(err) => {
                            //println!("작업종료send={:?}", err);
                            if tx_clone.is_empty() {
                                tx_clone.close();
                            }
                        }
                    };
                });
                i += 1;
            }
        });
        /*
            ======================================
                        recviver
            ======================================
        */
        let recv_size: usize = if (loop_size / 100) == 0 {
            1
        } else {
            loop_size / 100
        };
        let mut recv_manager: Vec<JoinHandle<()>> = Vec::new();
        for n in 0..recv_size {
            //println!("starting rx_name={}", n);
            recv_manager.push(recv(n, rx.clone(), pool).await);
        }

        s1.await.unwrap();

        for r in recv_manager {
            r.await.unwrap();
        }

        Ok(())
    }
}
pub async fn recv(rx_name: usize, rx: Receiver<Command>, pool: &Pool<Postgres>) -> JoinHandle<()> {
    let pool = pool.clone();
    tokio::spawn(async move {
        loop {
            let res = rx.recv().await;
            match res {
                Ok(cmd) => match cmd {
                    Command::Set { code_id, save_file } => {
                        let read_file_cnt = save_file.len();
                        info!(
                            "rx_name={}: ,code_id:{:?}, read_file_size={}",
                            &rx_name, &code_id, read_file_cnt
                        );
                        if read_file_cnt > 0 {
                            for i in 0..read_file_cnt {
                                let li = read_excel(code_id, save_file.get(i).unwrap().to_string());
                                if li.len() > 0 {
                                    save(&pool, li).await.unwrap();
                                }
                            }

                            // println!("saved: rx_name={}: ,code:{:?}, len={}", rx_name, key, size);
                        }
                    }
                },
                //Received because the channel is empty and closed.
                Err(err) => {
                    //println!("작업종료recv={:?}", err);
                    if rx.is_empty() {
                        rx.close();
                        break;
                    }
                }
            }
        }
    })
}

fn search_between(
    max_period: i64,
    start_dt: NaiveDate,
    end_dt: NaiveDate,
) -> Vec<(NaiveDate, NaiveDate)> {
    let max_period_day = Days::new(max_period as u64);
    let mut res: Vec<(NaiveDate, NaiveDate)> = Vec::new();
    let subtract = (end_dt - start_dt).num_days();
    let is_zero = (subtract % max_period) == 0;
    let size = if is_zero {
        subtract / max_period
    } else {
        (subtract / max_period) + 1
    };
    let mut tmp_start = start_dt;
    let mut tmp_end = start_dt.checked_add_days(max_period_day).unwrap();
    if tmp_end > end_dt {
        tmp_end = end_dt;
    }

    //println!("size=>{:#?}", size);
    for i in 0..size - 1 {
        tmp_start = tmp_start.checked_add_days(max_period_day).unwrap();
        tmp_end = tmp_end.checked_add_days(max_period_day).unwrap();
        if tmp_end > end_dt {
            tmp_end = end_dt;
        }
        //println!("i=>{:#?},({},{})", i, tmp_start, tmp_end);
        res.push((tmp_start, tmp_end));
    }

    res
}

fn read_excel(code_id: i64, fnm: String) -> Vec<AddPrice> {
    let mut parse_companys: Vec<AddPrice> = vec![];
    let path = std::path::Path::new(&fnm);

    info!("read_excel fnm={:#?}", path);

    let mut excel: Xlsx<_> = open_workbook(path).expect("엑셀파일 열기 실패..;");
    if let Ok(r) = excel.worksheet_range("Sheet1") {
        let mut i = 0;
        for row in r.rows() {
            let std_code = row[0].to_string();
            //println!("row={:?}, row[0]={:?}", row, row[0]);
            if i != 0 {
                let cmp = AddPriceXlsx {
                    code_id: code_id,
                    dt: row[0].to_string(),
                    cp: row[1].to_string(),
                    op: row[4].to_string(),
                    hp: row[5].to_string(),
                    lp: row[6].to_string(),
                    vol: row[7].to_string(),
                    t_val: row[8].to_string(),
                    stock_cnt: row[9].to_string(),
                    capital: row[10].to_string(),
                };

                parse_companys.push(cmp.to_add_price());
            }

            i = i + 1;
        }
    }
    parse_companys
}

const ADD_PRICE_QUERY: &str =
    "insert into hist.price (code_id, dt, op, hp, lp, cp, vol, t_val, stock_cnt, capital ) VALUES 
($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) on conflict (code_id, dt) do 
update set  op=$3, hp=$4, lp=$5, cp=$6, vol=$7, t_val=$8, stock_cnt=$9, capital=$10";

async fn save(pool: &Pool<Postgres>, list: Vec<AddPrice>) -> Result<(), sqlx::Error> {
    let mut conn = pool.begin().await?;
    let len = list.len();
    for item in list {
        // println!("{:#?}", item);
        query(ADD_PRICE_QUERY)
            .bind(item.code_id)
            .bind(item.dt)
            .bind(item.op)
            .bind(item.hp)
            .bind(item.lp)
            //
            .bind(item.cp)
            .bind(item.vol)
            .bind(item.t_val)
            .bind(item.stock_cnt)
            .bind(item.capital)
            //
            .execute(&mut *conn)
            .await?;
    }
    conn.commit().await?;
    info!("saved list.len()={:#?}", len);

    Ok(())
}

mod tests {
    use std::{fs, path::Path, str::FromStr};

    use chrono::NaiveDate;
    use rust_decimal::Decimal;

    use crate::job::input_data::{
        file::{fm::mkdir_without_fnm, FileInfo},
        setp_price_data_krx::{read_excel, search_between},
    };

    #[tokio::test]
    async fn test_mk_file_path() {
        let mut my_path="C:/Users/example/Documents/GitHub/stockbot/batch/data2/kr/price/999/098120/19970430~19990429.xlsx".to_string();
        mkdir_without_fnm(my_path);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_search_between() {
        let max_period = chrono::TimeDelta::days(365 * 2 - 1).num_days();
        //let start_dt: NaiveDate = NaiveDate::from_ymd_opt(1995, 05, 02).unwrap();
        let start_dt: NaiveDate = NaiveDate::from_ymd_opt(2023, 05, 02).unwrap();
        let end_dt: NaiveDate = NaiveDate::from_ymd_opt(2025, 09, 25).unwrap();

        let res = search_between(max_period, start_dt, end_dt);

        println!("res=>{:#?}", res);
        println!("res.cnt=>{:#?}", res.len());

        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_read_excel() {
        let code_id = 1;

        let fi = FileInfo::new();
        let savefile = fi.kr_price_dir + "001045.xlsx";

        let res = read_excel(code_id, savefile);

        println!("res=>{:#?}", res);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
