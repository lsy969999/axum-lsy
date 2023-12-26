use askama::Template;
use axum::response::{IntoResponse, Html};

use crate::config::HtmlTemplate;

#[derive(Template)]
#[template(path="index.html")]
struct IdxTemplate{}
pub async fn idx()-> impl IntoResponse {
  HtmlTemplate(IdxTemplate{})
} 


pub async fn message() -> impl IntoResponse {
  Html("<span class='test'>haha</span><script>console.log('dudu');</script>")
}