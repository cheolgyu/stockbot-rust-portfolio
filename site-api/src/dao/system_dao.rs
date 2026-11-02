use actix_web::web;

use batch_model::log::batch::LogBatch;
use batch_model::site::param::log_batch;
use batch_model::site::res::PageResult;
use batch_model::site::res::ResultPagingInfo;
use batch_model::tb_row::TbRow;

use crate::errors::*;
use crate::DBPool;

pub mod batch_exe {
    use tracing::info;

    use super::*;

    pub async fn select_health_check(pool: web::Data<DBPool>) -> Result<String> {
        let mut conn = pool.acquire().await.unwrap();

        let query =
            format!("SELECT TO_CHAR(dt at time zone 'Asia/Seoul', 'YYYY-MM-DD HH24:MI:SS') FROM system.info where id = 1;",);

        let row: (String,) = sqlx::query_as(&query).fetch_one(&mut *conn).await?;
        let res = row.0;
        info!(" select_health_check = {:?}", &res);
        Ok(res)
    }
}

pub async fn paging_batch(
    pool: web::Data<DBPool>,
    param: log_batch::Param,
) -> Result<PageResult<LogBatch>> {
    let mut conn = pool.acquire().await.unwrap();

    let query_total_cnt = format!(
        "SELECT count(B.ID)
        FROM SYSTEM.BATCH B
        LEFT JOIN SYSTEM.BATCH_STEP BS ON B.ID = BS.BATCH_ID
        WHERE 1=1 {batch_id}
        ",
        batch_id = param.format_sql_batch_id()
    );

    let row: (i64,) = sqlx::query_as(&query_total_cnt)
        .bind(150_i64)
        .fetch_one(&mut *conn)
        .await?;
    let totalcnt = row.0;

    let page_info = ResultPagingInfo::from2(&param.paging, totalcnt);

    let query = format!(
        "SELECT B.ID AS BATCH_ID,
            B.START_TIME            as BATCH_START_TIME  ,
            B.END_TIME              as BATCH_END_TIME,
            B.RUN_TIME_SEC::text    as BATCH_RUN_TIME_SEC,
            BS.ID                   AS STEP_ID,
            BS.JOB_NAME             as step_job_name,
            BS.STEP_NAME            as step_name,
            BS.START_TIME           as step_start_time,
            BS.END_TIME             as step_end_time,
            BS.RUN_TIME_SEC::text   as step_RUN_TIME_SEC
    FROM SYSTEM.BATCH B
    LEFT JOIN SYSTEM.BATCH_STEP BS ON B.ID = BS.BATCH_ID
    WHERE 1=1 {batch_id}
    {sort} limit {limit} offset {offset}",
        batch_id = param.format_sql_batch_id(),
        sort = param.paging.sql_print(),
        limit = param.paging.limit(),
        offset = param.paging.offset(),
    );

    println!("{}", &query);

    let rows = sqlx::query_as::<_, LogBatch>(&query)
        .fetch_all(&mut *conn)
        .await
        .unwrap();
    Ok(PageResult {
        data: rows,
        page_info,
    })
}
