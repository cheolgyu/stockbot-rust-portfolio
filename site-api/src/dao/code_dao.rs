use actix_web::web;

use batch_model::code::view::list::CodeViewList;
use batch_model::site::res::PageResultOne;
use batch_model::tb_row::TbRow;

use crate::errors::*;
use crate::DBPool;
const SQL: &str = "select * 
    from public.VIEW_CODE where SHORT_CODE = $1";

pub async fn by_code(
    pool: web::Data<DBPool>,
    short_code: String,
) -> Result<PageResultOne<CodeViewList>> {
    let mut conn = pool.acquire().await.unwrap();

    let row = sqlx::query_as::<_, CodeViewList>(&SQL)
        .bind(short_code)
        .fetch_one(&mut *conn)
        .await?;
    let res: CodeViewList = row.into();

    Ok(PageResultOne {
        data: res,
        tb_nm: vec![],
    })
}
