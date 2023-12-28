
use std::{sync::OnceLock, fmt::Display};
use askama::Template;
use axum::{response::{Html, IntoResponse, Response}, http::{StatusCode, request::Parts}, Json, async_trait, extract::FromRequestParts, RequestPartsExt};
use axum_extra::{TypedHeader, headers::{Authorization, authorization::Bearer}};
use jsonwebtoken::{EncodingKey, DecodingKey, Validation, decode};
use serde::{Serialize, Deserialize};
use serde_json::json;
use sqlx::PgPool;

pub struct HtmlTemplate<T>(pub T);
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

pub static JWT_KEYS: OnceLock<JwtKeys> = OnceLock::new();

pub struct JwtKeys {
  pub encoding: EncodingKey,
  pub decoding: DecodingKey
}


impl JwtKeys {
    pub fn new(secret: &[u8]) -> Self {
      Self {
        encoding: EncodingKey::from_secret(secret),
        decoding: DecodingKey::from_secret(secret)
      }
    }
}

#[derive(Debug)]
pub enum AuthError {
  WrongCredentials,
  MissingCredentials,
  TokenCreation,
  InvalidToken,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
  access_token: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
      Self {
        access_token
      }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
  pub client_id: String,
  pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub exp: usize,
  pub some: String
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "sub: {} exp: {} some: {}", self.sub, self.exp, self.some)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
          AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
          AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
          AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
          AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid Token")
        };

        let body = Json(json!({
          "error": error_message
        }));
        (status, body).into_response()
    }
}
pub struct AppError(pub anyhow::Error);
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
      let body = Json(json!({
        "error": format!("Something went wrong: {}", self.0)
      }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims where S: Send + Sync {
  type Rejection = AuthError;
  async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
    let TypedHeader(Authorization(bearer)) = parts.extract::<TypedHeader<Authorization<Bearer>>>().await.map_err(|_| AuthError::InvalidToken)?;
    let keys = JWT_KEYS.get().unwrap();
    let token_data = decode::<Claims>(bearer.token(), &keys.decoding, &Validation::default())
    .map_err(|_| AuthError::InvalidToken)?;
    Ok(token_data.claims)
  }
}

#[derive(Clone)]
pub struct AppState {
  pub pool: PgPool
}