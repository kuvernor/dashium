use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::models::{Block, User};
use crate::util::verify_gjp2;

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoForm {
    #[serde(rename = "accountID")]
    user_id: i32,

    #[serde(rename = "targetAccountID")]
    target_id: i32,

    gjp2: String,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    secret: String,
}

pub async fn info(
    State(pool): State<PgPool>,
    Form(form): Form<InfoForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let target_id = form.target_id;
    let gjp2 = &form.gjp2;

    if Block::is_blocked(&pool, target_id, user_id).await? {
        if verify_gjp2(&pool, user_id, gjp2).await? {
            return Ok("-1".to_string());
        }
    }

    let user = User::get_user(&pool, target_id).await?;
    let response = User::to_gd(user);

    Ok(response)
}
