use batch_model::app_type::batch_run_type::{BatchParams, BatchType, RunType};
use batch_model::app_type::country_type::CountryType;
use chrono::{DateTime, Datelike, TimeZone, Utc, Weekday};
use chrono_tz::{Asia::Seoul, Tz};
use sqlx::pool::PoolConnection;
use sqlx::{query, Pool};
use std::{env, thread::sleep, time::Duration};
use utils::db::connection;
use utils::env::envload;

const FMT: &str = "%Y-%m-%d %H:%M:%S";
const FMT2: &str = "%Y-%m-%d";

pub async fn kr_timer() {
    let batch_params = BatchParams {
        batch_type: BatchType::BATCH,
        country_type: CountryType::KR,
        run_type: RunType::START,
    };
    envload();
    let debug = env::var("BATCH_EXE_DEBUG").unwrap().parse().unwrap();
    let hour: u32 = env::var("BATCH_EXE_KR_RUN_TIME_HOUR")
        .unwrap()
        .parse()
        .unwrap();
    let min: u32 = env::var("BATCH_EXE_KR_RUN_TIME_MIN")
        .unwrap()
        .parse()
        .unwrap();
    let sec: u32 = env::var("BATCH_EXE_KR_RUN_TIME_SEC")
        .unwrap()
        .parse()
        .unwrap();
    let mut hist = Hist::new();
    let pool = connection(1).await;

    loop {
        let sleep_time = match debug {
            true => Duration::from_secs(1),
            false => Duration::from_secs(60),
        };

        let seoul_now: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
        let weekday = seoul_now.weekday();
        let if_open_day = match weekday {
            Weekday::Sat | Weekday::Sun => {
                if debug {
                    true
                } else {
                    false
                }
            }
            _ => true,
        };

        let seoul_now: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
        let id: String = seoul_now.format(FMT2).to_string();

        if hist.ymd != id {
            hist.ymd = id.clone();
            hist.run = false;
        }

        if if_open_day && hist.run == false {
            let if_seoul_open_time = Seoul
                .with_ymd_and_hms(
                    seoul_now.year(),
                    seoul_now.month(),
                    seoul_now.day(),
                    hour,
                    min,
                    sec,
                )
                .unwrap();
            let run_time = if_seoul_open_time.timestamp() < seoul_now.timestamp();
            if run_time {
                println!("시간됬다. {}, {:?}", seoul_now, &hist);
                hist.run = true;
                let client = batch_client::Client::new(Some(batch_params.clone())).unwrap();
                client.send_start().await.unwrap();

                // batch_exe에서 site-api로  start-cmd 요청보내기
                // site-api는 스크립트로 인스턴스 키고 + id 넘겨볼까?
                // 인스턴스는 배치를 pull 한후 실행시키고 배치가 마무리되면 site-api에 id 값넘겨서 배치 다했어요 하기.
            }
        }
        let pool = pool.clone();
        tokio::spawn(async { HealthCheck::update(pool).await });
        print!(", hist={:?}", hist);
        println!(" ,if_open_day={:?}", if_open_day);
        sleep(sleep_time);
    }
}

#[derive(Debug)]
struct Hist {
    pub ymd: String,
    pub run: bool,
}

impl Hist {
    fn new() -> Self {
        let seoul_now: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
        let id = seoul_now.format(FMT2).to_string();
        Self {
            ymd: id,
            run: false,
        }
    }
}
struct HealthCheck;
impl HealthCheck {
    pub async fn update(pool: Pool<sqlx::Postgres>) {
        let mut conn: PoolConnection<sqlx::Postgres> = pool.acquire().await.unwrap();

        let seoul_now: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
        print!("seoul_now={}", seoul_now.format(FMT));

        query("UPDATE system.info SET dt = $1 where id =1")
            .bind(seoul_now)
            .execute(&mut *conn)
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
    #[tokio::test]
    async fn it_works() {
        let pool = connection(1).await;

        HealthCheck::update(pool).await;

        assert_eq!(4, 4);
    }

    #[tokio::test]
    async fn it_works_send() {
        let batch_params = BatchParams {
            batch_type: BatchType::BATCH,
            country_type: CountryType::KR,
            run_type: RunType::START,
        };

        let client = batch_client::Client::new(Some(batch_params.clone())).unwrap();
        client.send_start().await.unwrap();

        assert_eq!(4, 4);
    }
}
