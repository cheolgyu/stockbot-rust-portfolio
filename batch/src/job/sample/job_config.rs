use crate::utils::batch::{err::JobError, job::JobRun, step::StepBuilder};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use utils::db::connection;

use super::step_::StepSample;

pub struct JobInputData;

#[async_trait]
impl JobRun for JobInputData {
    async fn run(&self, batch_id: i64, pool: &Pool<Postgres>) -> Result<(), JobError> {
        //let step0: Step = StepDownloadCode {};
        let mut step1 = StepBuilder {
            name: "StepSample".into(),
            step: StepSample {batch_id},
        };

        step1.run(batch_id, self.get_name(), &pool).await.unwrap();

        Ok(())
    }

    fn get_name(&self) -> String {
        "JobInputData".into()
    }
}
