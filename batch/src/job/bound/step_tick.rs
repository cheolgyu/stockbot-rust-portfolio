use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::utils::batch::{
    err::{ItemProcessorError, ItemReaderError, ItemWriterError},
    step::Step,
};

pub struct StepTick {}

#[async_trait]
impl Step for StepTick {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        Ok(())
    }

    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        Ok(())
    }

    /*
        make codes_insert
    */
    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        //////////////////////////////////////////////////////
        sqlx::query("CALL TICK.UPSERT_X_TICK ();")
            .execute(pool)
            .await
            .unwrap();

        Ok(())
    }
}
