use actix_web::web;

use batch_model::price::PriceView;
use batch_model::site::param::price_by_code;
use batch_model::site::res::PageResult;

use crate::errors::*;
use crate::DBPool;
use batch_model::site::res::ResultPagingInfo;

pub async fn list(
    pool: web::Data<DBPool>,
    _code: String,
    params: price_by_code::Param,
) -> Result<PageResult<PriceView>> {
    let mut conn = pool.acquire().await.unwrap();

    let query_total_cnt = format!("select count(*) from hist.price  where code_id= $1  ");

    let row: (i64,) = sqlx::query_as(&query_total_cnt)
        .bind(&params.code_id)
        .fetch_one(&mut *conn)
        .await?;
    let totalcnt = row.0;

    let page_info = ResultPagingInfo::from2(&params.paging, totalcnt);

    let query = format!(
        "
        select dt,
TO_CHAR(op , 'FM999,999,999') as op,
	TO_CHAR(hp, 'FM999,999,999') as hp, 
	TO_CHAR(lp, 'FM999,999,999') as lp, 
	TO_CHAR(cp, 'FM999,999,999') as cp, 
	TO_CHAR(vol, 'FM999,999,999') as vol
from(
select dt, op, hp,lp,cp,vol from hist.price t 
        where code_id= $1  {sort}   limit {limit} offset {offset} )",
        sort = params.paging.sql_print(),
        limit = params.paging.limit(),
        offset = params.paging.offset(),
    );

    let rows = sqlx::query_as::<_, PriceView>(&query)
        .bind(&params.code_id)
        .fetch_all(&mut *conn)
        .await
        .unwrap();

    Ok(PageResult {
        data: rows,
        page_info,
    })
}
