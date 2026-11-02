use actix_web::web::{self};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list::handler).service(by_code::handler);
}

pub mod list {
    use crate::{dao::find_dao, DBPool};
    use actix_web::{
        get,
        web::{self},
        HttpRequest, HttpResponse,
    };

    use batch_model::site::param::find_list::Param;
    use tracing::log::{error, info};

    #[utoipa::path(
        context_path = "/api/v1",
        // 
        params(
            Param
        ),
        responses(
            (status = 200, description = "find 목록"  )
        )
    )]
    #[get("/find")]
    pub async fn handler(
        pool: web::Data<DBPool>,
        req: HttpRequest,
        //web::Query(params): web::Query<ParamsFind>,
    ) -> HttpResponse {
        info!("req.query_string()={:?}", req.query_string());
        //    info!("params={:?}", params);

        let item = web::Query::<Param>::from_query(req.query_string());
        match item {
            Ok(params) => {
                let v = params.into_inner();
                match find_dao::list(pool, v).await {
                    Ok(list) => HttpResponse::Ok().json(list),
                    Err(err) => {
                        error!("err:{:?}", err);
                        HttpResponse::InternalServerError().json(err.to_string())
                    }
                }
            }
            Err(e) => {
                let res = format!("파라미터 확인:{:?}", e);
                info!("{}", &res);
                HttpResponse::InternalServerError().json(&res)
            }
        }
    }
}

pub mod by_code {

    use crate::{dao::find_dao, DBPool};
    use actix_web::{
        get,
        web::{self},
        HttpRequest, HttpResponse,
    };
    use batch_model::site::param::find_by_code::Param;
    use tracing::log::error;

    #[utoipa::path(
        context_path = "/api/v1",
        params(
            ("code_id" = String, Path, description = "code_id")
            ,Param
        ),
        responses(
            (status = 200, description = "거래량 by 코드", body = String)
        ),
    )]
    #[get("/find/{code_id}")]
    pub async fn handler(
        pool: web::Data<DBPool>,
        req: HttpRequest,
        web::Query(params): web::Query<Param>,
        code: web::Path<String>,
    ) -> HttpResponse {
        println!("req.query_string()={:?}", req.query_string());
        println!("params={:?}", params);

        match find_dao::by_code_list(pool, code.to_string(), params).await {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(err) => {
                error!("err:{:?}", err);
                HttpResponse::InternalServerError().json(err.to_string())
            }
        }
    }
}
