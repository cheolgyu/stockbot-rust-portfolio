use actix_web::web;

use batch_model::app_type::exchange_type::ExchangeType;
use futures::TryStreamExt;
use sqlx::Row;

use crate::errors::*;
use crate::DBPool;
const SQL: &str = "select unnest(enum_range(NULL::exchange_type));";

pub async fn exchange(pool: web::Data<DBPool>) -> Result<Vec<ExchangeType>> {
    let mut conn = pool.acquire().await.unwrap();

    let mut res: Vec<ExchangeType> = vec![];
    let mut rows = sqlx::query(&SQL).fetch(&mut *conn);
    while let Some(row) = rows.try_next().await? {
        let e: ExchangeType = row.try_get("unnest")?;
        res.push(e)
    }
    Ok(res)
}
