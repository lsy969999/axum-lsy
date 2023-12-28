use askama::Template;
use axum::{response::{IntoResponse, Html}, Json};
use jsonwebtoken::{encode, Header};


use crate::config::{HtmlTemplate, Claims, AuthError, AuthPayload, AuthBody, self};

#[derive(Template)]
#[template(path="index.html")]
struct IdxTemplate{}
pub async fn idx()-> impl IntoResponse {
  HtmlTemplate(IdxTemplate{})
} 


pub async fn message() -> impl IntoResponse {
  Html("<span class='test'>haha</span><script>console.log('dudu');</script>")
}

pub async fn protected(claims: Claims) -> Result<String, AuthError> {
  Ok(format!(
    "Welecom to the protected area! your data: {claims}"
  ))
}

pub async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
  if payload.client_id.is_empty() || payload.client_secret.is_empty() {
    return Err(AuthError::MissingCredentials);
  }

  if payload.client_id != "foo" || payload.client_secret != "bar" {
    return Err(AuthError::WrongCredentials);
  }

  let claims = Claims {
    sub: "b@b.com".to_owned(),
    some: "SOME".to_owned(),
    exp: 20000000000
  };
  let keys = config::JWT_KEYS.get().unwrap();
  let token = encode(&Header::default(), &claims, &keys.encoding).map_err(|_| AuthError::TokenCreation)?;
  Ok(Json(AuthBody::new(token)))
}