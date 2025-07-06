use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::models::Message;
use crate::util::verify_gjp2;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteForm {
    accountID: i32,
    binaryVersion: i16,
    gameVersion: i16,
    gjp2: String,
    secret: String,
    udid: String,
    uuid: String,
    #[serde(default)]
    messages: String,
    #[serde(default)]
    messageID: i32,
    #[serde(default)]
    isSender: i16,
}

pub async fn delete_message(
    State(pool): State<PgPool>,
    Form(form): Form<DeleteForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let message_id = form.messageID;
    let message_ids = form.messages;
    let is_sender = form.isSender;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("1".to_string());
    }

    if message_ids.is_empty() {
        match is_sender {
            0 => Message::delete(&pool, user_id, message_id, false).await?,
            1 => Message::delete(&pool, user_id, message_id, true).await?,
            _ => (),
        }
    } else {
        let message_ids: Vec<i32> = message_ids
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        match is_sender {
            0 => {
                for message_id in message_ids {
                    Message::delete(&pool, user_id, message_id, false).await?;
                }
            }
            1 => {
                for message_id in message_ids {
                    Message::delete(&pool, user_id, message_id, true).await?;
                }
            }
            _ => (),
        }
    }

    Ok("1".to_string())
}
