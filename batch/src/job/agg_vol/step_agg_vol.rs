use std::sync::Arc;

use crate::utils::batch::{
    err::{ItemProcessorError, ItemReaderError, ItemWriterError},
    step::Step,
};
use async_trait::async_trait;
use batch_model::app_type::{obs_type::ObsType, ObsPeriod};
use sqlx::{Pool, Postgres};
use tokio::task::JoinHandle;
use tracing::info;

pub struct StepAggVol {
    pub batch_id: i64,
}

#[async_trait]
impl Step for StepAggVol {
    async fn read(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemReaderError> {
        sqlx::query("CALL agg.insert_vol_data();")
            .execute(pool)
            .await
            .unwrap();
        Ok(())
    }

    async fn process(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemProcessorError> {
        sqlx::query("CALL agg.insert_vol();")
            .execute(pool)
            .await
            .unwrap();

        Ok(())
    }

    async fn write(&mut self, pool: &Pool<Postgres>) -> Result<(), ItemWriterError> {
        Ok(())
    }
}
