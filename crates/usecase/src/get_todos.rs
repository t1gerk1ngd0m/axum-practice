use serde::Serialize;
use uuid::Uuid;

pub async fn run() -> anyhow::Result<Vec<Todo>> {
  Ok(vec![
    Todo {
      id: TodoId(Uuid::new_v4()),
      text: TodoContent("test1".to_string()),
      completed: IsCompleted(true),
    },
    Todo {
      id: TodoId(Uuid::new_v4()),
      text: TodoContent("test2".to_string()),
      completed: IsCompleted(false),
    },
  ])
}

#[derive(Serialize)]
pub struct Todo {
  pub id: TodoId,
  pub text: TodoContent,
  pub completed: IsCompleted,
}

#[derive(Serialize)]
pub struct TodoId(Uuid);

#[derive(Serialize)]
pub struct TodoContent(String);

#[derive(Serialize)]
pub struct IsCompleted(bool);
