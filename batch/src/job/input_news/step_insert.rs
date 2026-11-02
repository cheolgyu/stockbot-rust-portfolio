use std::env::var;

use crate::utils::batch::{
    err::{ItemProcessorError, ItemReaderError, ItemWriterError},
    step::Step,
};
use async_channel::Receiver;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::{Asia::Seoul, Tz};
use sqlx::{Pool, Postgres, Row};
use tokio::task::JoinHandle;

use super::naver::scraping::{Scraping, ScrapingParams};

const SELECT_CODE: &str = "SELECT C.ID,
	D.SHORT_NAME,
	COALESCE(max(n.dt), $1::Date ) as dt
FROM CODE C
left join hist.news n on c.id = n.code_id
LEFT JOIN CODE_DETAIL D ON C.ID = D.CODE_ID
group by c.id,d.short_name
";

pub struct StepInsert {}
#[async_trait]
impl Step for StepInsert {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        Ok(())
    }

    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        let dt_format: &str = "%Y-%m-%d";
        let kr_news_default_start_date = var("KR_NEWS_DEFAULT_START_DATE").unwrap();
        let b_dt = NaiveDate::parse_from_str(&format!("{}", kr_news_default_start_date), dt_format)
            .unwrap();

        let codes = sqlx::query(SELECT_CODE)
            .bind(b_dt)
            .fetch_all(&pool.clone())
            .await
            .unwrap();
        let seoul_now: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
        let seoul_now_dt = seoul_now.naive_utc().date();

        //////////////////////////////////////////////////////
        let mut i = 0;
        let cap = 16;
        let (tx, rx) = async_channel::bounded::<Scraping>(cap);
        let loop_size = codes.len();
        //////////////////////////////////////////////////////
        ///
        let s1 = tokio::spawn(async move {
            for item in codes {
                let id = item.get("id");
                let short_name = item.get("short_name");
                let dt: NaiveDate = item.get("dt");
                let naver = Scraping {
                    code_id: id,
                    now_date: seoul_now_dt,
                    params: ScrapingParams {
                        search_text: short_name,
                        dt_start: dt,
                        dt_end: seoul_now_dt,
                        page: 1,
                    },
                };

                //   naver.run(&pool.clone()).await.unwrap();
                let d = format!("{:?}", naver);

                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    match tx_clone.send(naver).await {
                        Ok(_) => {
                            //println!("sender: i={}, send-{:?}", i, d)
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
            // println!("starting rx_name={}", n);
            recv_manager.push(recv(n, rx.clone(), pool).await);
        }

        s1.await.unwrap();

        for r in recv_manager {
            r.await.unwrap();
        }
        //////////////////////////////
        Ok(())
    }

    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        Ok(())
    }
}
pub async fn recv(rx_name: usize, rx: Receiver<Scraping>, pool: &Pool<Postgres>) -> JoinHandle<()> {
    let pool = pool.clone();
    tokio::spawn(async move {
        loop {
            let res = rx.recv().await;
            match res {
                Ok(mut item) => {
                    let d = format!("{:?}", item);
                    item.run(&pool);

                    //println!("saved: rx_name={}: ,len={}", rx_name, d);
                }
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
