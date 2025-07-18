use axum::{
    Json,
    extract::{Path, Query, State},
};
use dashium_core::models::List;
use serde::Deserialize;
use sqlx::PgPool;

use crate::ApiError;

pub async fn get(
    State(pool): State<PgPool>,
    Path(list_id): Path<String>,
) -> Result<Json<List>, ApiError> {
    let list_id = list_id.parse::<i32>()?;
    let list = List::get(&pool, list_id).await?;

    Ok(Json(list))
}

#[derive(Deserialize)]
pub struct ListQuery {
    search: String,
}

pub async fn search(
    State(pool): State<PgPool>,
    Query(search): Query<ListQuery>,
) -> Result<Json<Vec<List>>, ApiError> {
    let list_name = &search.search;
    let lists = List::get_by_name(&pool, list_name).await?;

    Ok(Json(lists))
}

pub async fn count(State(pool): State<PgPool>) -> Result<String, ApiError> {
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM lists")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);

    Ok(count.to_string())
}

pub async fn user(
    State(pool): State<PgPool>,
    Path(username): Path<String>,
) -> Result<Json<Vec<List>>, ApiError> {
    let lists = sqlx::query_as!(
        List,
        "SELECT * FROM lists WHERE username ILIKE $1",
        username
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(lists))
}
