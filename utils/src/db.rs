use log::{self, LevelFilter};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use std::time::Duration;
use tokio::time::timeout;
pub async fn connection(max_pool: u32) -> PgPool {
    let package_name = env!("CARGO_PKG_NAME");
    let port: u16 = dotenvy::var("ADDR_DB_PG_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let options = PgConnectOptions::new()
        .host(&dotenvy::var("ADDR_DB_PG_HOST").unwrap())
        .port(port)
        .username(&dotenvy::var("ADDR_DB_PG_USERNAME").unwrap())
        .password(&dotenvy::var("ADDR_DB_PG_PASSWORD").unwrap())
        .database(&dotenvy::var("ADDR_DB_PG_DATABASE").unwrap())
        .application_name(package_name)
        // https://github.com/launchbadge/sqlx/issues/2131#issuecomment-1264685776
        // 로그 출력하고싶은면  options.disable_statement_logging() 주석처리하기.
        .disable_statement_logging()
        .log_slow_statements(LevelFilter::Off, Duration::ZERO);

    let pg_pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(30))
        //.max_connections(max_pool)
        .connect_with(options)
        .await
        .unwrap();

    pg_pool
}
