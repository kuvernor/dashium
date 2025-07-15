use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::models::message::Message;
use crate::models::{Block, Friendship};
use crate::util::verify_gjp2;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendForm {
    accountID: i32,
    gjp2: String,
    toAccountID: i32,
    subject: String,
    body: String,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
    uuid: String,
    udid: String,
}

pub async fn send_message(
    State(pool): State<PgPool>,
    Form(form): Form<SendForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let subject = &form.subject;
    let body = &form.body;
    let target_id = form.toAccountID;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    if Block::is_blocked(&pool, user_id, target_id).await? {
        return Ok("-1".to_string());
    }

    let message_setting =
        sqlx::query_scalar!("SELECT message_setting FROM users WHERE id = $1", target_id)
            .fetch_one(&pool)
            .await?;

    match message_setting {
        1 => {
            if !Friendship::exists(&pool, user_id, target_id).await? {
                return Ok("-1".to_string());
            }
        }
        2 => return Ok("-1".to_string()),
        _ => (),
    }

    Message::send(&pool, user_id, target_id, subject, body).await?;
    Ok("1".to_string())
}
