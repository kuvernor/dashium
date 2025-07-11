use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

use crate::{AppError, models::User};

pub async fn get(
    State(pool): State<PgPool>,
    Path(username): Path<String>,
) -> Result<Json<User>, AppError> {
    let user_id = User::id_from_username(&pool, &username.to_lowercase()).await?;
    let user = User::get_user(&pool, user_id).await?;

    Ok(Json(user))
}
