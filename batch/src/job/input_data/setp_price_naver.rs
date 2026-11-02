use crate::{
    downloader::naver::{
        naver_download::NaverDownlad, naver_param_price::NaverParamPrice, parer_file::ParserFile,
    },
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
    price::{AddPrice, PriceStartDate},
};
use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::{Asia::Seoul, Tz};

use sqlx::{query, Executor, Pool, Postgres};
use tokio::task::JoinHandle;
use tracing::info;

pub struct StepPirceNaver {
    pub batch_id: i64,
    pub country: CountryType,
    pub code_start_dt: Vec<PriceStartDate>,
    pub end_dt: Option<NaiveDate>,
    pub kr_price_dir: String,
}
const SELECT_START_DT: &str = "
select c.id, c.code, c.name, c.std_code , coalesce(max(p.dt), $1) as start_dt 
from code c  left join hist.price p on c.id = p.code_id 
where c.country='KR' group by c.id    ";

#[derive(Debug, Clone)]
pub enum Command {
    Set { code_id: i64, fnm: String },
}

#[async_trait]
impl Step for StepPirceNaver {
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
        if !allow_download {
            return Ok(());
        }

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
        let batch_id = self.batch_id;
        let kr_price_dir = self.kr_price_dir.clone();

        let s1 = tokio::spawn(async move {
            for item in items {
                let code_id = item.id;
                let start_dt = item.start_dt;
                if start_dt == end_dt {
                    continue;
                }
                let param = NaverParamPrice {
                    batch_id,
                    code_id,
                    short_code: item.code.clone(),
                    start_time: item.start_dt.format("%Y%m%d").to_string(),
                    end_time: end.clone(),
                };
                let fnm = fmt_save_fnm(&kr_price_dir, &item.code);
                let download = NaverDownlad {
                    param,
                    fnm: fnm.clone(),
                };
                download.request_save_file().await.unwrap();

                let cmd = Command::Set {
                    code_id: item.id.clone(),
                    fnm,
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
                    Command::Set { code_id, fnm } => {
                        info!("rx_name={}: ,code_id:{:?}  ", &rx_name, &code_id,);
                        let pf = ParserFile { code_id, fnm };
                        let list = pf.run().await;
                        save(&pool, list).await.unwrap();
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

fn fmt_save_fnm(kr_price_dir: &str, short_code: &str) -> String {
    format!("{}{}", kr_price_dir, short_code)
}

mod tests {}
