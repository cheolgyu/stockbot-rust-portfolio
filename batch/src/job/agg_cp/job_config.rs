use crate::{
    job::agg_cp::step_agg_cp::StepAggCP,
    utils::batch::{err::JobError, job::JobRun, step::StepBuilder},
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct JobAggCP;

#[async_trait]
impl JobRun for JobAggCP {
    async fn run(&self, batch_id: i64, pool: &Pool<Postgres>) -> Result<(), JobError> {
        let mut step1 = StepBuilder {
            name: "StepAggCP".into(),
            step: StepAggCP { batch_id },
        };

        step1.run(batch_id, self.get_name(), &pool).await.unwrap();

        // let mut step1 = StepBuilder {
        //     name: "StepAggPrice".into(),
        //     step: StepAggPrice { batch_id },
        // };

        // step1.run(batch_id, self.get_name(), &pool).await.unwrap();

        Ok(())
    }

    fn get_name(&self) -> String {
        "JobAggCP".into()
    }
}
