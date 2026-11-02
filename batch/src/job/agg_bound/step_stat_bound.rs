use crate::utils::batch::{
    err::{ItemProcessorError, ItemReaderError, ItemWriterError},
    step::Step,
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct StepAggBound {
    pub batch_id: i64,
}
#[async_trait]
impl Step for StepAggBound {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        sqlx::query("CALL agg.insert_bound_data();")
            .execute(pool)
            .await
            .unwrap();
        Ok(())
    }

    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        sqlx::query("CALL agg.insert_bound();")
            .execute(pool)
            .await
            .unwrap();

        Ok(())
    }

    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        Ok(())
    }
}
