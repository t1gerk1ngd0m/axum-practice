use axum::{
  extract::{Query, State},
  http::StatusCode,
  response,
  response::IntoResponse,
  routing::*,
  Json, Router, Server,
};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  net::SocketAddr,
  sync::{Arc, RwLock},
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let app = Router::new()
    .route("/", get(root))
    .route("/ping", post(ping));
  // .route("todos", get(todos_index));
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

#[derive(Deserialize)]
struct Pagination {
  limit: Option<u32>,
  offset: Option<u32>,
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

async fn todos_index(
  pagination: Option<Query<Pagination>>,
  State(db): State<Db>,
) -> impl IntoResponse {
}

type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;

#[derive(Serialize)]
struct Todo {
  id: Uuid,
  text: String,
  cmpleted: bool,
}
