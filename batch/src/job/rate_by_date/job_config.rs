use crate::{
    job::rate_by_date::{step_rate_by_date::StepRateByDate, step_sum_by_date::StepSumByDate},
    utils::batch::{err::JobError, job::JobRun, step::StepBuilder},
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct JobRateByDate;

#[async_trait]
impl JobRun for JobRateByDate {
    async fn run(&self, batch_id: i64, pool: &Pool<Postgres>) -> Result<(), JobError> {
        let mut step1 = StepBuilder {
            name: "StepSumByDate".into(),
            step: StepSumByDate {},
        };

        step1.run(batch_id, self.get_name(), &pool).await.unwrap();

        let mut step2 = StepBuilder {
            name: "StepRateByDate".into(),
            step: StepRateByDate {
                batch_id,
                code: Vec::new(),
            },
        };

        step2.run(batch_id, self.get_name(), &pool).await.unwrap();

        Ok(())
    }

    fn get_name(&self) -> String {
        "JobInputData".into()
    }
}
