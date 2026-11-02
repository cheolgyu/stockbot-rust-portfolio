#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

use batch::job::{
    agg_bound::job_config::JobAggBound, agg_cp::job_config::JobAggCP, agg_vol::job_config::JobAggVol, bound::job_config::JobBound, find::job_config::JobFind, input_data::{file::fm, job_config::JobInputData}, rate_by_date::job_config::JobRateByDate
};
use batch::utils::batch::batch::BatchBuilder;
use tracing::info;
use utils::{db::connection, env::envload};

#[tokio::main]
async fn main() {
    envload();

    let pool = connection(50).await;
    tracing_subscriber::fmt::init();
    info!("Hello, world!");
    fm::mkdir();

    let mut batch_builer = BatchBuilder::new(&pool).await;


    batch_builer.push_job(Box::new(JobInputData));
    batch_builer.push_job(Box::new(JobBound));
    batch_builer.push_job(Box::new(JobAggCP));
    batch_builer.push_job(Box::new(JobAggVol));
    batch_builer.push_job(Box::new(JobAggBound));
    batch_builer.push_job(Box::new(JobRateByDate));

    batch_builer.start(&pool).await;

    let op = say_world();

    println!("hello");

    op.await;
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
        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
