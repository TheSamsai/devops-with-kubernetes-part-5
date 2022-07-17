use std::fs::File;
use std::io::prelude::*;
use std::net::SocketAddr;

use axum::{
    routing::get,
    http::StatusCode,
    Router, Extension,
};

#[derive(Clone)]
struct PingPongSvc(String);

#[derive(Clone)]
struct Message(String);

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").map(|val| val.parse::<u16>().unwrap()).unwrap_or(3000);
    let ping_pong_svc = std::env::var("PING_PONG_SVC").unwrap_or(String::from("http://ping-pong-svc"));
    let message = std::env::var("MESSAGE").unwrap_or(String::from(""));

    let app = Router::new()
        .route("/", get(log_view))
        .route("/health", get(health_check))
        .layer(Extension(PingPongSvc(ping_pong_svc)))
        .layer(Extension(Message(message)));

    let addr = SocketAddr::from(([0,0,0,0], port));

    println!("Started at port {}", port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn log_view(
    Extension(ping_pong_svc): Extension<PingPongSvc>,
    Extension(message): Extension<Message>
) -> String {
    let mut log_file = File::open("/shared/log.txt").expect("Failed to open file!");

    let mut log_message = String::new();
    log_file.read_to_string(&mut log_message).expect("Failure to read file!");

    let ping_pong_message = reqwest::get(format!("{}/count", ping_pong_svc.0)).await.unwrap().text().await.unwrap();

    format!("{}\n{}\nPing / Pongs: {}", message.0, log_message, ping_pong_message)
}

async fn health_check(
    Extension(ping_pong_svc): Extension<PingPongSvc>
) -> Result<String, StatusCode> {
    let ping_pong_up = reqwest::get(format!("{}/count", ping_pong_svc.0)).await.is_ok();

    if ping_pong_up {
        return Ok("Healthy".to_string());
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
}
