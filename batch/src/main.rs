#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

use crate::job::{
    agg_bound::job_config::JobAggBound,
    bound::job_config::JobBound,
    input_data::{file::fm, job_config::JobInputData},
};
use crate::utils::batch::batch::BatchBuilder;
use ::utils::{
    db::connection,
    env::{envload, envload_batch_plan},
};
use batch::{
    job::{
        agg_cp::job_config::JobAggCP, agg_vol::job_config::JobAggVol,
        rate_by_date::job_config::JobRateByDate,
    },
    utils,
};
use batch_model::app_type::{
    batch_run_type::{BatchParams, BatchType},
    country_type::CountryType,
};
use tracing::info;

mod agg;
mod downloader;
mod errors;
mod job;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // let subscriber = tracing_subscriber::fmt().with_timer(time::UtcTime::rfc_3339());

    envload();
    envload_batch_plan().unwrap();

    let client = batch_client::Client::new(None).unwrap();

    let plan: BatchParams = client.params.clone();
    info!("plan=>{:#?}", &plan);

    info!("Hello, world!");
    fm::mkdir();
    if BatchType::BATCH == plan.batch_type {
        if CountryType::KR == plan.country_type {
            batch_kr().await;
        }
    }

    // 인스턴스죽이라고 요청보내기.
    #[cfg(not(debug_assertions))]
    client.send_exit().await.unwrap();
    let op = say_world();
    println!("hello");
    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}

async fn batch_kr() {
    let pool = connection(50).await;

    let mut batch_builer = BatchBuilder::new(&pool).await;

    batch_builer.push_job(Box::new(JobInputData));
    batch_builer.push_job(Box::new(JobBound));
    batch_builer.push_job(Box::new(JobAggCP));
    batch_builer.push_job(Box::new(JobAggVol));
    batch_builer.push_job(Box::new(JobAggBound));
    batch_builer.push_job(Box::new(JobRateByDate));

    batch_builer.start(&pool).await;
}

async fn say_world() {
    println!("world");
}
#[cfg(test)]
mod tests {
    use std::env;
    use utils::env::envload;

    #[test]
    fn it_works() {
        envload();
        let _ = dotenvy::var("ADDR_DB_PG_PORT");
        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
