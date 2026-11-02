
use actix_web::web;

use batch_model::agg::view::ViewAggFreq;
use batch_model::site::param::{agg_cp_by_code, agg_cp_list, SqlObsVal};
use batch_model::site::res::PageResult;

use batch_model::site::res::ResultPagingInfo;



use crate::{errors::* };
use crate::DBPool;

pub async fn by_code_list(
    pool: web::Data<DBPool>,
    _code: String,
    param: agg_cp_by_code::Param,
) -> Result<PageResult<ViewAggFreq>> {
    let mut conn = pool.acquire().await.unwrap();

    let query_total_cnt = format!(" 
SELECT COUNT(v.code_id)
FROM public.view_agg_cp_freq v
WHERE v.CODE_ID = $1 ",);

    
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
    FROM public.view_agg_cp_freq v
    WHERE v.CODE_ID = $1 
    {sort}  limit {limit} offset {offset}",
        sort = param.paging.sql_print(),
        limit = param.paging.limit(),
        offset = param.paging.offset(),
    );

    println!("{}", &query);

   
    let rows = sqlx::query_as::<_, ViewAggFreq>(&query)
    .bind(&param.code_id)
    .fetch_all(&mut *conn)
    .await
    .unwrap();
    Ok(PageResult {
        data: rows,
        page_info,
    })

}


pub async fn paging(pool: web::Data<DBPool>, param: agg_cp_list::Param,) ->Result<PageResult<ViewAggFreq>>  {
    let mut conn = pool.acquire().await.unwrap();

    let mut query_total_cnt = format!(
        "select  count(*) from public.view_agg_cp_freq WHERE exchange in ({exchange})  ",
        exchange = param.exchange.format_sql_in()
    );
    let _search = param.search.search.clone();
    query_total_cnt = param.search.sql_query(query_total_cnt);

    query_total_cnt = format!(
        " {query} AND  obs_epoch = '{obs_epoch}' AND  obs_target = '{obs_target}'   ",
        query = query_total_cnt,
        obs_epoch = param.obs_epoch,
        obs_target = param.obs_target,
    );

    let q_obs_val  = param.obs_val_to_query(param.obs_val);
    query_total_cnt = format!(" {query_total_cnt} {q_obs_val} ");

    let row: (i64,) = sqlx::query_as(&query_total_cnt)
        .bind(150_i64)
        .fetch_one(&mut *conn)
        .await?;
    let totalcnt = row.0;

    let page_info = ResultPagingInfo::from2(&param.paging, totalcnt);

    let mut search_query = "".to_string();
    search_query = param.search.sql_query(search_query);
    let q_obs_val  = param.obs_val_to_query(param.obs_val);

    let query = format!(
        "select  * from public.view_agg_cp_freq t where obs_epoch = '{obs_epoch}' AND  obs_target = '{obs_target}'   {q_obs_val} AND  exchange in ({exchange}) {search_query} {sort}  limit {limit} offset {offset}",
        obs_epoch = param.obs_epoch,
        obs_target = param.obs_target,
        sort = param.paging.sql_print(),
        exchange = param.exchange.format_sql_in(),
        search_query=search_query,
        limit = param.paging.limit(),
        offset = param.paging.offset(),
        
    );

    println!("{}",&query);

    let rows = sqlx::query_as::<_,ViewAggFreq>(&query)
    .fetch_all(&mut *conn)
    .await
    .unwrap();
        Ok(PageResult {
            data: rows,
            page_info,
        })
    }
 
