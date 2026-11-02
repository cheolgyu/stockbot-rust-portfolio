#[cfg(test)]
mod tests {
    use tracing::info;

    use crate::utils::batch::{
        err::*,
        step::{Step, StepBuilder},
    };

    // struct StepMakeTBCode {}

    // impl Step for StepMakeTBCode {
    //     fn read(&self) -> Result<bool, ItemReaderError> {
    //         println!("http 코드목록 파일 다운로드");
    //         Ok(true)
    //     }

    //     fn write(&self) -> Result<bool, ItemWriterError> {
    //         println!("코드테이블에 저장");
    //         Ok(true)
    //     }

    //     fn process(&self) -> Result<bool, ItemProcessorError> {
    //         println!(
    //             "다운로드한 코드목록 특수문자 변환  및 새로운 코드인지 파악, 새로운 코드만 저장"
    //         );
    //         Ok(true)
    //     }
    // }

    // struct StepMakeTbPrice {}

    // impl Step for StepMakeTbPrice {
    //     fn read(&self) -> Result<bool, ItemReaderError> {
    //         println!("HTTP 요청 코드별 가격 목록 파일로 저장");
    //         Ok(true)
    //     }

    //     fn write(&self) -> Result<bool, ItemWriterError> {
    //         println!("코드별 가격 목록을 가격테이블에 저장");
    //         Ok(true)
    //     }

    //     fn process(&self) -> Result<bool, ItemProcessorError> {
    //         println!("코드별 가격 목록 변환??????????????");
    //         Ok(true)
    //     }
    // }

    #[test]
    fn it_works() {
        tracing_subscriber::fmt::init();
        info!("dddddddddddddddddddddd");

        // let step1 = StepBuilder {
        //     name: "STEP_코드목록_만들기".into(),
        //     step: Box::new(StepMakeTBCode {}),
        // };
        // let step2 = StepBuilder {
        //     name: "STEP_코드별_가격목록_만들기".into(),
        //     step: Box::new(StepMakeTbPrice {}),
        // };

        // let job = JobBuilder {
        //     name: "데이터 만들기".into(),
        //     steps: vec![step1, step2],
        // };
        // let jobresp = JobRepository { jobs: vec![job] };

        // for job in jobresp.jobs.iter() {
        //     job.run().unwrap();
        // }

        assert_eq!(2 + 2, 4);
    }
}
