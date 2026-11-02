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
use batch_model::{
    app_type::price_type::PriceType, bound::AddBound, graph::DirectionType,
    price::PriceWithConvertDate,
};
use chrono::NaiveDate;

use rust_decimal::Decimal;
use sqlx::{query, Execute, Pool, Postgres, QueryBuilder};
use tokio::task::JoinHandle;
use tracing::log::info;

use crate::job::bound::hist_make_bound::MakeBound;

use super::code_start_dt::CodeStartDt;

pub struct StepHistInsert {
    pub code: Vec<CodeStartDt>,
    //pub data: ,
}

type ChannelDataType = (Vec<PriceWithConvertDate>, i64, PriceType);
#[async_trait]
impl Step for StepHistInsert {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        let code = hist_code_with_start_dt_list(pool).await.unwrap();
        self.code = code;
        Ok(())
    }

    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        ////////////////////////////////////////////////////
        info!("StepHistInsert process start");
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
        let r = rx.clone();
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
    let insert_size = false;
    let insert_size_rows = 1000;
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
                            if insert_size {
                                // 천개씩 돌리기
                                let count = if size % insert_size_rows == 0 {
                                    size / insert_size_rows
                                } else {
                                    size / insert_size_rows + 1
                                };

                                for i in 0..count {
                                    let start = insert_size_rows * i;
                                    let mut end = insert_size_rows * (i + 1);
                                    end = if end > size { size } else { end };
                                    let list = list_bound[start..end].to_vec();
                                    insert_unset(&pool, list).await.unwrap();
                                }
                            } else {
                                insert_unset(&pool, list_bound).await.unwrap();
                            }
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

/*
1995-05-02 ~ 20205-09-25 까지 23분 25초 걸림.
*/
async fn process_test_one_by_one_is_ok(code: Vec<CodeStartDt>, pool: &Pool<Postgres>) {
    let inser_one: usize = 500;
    for item in code {
        let code_id = item.id.clone();
        let exchange = item.exchange.clone();

        for &line in PriceType::iterator() {
            let p1x = item.get_dt(line);

            //println!("code={:?},line={:?},p2.x={:?}", code_id, line, p1x);
            let list: Vec<PriceWithConvertDate> = get_price_after_p1x(pool.clone(), code_id, p1x)
                .await
                .expect(&format!(
                    "get_price_after_p2x 오류 code_id={}, exchange={:?},p1x={:?}",
                    code_id, exchange, p1x
                ));
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
                    // 천개씩 돌리기

                    let count = if size % inser_one == 0 {
                        size / inser_one
                    } else {
                        size / inser_one + 1
                    };

                    for i in 0..count {
                        let start = inser_one * i;
                        let mut end = 1000 * (i + 1);
                        end = if end > size { size } else { end };
                        let list = list_bound[start..end].to_vec();
                        insert_unset(&pool, list).await.unwrap();
                    }
                }
            }
        }
    }
}

async fn insert_push_bind(
    pool: &Pool<Postgres>,
    list: Vec<AddBound>,
) -> Result<(), Box<dyn Error>> {
    // sql 문 생성후 저장 시 오류난 거보면 sql문이 제대로 안만들어짐.
    let mut conn = pool.acquire().await.unwrap();
    let size = list.len();
    //let mut conn = conn_acquire.begin().await?;

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("INSERT INTO hist.bound( code_id, line, p1x, p1y, p2x, p2y, direction, x_cnt, y_cnt, change_rate, ymxb_p3y, ymxb_m, ymxb_b)  ");
    query_builder.push_values(list, |mut b, item| {
        b.push_bind(item.code_id)
            .push_bind(item.line)
            .push_bind(item.p1x)
            .push_bind(item.p1y)
            .push_bind(item.p2x)
            .push_bind(item.p2y)
            .push_bind(item.direction)
            .push_bind(item.x_cnt)
            .push_bind(item.y_cnt)
            .push_bind(item.change_rate.round_dp(2))
            .push_bind(item.ymxb_p3y)
            .push_bind(item.ymxb_m)
            .push_bind(item.ymxb_b);
    });

    //실행중 자꾸 오류남 . 쿼리를 파일로 저장 하자.
    //query_builder.sql()

    let mut query = query_builder.build();

    let q2 = query.sql();

    query
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            info!("오류냠!!");
            info!("q2={}", q2);

            e
        })
        .unwrap();
    info!("저장완료 size ={}", size);

    //conn.commit().await?;
    Ok(())
}

