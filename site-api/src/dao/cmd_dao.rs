use actix_web::web;

use batch_model::app_type::batch_run_type::BatchParams;
use batch_model::batch_timer::BatchTimerPlan;
use batch_model::site::param;
use batch_model::site::res::PageResult;
use batch_model::site::res::ResultPagingInfo;
use batch_model::tb_row::TbRow;

use sqlx::Row;

use crate::errors::*;
use crate::DBPool;

const SQL: &str = "
INSERT INTO SYSTEM.BATCH_TIMER_PLAN(
	batch_type,
	country_type,
    run_type)
VALUES ( $1, $2, $3 ) RETURNING id; ";

pub async fn start_cmd(pool: web::Data<DBPool>, params: BatchParams) -> Result<i64> {
    let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = pool.acquire().await.unwrap();
    //let c = CountryType::from_str(&country).unwrap();
    let row = sqlx::query(SQL)
        .bind(&params.batch_type)
        .bind(&params.country_type)
        .bind(&params.run_type)
        .fetch_one(&mut *conn)
        .await
        .unwrap();
    let id: i64 = row.try_get("id").unwrap();
    Ok(id)
}

const UPDATE_SQL: &str = "
UPDATE SYSTEM.BATCH_TIMER_PLAN
	SET  updated_dt = now() , RUN_TYPE='EXIT'
	WHERE ID = $1;
";

pub async fn exit_cmd(pool: web::Data<DBPool>, id: i64) -> Result<()> {
    let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = pool.acquire().await.unwrap();

    sqlx::query(UPDATE_SQL)
        .bind(&id)
        .execute(&mut *conn)
        .await
        .unwrap();
    Ok(())
}

pub async fn paging(
    pool: web::Data<DBPool>,
    page: param::batch_timer_plan::Param,
) -> Result<PageResult<BatchTimerPlan>> {
    let mut conn = pool.acquire().await.unwrap();

    let query_total_cnt = format!("select  count(*) from SYSTEM.BATCH_TIMER_PLAN   ",);

    let row: (i64,) = sqlx::query_as(&query_total_cnt)
        .bind(150_i64)
        .fetch_one(&mut *conn)
        .await?;
    let totalcnt = row.0;

    let page_info = ResultPagingInfo::from2(&page.paging, totalcnt);

    let query = format!(
        "SELECT ID,
            batch_type,
            country_type,
            run_type,
            updated_dt,
            crated_dt
        FROM SYSTEM.BATCH_TIMER_PLAN
         {sort} limit {limit} offset {offset}",
        sort = page.paging.sql_print(),
        limit = page.paging.limit(),
        offset = page.paging.offset(),
    );

    println!("{}", &query);

    let rows = sqlx::query_as::<_, BatchTimerPlan>(&query)
        .fetch_all(&mut *conn)
        .await
        .unwrap();
    Ok(PageResult {
        data: rows,
        page_info,
    })
}
