use std::str::FromStr;

use actix_web::web;

use batch_model::app_type::find_why::Why;
use batch_model::find::view::by_code::FindViewByCode;
use batch_model::find::view::list::FindViewList;
use batch_model::site::param::{find_by_code, find_list};
use batch_model::site::res::{PageResult, PageResultFind};
use futures::TryStreamExt;
use sqlx::Row;

use crate::errors::*;
use crate::DBPool;

use batch_model::site::res::ResultPagingInfo;

pub async fn by_code_list(
    pool: web::Data<DBPool>,
    _code: String,
    param: find_by_code::Param,
) -> Result<PageResult<FindViewByCode>> {
    let mut conn = pool.acquire().await.unwrap();

    let query_total_cnt =
        format!("select count(*) from hist.find where code_id= $1 AND WHAT = $2 AND WHY = $3");

    let row: (i64,) = sqlx::query_as(&query_total_cnt)
        .bind(&param.code_id)
        .bind(&param.what)
        .bind(&param.why)
        .fetch_one(&mut *conn)
        .await?;
    let totalcnt = row.0;

    let page_info = ResultPagingInfo::from(&param.paging, totalcnt);

    let query = format!(
        "select 
        dt, what, why, order_num, code_id, line, p1x, change_rate
         from hist.find where code_id= $1 AND WHAT = $2 AND WHY = $3  order by {sort} {desc} limit {limit} offset {offset}",
        sort = param.paging.sort(),
        desc = param.paging.desc(),
        limit = param.paging.limit(),
        offset = param.paging.offset(),
    );

    let rows = sqlx::query_as::<_, FindViewByCode>(&query)
        .bind(param.code_id)
        .bind(&param.what)
        .bind(&param.why)
        .fetch_all(&mut *conn)
        .await
        .unwrap();

    Ok(PageResult {
        data: rows,
        page_info,
    })
}

pub async fn list(
    pool: web::Data<DBPool>,
    param: find_list::Param,
) -> Result<PageResultFind<FindViewList>> {
    let mut conn = pool.acquire().await.unwrap();

    let mut query_total_cnt = format!(
        "select  count(*) from VIEW_FIND WHERE  WHAT = $1 AND WHY = $2 AND exchange in ({exchange})  ",
        exchange = param.exchange.format_sql_in()
    );
    query_total_cnt = param.search.sql_query(query_total_cnt);

    let row: (i64,) = sqlx::query_as(&query_total_cnt)
        //.bind(param.dt)
        .bind(param.what)
        .bind(param.why)
        .fetch_one(&mut *conn)
        .await?;
    let totalcnt = row.0;

    let page_info = ResultPagingInfo::from(&param.paging, totalcnt);
    let mut search_query = "".to_string();
    search_query = param.search.sql_query(search_query);
    //
    //
    //
    let mut conn = pool.acquire().await.unwrap();
    let query = format!(
        "
SELECT lb.nm as why,
    coalesce(t.count, 0) as count
FROM (
        SELECT a.enumlabel as nm
        FROM pg_catalog.pg_enum a
            left join pg_catalog.pg_type b on a.enumtypid = b.oid
        where 1 = 1
            and b.typname = 'find_why'
        order by enumsortorder asc
    ) lb
    left outer join (
        select why,
            count(id)
        from VIEW_FIND
        WHERE EXCHANGE IN ({exchange})   {search_query}
            AND WHAT =  $1
        group by why
    ) t on lb.nm = t.why::text
 ",
        exchange = param.exchange.format_sql_in(),
        search_query = search_query,
    );

    let mut select_box: Vec<(Why, i64)> = vec![];
    let mut rows_sb = sqlx::query(&query).bind(param.what).fetch(&mut *conn);
    while let Some(row) = rows_sb.try_next().await? {
        // map the row into a user-defined domain type
        select_box.push((
            Why::from_str(row.try_get("why")?).unwrap(),
            row.try_get("count")?,
        ))
    }

    //
    //
    //
    let mut conn = pool.acquire().await.unwrap();
    let query = format!(
        "select  * from VIEW_FIND where  WHAT = $1 AND WHY = $2 AND exchange in ({exchange}) {search_query} order by {sort} {desc} limit {limit} offset {offset}",
        sort = param.paging.sort(),
        desc = param.paging.desc(),
        exchange = param.exchange.format_sql_in(),
        search_query=search_query,
        limit = param.paging.limit(),
        offset = param.paging.offset(),
    );

    let rows = sqlx::query_as::<_, FindViewList>(&query)
        .bind(param.what)
        .bind(param.why)
        .fetch_all(&mut *conn)
        .await
        .unwrap();

    Ok(PageResultFind {
        data: rows,
        page_info,
        search_info: select_box,
    })
}
