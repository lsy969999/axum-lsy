use axum::{routing::get, Router, extract::State, response::{Html, IntoResponse}};
use sqlx::PgPool;
use tracing::info;

async fn hello_world(State(pool): State<PgPool>) -> impl IntoResponse {
  let row: (i64,) = sqlx::query_as("SELECT $1")
    .bind(150_i64).fetch_one(&pool).await.expect("db fetch fail!");
    let st = format!("hello world {}", row.0.to_string());
    Html(st)
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres(
  local_uri = "postgresql://myuser:mypassword@localhost:5432/mydatabase"
)] pool: PgPool,) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
          .route("/", get(hello_world))
          .with_state(pool);
    info!("hi");
    Ok(router.into())
}
