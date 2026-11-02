use std::str::from_utf8;

use crate::errors::*;
use batch_model::news::News;
use bytes::Bytes;
use chrono::NaiveDate;
use reqwest::{header, Client, StatusCode};
use scraper::{Html, Selector};
use sqlx::{Pool, Postgres};
use tracing::info;
use urlencoding::encode;
const DT_FORMAT_1: &str = "%Y.%m.%d";
const DT_FORMAT_2: &str = "%Y%m%d";
const ADDR: &str = "https://search.naver.com/search.naver?where=news";
const PARAMS :&str = "&sm=tab_opt&sort=1&photo=0&field=0&pd=3&docid=&related=0&mynews=0&office_type=0&office_section_code=0&news_office_checked=&is_sug_officeid=0";
const QUERY_INSERT :&str = "
INSERT INTO hist.news (code_id, dt, title, link, media_type) VALUES ( $1, $2, $3, $4, $5) ON CONFLICT (code_id, dt, link)  DO NOTHING;";
#[derive(Debug, Clone)]
pub struct Scraping {
    pub code_id: i64,
    pub params: ScrapingParams,
    pub now_date: NaiveDate,
}
impl Scraping {
    pub async fn run(&mut self, pool: &Pool<Postgres>) -> Result<()> {
        let mut vec_news: Vec<News> = vec![];
        let mut request_news_cnt: usize = 1;
        while request_news_cnt > 0 {
            let mut list_new: Vec<News> = self.request().await?;
            request_news_cnt = list_new.len();
            vec_news.append(&mut list_new);
            self.params.page = self.params.page + 1;
            info!("naver-request: {:?}", &self);
            if vec_news.len() > 100 || request_news_cnt == 0 {
                let items: Vec<News> = vec_news.clone();
                self.save_insert(items, pool.clone()).await?;
                vec_news.clear();
            }
        }

        Ok(())
    }

    async fn save_insert(&self, items: Vec<News>, pool: Pool<Postgres>) -> Result<()> {
        let mut conn = pool.begin().await.unwrap();
        for item in items {
            sqlx::query(QUERY_INSERT)
                .bind(&item.code_id)
                .bind(&item.dt)
                .bind(&item.title)
                .bind(&item.link)
                .bind(&item.media_type)
                .execute(&mut *conn)
                .await
                .unwrap();
        }
        conn.commit().await.unwrap();

        Ok(())
    }

    #[warn(dead_code)]
    async fn save_array(&self, items: Vec<News>, pool: Pool<Postgres>) -> Result<()> {
        let mut v_code_id: Vec<i64> = Vec::new();
        let mut v_dt: Vec<NaiveDate> = Vec::new();
        let mut v_title: Vec<String> = Vec::new();
        let mut v_link: Vec<String> = Vec::new();
        let mut v_media_type: Vec<String> = Vec::new();
        items.iter().for_each(|item| {
            v_code_id.push(item.code_id);
            v_dt.push(item.dt);
            v_title.push(item.title.to_owned());
            v_link.push(item.link.to_owned());
            v_media_type.push(item.media_type.to_owned());
        });
        sqlx::query(
            r#"INSERT INTO hist.news (code_id, dt, title, link, media_type)
            SELECT *
            FROM UNNEST($1, $2, $3, $4, $5)
            "#,
        )
        .bind(&v_code_id)
        .bind(&v_dt)
        .bind(&v_title)
        .bind(&v_link)
        .bind(&v_media_type)
        .execute(&pool)
        .await?;

        Ok(())
    }

    async fn request(&self) -> Result<Vec<News>> {
        let url = self.params.get_url();
        let mut list: Vec<News> = vec![];

        let client: Client = Client::default();
        let mut res = client
            .get(&url) // <- Create request builder
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(header::USER_AGENT, "curl")
            .send() // <- Send http request
            .await
            .unwrap();

        if res.status() == StatusCode::OK {
            let body: Bytes = res.bytes().await.unwrap();
            let string_value = from_utf8(&body).unwrap();
            list = self.parser(string_value).await;

            // let fnm = "./test.file.Html";
            // fm::write_all(fnm, &body_bytes);

            //let fragment = scraper::Html::parse_document(&string_value);
            // 파싱 하기
            // let fragment = Html::parse_fragment(sel1).unwrap();
            // ///////////
        } else {
            println!("--------------------------------------");
            info!("네이버 뉴스 실패!!: url=>{:?}", url);
            info!("네이버 뉴스 실패!!: resp=>{:?}", res);
            println!("오류:{:?}", res);

            panic!("네이버 뉴스 실패!!: resp.status() = {}", res.status())
        }
        Ok(list)
    }

