use actix_web::web::ServiceConfig;



pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(list::handler).service(code_detail::handler);
}

pub mod list{
    use actix_web::{HttpRequest, web::{self, Query}, HttpResponse, get};
    
   use tracing::log::{error};
    
    use batch_model::{site::param::bound_list::Param};
    use crate::{DBPool, dao::bound_dao};


 
    #[utoipa::path(
        context_path = "/api/v1",
        
        params(
            Param
        ),
        responses(
            (status = 200, description = "바운드 목록" )
        )
    )]
    #[get("/bound")]
    pub async fn handler(pool: web::Data<DBPool>, req: HttpRequest) -> HttpResponse {
        println!("req.query_string()={:?}", req.query_string());

        let item = Query::<Param>::from_query(req.query_string());

            

        match item {
            Ok(v) => {
                let v = v.into_inner();
                let (valid,msg) = v.exchange.valid();
                
                if valid {
                    match bound_dao::paging(pool, v).await {
                        Ok(list) => {
                            HttpResponse::Ok().json(list)
                        }
                        Err(err) => {
                            error!("err:{:?}",err);
                            HttpResponse::InternalServerError().json(err.to_string())
                        }
                    }
                }else{
                    HttpResponse::BadRequest().json(msg)
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
pub mod code_detail{
    use actix_web::{HttpRequest, web::{Data,  Query}, HttpResponse, get};
    use batch_model::{site::param::{bound_by_code::Param}};
    use tracing::log::error;
    use crate::{DBPool, dao::bound_dao};




    #[utoipa::path(
        context_path = "/api/v1",
        
        params(
            ("code_id" = String, Path, description = "code_id")
            ,Param
        ),
        responses(
            (status = 200, description = "bound by 코드", body = String)
        ),
    )]
    #[get("/bound/{code_id}")]
    pub async fn handler(
        pool: Data<DBPool>,
        req: HttpRequest,
        params: Query<Param>,
    ) -> HttpResponse {
        println!("req.query_string()={:?}", req.query_string());
        println!("params={:?}", params);
    
        match bound_dao::bound_code_detail(pool, params.into_inner()).await {
            Ok(list) => HttpResponse::Ok().json(list),
            Err(err) => {
                error!("err:{:?}",err);
                HttpResponse::InternalServerError().json(err.to_string())
            }
        }
    }
}

