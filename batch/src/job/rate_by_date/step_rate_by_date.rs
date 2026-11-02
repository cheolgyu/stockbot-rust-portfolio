use std::error::Error;

use crate::{
    job::utils::{calculate_rate, get_default_start_dt},
    utils::batch::{
        err::{ItemProcessorError, ItemReaderError, ItemWriterError},
        step::Step,
    },
};
use async_channel::Receiver;
use async_trait::async_trait;
use batch_model::{
    rate_by_date::{sum_by_date::SumByDateItem, RateByDate},
    utils::start_date::StartDate,
};

use sqlx::{query, query_as, Pool, Postgres, QueryBuilder};
use tokio::task::JoinHandle;
use tracing::info;

pub struct StepRateByDate {
    pub batch_id: i64,
    pub code: Vec<StartDate>,
}
type ChannelDataType = Vec<RateByDate>;

#[async_trait]
impl Step for StepRateByDate {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        let default_start_dt = get_default_start_dt().unwrap();
        self.get_targets(pool).await.unwrap();

        Ok(())
    }

    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        ////////////////////////////////////////////////////
        info!("StepRateByDate process start");
        let items = self.code.clone();
        let loop_size: usize = items.len();
        //let cap = 16;
        let cap = 1;
        let (tx, rx) = async_channel::bounded::<ChannelDataType>(cap);

        /*
            ======================================
                        sender
            ======================================
        */
        let r = rx.clone();
        let pool1 = pool.clone();
        let items = self.code.clone();
        let s1 = tokio::spawn(async move {
            for start_date in items {
                let pool1 = pool1.clone();
                let insert_data = Self::make_rate(&pool1.clone(), start_date.clone())
                    .await
                    .unwrap();
                if insert_data.len() > 0 {
                    match tx.send(insert_data.clone()).await {
                        Ok(_) => {
                            //info!("send: code_id={code_id},line={line},len={}", len);
                        }
                        Err(err) => {
                            println!("작업종료send={:?}", err);
                            if tx.is_empty() {
                                tx.close();
                            }
                        }
                    };
                }
            }
        });
        /*
            ======================================
                        recviver
            ======================================
        */
        let mut recv_size: usize = loop_size / 100;
        if recv_size < 1 {
            recv_size = 1;
        }

        let mut recv_manager: Vec<JoinHandle<()>> = Vec::new();

        for n in 0..recv_size {
            //fo!("rx make ={}", n);
            recv_manager.push(recv(n, rx.clone(), &pool.clone()).await);
        }

        for r in recv_manager {
            r.await
                .map_err(|err| {
                    info!("무슨 오류지? {:?}", err);
                    panic!("{}", err)
                })
                .unwrap();
        }

        s1.await.unwrap();
        Ok(())
    }

    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        Ok(())
    }
}

