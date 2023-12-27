use axum::{extract::Request, middleware::Next, response::Response, http::StatusCode};
use tracing::debug;

pub async fn test_layer(req: Request, next: Next) -> Result<Response, StatusCode>{
  debug!("layer_test");
  Ok(next.run(req).await)
}