use std::sync::Arc;
use axum::{extract::State, response::{IntoResponse, Html}};
use crate::config::{AppState, AppError};

pub async fn hello_world(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
  let pool = &state.pool;
  let tx = pool.begin().await?;
  
  let dodo =  (|| async {
      let row: (i64,) = sqlx::query_as("SELECT $1")
      .bind(150_i64).fetch_one(pool).await.expect("db fetch fail!");
      Ok(row)
    }
  )();

  match dodo.await {
      Ok(row)=>{
        tx.commit().await?;
        let st = format!("hello world {}", row.0.to_string());
        Ok(Html(st))
      },
      Err(e)=>{
        tx.rollback().await?;
        Err(AppError(e))
      },
  }
}