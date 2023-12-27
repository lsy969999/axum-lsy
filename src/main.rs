use std::sync::Arc;
use axum::{Router, routing::{get, post}, extract::{Request, FromRequest}, response::{IntoResponse, Response}, async_trait, Json, Form, http::{header::CONTENT_TYPE, StatusCode}, RequestExt, middleware::{Next, self}};
use serde::{Serialize, Deserialize};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use anyhow::anyhow;
use tower_http::services::ServeDir;
use tower_http::compression::CompressionLayer;
use tracing::{info, debug};
use crate::{config::{JwtKeys, AppState}, api::hello_world::hello_world, controller::index_controller::{idx, message, authorize, protected}, layers::test_layer::test_layer};
mod config;
mod controller;
mod api;
mod layers;

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

    let state = Arc::new(AppState{pool});

    let auth_router = Router::new();

    let api_router = Router::new()
                          .route("/hello_world", get(hello_world))
                          .with_state(state.clone());

    let view_router = Router::new()
                          .route("/", get(idx)).route_layer(middleware::from_fn(check_hello_world))
                          .with_state(state.clone());
                          
    let test_router = Router::new() 
                          .route("/messages", get(message))
                          .route("/extract", post(extract))
                          .route("/authorize", post(authorize))
                          .route("/protected", post(protected))
                          .with_state(state.clone())
                          .layer(middleware::from_fn(test_layer));
    let router = Router::new()
          .nest("/auth", auth_router)
          .nest("/", view_router)
          .nest("/api", api_router)
          .nest("/test", test_router)
          .nest_service("/assets", ServeDir::new("assets"))
          .layer(CompressionLayer::new());
    Ok(router.into())
}

async fn check_hello_world(req: Request, next: Next) -> Result<Response, StatusCode> {
  // if req.headers().get(CONTENT_TYPE).unwrap() != "application/json" {
  //   return Err(StatusCode::BAD_REQUEST);
  // }
  debug!("check_hello_world");

  Ok(next.run(req).await)
}


#[derive(Debug, Serialize, Deserialize)]
struct Payload {
  foo: String
}

async fn extract(JsonOrForm(payload): JsonOrForm<Payload>) {
  debug!("payload: {:?}", payload.foo)
}

struct JsonOrForm<T>(T);


#[async_trait]
impl<S, T> FromRequest<S> for JsonOrForm<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<()>,
    Form<T>: FromRequest<()>,
    T: 'static,
{
  type Rejection = Response;

  async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
      let content_type_header = req.headers().get(CONTENT_TYPE);
      let content_type = content_type_header.and_then(|value| value.to_str().ok());

      if let Some(content_type) = content_type {
          if content_type.starts_with("application/json") {
              let Json(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
              return Ok(Self(payload));
          }

          if content_type.starts_with("application/x-www-form-urlencoded") {
              let Form(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
              return Ok(Self(payload));
          }
      }

      Err(StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response())
  }
}

// async fn shutdown_signal() {
//   let ctrl_c = async {
//     signal::ctrl_c().await.expect("failed to install Ctrl+C handler")
//   };

//   #[cfg(unix)]
//   let terminate = async {
//       signal::unix::signal(signal::unix::SignalKind::terminate())
//           .expect("failed to install signal handler")
//           .recv()
//           .await;
//   };

//   #[cfg(not(unix))]
//   let terminate = std::future::pending::<()>();
//   tokio::select! {
//     _ = ctrl_c => {},
//     _ = terminate => {}
//   }
// }