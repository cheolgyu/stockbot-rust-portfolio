use std::error::Error;

use batch_model::price::PriceWithConvertDate;
use chrono::NaiveDate;
use dotenvy::var;
use sqlx::{query_as, Pool, Postgres};

use crate::job::{bound::code_start_dt::CodeStartDt, utils::get_default_start_dt};

////////////////////////////////////////////////////////////////////////////////
///
///
////////////////////////////////////////////////////////////////////////////////
const SELECT_HIST_PRICE_AFTER_P2X_SQL: &str = "
SELECT
	P.CODE_ID,
	P.DT,
	X.TICK AS X_TICK,
	OP,
	HP,
	LP,
	CP
FROM
	CODE C
	LEFT JOIN HIST.PRICE P ON C.ID = P.CODE_ID
	Right JOIN TICK.X_TICK X ON P.DT = X.DT
	AND X.EXCHANGE = C.EXCHANGE
WHERE
	C.ID = $1
	AND P.DT > $2
ORDER BY
	P.DT ASC
    ";

pub async fn get_price_after_p1x(
    pool: Pool<Postgres>,
    code_id: i64,
    p1x: NaiveDate,
) -> Result<Vec<PriceWithConvertDate>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let stream = sqlx::query_as::<_, PriceWithConvertDate>(SELECT_HIST_PRICE_AFTER_P2X_SQL)
        .bind(code_id)
        .bind(p1x)
        .fetch_all(&mut *conn)
        .await?;
    conn.close().await?;
    Ok(stream)
}
////////////////////////////////////////////////////////////////////////////////
///
///
////////////////////////////////////////////////////////////////////////////////
const GET_CODES_WITH_START_DT: &str = "
SELECT C.ID
    ,C.EXCHANGE
    ,COALESCE(MAX(CASE WHEN LINE = 'CLOSE' THEN P1X END), $1) AS  close
    ,COALESCE(MAX(CASE WHEN LINE = 'OPEN' THEN P1X END), $1) AS   open
    ,COALESCE(MAX(CASE WHEN LINE = 'LOW' THEN P1X END), $1) AS    low
    ,COALESCE(MAX(CASE WHEN LINE = 'HIGH' THEN P1X END), $1) AS   high
FROM  CODE C  LEFT JOIN HIST.BOUND H ON C.ID = H.CODE_ID
GROUP BY C.ID ORDER BY C.ID ASC
";

pub async fn hist_code_with_start_dt_list(
    pool: &Pool<Postgres>,
) -> Result<Vec<CodeStartDt>, Box<dyn Error>> {
    let default_start_dt = get_default_start_dt()?;
    let mut conn = pool.acquire().await?;
    let res = query_as::<_, CodeStartDt>(GET_CODES_WITH_START_DT)
        .bind(default_start_dt)
        .fetch_all(&mut *conn)
        .await?;
    conn.close().await?;
    Ok(res)
}
