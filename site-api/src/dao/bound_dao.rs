use actix_web::web;

use batch_model::bound::view::list::{BoundCodeDetail, BoundViewList};
use batch_model::site::param::{self, bound_by_code};
use batch_model::site::res::PageResult;
use batch_model::site::res::ResultPagingInfo;

use sqlx::PgPool;

use crate::errors::*;
use crate::DBPool;

pub async fn bound_code_detail(
    pool: web::Data<PgPool>,
    param: bound_by_code::Param,
) -> Result<PageResult<BoundCodeDetail>> {
    let mut conn = pool.acquire().await.unwrap();

    let query_total_cnt =
        format!("select  count(*) from hist.bound where code_id= $1 and line= $2 ",);

    let row: (i64,) = sqlx::query_as(&query_total_cnt)
        .bind(&param.code_id)
        .bind(&param.line)
        .fetch_one(&mut *conn)
        .await?;
    let totalcnt = row.0;

    let page_info = ResultPagingInfo::from2(&param.paging, totalcnt);

    let query = format!(
        "select 
        CODE_ID,
            case
            when DIRECTION = 'DECREASE' then '감소'
            when DIRECTION = 'INCREASE' then '증가'
            when DIRECTION = 'CONSTANT' then '유지'
        end as DIRECTION,
        P1X,
        TO_CHAR(P1Y, 'FM999,999,999') as P1Y,
        P2X,
        TO_CHAR(P2Y, 'FM999,999,999') as P2Y,
        TO_CHAR(X_CNT, 'FM999,999,999') as X_CNT,
        TO_CHAR(Y_CNT, 'FM999,999,999') as Y_CNT,
        change_rate,
        TO_CHAR(YMXB_P3Y, 'FM999,999,999') as YMXB_P3Y,
        TO_CHAR(YMXB_M, 'FM999,999,999') as YMXB_M,
        TO_CHAR(YMXB_B, 'FM999,999,999') as YMXB_B
        from (select * 
        from hist.bound t where code_id= $1 and line=$2   {sort}  limit {limit} offset {offset} )",
        sort = param.paging.sql_print(),
        limit = param.paging.limit(),
        offset = param.paging.offset(),
    );

    let rows = sqlx::query_as::<_, BoundCodeDetail>(&query)
        .bind(&param.code_id)
        .bind(&param.line)
        .fetch_all(&mut *conn)
        .await
        .unwrap();
    Ok(PageResult {
        data: rows,
        page_info,
    })
}

pub async fn paging(
    pool: web::Data<DBPool>,
    param: param::bound_list::Param,
) -> Result<PageResult<BoundViewList>> {
    let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = pool.acquire().await.unwrap();

    let mut query_total_cnt = format!(
        "select  count(*) from public.view_bound WHERE exchange in ({exchange})  ",
        exchange = param.exchange.format_sql_in()
    );
    query_total_cnt = param.search.sql_query(query_total_cnt);

    let row: (i64,) = sqlx::query_as(&query_total_cnt)
        .bind(150_i64)
        .fetch_one(&mut *conn)
        .await?;
    let totalcnt = row.0;

    let page_info = ResultPagingInfo::from2(&param.paging, totalcnt);
    let line_query = param.line.format_sql();
    let mut search_query = "".to_string();
    search_query = param.search.sql_query(search_query);
    let query = format!(
        "
    select      
        ID,
        STD_CODE ,
        code,
        name,
        COUNTRY,
        EXCHANGE,
        line,
        CODE_ID,
            case
            when DIRECTION = 'DECREASE' then '감소'
            when DIRECTION = 'INCREASE' then '증가'
            when DIRECTION = 'CONSTANT' then '유지'
        end as DIRECTION,
        P1X,
        TO_CHAR(P1Y, 'FM999,999,999') as P1Y,
        P2X,
        TO_CHAR(P2Y, 'FM999,999,999') as P2Y,
        TO_CHAR(X_CNT, 'FM999,999,999') as X_CNT,
        TO_CHAR(Y_CNT, 'FM999,999,999') as Y_CNT,
        change_rate,
        TO_CHAR(YMXB_P3Y, 'FM999,999,999') as YMXB_P3Y,
        TO_CHAR(YMXB_M, 'FM999,999,999') as YMXB_M,
        TO_CHAR(YMXB_B, 'FM999,999,999') as YMXB_B, 
        CREATED_AT 
        from (select * 
    from public.view_bound  t where exchange in ({exchange}) {search_query} {line_query}  {sort} limit {limit} offset {offset} )",
        sort = param.paging.sql_print(),
        exchange = param.exchange.format_sql_in(),
        search_query=search_query,
        limit = param.paging.limit(),
        offset = param.paging.offset(),
    );
     println!("{}", &query);

    let rows = sqlx::query_as::<_, BoundViewList>(&query)
        .fetch_all(&mut *conn)
        .await
        .unwrap();

    Ok(PageResult {
        data: rows,
        page_info,
    })
}
