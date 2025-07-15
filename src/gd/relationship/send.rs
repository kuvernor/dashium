use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    AppError,
    models::{Block, FriendRequest},
    util::verify_gjp2,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct SendForm {
    accountID: i32,
    binaryVersion: i16,
    comment: String,
    gameVersion: i16,
    gjp2: String,
    secret: String,
    toAccountID: i32,
    udid: String,
    uuid: String,
}

pub async fn send_friend_request(
    State(pool): State<PgPool>,
    Form(form): Form<SendForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let target_id = form.toAccountID;
    let body = &form.comment;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let friend_setting =
        sqlx::query_scalar!("SELECT friend_setting FROM users WHERE id = $1", target_id)
            .fetch_one(&pool)
            .await?;

    if friend_setting == 1 {
        return Ok("-1".to_string());
    }

    if Block::is_blocked(&pool, user_id, target_id).await? {
        return Ok("-1".to_string());
    }

    if FriendRequest::exists(&pool, user_id, target_id).await? {
        return Ok("-1".to_string());
    }

    FriendRequest::send(&pool, user_id, target_id, body).await?;
    Ok("1".to_string())
}
