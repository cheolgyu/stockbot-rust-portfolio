use actix_web::{
    web::{self},
};



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check::handler).service(log_batch::handler);
}

pub mod health_check{

    use actix_web::{
        get,
        web::{self},
        HttpRequest, HttpResponse,
    };
    
    use crate::{dao::{system_dao::{batch_exe::select_health_check}}, DBPool};
    use tracing::log::{error};
    #[utoipa::path(
        context_path = "/api/v1",
        responses(
            (status = 200, description = "batch_exe health_check")
        )
    )]
    #[get("/system/health/batch_timer")]
    pub async fn handler(pool: web::Data<DBPool>, _req: HttpRequest) -> HttpResponse {
        
        match select_health_check(pool).await {
            Ok(res) => {
                HttpResponse::Ok().json(res)
            }
            Err(err) => {
                error!("err:{:?}",err);
                HttpResponse::InternalServerError().json(err.to_string())
            }
        }
    }
    
}

pub mod log_batch{

    use actix_web::{
        get,
        web::{self, Query},
        HttpRequest, HttpResponse,
    };
    use batch_model::{site::param::{log_batch::Param}};
    use crate::{dao::{system_dao::{self}}, DBPool};
    use tracing::log::{error};




    #[utoipa::path(
        context_path = "/api/v1",
        
        params(
            Param
        ),
        responses(
            (status = 200, description = "로그 배치")
        )
    )]
    #[get("/system/logs/batch")]
    pub async fn handler(pool: web::Data<DBPool>, req: HttpRequest) -> HttpResponse {
        println!("req.query_string()={:?}", req.query_string());
    
        let item = Query::<Param>::from_query(req.query_string());
    
        match item {
            Ok(v) => {
                let v = v.into_inner();
                match system_dao::paging_batch(pool, v).await {
                    Ok(list) => {
                        // info!("========================================================");
                        // info!("결과값확인:{:?}", &list);
                        HttpResponse::Ok().json(list)
                    }
                    Err(err) => {
                        error!("err:{:?}",err);
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
