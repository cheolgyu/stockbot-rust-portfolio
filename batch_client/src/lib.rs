use std::error::Error;

use batch_model::app_type::{
    batch_run_type::{BatchParams, BatchType, RunType},
    country_type::CountryType,
};
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use std::str::FromStr;
use utils::env::envload;

pub struct Client {
    pub params: BatchParams,
}
impl Client {
    pub fn new(batch_params: Option<BatchParams>) -> Result<Self, Box<dyn Error>> {
        let params = if batch_params.is_none() {
            Self::get_pamas()?
        } else {
            batch_params.unwrap()
        };
        Ok(Self { params })
    }

    pub async fn send_start(&self) -> Result<(), Box<dyn Error>> {
        let url = Self::get_api(RunType::START)?;

        let client = reqwest::Client::new().post(url);
        println!("client:\n{:#?}", client);
        let res = client
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .json(&self.params)
            .send()
            .await;
        match res {
            Ok(resp) => {
                println!("resp:\n{:#?}", resp);
                Ok(())
            }
            Err(e) => {
                println!("err==>{:#?}", e);
                Err(e.into())
            }
        }
    }

    pub async fn send_exit(&self) -> Result<(), Box<dyn Error>> {
        let batch_plan_id: i64 = dotenvy::var("BATCH_TIMER_PLAN_ID")?
            .to_string()
            .parse::<i64>()?;

        let url = format!("{}/{}", Self::get_api(RunType::EXIT)?, batch_plan_id);
        let client = reqwest::Client::builder()
            .build()?
            .delete(url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        println!("client:\n{:#?}", client);
        let res = client.send().await;
        match res {
            Ok(resp) => {
                println!("resp:\n{:#?}", resp);
                Ok(())
            }
            Err(e) => {
                println!("err==>{:#?}", e);
                Err(e.into())
            }
        }
    }

    fn get_pamas() -> Result<BatchParams, Box<dyn Error>> {
        utils::env::envload_batch_plan().unwrap();

        let batch_type: BatchType = BatchType::from_str(dotenvy::var("BATCH_TYPE")?.as_str())?;
        let country_type: CountryType =
            CountryType::from_str(dotenvy::var("COUNTRY_TYPE")?.as_str())?;
        let run_type: RunType = RunType::from_str(dotenvy::var("RUN_TYPE")?.as_str())?;

        let batch_params: BatchParams = BatchParams {
            batch_type,
            country_type,
            run_type,
        };
        Ok(batch_params)
    }

    fn get_api(run_type: RunType) -> Result<String, Box<dyn Error>> {
        envload();
        match run_type {
            RunType::START => {
                let cmd_point_start = dotenvy::var("BATCH_POINT_START")?;
                Ok(cmd_point_start.to_string())
            }
            RunType::EXIT => {
                let cmd_point_end = dotenvy::var("BATCH_POINT_EXIT")?;
                Ok(cmd_point_end.to_string())
            }
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works_send() {
        let batch_params = BatchParams {
            batch_type: BatchType::BATCH,
            country_type: CountryType::KR,
            run_type: RunType::START,
        };

        let client = Client::new(Some(batch_params.clone())).unwrap();
        client.send_start().await.unwrap();

        assert_eq!(4, 4);
    }

    #[tokio::test]
    async fn it_works_send_exit() {
        let client = Client::new(None).unwrap();
        client.send_exit().await.unwrap();

        assert_eq!(4, 4);
    }
}
