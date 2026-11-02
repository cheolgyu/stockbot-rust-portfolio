use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use tracing::info;

use super::{err::JobError, model::TbBatch};

use super::step::Step;

pub struct Job<S: Step> {
    pub name: String,
    pub steps: Vec<S>,
}

#[async_trait]
pub trait JobRun {
    async fn run(&self, batch_id: i64, pool: &Pool<Postgres>) -> Result<(), JobError>;

    fn get_name(&self) -> String;

    async fn run_before(&self, item: TbBatch, pool: &Pool<Postgres>) {
        info!("start job {:?} ------------------------- ", item);

        item.insert(pool).await;
    }
    async fn run_after(&self, item: TbBatch, pool: &Pool<Postgres>) {
        info!("end job {:?} ------------------------- ", item);
        let mut item = item;
        item.set_end_time();
        item.insert(pool).await;
    }
    async fn exec(&self, pool: &Pool<Postgres>, batch_id: i64) -> Result<(), JobError> {
        let item: TbBatch = TbBatch::default(batch_id, Some(self.get_name()), None);
        info!("exec job {:?} ------------------------- ", item);
        self.run_before(item.clone(), pool).await;
        self.run(batch_id, pool).await?;
        self.run_after(item, pool).await;
        Ok(())
    }
}

// pub struct JobRepository<T: Step> {
//     pub jobs: Vec<JobBuilder<T>>,
// }

// impl<T> JobRepository<T>
// where
//     T: Step,
// {
//     pub fn run(&self) -> Result<bool, JobError> {
//         info!("Run JobRepository ");
//         for job in self.jobs.iter() {
//             job.run()?;
//         }

//         Ok(true)
//     }
// }
