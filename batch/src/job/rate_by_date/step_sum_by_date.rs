use crate::utils::batch::{
    err::{ItemProcessorError, ItemReaderError, ItemWriterError},
    step::Step,
};
use async_trait::async_trait;

use sqlx::{query, Pool, Postgres};

pub struct StepSumByDate {}
#[async_trait]
impl Step for StepSumByDate {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        query(" select hist.upsert_sum_by_date();")
            .execute(pool)
            .await
            .unwrap();

        Ok(())
    }

    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        Ok(())
    }

    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        Ok(())
    }
}
