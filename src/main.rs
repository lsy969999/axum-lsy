use axum::{routing::{get, post}, Router};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use tower_http::services::ServeDir;
use anyhow::anyhow;
use tracing::info;
use crate::{controller::index_controller::{idx, message, authorize, protected}, api::hello_world::hello_world, config::JwtKeys};
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
      return Err(anyhow!("[SHUTTLE-SECRET]env was not found").into());
    };
    info!("hi env is {}", env);

    let jwt_scret = if let Some(secret) = secret_store.get("JWT_SECRET"){
      secret
    } else {
      return Err(anyhow!("[SHUTTLE-SECRET]jwtScret was not found").into());
    };
    config::JWT_KEYS.get_or_init(||JwtKeys::new(jwt_scret.as_bytes()));

    let router = Router::new()
          .route("/hello_world", get(hello_world))
          .route("/", get(idx))
          .route("/messages", get(message))

          .route("/authorize", post(authorize))
          .route("/protected", post(protected))

          .nest_service("/assets", ServeDir::new("assets"))
          .with_state(pool);
    Ok(router.into())
}
