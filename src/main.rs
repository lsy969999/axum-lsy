use askama::Template;
use axum::{routing::get, Router, extract::State, response::{Html, IntoResponse, Response}, http::StatusCode};
use sqlx::PgPool;
use tower_http::services::ServeDir;
use tracing::info;

async fn hello_world(State(pool): State<PgPool>) -> impl IntoResponse {
  let row: (i64,) = sqlx::query_as("SELECT $1")
    .bind(150_i64).fetch_one(&pool).await.expect("db fetch fail!");
    let st = format!("hello world {}", row.0.to_string());
    Html(st)
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres(
  local_uri = "postgresql://myuser:mypassword@localhost:5432/mydatabase"
)] pool: PgPool,) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
          .route("/hello_world", get(hello_world))
          .route("/", get(idx))
          .route("/messages", get(||async {
            Html("<span class='test'>haha</span><script>console.log('dudu');</script>")
          }))
          .nest_service("/assets", ServeDir::new("assets"))
          .with_state(pool);
    info!("hi");
    Ok(router.into())
}

#[derive(Template)]
#[template(path="index.html")]
struct IdxTemplate{}
async fn idx()-> impl IntoResponse{
  HtmlTemplate(IdxTemplate{})
}

struct HtmlTemplate<T>(T);

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
