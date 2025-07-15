use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    AppError,
    models::{Block, FriendRequest, Friendship},
    util::verify_gjp2,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct AcceptForm {
    accountID: i32,
    targetAccountID: i32,
    gjp2: String,
    requestID: i32,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
    udid: String,
    uuid: String,
}

pub async fn accept_friend_request(
    State(pool): State<PgPool>,
    Form(form): Form<AcceptForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let target_id = form.targetAccountID;
    let gjp2 = &form.gjp2;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    FriendRequest::delete(&pool, user_id, target_id, false).await?;

    if Friendship::exists(&pool, user_id, target_id).await? {
        return Ok("-1".to_string());
    }

    if Block::is_blocked(&pool, user_id, target_id).await? {
        return Ok("-1".to_string());
    }

    Friendship::create(&pool, user_id, target_id).await?;
    Ok("1".to_string())
}
