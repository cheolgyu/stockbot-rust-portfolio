use actix_web::{
    web::{self},
};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(by_code::handler);
}
pub mod by_code{

    use actix_web::{
        get,
        web::{self},
        HttpRequest, HttpResponse,
    };
    
    use crate::{dao::code_dao, DBPool};
    use tracing::log::{error};
 

    #[utoipa::path(
        context_path = "/api/v1",
        
        params(
            ("short_code" = String, Path, description = "short_code")
        ),
        responses(
            (status = 200, description = "code by 코드", body = String)
        ),
    )]
    #[get("/code/{short_code}")]
    pub async fn handler (
        pool: web::Data<DBPool>,
        req: HttpRequest,
        short_code: web::Path<String>,
    ) -> HttpResponse {
        println!("req.query_string()={:?}", req.query_string());
        println!("short_code={:?}", short_code);
    
        match code_dao::by_code(pool, short_code.into_inner()).await {
            Ok(list) => HttpResponse::Ok().json(list),
            Err(err) => {
                error!("err:{:?}",err);
                HttpResponse::InternalServerError().json(err.to_string())
            }
        }
    }
    
}