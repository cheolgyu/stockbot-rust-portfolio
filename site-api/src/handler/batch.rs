use actix_web::{
    web::{self},
    HttpRequest, HttpResponse, post,
};

use tracing::log::{error};
use crate::{DBPool};



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(action::start_cmd).service(action::exit_cmd).service(batch_timer_plan::handler);
}
pub mod action { 
    
    use actix_web::{delete};
    use batch_model::app_type::batch_run_type::BatchParams;
    use batch_model::app_type::batch_run_type::RunType;
    use tracing::info;
    use utils::cmd;

    use crate::{dao::cmd_dao};


    #[cfg(not(debug_assertions))]
    fn get_fnm(run_type:RunType)->String{
        match run_type {
            RunType::START => "./ec2-start.sh".to_string(),
            RunType::EXIT => "./ec2-stop.sh".to_string(),
        }
    }

    #[cfg(debug_assertions)]
    fn get_fnm(run_type:RunType)->String{
        match run_type {
            RunType::START => "./script/ec2-start-test.sh".to_string(),
            RunType::EXIT => "./script/ec2-stop-test.sh".to_string(),
        }
    }

    use super::*;

    #[utoipa::path(
        context_path = "/api/v1",
        request_body = BatchParams,
        responses(
            (status = 200, description = "배치 실행", body = String)
        ),
    )]
    #[post("/batch")]
    pub async fn start_cmd(
        pool: web::Data<DBPool>,
        params: web::Json<BatchParams>,
        req: HttpRequest,
    ) -> HttpResponse   {
        info!("req.query_string()={:?}", req.query_string());
        info!("params.query_string()={:?}", params);
        let p: BatchParams = params.into_inner();
        
        match cmd_dao::start_cmd(pool,p.clone()).await {
        Ok(id) => {
            let mut args = vec![get_fnm(RunType::START)];
            let mut v2 = p.to_args();
            args.append(&mut v2);
            args.push(id.to_string());
            
            
            let p1 = cmd::CMD{args:args};
            p1.run();
            HttpResponse::Ok().json(id)},
        Err(err) => {
            error!("err:{:?}",err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
         }
                
       
    }

    #[utoipa::path(
        context_path = "/api/v1",
        params(
            ("id" = i64, Path, description = "batch_plan_id")
        ),
        responses(
            (status = 200, description = "batch 종료", body = String)
        ),
    )]
    #[delete("/batch/{id}")]
    pub async fn exit_cmd(
        pool: web::Data<DBPool>,
        req: HttpRequest,
        id: web::Path<i64>,
    ) -> HttpResponse {
        info!("req.query_string()={:?}", req.query_string());
        let args :Vec<String> = vec![get_fnm(RunType::EXIT)];
        let p1 = cmd::CMD{args};
        p1.run();

        match cmd_dao::exit_cmd(pool,id.into_inner()).await {
            Ok(id) => HttpResponse::Ok().json(id),
            Err(err) => {
                error!("err:{:?}",err);
                HttpResponse::InternalServerError().json(err.to_string())
            }
        }
    }
}

pub mod batch_timer_plan{

    use actix_web::{
        get,
        web::{self, Query},
        HttpRequest, HttpResponse,
    };
    use batch_model::{site::param::{batch_timer_plan::Param}};
    use crate::{dao::{ cmd_dao}, DBPool};
    

    #[utoipa::path(
        context_path = "/api/v1",
        
        params(
           Param
        ),
        responses(
            (status = 200, description = "batch timer plan   목록" )
        )
    )]
    #[get("/batch_timer_plan")]
    pub async fn handler(pool: web::Data<DBPool>, req: HttpRequest) -> HttpResponse {
        println!("req.query_string()={:?}", req.query_string());
    
        let item = Query::<Param>::from_query(req.query_string());
    
        match item {
            Ok(v) => {
                match cmd_dao::paging(pool, v.into_inner()).await {
                    Ok(list) => {
                        // info!("========================================================");
                        // info!("결과값확인:{:?}", &list);
                        HttpResponse::Ok().json(list)
                    }
                    Err(err) => {
                        println!("err:{}",err.to_string());
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



mod tests {
    

    

    #[test]
    fn it_script_test() {
        // CMD {
        //     run_file: "dir".to_string(),
        // }
        // .run();
        // CMD {
        //     run_file: "./script/ec2_start.sh".to_string(),
        // }
        // .run();

        assert_eq!(4, 4);
    }
}
