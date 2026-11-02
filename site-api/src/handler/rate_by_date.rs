use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list::handler).service(by_code::handler);
}

pub mod list {
    use crate::{dao::rate_by_date_dao::paging, DBPool};
    use actix_web::{
        get,
        web::{self, Query},
        HttpRequest, HttpResponse, Responder,
    };
    use batch_model::site::param::rate_by_date_list::Param;
    use tracing::log::error;

    #[utoipa::path(
        context_path = "/api/v1",
        params(
            Param
        ),
        responses(
            (status = 200, description = "일별 거래량 비율 목록" )
        )
    )]
    #[get("/rate_by_dt")]
    pub async fn handler(pool: web::Data<DBPool>, req: HttpRequest) -> impl Responder {
        println!("req.query_string()={:?}", req.query_string());

        let item = Query::<Param>::from_query(req.query_string());

        match item {
            Ok(v) => {
                let v = v.into_inner();
                match paging(pool, v).await {
                    Ok(list) => HttpResponse::Ok().json(list),
                    Err(err) => {
                        error!("err:{:?}", err);
                        HttpResponse::InternalServerError().json(err.to_string())
                    }
                }
            }
            Err(e) => {
                let res = format!("파라미터 확인:{:?}", e);
                println!("{}", &res);
                HttpResponse::InternalServerError().json(&res)
            }
        }
    }
}
pub mod by_code {
    use actix_web::{
        get,
        web::{self},
        HttpRequest, HttpResponse,
    };

    use crate::{
        dao::rate_by_date_dao::{self},
        DBPool,
    };
    use batch_model::site::param::rate_by_date_by_code::Param;
    use tracing::log::error;

    #[utoipa::path(
        context_path = "/api/v1",
        params(
            ("code_id" = String, Path, description = "code_id")
            ,Param
        ),
        responses(
            (status = 200, description = "일별 거래량 비율 by 코드", body = String)
        ),
    )]
    #[get("/rate_by_dt/{code_id}")]
    pub async fn handler(
        pool: web::Data<DBPool>,
        req: HttpRequest,
        web::Query(params): web::Query<Param>,
        code: web::Path<String>,
    ) -> HttpResponse {
        println!("req.query_string()={:?}", req.query_string());
        println!("params={:?}", params);

        match rate_by_date_dao::by_code_list(pool, code.to_string(), params).await {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(err) => {
                error!("err:{:?}", err);
                HttpResponse::InternalServerError().json(err.to_string())
            }
        }
    }
}
