use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::FriendRequest, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct SendForm {
    #[serde(rename = "accountID")]
    user_id: i32,
    gjp2: String,

    #[serde(rename = "comment")]
    body: String,

    #[serde(rename = "toAccountID")]
    target_id: i32,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    secret: String,
    udid: String,
    uuid: String,
}

pub async fn send_friend_request(
    State(pool): State<PgPool>,
    Form(form): Form<SendForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let gjp2 = &form.gjp2;
    let target_id = form.target_id;
    let body = &form.body;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    if FriendRequest::exists(&pool, user_id, target_id).await? {
        return Ok("-1".to_string());
    }

    match FriendRequest::send(&pool, user_id, target_id, body).await {
        Ok(_) => return Ok("1".to_string()),
        Err(_) => return Ok("-1".to_string()),
    }
}
