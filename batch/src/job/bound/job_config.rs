use crate::{
    job::bound::{
        step_bound_upsert::StepBoundUpsert, step_bound_view_tmp::StepBoundView, step_tick::StepTick,
    },
    utils::batch::{err::JobError, job::JobRun, step::StepBuilder},
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct JobBound;

#[async_trait]
impl JobRun for JobBound {
    async fn run(&self, batch_id: i64, pool: &Pool<Postgres>) -> Result<(), JobError> {
        let mut step1 = StepBuilder {
            name: "SET X_TICK".into(),
            step: StepTick {},
        };
        step1.run(batch_id, self.get_name(), &pool).await.unwrap();

        let mut step2 = StepBuilder {
            name: "make bound and insert hist.bound".into(),
            step: StepBoundUpsert { code: vec![] },
        };
        step2.run(batch_id, self.get_name(), &pool).await.unwrap();

        let mut step3 = StepBuilder {
            name: "insert bound_view_tmp".into(),
            step: StepBoundView {},
        };

        step3.run(batch_id, self.get_name(), &pool).await.unwrap();
        Ok(())
    }

    fn get_name(&self) -> String {
        "JobBound".into()
    }
}
