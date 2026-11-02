use crate::{
    job::agg_vol::step_agg_vol::StepAggVol,
    utils::batch::{err::JobError, job::JobRun, step::StepBuilder},
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct JobAggVol;

#[async_trait]
impl JobRun for JobAggVol {
    async fn run(&self, batch_id: i64, pool: &Pool<Postgres>) -> Result<(), JobError> {
        let mut step1 = StepBuilder {
            name: "StepAggVol".into(),
            step: StepAggVol { batch_id },
        };

        step1.run(batch_id, self.get_name(), &pool).await.unwrap();

        Ok(())
    }

    fn get_name(&self) -> String {
        "JobAggVol".into()
    }
}
