use axum::{http::StatusCode, response, routing::*, Json, Router, Server};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let app = Router::new()
    .route("/", get(root))
    .route("/ping", post(ping));
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::debug!("Listening on {}", addr);
  Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

#[derive(Deserialize)]
struct Ping {
  count: i64,
}

#[derive(Serialize)]
struct Pong {
  count: i64,
}

async fn root() -> (StatusCode, &'static str) {
  (StatusCode::OK, "Hello, World!")
}

async fn ping(Json(ping): Json<Ping>) -> (StatusCode, Json<Pong>) {
  (
    StatusCode::ACCEPTED,
    response::Json(Pong {
      count: ping.count + 1,
    }),
  )
}
