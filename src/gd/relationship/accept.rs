use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    AppError,
    models::{FriendRequest, Friendship},
    util::verify_gjp2,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct AcceptForm {
    #[serde(rename = "accountID")]
    user_id: i32,

    #[serde(rename = "targetAccountID")]
    target_id: i32,
    gjp2: String,

    #[serde(rename = "requestID")]
    friend_request_id: i32,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    secret: String,
    udid: String,
    uuid: String,
}

pub async fn accept_friend_request(
    State(pool): State<PgPool>,
    Form(form): Form<AcceptForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let target_id = form.target_id;
    let gjp2 = &form.gjp2;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    FriendRequest::delete(&pool, user_id, target_id, false).await?;

    if Friendship::exists(&pool, user_id, target_id).await? {
        return Ok("-1".to_string());
    }

    match Friendship::create(&pool, user_id, target_id).await {
        Ok(_) => return Ok("1".to_string()),
        Err(_) => return Ok("-1".to_string()),
    }
}
