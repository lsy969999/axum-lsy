use axum::{Router, routing::{get, post}};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use tokio::signal;
use anyhow::anyhow;
use tower_http::services::ServeDir;
use tracing::info;

use crate::{config::{JwtKeys, AppState}, api::hello_world::hello_world, controller::index_controller::{idx, message, authorize, protected}};

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

    let state = AppState{pool};

    let router = Router::new()
          .route("/hello_world", get(hello_world))
          .route("/", get(idx))
          .route("/messages", get(message))
          // .route("/cookieTest", get(cookie_test))

          .route("/authorize", post(authorize))
          .route("/protected", post(protected))

          .nest_service("/assets", ServeDir::new("assets"))
          .with_state(state);

    Ok(router.into())
}

async fn shutdown_signal() {
  let ctrl_c = async {
    signal::ctrl_c().await.expect("failed to install Ctrl+C handler")
  };

  #[cfg(unix)]
  let terminate = async {
      signal::unix::signal(signal::unix::SignalKind::terminate())
          .expect("failed to install signal handler")
          .recv()
          .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();
  tokio::select! {
    _ = ctrl_c => {},
    _ = terminate => {}
  }
}