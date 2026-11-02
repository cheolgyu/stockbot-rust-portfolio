use crate::utils::batch::{err::JobError, job::JobRun, step::StepBuilder};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use utils::db::connection;

use super::step_save::StepSave;

pub struct JobSaveDisclosure;

#[async_trait]
impl JobRun for JobSaveDisclosure {
    async fn run(&self, batch_id: i64, pool: &Pool<Postgres>) -> Result<(), JobError> {
        //let step0: Step = StepDownloadCode {};
        let mut step1 = StepBuilder {
            name: "공시저장".into(),
            step: StepSave {},
        };

        step1.run(batch_id, self.get_name(), &pool).await.unwrap();

        Ok(())
    }

    fn get_name(&self) -> String {
        "JobSaveDisclosure".into()
    }
}
