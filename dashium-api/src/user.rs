use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::ApiError;
use dashium_core::models::User;

pub async fn get(
    State(pool): State<PgPool>,
    Path(username): Path<String>,
) -> Result<Json<User>, ApiError> {
    let user_id = User::id_from_username(&pool, &username).await?;
    let user = User::get_user(&pool, user_id).await?;

    Ok(Json(user))
}

#[derive(Deserialize)]
pub struct UserQuery {
    pub search: String,
}

pub async fn search(
    State(pool): State<PgPool>,
    Query(query): Query<UserQuery>,
) -> Result<Json<Vec<User>>, ApiError> {
    let username = &query.search;
    let users = User::get_by_name(&pool, username).await?;

    Ok(Json(users))
}

pub async fn count(State(pool): State<PgPool>) -> Result<String, ApiError> {
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);

    Ok(count.to_string())
}
