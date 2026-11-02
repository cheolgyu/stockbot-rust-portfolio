use crate::utils::batch::{
    err::{ItemProcessorError, ItemReaderError, ItemWriterError},
    step::Step,
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct StepFind {}
#[async_trait]
impl Step for StepFind {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        Ok(())
    }

    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        sqlx::query(INSERT_HIST_FIND).execute(pool).await.unwrap();
        Ok(())
    }

    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        Ok(())
    }
}
const INSERT_HIST_FIND: &str = " call hist.insert_find()  ";
