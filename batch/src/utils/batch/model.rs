use chrono::{DateTime, Duration, Utc};
use chrono_tz::{Asia::Seoul, Tz};
use sqlx::{query, Pool, Postgres};

use super::utils::get_run_time_sec;
#[derive(Clone, Debug)]
pub struct TbBatch {
    batch_id: i64,
    pub job_name: Option<String>,
    pub step_name: Option<String>,
    pub start_time: DateTime<Tz>,
    pub end_time: Option<DateTime<Tz>>,
    pub run_time_sec: Option<Duration>,
}

impl TbBatch {
    pub fn default(batch_id: i64, job_name: Option<String>, step_name: Option<String>) -> Self {
        let seoul_now: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
        Self {
            batch_id,
            job_name: job_name,
            step_name: step_name,
            start_time: seoul_now,
            end_time: None,
            run_time_sec: None,
        }
    }

    pub fn set_end_time(&mut self) {
        let end: DateTime<Tz> = Utc::now().with_timezone(&Seoul);

        self.run_time_sec = Some(get_run_time_sec(self.start_time, end));
        self.end_time = Some(end);
    }

    pub async fn insert(&self, pool: &Pool<Postgres>) {
        let mut conn: sqlx::pool::PoolConnection<Postgres> = pool.acquire().await.unwrap();
        query(INSERT)
            .bind(self.batch_id)
            .bind(self.job_name.clone())
            .bind(self.step_name.clone())
            .bind(self.start_time)
            .bind(self.end_time)
            .bind(self.run_time_sec)
            .execute(&mut *conn)
            .await
            .unwrap();
    }
}
const INSERT: &str = "INSERT INTO SYSTEM.BATCH_STEP(
	batch_id, job_name, step_name, start_time, end_time, run_time_sec)
	VALUES ($1, $2, $3, $4, $5, $6);";
