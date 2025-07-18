use axum::extract::State;
use sqlx::PgPool;

use crate::ApiError;

pub async fn count(State(pool): State<PgPool>) -> Result<String, ApiError> {
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM lists")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);

    Ok(count.to_string())
}
