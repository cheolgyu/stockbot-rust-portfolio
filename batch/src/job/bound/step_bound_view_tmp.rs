use crate::utils::batch::{
    err::{ItemProcessorError, ItemReaderError, ItemWriterError},
    step::Step,
};
use async_trait::async_trait;
use batch_model::bound::{Bound, BoundDaily};
use chrono::NaiveDate;
use futures::TryStreamExt;
use sqlx::{query, Pool, Postgres, Row};
use std::error::Error;
use tracing::log::info;

pub struct StepBoundView {}
#[async_trait]
impl Step for StepBoundView {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        Ok(())
    }

    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        run_process(pool).await.unwrap();
        Ok(())
    }

    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        Ok(())
    }
}

/*

select *
from hist.bound
where code_id = 184
and line = 'LOW'
order by p1x desc ;

delete from hist.bound where code_id = 184;

 */

const SELECT_MAX_P2X_HIST_BOUND_SQL: &str = "SELECT MAX(P2X) FROM HIST.BOUND  ";

const SELECT_HIST_BOUND_SQL: &str = "
    select 
        JSON_AGG(
            JSON_BUILD_OBJECT(
                'code_id',
                CODE_ID,
                'line',
                LINE,
                'p1x',
                P1X,
                'p1y',
                P1Y,
                'p2x',
                P2X,
                'p2y',
                P2Y,
                'direction',
                DIRECTION,
                'x_cnt',
                X_CNT,
                'y_cnt',
                Y_CNT,
                'change_rate',
                change_rate,
                'ymxb_p3y',
                YMXB_P3Y,
                'ymxb_m',
                YMXB_M,
                'ymxb_b',
                YMXB_B
            )
        ) as jdata
    from hist.bound     
    where p2x = $1
    group by code_id 
     ";
const INSERT_HIST_BOUND_DAILY_SQL: &str = "
     INSERT INTO hist.bound_view_tmp(
        code_id, line,
        p1x, p1y, p2x, p2y, direction, x_cnt, y_cnt, change_rate, ymxb_p3y, ymxb_m, ymxb_b
    )
        VALUES (
            $1, $2, 
            $3, $4, $5, $6, $7, $8, $9, $10, $11, $12,  $13
        );
        ";
async fn run_process(pool: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    let max_p2x: (NaiveDate,) = sqlx::query_as(SELECT_MAX_P2X_HIST_BOUND_SQL)
        .fetch_one(pool)
        .await?;

    let mut rows = sqlx::query(SELECT_HIST_BOUND_SQL)
        .bind(max_p2x.0)
        .fetch(pool);
    let mut vec_bound_daily: Vec<BoundDaily> = vec![];

    while let Some(row) = rows.try_next().await? {
        // map the row into a user-defined domain type
        let json_value: serde_json::Value = row.try_get("jdata")?;
        //let json_str = json_value.as_str().expect("제이슨 to str 오류");
        // println!("json_value = {:?}", json_value);

        let mut bound_daily: BoundDaily = BoundDaily::default();

        for elem in json_value
            .as_array()
            .expect("board_today json_row error")
            .iter()
        {
            //println!("elem = {:?}", elem);
            // thread 'main' panicked at 'json value 를 bound로 변환 실패: Error("invalid type: integer `20230726`, expected a formatted date string", line: 1, column: 66)', src/job/bound/step_daily.rs:126:57
            // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

            let b_item: Bound = match serde_json::from_str(&elem.to_string()) {
                Ok(v) => v,
                Err(err) => {
                    info!("json value 를 bound로 변환 실패");
                    info!("=>{:#?}", elem);
                    panic!("=>{},{} ", elem, err.to_string());
                }
            };
            bound_daily.code_id = b_item.code_id;
            bound_daily.line = b_item.line;
            bound_daily.p1x = b_item.p1x;
            bound_daily.p1y = b_item.p1y;
            bound_daily.p2x = b_item.p2x;
            bound_daily.p2y = b_item.p2y;
            bound_daily.direction = b_item.direction;
            bound_daily.x_cnt = b_item.x_cnt;
            bound_daily.y_cnt = b_item.y_cnt;
            bound_daily.change_rate = b_item.change_rate;
            bound_daily.ymxb_p3y = b_item.ymxb_p3y;
            bound_daily.ymxb_m = b_item.ymxb_m;
            bound_daily.ymxb_b = b_item.ymxb_b;
        }
        vec_bound_daily.push(bound_daily);
    }
    save(pool, vec_bound_daily).await?;
    Ok(())
}

async fn save(pool: &Pool<Postgres>, list: Vec<BoundDaily>) -> Result<(), Box<dyn Error>> {
    let mut conn = pool.begin().await?;
    query("truncate table hist.bound_view_tmp ")
        .execute(&mut *conn)
        .await?;

    for item in list {
        //println!("{:?}",&item);

        query(INSERT_HIST_BOUND_DAILY_SQL)
            .bind(item.code_id)
            .bind(item.line)
            //
            .bind(item.p1x)
            .bind(item.p1y)
            .bind(item.p2x)
            .bind(item.p2y)
            .bind(item.direction)
            .bind(item.x_cnt)
            .bind(item.y_cnt)
            .bind(item.change_rate)
            .bind(item.ymxb_p3y)
            .bind(item.ymxb_m)
            .bind(item.ymxb_b)
            //
            .execute(&mut *conn)
            .await?;
    }
    conn.commit().await?;
    Ok(())
}
