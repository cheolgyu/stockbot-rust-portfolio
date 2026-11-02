use crate::utils::batch::{err::JobError, job::JobRun, step::StepBuilder};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use super::step_insert::StepInsert;

pub struct JobInputNews;

#[async_trait]
impl JobRun for JobInputNews {
    async fn run(&self, batch_id: i64, pool: &Pool<Postgres>) -> Result<(), JobError> {
        //let step0: Step = StepDownloadCode {};
        let mut step1 = StepBuilder {
            name: "StepInsert".into(),
            step: StepInsert {},
        };

        step1.run(batch_id, self.get_name(), &pool).await.unwrap();

        Ok(())
    }

    fn get_name(&self) -> String {
        "JobInputNews".into()
    }
}
