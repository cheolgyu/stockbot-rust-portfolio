use std::str::FromStr;

use actix_web::web;

use batch_model::app_type::exchange_type::ExchangeType;
use futures::TryStreamExt;
use sqlx::types::chrono::NaiveDate;
use sqlx::Row;

use crate::errors::*;
use crate::DBPool;

use batch_model::site::res::PageResulDt;
const SQL: &str = "
SELECT DT
FROM UTILS.CONVERT_DATE
WHERE EXCHANGE = $1
ORDER BY DT DESC ;
";

pub async fn get_list_dt(pool: web::Data<DBPool>, exchange: ExchangeType) -> Result<PageResulDt> {
    let bind_exchange: ExchangeType = ExchangeType::from_str(&exchange.to_string()).unwrap();

    let mut conn = pool.acquire().await.unwrap();
    let mut res: Vec<NaiveDate> = vec![];
    let mut rows = sqlx::query(&SQL).bind(bind_exchange).fetch(&mut *conn);
    while let Some(row) = rows.try_next().await? {
        // map the row into a user-defined domain type
        res.push(row.try_get("dt")?)
    }
    Ok(PageResulDt {
        dt: res,
        exchange: exchange,
    })
}
