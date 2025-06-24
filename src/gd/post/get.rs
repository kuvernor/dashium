use axum::{Form, extract::State};
use serde::Serialize;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;
use sqlx::PgPool;

use crate::AppError;
use crate::models::Post;

#[derive(Serialize, DeserializeFirstDuplicate, Debug)]
pub struct GetForm {
    #[serde(rename = "accountID")]
    target_id: i32,
    gjp2: String,
    page: i32,
    total: i32,
    #[serde(rename = "gameVersion")]
    game_version: Option<i16>,
    #[serde(rename = "binaryVersion")]
    binary_version: Option<i16>,
    #[serde(default)]
    gdw: u8,
    secret: String,
    uuid: String,
    udid: String,
}

pub async fn get_posts(
    State(pool): State<PgPool>,
    Form(form): Form<GetForm>,
) -> Result<String, AppError> {
    let post = Post::to_gd(&pool, form.target_id, form.page).await?;
    Ok(post)
}
