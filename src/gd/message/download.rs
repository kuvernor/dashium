use axum::{Form, extract::State};
use serde::Serialize;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;
use sqlx::PgPool;

use crate::{AppError, models::Message, util::verify_gjp2};

#[derive(Serialize, DeserializeFirstDuplicate, Debug)]
pub struct DownloadForm {
    #[serde(rename = "accountID")]
    user_id: i32,
    gjp2: String,

    #[serde(rename = "messageID")]
    message_id: i32,

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

pub async fn download_message(
    State(pool): State<PgPool>,
    Form(form): Form<DownloadForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let message_id = form.message_id;
    let gjp2 = &form.gjp2;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let message = Message::download(&pool, message_id, user_id).await?;
    let response = Message::to_gd(message);

    Ok(response)
}
