use axum::{routing::get, Router};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use tower_http::services::ServeDir;
use anyhow::anyhow;
use tracing::info;
use crate::{controller::index_controller::{idx, message}, api::hello_world::hello_world};

mod config;
mod controller;
mod api;

#[shuttle_runtime::main]
async fn main(
  #[shuttle_shared_db::Postgres(
    local_uri = "postgresql://myuser:mypassword@localhost:5432/mydatabase"
  )] pool: PgPool,
  #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let env = if let Some(secret) = secret_store.get("ENV"){
      secret
    } else {
      return Err(anyhow!("secret was not found").into());
    };
    info!("hi env is {}", env);
    let router = Router::new()
          .route("/hello_world", get(hello_world))
          .route("/", get(idx))
          .route("/messages", get(message))
          .nest_service("/assets", ServeDir::new("assets"))
          .with_state(pool)
          .with_state(secret_store);
    Ok(router.into())
}