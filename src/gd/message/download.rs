use axum::{Form, extract::State};
use serde::Serialize;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;
use sqlx::PgPool;

use crate::{AppError, GDResponse, models::Message, util::verify_gjp2};

#[derive(Serialize, DeserializeFirstDuplicate, Debug)]
pub struct DownloadForm {
    accountID: i32,
    gjp2: String,
    messageID: i32,
    secret: String,
}

pub async fn download_message(
    State(pool): State<PgPool>,
    Form(form): Form<DownloadForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let message_id = form.messageID;
    let gjp2 = &form.gjp2;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let message = Message::download(&pool, message_id, user_id).await?;
    let response = message.to_gd();

    Ok(response)
}
