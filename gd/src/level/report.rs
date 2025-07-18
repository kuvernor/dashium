use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;

#[derive(Deserialize, Serialize, Debug)]
pub struct reportGJLevel {
    levelID: i32,
    secret: String,
}

pub async fn reportGJLevel(
    State(pool): State<PgPool>,
    Form(form): Form<reportGJLevel>,
) -> Result<String, AppError> {
    let level_id = form.levelID;

    sqlx::query!("INSERT INTO level_reports (level_id) VALUES ($1)", level_id)
        .execute(&pool)
        .await?;

    Ok("1".to_string())
}
