
use actix_web::web;


use batch_model::rate_by_date::view::ViewRateByDate;
use batch_model::site::param::{rate_by_date_by_code, rate_by_date_list};
use batch_model::site::res::PageResultDt;

use batch_model::site::res::ResultPagingInfo;
use chrono::NaiveDate;



use crate::{errors::* };
use crate::DBPool;

pub async fn by_code_list(
    pool: web::Data<DBPool>,
    _code: String,
    param: rate_by_date_by_code::Param,
) -> Result<PageResultDt<ViewRateByDate>> {
    let mut conn = pool.acquire().await.unwrap(); 

   let dt = match param.dt {
        Some(v) => v,
        None => {
             let row: (NaiveDate,) = sqlx::query_as("select  max(dt) from HIST.PRICE ;")
            .fetch_one(&mut *conn)
            .await?;
            let res = row.0;
            res
        },
    };

    let query_total_cnt = format!(" 
SELECT COUNT(v.code_id)
FROM public.view_rate_by_date v
WHERE v.CODE_ID = $1   ",);

    
    let row: (i64,) = sqlx::query_as(&query_total_cnt)
        .bind(&param.code_id)
        .fetch_one(&mut *conn)
        .await?;
    let totalcnt = row.0;

    let page_info = ResultPagingInfo::from2(&param.paging, totalcnt);

    let query = format!(
        "
    SELECT 
        *
    FROM public.view_rate_by_date v
    WHERE v.CODE_ID = $1  
     {sort}  limit {limit} offset {offset}",
        sort = param.paging.sql_print(),
        limit = param.paging.limit(),
        offset = param.paging.offset(),
    );

    println!("{}", &query);

   
    let rows = sqlx::query_as::<_, ViewRateByDate>(&query)
    .bind(&param.code_id)
    .fetch_all(&mut *conn)
    .await
    .unwrap();
    Ok(PageResultDt {
        data: rows,
        page_info,
        dt: Some(dt),
    })

}


pub async fn paging(pool: web::Data<DBPool>, param: rate_by_date_list::Param,) ->Result<PageResultDt<ViewRateByDate>>  {
    let mut conn = pool.acquire().await.unwrap();
       let dt = match param.dt {
        Some(v) => v,
        None => {
             let row: (NaiveDate,) = sqlx::query_as("select  max(dt) from HIST.PRICE ;")
            .fetch_one(&mut *conn)
            .await?;
            let res = row.0;
            res
        },
    };


    let mut query_total_cnt = format!(
        "select  count(*) from public.view_rate_by_date WHERE exchange in ({exchange}) and dt='{dt}'  ",
        exchange = param.exchange.format_sql_in()
    );
    let _search = param.search.search.clone();
    query_total_cnt = param.search.sql_query(query_total_cnt);


    let row: (i64,) = sqlx::query_as(&query_total_cnt)
        .bind(150_i64)
        .fetch_one(&mut *conn)
        .await?;
    let totalcnt = row.0;

    let page_info = ResultPagingInfo::from2(&param.paging, totalcnt);

    let mut search_query = "".to_string();
    search_query = param.search.sql_query(search_query);
    


    let query = format!(
        "select  * from public.view_rate_by_date t where   dt='{dt}' and  exchange in ({exchange}) {search_query}  {sort} limit {limit} offset {offset}",
        sort = param.paging.sql_print(),
        exchange = param.exchange.format_sql_in(),
        search_query=search_query,
        limit = param.paging.limit(),
        offset = param.paging.offset(),
        
    );

    println!("{}",&query);

    let rows = sqlx::query_as::<_,ViewRateByDate>(&query)
    .fetch_all(&mut *conn)
    .await
    .unwrap();
        Ok(PageResultDt {
            data: rows,
            page_info,
            dt: Some(dt),
        })
    }
 
