
use chrono::{DateTime, Duration, Utc};
use chrono_tz::{Asia::Seoul, Tz};
use sqlx::{Pool, Postgres, Row};
use tracing::info;

use super::{job::JobRun, utils::get_run_time_sec};
const NEW_BATCH_SQL: &str = "INSERT INTO system.batch(
	start_time, end_time, run_time_sec)
	VALUES ($1, NULL, NULL) RETURNING id ; ";
const UPDATE_BATCH: &str = "UPDATE system.batch
SET id=$1, end_time=$2, run_time_sec=$3
WHERE id = $1; ";

pub struct BatchBuilder {
    pub id: i64,
    pub start_time: DateTime<Tz>,
    pub end_time: Option<DateTime<Tz>>,
    pub run_time_sec: Option<Duration>,
    pub jobs: Vec<Box<dyn JobRun + Sync>>,
}

// async fn job_exe<J>(j: &J, pool: &Pool<Postgres>, batch_id: i64)
// where
//     J: JobRun + std::marker::Sync,
// {
//     // JobRun::exec(j, pool, batch_id).await.unwrap();
//     j.exec(pool, batch_id).await.unwrap();
// }

impl BatchBuilder {
    pub async fn new(pool: &Pool<Postgres>) -> Self {
        let start_time: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
        let id = Self::get_new_id(pool, start_time).await;
        info!("배치를 생성합니다.. id={}, start_time={}", &id, &start_time);
        Self {
            id,
            start_time,
            end_time: None,
            run_time_sec: None,
            jobs: vec![],
        }
    }

    pub async fn start(&mut self, pool: &Pool<Postgres>) {
        info!("배치를 시작합니다. id={}", &self.id);
        for job in self.jobs.iter() {
            job.exec(pool, self.id.clone()).await.unwrap();
        }
        self.finish(pool).await;
    }

    pub fn push_job(&mut self, job: Box<dyn JobRun + Sync>) {
        self.jobs.push(job);
    }

    async fn finish(&mut self, pool: &Pool<Postgres>) {
        self.set_end_time().await;
        self.update_end_time(pool).await;
        info!(
            "배치가 종료됬습니다.. id={}, start_time={}, end_time={:?}, run_time_sec={:?}",
            &self.id, &self.start_time, &self.end_time, &self.run_time_sec
        );
    }

    async fn set_end_time(&mut self) {
        let end: DateTime<Tz> = Utc::now().with_timezone(&Seoul);

        self.run_time_sec = Some(get_run_time_sec(self.start_time, end));
        self.end_time = Some(end);
    }

    async fn get_new_id(pool: &Pool<Postgres>, start_time: DateTime<Tz>) -> i64 {
        let mut conn = pool.acquire().await.unwrap();
        let row = sqlx::query(NEW_BATCH_SQL)
            .bind(&start_time)
            .fetch_one(&mut *conn)
            .await
            .unwrap();
        let id: i64 = row.try_get("id").unwrap();
        id
    }

    async fn update_end_time(&mut self, pool: &Pool<Postgres>) {
        let mut conn: sqlx::pool::PoolConnection<Postgres> = pool.acquire().await.unwrap();
        sqlx::query(UPDATE_BATCH)
            .bind(self.id)
            .bind(self.end_time)
            .bind(self.run_time_sec)
            .execute(&mut *conn)
            .await
            .unwrap();
    }
}
