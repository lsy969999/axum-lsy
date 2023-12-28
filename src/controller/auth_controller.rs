use askama::Template;
use axum::{response::{IntoResponse, Response, Html}, Form};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;

use crate::config::AppError;

#[derive(Template)]
#[template(path="parts/auth/emailAuthSuccessPart.html")]
struct EmailAuthSuccessTemplate{}

#[derive(Template)]
#[template(path="parts/auth/emailAuthFailPart.html")]
struct EmailAuthFailTemplate{}

#[derive(Deserialize)]
pub struct EmailAuthForm {
  email: String,
  password: String
}

pub async fn email_auth(
  jar: CookieJar,
  Form(email_auth): Form<EmailAuthForm>,
) -> Result<(CookieJar, Response), AppError> {
  
  let cond = email_auth.email == "lsy" && email_auth.password == "lsy";
  if cond {
    let r = EmailAuthSuccessTemplate{}.render();
    let cookie = Cookie::build(("authTest", "value")).path("/").http_only(true);
    match r {
      Ok(html) => Ok((jar.add(cookie), Html(html).into_response())),
      Err(err) => Err(AppError(err.into())),
    }
  } else {
    let r = EmailAuthFailTemplate{}.render();
    let cookie = Cookie::build(("authTest", "value")).path("/").http_only(true);
    match r {
      Ok(html) => Ok((jar.remove(cookie), Html(html).into_response())),
      Err(err) => Err(AppError(err.into())),
    }
  }
}

pub async fn coookie(jar: CookieJar,) -> Result<(CookieJar, Response), AppError>{
         Ok((jar.add(Cookie::new("session_id", "session_id")), Html("ttt").into_response()))
}