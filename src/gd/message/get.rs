use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::models::Message;
use crate::util::verify_gjp2;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetForm {
    #[serde(rename = "accountID")]
    user_id: i32,
    gjp2: String,
    page: i32,
    total: i32,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    #[serde(default)]
    gdw: u8,
    secret: String,
    uuid: String,
    udid: String,

    #[serde(rename = "getSent")]
    #[serde(default)]
    get_sent: i16,
}

pub async fn get_messages(
    State(pool): State<PgPool>,
    Form(form): Form<GetForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let gjp2 = &form.gjp2;
    let page = form.page;
    let get_sent = form.get_sent;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let messages: Vec<Message>;

    if get_sent == 1 {
        messages = Message::get_sent_messages(&pool, user_id).await?;
    } else {
        messages = Message::get_messages(&pool, user_id).await?;
    }

    let offset = page * 10;
    let count = messages.len();
    let end_string = format!("#{}:{}", count, offset);

    let mut response = String::new();

    for message in messages {
        let temp = Message::to_gd(message);
        response.push_str(&temp);
        response.push('|');
    }

    if response.is_empty() {
        return Ok(end_string);
    }

    // remove the last `|`
    response.pop();
    response.push_str(&end_string);

    Ok(response)
}
