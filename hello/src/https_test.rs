use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;

pub async fn run() {
    let fnm = "./rm_test.txt";
    let url = "https://api.finance.naver.com/siseJson.naver?symbol=005930&requestType=1&startTime=20230423&endTime=20230629&timeframe=day";
    request_file(fnm, url.to_string()).await;
}
async fn request_file(fnm: &str, url: String) {
    let time_out: std::time::Duration = std::time::Duration::new(15, 0);

    let https = HttpsConnector::new();
    let client = Client::builder()
        .pool_idle_timeout(Some(time_out))
        .build::<_, hyper::Body>(https);
    let req = Request::builder()
        .method(Method::GET)
        .uri(&url)
        .body(Body::empty())
        .unwrap();

    // POST it...
    let resp = client.request(req).await.unwrap();

    if resp.status() == StatusCode::OK {
        let body_bytes: hyper::body::Bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        println!("성공=>{:?}", body_bytes);
    } else {
        println!("네이버 차트 실패!!: url=>{:?}", url);
        println!("네이버 차트 실패!!: resp=>{:?}", resp);
        panic!("네이버 차트 실패!!: resp.status() = {}", resp.status())
    }
}
