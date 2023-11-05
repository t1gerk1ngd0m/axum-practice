use axum::{
  body::Body,
  http::StatusCode,
  response,
  response::{IntoResponse, Response},
  routing::*,
  Json, Router, Server,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use usecase::*;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let app = Router::new()
    .route("/", get(root))
    .route("/ping", post(ping))
    .route("/todos", get(get_todos));
  // .route("/todos", post(create_todo))
  // .route("/todos/:id", get(get_todo))
  // .route("/todos/:id", put(update_todo))
  // .route("/todos/:id", delete(delete_todo));
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

async fn get_todos() -> impl IntoResponse {
  let todos = get_todos::run().await.unwrap();
  Response::builder()
    .status(StatusCode::OK)
    .body(Body::from(serde_json::to_string(&todos).unwrap()))
    .unwrap()
}

// #[derive(Serialize)]
// struct TodoResponse {
//   id: Uuid,
//   text: String,
//   cmpleted: bool,
// }

// #[derive(Serialize)]
// struct TodosResponse {
//   todos: Vec<TodoResponse>,
// }
