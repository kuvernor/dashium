use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::models::Post;
use crate::util::verify_gjp2;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteForm {
    #[serde(rename = "accountID")]
    user_id: i32,
    gjp2: String,

    #[serde(rename = "commentID")]
    post_id: i32,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    #[serde(default)]
    gdw: u8,
    secret: String,
    uuid: String,
    udid: String,
}

pub async fn delete_post(
    State(pool): State<PgPool>,
    Form(form): Form<DeleteForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let post_id = form.post_id;
    let gjp2 = &form.gjp2;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    match Post::delete(&pool, post_id, user_id).await {
        Ok(_) => return Ok("1".to_string()),
        Err(_) => return Ok("-1".to_string()),
    };
}
