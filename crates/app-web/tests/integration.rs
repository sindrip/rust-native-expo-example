#![allow(clippy::unwrap_used)]

use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

#[tokio::test]
async fn add_returns_sum() {
    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, 0)))
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app_web::router()).await.unwrap();
    });

    let response = reqwest::get(format!("http://{addr}/add/3/4"))
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(response.text().await.unwrap(), "7");
}
