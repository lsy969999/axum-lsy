use std::sync::Arc;

use axum::{extract::State, response::{IntoResponse, Html}};

use crate::config::AppState;

pub async fn hello_world(State(pool): State<Arc<AppState>>) -> impl IntoResponse {
  let row: (i64,) = sqlx::query_as("SELECT $1")
    .bind(150_i64).fetch_one(&pool.pool).await.expect("db fetch fail!");
    let st = format!("hello world {}", row.0.to_string());
    Html(st)
}