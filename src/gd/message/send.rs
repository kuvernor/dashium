use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::models::message::Message;
use crate::util::verify_gjp2;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendForm {
    #[serde(rename = "accountID")]
    user_id: i32,
    gjp2: String,

    #[serde(rename = "toAccountID")]
    target_id: i32,

    subject: String,
    body: String,

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

pub async fn send_message(
    State(pool): State<PgPool>,
    Form(form): Form<SendForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let gjp2 = &form.gjp2;
    let subject = &form.subject;
    let body = &form.body;
    let target_id = form.target_id;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    match Message::send(&pool, user_id, target_id, subject, body).await {
        Ok(_) => return Ok("1".to_string()),
        Err(_) => return Ok("-1".to_string()),
    };
}