pub async fn recv(
    rx_name: usize,
    rx: Receiver<ChannelDataType>,
    pool: &Pool<Postgres>,
) -> JoinHandle<()> {
    let pool = pool.clone();
    tokio::spawn(async move {
        loop {
            let res = rx.recv().await;
            match res {
                Ok(list) => {
                    StepRateByDate::insert_query_mutil(&pool.clone(), list)
                        .await
                        .unwrap();
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

impl StepRateByDate {
    async fn process_one_by_one(
        list: Vec<StartDate>,
        pool: &Pool<Postgres>,
    ) -> Result<(), Box<dyn Error>> {
        for start_date in list {
            let insert_data = Self::make_rate(&pool.clone(), start_date.clone())
                .await
                .unwrap();
            Self::insert_query_one(pool, insert_data).await.unwrap();
        }
        Ok(())
    }
    async fn insert_query_one(
        pool: &Pool<Postgres>,
        list: Vec<RateByDate>,
    ) -> Result<(), Box<dyn Error>> {
        let sql = "INSERT INTO hist.rate_by_date( code_id, dt,vol_rate, t_val_rate)  
        VALUES ($1, $2, $3, $4) on conflict (code_id, dt) 
do update set vol_rate= $3, t_val_rate=$4";
        let mut conn = pool.acquire().await.unwrap();
        for i in list {
            query(sql)
                .bind(i.code_id)
                .bind(i.dt)
                .bind(i.vol_rate)
                .bind(i.t_val_rate)
                .execute(&mut *conn)
                .await
                .map_err(|e| {
                    info!("insert_query_one 오류: i={:?}", i);
                    e
                })?;
        }

        Ok(())
    }

    async fn insert_query_mutil(
        pool: &Pool<Postgres>,
        list: Vec<RateByDate>,
    ) -> Result<(), Box<dyn Error>> {
        let mut conn = pool.acquire().await.unwrap();
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO hist.rate_by_date( code_id, dt, vol_rate, t_val_rate)  ",
        );
        query_builder.push_values(list, |mut b, item| {
            b.push_bind(item.code_id)
                .push_bind(item.dt)
                .push_bind(item.vol_rate.round_dp(3))
                .push_bind(item.t_val_rate.round_dp(3));
        });
        // query_builder.push(
        //     " on conflict (code_id, dt) do update set vol_rate = excluded.vol_rate,  t_val_rate = excluded.t_val_rate ",
        // );
        let mut query = query_builder.build();

        query.execute(&mut *conn).await.map_err(|e| {
            info!("오류냠!!-insert_query");
            e
        })?;
        Ok(())
    }

    const SQL_GET_TARGETS: &str =
        " select c.id, c.code, c.name, c.exchange, c.std_code , coalesce(max(p.dt), $1)as start_dt 
from code c  left join hist.sum_by_date p on c.exchange = p.exchange  
where c.country='KR' group by c.id   ";
    async fn get_targets(&mut self, pool: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
        let default_start_dt = get_default_start_dt()?;
        let mut conn = pool.acquire().await?;
        let res = query_as::<_, StartDate>(Self::SQL_GET_TARGETS)
            .bind(default_start_dt)
            .fetch_all(&mut *conn)
            .await?;
        self.code = res;
        conn.close().await?;
        Ok(())
    }
    const SQL_GET_PRICE: &str = " 
WITH
	T AS (
		SELECT
			*
		FROM
			HIST.SUM_BY_DATE
		WHERE
			EXCHANGE = $2
	)
SELECT
	P.CODE_ID,
	P.DT,
	P.VOL::NUMERIC,
	P.T_VAL::NUMERIC,
	T.SUM_VOL,
	T.SUM_T_VAL
FROM
	HIST.PRICE AS P
	LEFT JOIN T ON P.DT = T.DT
where p.code_id=$1 and p.dt > $3
ORDER BY
	P.DT ASC
          ";
    async fn make_rate(
        pool: &Pool<Postgres>,
        start_date: StartDate,
    ) -> Result<Vec<RateByDate>, Box<dyn Error>> {
        let mut res: Vec<RateByDate> = Vec::new();
        let mut conn = pool.acquire().await?;
        let list = sqlx::query_as::<_, SumByDateItem>(Self::SQL_GET_PRICE)
            .bind(start_date.id)
            .bind(start_date.exchange)
            .bind(start_date.start_dt)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                info!(
                    "make_rate 오류발생:SQL_GET_PRICE, start_date={:?}",
                    start_date
                );
                e
            })?;

        for i in list {
            let t_val_rate = calculate_rate(i.sum_t_val, i.t_val)?;
            let vol_rate = calculate_rate(i.sum_vol, i.vol)?;

            let item = RateByDate {
                code_id: i.code_id,
                dt: i.dt,
                vol_rate,
                t_val_rate,
            };
            //info!("i={:?},   item={:?}", i, item);
            res.push(item);
        }

        conn.close().await?;
        Ok(res)
    }
}