    pub async fn parser(&self, data: &str) -> Vec<News> {
        let mut res: Vec<News> = Vec::new();
        // let mut f = fm::open(fnm);
        // let mut buffer = Vec::new();
        // f.read_to_end(&mut buffer).unwrap();
        // let data_string = String::from_utf8(buffer).unwrap();

        let fragment = Html::parse_fragment(&data);

        let news_area: &str = "div.news_area";
        let news_area_date = "div.info_group > span.info";
        let news_area_media = "div.info_group > a.press";
        let news_area_title = "a.news_tit";
        let sel_1 = Selector::parse(news_area).unwrap();
        let sel_1_1 = Selector::parse(news_area_date).unwrap();
        let sel_1_2 = Selector::parse(news_area_media).unwrap();
        let sel_1_3 = Selector::parse(news_area_title).unwrap();

        for element in fragment.select(&sel_1) {
            // println!("-------------------------------------------");

            let el_1 = element.select(&sel_1_1).next().unwrap();
            let el_2 = element.select(&sel_1_2).next().unwrap();
            let el_3 = element.select(&sel_1_3).next().unwrap();

            let date = el_1.inner_html();
            let dt_news = match date.find(".") {
                Some(i) => {
                    let dd: Vec<&str> = date.split(".").collect();
                    let y: i32 = dd.get(0).unwrap().parse::<i32>().unwrap();
                    let m: u32 = dd.get(1).unwrap().parse::<u32>().unwrap();
                    let d: u32 = dd.get(2).unwrap().parse::<u32>().unwrap();
                    NaiveDate::from_ymd_opt(y, m, d).unwrap()
                }
                None => self.now_date.clone(),
            };

            let media = el_2
                .text()
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .replace("\n", "")
                .replace(" ", "");
            let title = el_3.value().attr("title").unwrap();
            let link = el_3.value().attr("href").unwrap();
            //println!("{:?},{:?} {:?},{:?}", date, media, &title, &link);
            res.push(News {
                code_id: self.code_id,
                dt: dt_news,
                title: title.to_string(),
                link: link.to_string(),
                media_type: media,
            });
        }

        res
    }
}

#[derive(Debug, Clone)]
pub struct ScrapingParams {
    pub search_text: String,
    pub dt_start: NaiveDate,
    pub dt_end: NaiveDate,
    pub page: i32,
}
impl ScrapingParams {
    pub fn get_url(&self) -> String {
        let ds: (String, String) = self.get_dt_start(true);
        let de = self.get_dt_start(false);
        let uri_fmt = format!(
            "{}&query={}{}&ds={}&de={}&nso=so%3Ar%2Cp%3Afrom{}to{}&start={}",
            ADDR,
            encode(&self.search_text),
            PARAMS,
            ds.0,
            de.0,
            ds.1,
            de.1,
            self.get_page()
        );
        //uri_fmt.to_ascii_lowercase()
        uri_fmt
    }
    fn get_page(&self) -> i32 {
        (self.page - 1) * 10 + 1
    }
    pub fn get_dt_start(&self, is_start: bool) -> (String, String) {
        if is_start {
            (
                self.dt_start.format(DT_FORMAT_1).to_string(),
                self.dt_start.format(DT_FORMAT_2).to_string(),
            )
        } else {
            (
                self.dt_end.format(DT_FORMAT_1).to_string(),
                self.dt_end.format(DT_FORMAT_2).to_string(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, Utc};
    use chrono_tz::{Asia::Seoul, Tz};

    use utils::db::connection;

    use super::{Scraping, ScrapingParams};

    #[tokio::test]
    async fn test_run() {
        // let pool = connection(1).await;
        // let mut req = get_obj();
        // req.run(&pool).await.unwrap();
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn test_parser2() {
        use scraper::{Html, Selector};

        let html = r#"
           <div>
           <ul>
           <li class="abc">Foo</li>
           <li>Bar</li>
           <li>Baz</li>
       </ul>
       </div>
          
        "#;

        let fragment = Html::parse_fragment(html);
        let selector = Selector::parse("li.abc").unwrap();

        for element in fragment.select(&selector) {
            //   println!("-------------------------------------------");
            //  println!("{:?}", element.value().name());

            //assert_eq!("li", element.value().name());
        }
    }
    #[tokio::test]
    async fn test_parser() {
        let item = get_obj();
        // item.parser("./test.file.Html".to_string()).await;
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    fn get_obj() -> Scraping {
        let seoul_now: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
        let search_end_dt = seoul_now.naive_utc().date();

        Scraping {
            code_id: 100,
            now_date: search_end_dt,
            params: ScrapingParams {
                search_text: "한화솔루션".to_string(),
                dt_start: NaiveDate::from_ymd_opt(2023, 05, 01).unwrap(),
                dt_end: NaiveDate::from_ymd_opt(2023, 05, 02).unwrap(),
                page: 1,
            },
        }
    }

    #[tokio::test]
    async fn save_file() {
        let req = get_obj();
        req.request().await.unwrap();
        assert_eq!(2 + 2, 4);
    }
}
