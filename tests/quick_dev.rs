use axum::http::status;
use leafslug::health_check::SimpleResp;
use reqwest::{Client, Method, Url};
use serde::Deserialize;

#[tokio::test]
#[cfg_attr(not(feature = "local"), ignore)] // needs the service to be running
async fn quick_dev() {
    let c = send_get_api(at_api_v1("/health_check/simple")).await;
    println!("{}", at_api_v1("/health_check/simple"));
    print_response_with_test::<SimpleResp>(c, |t, s| {
        assert_eq!(" I am up!".to_owned(), t.message);
        assert_eq!(s, status::StatusCode::OK);
    })
    .await;
}

async fn print_response_with_test<T>(
    res: reqwest::Response,
    checks: impl FnOnce(T, status::StatusCode),
) where
    for<'a> T: Deserialize<'a>,
{
    let s = res.status();
    let bdy = &res.text().await.expect("could not get body text");
    let t: T = serde_json::from_str(bdy).expect("could not deserialize information");
    println!("{s}\t{}", bdy);
    checks(t, s);
}

async fn send_get_api(addr: Url) -> reqwest::Response {
    let c = clint().request(Method::GET, addr);
    c.send().await.expect("could not send request")
}

fn at_api_v1(addr: &str) -> Url {
    at_addr(&format!("/api/v1{addr}"))
}

fn at_addr(addr: &str) -> Url {
    let mut u = Url::parse("http://0.0.0.0").expect("could not parse addr");
    u.set_port(Some(8095)).expect("could not set port");
    u.join(addr).expect("could not pars the parameter")
}

fn clint() -> Client {
    reqwest::Client::builder()
        .build()
        .expect("could not unwrap")
}
