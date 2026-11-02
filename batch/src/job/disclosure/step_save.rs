use crate::utils::batch::{
    err::{ItemProcessorError, ItemReaderError, ItemWriterError},
    step::Step,
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct StepSave {}
#[async_trait]
impl Step for StepSave {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        Ok(())
    }

    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        Ok(())
    }

    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        Ok(())
    }
}
