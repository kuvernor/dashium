use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

use crate::{AppError, models::Level};

pub async fn get(
    State(pool): State<PgPool>,
    Path(level_id): Path<String>,
) -> Result<Json<Level>, AppError> {
    let level_id = level_id.parse::<i32>()?;
    let level = Level::get(&pool, level_id).await?;

    Ok(Json(level))
}
