use std::error::Error;

use crate::{
    job::bound::utils::{get_price_after_p1x, hist_code_with_start_dt_list},
    utils::batch::{
        err::{ItemProcessorError, ItemReaderError, ItemWriterError},
        step::Step,
    },
};
use async_channel::Receiver;
use async_trait::async_trait;
use batch_model::{app_type::price_type::PriceType, bound::AddBound, price::PriceWithConvertDate};
use sqlx::{query, Pool, Postgres};
use tokio::task::JoinHandle;
use tracing::log::info;

use crate::job::bound::hist_make_bound::MakeBound;

use super::code_start_dt::CodeStartDt;

pub struct StepBoundUpsert {
    pub code: Vec<CodeStartDt>,
}

type ChannelDataType = (Vec<PriceWithConvertDate>, i64, PriceType);
#[async_trait]
impl Step for StepBoundUpsert {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        let code = hist_code_with_start_dt_list(pool).await.unwrap();
        self.code = code;
        Ok(())
    }

    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        ////////////////////////////////////////////////////
        let items = self.code.clone();
        let loop_size: usize = items.len() * 4;
        //let cap = 16;
        let cap = 1;
        let (tx, rx) = async_channel::bounded::<ChannelDataType>(cap);

        /*
            ======================================
                        sender
            ======================================
        */

        let pool1 = pool.clone();
        let s1 = tokio::spawn(async move {
            for item in items.iter() {
                let code_id = item.id.clone();
                let pool2 = pool1.clone();

                for &line in PriceType::iterator() {
                    let p1x = item.get_dt(line);
                    let pool3 = pool2.clone();

                    let list: Vec<PriceWithConvertDate> = get_price_after_p1x(pool3, code_id, p1x)
                        .await
                        .expect(&format!(
                            "get_price_after_p1x_v2 오류 code_id={}, p1x={:?}",
                            code_id, p1x
                        ));
                    let len = &list.len();

                    match tx.send((list.clone(), code_id, line)).await {
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
            info!("rx make ={}", n);
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
                Ok((list, code_id, line)) => {
                    info!(
                        "recv({rx_name}): code_id={code_id},line={line},len={}",
                        list.len()
                    );
                    if list.len() > 2 {
                        let i = MakeBound {
                            code_id,
                            line,
                            list,
                        };
                        //let sender_save = sender_save.clone();

                        let list_bound: Vec<AddBound> = i.run();
                        let size = list_bound.len();
                        if size > 0 {
                            upsert_one(&pool, list_bound).await.unwrap();
                        }
                    }
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

const UPSERT_HIST_BOUND_SQL: &str = "
INSERT INTO hist.bound( code_id, line, p1x, p1y, p2x, p2y, direction, x_cnt, y_cnt, change_rate, ymxb_p3y, ymxb_m, ymxb_b) 
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) on conflict (code_id, line, p1x) 
do update set p1x=$3, p1y=$4, p2x=$5, p2y=$6, direction=$7, x_cnt=$8, y_cnt=$9, change_rate=$10, ymxb_p3y=$11, ymxb_m=$12, ymxb_b=$13";
async fn upsert_one(pool: &Pool<Postgres>, list: Vec<AddBound>) -> Result<(), Box<dyn Error>> {
    // let mut conn = pool.begin().await?;

    for item in list {
        //println!("{:#?}", &item);

        query(UPSERT_HIST_BOUND_SQL)
            .bind(item.code_id)
            .bind(item.line)
            .bind(item.p1x)
            .bind(item.p1y)
            .bind(item.p2x)
            .bind(item.p2y)
            .bind(item.direction)
            .bind(item.x_cnt)
            .bind(item.y_cnt)
            .bind(item.change_rate.round_dp(2))
            .bind(item.ymxb_p3y)
            .bind(item.ymxb_m)
            .bind(item.ymxb_b)
            .execute(pool)
            .await
            .map_err(|e| {
                info!("오류발생 !!!? {:?}", e);
                info!("오류발생한 item={:#?}", item);
                info!("오류발생한 change_rate={}", item.change_rate.round_dp(2));
                panic!("바운드 저장오류=>{}", e)
            })
            .err();
    }
    // conn.commit().await?;
    Ok(())
}
