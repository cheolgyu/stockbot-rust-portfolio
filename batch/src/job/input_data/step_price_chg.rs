use crate::utils::batch::{
    err::{ItemProcessorError, ItemReaderError, ItemWriterError},
    step::Step,
};

use async_trait::async_trait;

use sqlx::{Executor, Pool, Postgres};

pub struct StepPirceChg {}

#[async_trait]
impl Step for StepPirceChg {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        pool.execute(" call hist.insert_price_chg(); ")
            .await
            .unwrap();
        Ok(())
    }

    async fn write(&mut self, _pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        Ok(())
    }

    async fn process(&mut self, _pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        Ok(())
    }
}
mod tests {}
