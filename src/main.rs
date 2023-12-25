use axum::{routing::get, Router};
use sqlx::PgPool;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres(
  local_uri = "postgresql://myuser:mypassword@localhost:5432/mydatabase"
)] pool: PgPool,) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
          .route("/", get(hello_world))
          .with_state(pool);

    Ok(router.into())
}
