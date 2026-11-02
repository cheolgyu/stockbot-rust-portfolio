use actix_web::web;



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(by_code::handler);
}

pub mod by_code {
    use actix_web::{
        get,
        web::{self},
        HttpRequest, HttpResponse,
    };
    
    use tracing::log::{error};
    use crate::{dao::price_dao, DBPool};
    use batch_model::{site::param::{price_by_code::Param}};
    

    #[utoipa::path(
        context_path = "/api/v1",
        
        params(
            ("code_id" = String, Path, description = "code_id")
            ,Param
        ),
        responses(
            (status = 200, description = "price by 코드", body = String)
        ),
    )]
    #[get("/price/{code_id}")]
    pub async fn handler(
        pool: web::Data<DBPool>,
        req: HttpRequest,
        web::Query(params): web::Query<Param>,
        code: web::Path<String>,
    ) -> HttpResponse {
        println!("req.query_string()={:?}", req.query_string());
        println!("params={:?}", params);
    
        match price_dao::list(pool, code.into_inner(), params).await {
            Ok(list) => HttpResponse::Ok().json(list),
            Err(err) => {
                error!("err:{:?}",err);
                HttpResponse::InternalServerError().json(err.to_string())
            }
        }
    }
    
}