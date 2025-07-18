use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::ApiError;
use gd::models::Level;

pub async fn get(
    State(pool): State<PgPool>,
    Path(level_id): Path<String>,
) -> Result<Json<Level>, ApiError> {
    let level_id = level_id.parse::<i32>()?;
    let level = Level::get(&pool, level_id).await?;

    Ok(Json(level))
}

#[derive(Deserialize)]
pub struct LevelQuery {
    search: String,
}

pub async fn search(
    State(pool): State<PgPool>,
    Query(search): Query<LevelQuery>,
) -> Result<Json<Vec<Level>>, ApiError> {
    let username = &search.search;
    let users = Level::get_by_name(&pool, username).await?;

    Ok(Json(users))
}

pub async fn count(State(pool): State<PgPool>) -> Result<String, ApiError> {
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM levels")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);

    Ok(count.to_string())
}

pub async fn daily(State(pool): State<PgPool>) -> Result<Json<Level>, ApiError> {
    let level_id =
        sqlx::query_scalar!("SELECT level_id FROM daily_levels ORDER BY created_at DESC LIMIT 1")
            .fetch_one(&pool)
            .await?;
    let level = Level::get(&pool, level_id).await?;

    Ok(Json(level))
}

pub async fn weekly(State(pool): State<PgPool>) -> Result<Json<Level>, ApiError> {
    let level_id =
        sqlx::query_scalar!("SELECT level_id FROM weekly_demons ORDER BY created_at DESC LIMIT 1")
            .fetch_one(&pool)
            .await?;
    let level = Level::get(&pool, level_id).await?;

    Ok(Json(level))
}

pub async fn event(State(pool): State<PgPool>) -> Result<Json<Level>, ApiError> {
    let level_id =
        sqlx::query_scalar!("SELECT level_id FROM event_levels ORDER BY created_at DESC LIMIT 1")
            .fetch_one(&pool)
            .await?;
    let level = Level::get(&pool, level_id).await?;

    Ok(Json(level))
}

pub async fn user(
    State(pool): State<PgPool>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Level>>, ApiError> {
    let levels = sqlx::query_as!(
        Level,
        "SELECT * FROM levels WHERE username ILIKE $1",
        username
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(levels))
}
