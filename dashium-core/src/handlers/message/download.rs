use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, GDResponse, models::Message, util::verify_gjp2};

#[derive(Serialize, Deserialize, Debug)]
pub struct downloadGJMessage20 {
    accountID: i32,
    gjp2: String,
    messageID: i32,
    secret: String,
}

pub async fn downloadGJMessage20(
    State(pool): State<PgPool>,
    Form(form): Form<downloadGJMessage20>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let message_id = form.messageID;
    let gjp2 = &form.gjp2;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let message = Message::download(&pool, message_id, user_id).await?;

    let mut response = message.to_gd();

    let content = format!("5:{}:", message.body);
    response.insert_str(0, &content);

    Ok(response)
}
