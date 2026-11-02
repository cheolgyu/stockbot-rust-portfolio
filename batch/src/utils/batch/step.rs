use sqlx::{Pool, Postgres};
use tracing::{debug, info};

use super::{
    err::{ItemProcessorError, ItemReaderError, ItemWriterError, StepError},
    model::TbBatch,
};

use async_trait::async_trait;
pub struct StepBuilder<T: Step> {
    pub name: String,
    pub step: T,
}

impl<T> StepBuilder<T>
where
    T: Step,
{
    pub async fn run(
        &mut self,
        batch_id: i64,
        job_name: String,
        pool: &Pool<Postgres>,
    ) -> Result<(), StepError> {
        info!(
            "=================================== Run StepBuilder ={} =================================",
            &self.name
        );
        let mut item: TbBatch = TbBatch::default(batch_id, Some(job_name), Some(self.name.clone()));
        item.insert(pool).await;
        info!("---------- ---------- ---------- Run ItemReader  ");
        self.step.read(pool).await?;
        info!("---------- ---------- ----------  Run process  ");
        self.step.process(pool).await?;
        info!("---------- ---------- ---------- Run write  ");
        self.step.write(pool).await?;
        item.set_end_time();
        item.insert(pool).await;

        Ok(())
    }
}

#[async_trait]
pub trait Step {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError>;
    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError>;
    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError>;
}

// pub trait Step: Sized {
//     fn reader(&self, reader: &impl Reader) -> Result<bool, StepError>;
//     fn writer(&self, writer: &impl Writer) -> Result<bool, StepError>;
//     fn processor(&self, processor: &impl Processor) -> Result<bool, StepError>;
// }
