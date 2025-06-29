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

    messages: String,

    #[serde(rename = "isSender")]
    #[serde(default)]
    is_sender: i16,

    #[serde(rename = "gameVersion")]
    game_version: Option<i16>,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,
    #[serde(default)]
    gdw: u8,
    secret: String,
    uuid: String,
    udid: String,
}

pub async fn delete_message(
    State(pool): State<PgPool>,
    Form(form): Form<DeleteForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let messages = form.messages;
    let gjp2 = &form.gjp2;

    let message_ids: Vec<i32> = messages
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("1".to_string());
    }

    for message_id in message_ids {
        Message::delete(&pool, message_id, user_id).await?;
    }

    Ok("1".to_string())
}
