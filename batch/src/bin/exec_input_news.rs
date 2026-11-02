// #![recursion_limit = "1024"]

// // Import the macro. Don't forget to add `error-chain` in your
// // `Cargo.toml`!
// #[macro_use]
// extern crate error_chain;

// use batch::{db::connection, news::job_config::JobInputNews};
// use crate::utils::batch::batch::BatchBuilder;
// use tracing::info;

#[tokio::main]
async fn main() {
    // let pool = connection().await;
    // tracing_subscriber::fmt::init();
    // info!("Hello, world!");

    // let mut batch_builer = BatchBuilder::new(&pool).await;

    // batch_builer.push_job(Box::new(JobInputNews));

    // batch_builer.start(&pool).await;

    let op = say_world();

    // This println! comes first
    println!("hello");

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}
async fn say_world() {
    println!("world");
}
