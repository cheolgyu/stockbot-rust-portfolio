use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(open_dt::handler).service(exchange::handler);
}
pub mod open_dt {
    use actix_web::{
        get,
        web::{self},
        HttpRequest, HttpResponse,
    };
    use batch_model::{site::param::open_dt::Param};
    use tracing::log::{error};
    

    use crate::{dao::utils_dao::get_list_dt, DBPool};


    #[utoipa::path(
        context_path = "/api/v1",
        
        params(Param),
        responses(
            (status = 200, description = "open_dt 목록", body = String)
        ),
    )]
    #[get("/utils/open_dt_list")]
    pub async fn handler(
        pool: web::Data<DBPool>,
        req: HttpRequest,
        web::Query(params): web::Query<Param>,
    ) -> HttpResponse {
        println!("req.query_string()={:?}", req.query_string());
        println!("params={:?}", params);

        match get_list_dt(pool, params.exchange).await {
            Ok(list) => HttpResponse::Ok().json(list),
            Err(err) => {
                error!("err:{:?}",err);
                HttpResponse::InternalServerError().json(err.to_string())
            }
        }
    }
}

pub mod exchange {
    use actix_web::{
        get,
        web::{self},
        HttpRequest, HttpResponse,
    };
    use tracing::log::{ error};
    use crate::{dao::enum_type::exchange, DBPool};

    #[utoipa::path(
        context_path = "/api/v1",
        
        responses(
            (status = 200, description = "exchange 목록", body = String)
        ),
    )]
    #[get("/utils/exchange_list")]
    pub async fn handler(pool: web::Data<DBPool>, _req: HttpRequest) -> HttpResponse {
        match exchange(pool).await {
            Ok(list) => HttpResponse::Ok().json(list),
            Err(err) => {
                error!("err:{:?}",err);
                HttpResponse::InternalServerError().json(err.to_string())
            }
        }
    }
}