/*



select count(*) from hist.bound  5512470

1950-05-02~2025-09-25
TEST-1: recv_size = 10;
*/

async fn insert_unset(pool: &Pool<Postgres>, list: Vec<AddBound>) -> Result<(), Box<dyn Error>> {
    // sql 문 생성후 저장 시 오류난 거보면 sql문이 제대로 안만들어짐.
    let mut conn = pool.acquire().await.unwrap();
    let size = list.len();
    //let mut conn = conn_acquire.begin().await?;

    let mut bound_code_id: Vec<i64> = Vec::new();
    let mut bound_line: Vec<PriceType> = Vec::new();
    let mut bound_p1x: Vec<NaiveDate> = Vec::new();
    let mut bound_p1y: Vec<Decimal> = Vec::new();
    let mut bound_p2x: Vec<NaiveDate> = Vec::new();
    let mut bound_p2y: Vec<Decimal> = Vec::new();
    let mut bound_direction: Vec<DirectionType> = Vec::new();
    let mut bound_x_cnt: Vec<Decimal> = Vec::new();
    let mut bound_y_cnt: Vec<Decimal> = Vec::new();
    let mut bound_change_rate: Vec<Decimal> = Vec::new();
    let mut bound_ymxb_p3y: Vec<Decimal> = Vec::new();
    let mut bound_ymxb_m: Vec<Decimal> = Vec::new();
    let mut bound_ymxb_b: Vec<Decimal> = Vec::new();

    for i in list {
        bound_code_id.push(i.code_id);
        bound_line.push(i.line);
        bound_p1x.push(i.p1x);
        bound_p1y.push(i.p1y);
        bound_p2x.push(i.p2x);
        bound_p2y.push(i.p2y);
        bound_direction.push(i.direction);
        bound_x_cnt.push(i.x_cnt);
        bound_y_cnt.push(i.y_cnt);
        bound_change_rate.push(i.change_rate);
        bound_ymxb_p3y.push(i.ymxb_p3y);
        bound_ymxb_m.push(i.ymxb_m);
        bound_ymxb_b.push(i.ymxb_b);
    }

    query("
       INSERT INTO hist.bound( code_id, line, p1x, p1y, p2x, p2y, direction, x_cnt, y_cnt, change_rate, ymxb_p3y, ymxb_m, ymxb_b) 
        SELECT * FROM UNNEST($1::BIGINT[], $2::price_type[], $3::DATE[], $4::NUMERIC[], $5::DATE[], $6::NUMERIC[], $7::direction_type[], $8::NUMERIC[], $9::NUMERIC[], $10::NUMERIC[], $11::NUMERIC[], $12::NUMERIC[], $13::NUMERIC[])
    ")
     .bind(&bound_code_id[..])
    .bind(&bound_line[..])
    .bind(  &bound_p1x[..])
    .bind(  &bound_p1y[..])
    .bind(  &bound_p2x[..])
    .bind(&bound_p2y[..])
    .bind(&bound_direction[..])
    .bind(&bound_x_cnt[..])
    .bind(&bound_y_cnt[..])
    .bind(&bound_change_rate[..])
    .bind( &bound_ymxb_p3y[..])
    .bind(&bound_ymxb_m[..],)
    .bind( &bound_ymxb_b[..])

    .execute(&mut *conn)
    .await.map_err(|e| {
            info!("오류냠!!");
            e
        })
        .unwrap();

    //info!("저장완료 size ={}", size);
    conn.close().await.unwrap();
    //conn.commit().await?;
    Ok(())
}
