use crate::{
    job::input_data::{
        file::FileInfo, setp_price_naver::StepPirceNaver, step_price_chg::StepPirceChg,
    },
    utils::batch::{
        err::JobError,
        job::JobRun,
        step::{Step, StepBuilder},
    },
};
use async_trait::async_trait;

use batch_model::app_type::country_type::CountryType;
use sqlx::{Pool, Postgres};

use super::{setp_code_detail::StepCodeDetail, setp_code_state::StepCodeState};

pub struct JobInputData;

#[async_trait]
impl JobRun for JobInputData {
    async fn run(&self, batch_id: i64, pool: &Pool<Postgres>) -> Result<(), JobError> {
        let fi = FileInfo::new();
        let kr_price_dir = fi.kr_price_dir;

        let mut step1 = StepBuilder {
            name: "code_detail and code".into(),
            step: StepCodeDetail { data: None },
        };
        step1.run(batch_id, self.get_name(), &pool).await.unwrap();

        let mut step2 = StepBuilder {
            name: "code_state".into(),
            step: StepCodeState { data: None },
        };
        step2.run(batch_id, self.get_name(), &pool).await.unwrap();

        let mut step3 = StepBuilder {
            name: "price".into(),
            step: StepPirceNaver {
                batch_id: batch_id,
                country: CountryType::KR,
                code_start_dt: vec![],
                end_dt: None,
                kr_price_dir: kr_price_dir,
            },
        };
        step3.run(batch_id, self.get_name(), &pool).await.unwrap();

        let mut step4 = StepBuilder {
            name: "price".into(),
            step: StepPirceChg {},
        };
        step4.run(batch_id, self.get_name(), &pool).await.unwrap();

        Ok(())
    }

    fn get_name(&self) -> String {
        "JobInputData".into()
    }
}
