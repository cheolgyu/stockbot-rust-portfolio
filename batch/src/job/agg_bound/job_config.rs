use crate::{
    job::agg_bound::step_stat_bound::StepAggBound,
    utils::batch::{err::JobError, job::JobRun, step::StepBuilder},
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct JobAggBound;

#[async_trait]
impl JobRun for JobAggBound {
    async fn run(&self, batch_id: i64, pool: &Pool<Postgres>) -> Result<(), JobError> {
        let mut step1 = StepBuilder {
            name: "StepAggBound".into(),
            step: StepAggBound { batch_id },
        };

        step1.run(batch_id, self.get_name(), &pool).await.unwrap();

        Ok(())
    }

    fn get_name(&self) -> String {
        "JobAggBound".into()
    }
}
