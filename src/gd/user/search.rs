use crate::AppError;
use crate::models::User;
use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchForm {
    gjp2: String,
    str: String,
    page: i32,
    total: i32,
    udid: String,
    uuid: String,
    #[serde(rename = "accountID")]
    user_id: i32,
    secret: String,
    #[serde(rename = "gameVersion")]
    game_version: i16,
    #[serde(rename = "binaryVersion")]
    binary_version: i16,
}

pub async fn search(
    State(pool): State<PgPool>,
    Form(form): Form<SearchForm>,
) -> Result<String, AppError> {
    let users = User::search(&pool, &form.str, form.page).await?;
    Ok(users)
}
