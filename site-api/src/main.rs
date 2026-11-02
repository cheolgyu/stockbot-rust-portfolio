// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
use std::env::var;

use actix_cors::Cors;

use actix_web::{
    get,
    middleware::Logger,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use batch_model::app_type::{
    batch_run_type::{BatchParams, BatchType, RunType},
    country_type::CountryType,
};
use chrono::{DateTime, Local};
use env_logger::Env;
use sqlx::PgPool;
use tracing::info;
use utils::{db::connection, env::envload};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

pub type DBPool = PgPool;
pub mod dao;
mod errors;
pub mod handler;

//https://actix.rs/docs/middleware/
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    envload();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    //tracing_subscriber::fmt::init();

    let allow_frontend_addr = var("ALLOW_FRONTEND_ADDR").unwrap();
    let allow_backend_addr = var("ALLOW_BACKEND_ADDR").unwrap();

    let addr_api_addr = var("ADDR_API_ADDR").unwrap();
    let addr_api_port = var("ADDR_API_PORT")
        .unwrap()
        .as_str()
        .parse::<u16>()
        .unwrap();
    info!("서버 시작 -IPv4 : {}:{}", addr_api_addr, addr_api_port);
    info!("서버 시작 -IPv4 : {}:{}", addr_api_addr, addr_api_port);

    let addr_ipv6 = format!("[::1]:{}", addr_api_port);
    info!("서버 시작 -IPv6=>{}", addr_ipv6);
    #[derive(OpenApi)]
    #[openapi(
        paths(
            ping,
            handler::price_chg::list::handler,
            handler::price_chg::by_code::handler,
            handler::bound::list::handler,
            handler::bound::code_detail::handler,
            handler::agg_vol::list::handler,
            handler::agg_vol::by_code::handler,
            handler::agg_cp::list::handler,
            handler::agg_cp::by_code::handler,
            handler::agg_bound::list::handler,
            handler::agg_bound::by_code::handler,
            handler::find::list::handler,
            handler::find::by_code::handler,
            handler::price::by_code::handler,
            handler::utils::open_dt::handler,
            handler::utils::exchange::handler,
            handler::system::log_batch::handler,
            handler::system::health_check::handler,
            handler::batch::action::start_cmd,
            handler::batch::action::exit_cmd,
            handler::batch::batch_timer_plan::handler,
            handler::rate_by_date::list::handler,
            handler::rate_by_date::by_code::handler,
        ),
        components(schemas(BatchParams, BatchType, CountryType, RunType,))
    )]
    struct ApiDoc1;

    let pool: DBPool = connection(10).await;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    // .allowed_origin("http://127.0.0.1:8080")
                    // .allowed_origin("http://localhost:8080")
                    // .allowed_origin("http://127.0.0.1:8001")
                    // .allowed_origin("http://localhost:8001")
                    // .allowed_origin("http://[::1]:80")
                    // .allowed_origin("http://[::0]:80")
                    // .allowed_origin(&allow_frontend_addr)
                    // .allowed_origin(&allow_backend_addr)
                    .allowed_origin("*")
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(ping)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![(
                Url::new("api1", "/api-docs/openapi1.json"),
                ApiDoc1::openapi(),
            )]))
            .service(
                web::scope("/api/v1")
                    // .guard(guard::Header("content-type", "application/json"))
                    .configure(handler::bound::config)
                    .configure(handler::price_chg::config)
                    .configure(handler::agg_bound::config)
                    .configure(handler::agg_vol::config)
                    .configure(handler::agg_cp::config)
                    .configure(handler::find::config)
                    .configure(handler::price::config)
                    .configure(handler::code::config)
                    .configure(handler::utils::config)
                    .configure(handler::system::config)
                    .configure(handler::rate_by_date::config)
                    .configure(handler::batch::config),
            )
    })
    .bind((addr_api_addr, addr_api_port))?
    .bind(addr_ipv6)?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    let now_local = Local::now();
    let res = format!("Hello world!={now_local}");
    HttpResponse::Ok().body(res)
}

#[utoipa::path(
    responses(
        (status = 200, description = "ping", body = String)
    )
)]
#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("ping 핑12퐁1ss핑1퐁")
}

mod api1 {
    use actix_web::get;

    #[utoipa::path(
        responses(
            (status = 200, description = "Hello from ap i 1", body = String)
        )
    )]
    #[get("/hello")]
    pub(super) async fn hello1() -> String {
        "hello fr   om api 1".to_string()
    }
}
