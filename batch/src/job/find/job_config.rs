use crate::utils::batch::{err::JobError, job::JobRun, step::StepBuilder};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use super::step_find::StepFind;

pub struct JobFind;

#[async_trait]
impl JobRun for JobFind {
    async fn run(&self, batch_id: i64, pool: &Pool<Postgres>) -> Result<(), JobError> {
        //let step0: Step = StepDownloadCode {};
        let mut step1 = StepBuilder {
            name: "StepFind".into(),
            step: StepFind {},
        };

        step1.run(batch_id, self.get_name(), &pool).await.unwrap();

        Ok(())
    }

    fn get_name(&self) -> String {
        "JobFind".into()
    }
}
