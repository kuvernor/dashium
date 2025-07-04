use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::models::Message;
use crate::util::verify_gjp2;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteForm {
    #[serde(rename = "accountID")]
    user_id: i32,
    gjp2: String,

    #[serde(default)]
    #[serde(rename = "messages")]
    message_ids: String,

    #[serde(default)]
    #[serde(rename = "messageID")]
    message_id: i32,

    #[serde(rename = "isSender")]
    #[serde(default)]
    is_sender: i16,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    secret: String,
    uuid: String,
    udid: String,
}

pub async fn delete_message(
    State(pool): State<PgPool>,
    Form(form): Form<DeleteForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let gjp2 = &form.gjp2;
    let message_id = form.message_id;
    let message_ids = form.message_ids;
    let is_sender = form.is_sender;

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
