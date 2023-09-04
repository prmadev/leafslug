use axum::http::status;
use leafslug::{health_check, login};
use reqwest::{Client, Method, Url};
use serde::{Deserialize, Serialize};

#[tokio::test]
#[cfg_attr(not(feature = "local"), ignore)] // needs the service to be running
async fn quick_dev() {
    println!("=>\t{}", at_api_v1("/health_check/simple"));
    let c = send_get_api(at_api_v1("/health_check/simple")).await;
    print_response_with_test::<health_check::Response>(c, |t, s| {
        assert_eq!(" I am up!".to_owned(), t.message);
        assert_eq!(s, status::StatusCode::OK);
    })
    .await;

    println!("=>\t{}", at_api_v1("/login/simple"));
    let login_resp = send_post_api(
        at_api_v1("/login/simple"),
        &login::Request {
            username: "alphauser".to_string(),
            password: "alphapass".to_string(),
        },
    )
    .await;

    print_response_with_test::<login::Response>(login_resp, |r, s| {
        assert_eq!(s, status::StatusCode::OK);
        assert!(r.success);
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
    println!("{s}\t{}", bdy);
    let t: T = serde_json::from_str(bdy).expect("could not deserialize information");
    checks(t, s);
}

async fn send_get_api(addr: Url) -> reqwest::Response {
    let c = clint().request(Method::GET, addr);
    c.send().await.expect("could not send request")
}

async fn send_post_api(addr: Url, payload: &impl Serialize) -> reqwest::Response {
    let c = clint().request(Method::POST, addr).json(payload);
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
