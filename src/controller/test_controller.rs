use askama::Template;
use axum::response::IntoResponse;

use crate::config::HtmlTemplate;

#[derive(Template)]
#[template(path="test/index.html")]
struct TestIndexTemplate{
  var: String
}
pub async fn test_index()-> impl IntoResponse {
  HtmlTemplate(TestIndexTemplate{var: "var var var".to_string()})
} 
