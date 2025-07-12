use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::Message;
use crate::util::verify_gjp2;
use crate::{AppError, GDResponse};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetForm {
    accountID: i32,
    gjp2: String,
    page: i32,
    total: i32,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
    uuid: String,
    udid: String,

    #[serde(default)]
    getSent: i16,
}

pub async fn get_messages(
    State(pool): State<PgPool>,
    Form(form): Form<GetForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let page = form.page;
    let get_sent = form.getSent;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let messages: Vec<Message> = match get_sent {
        1 => Message::get_sent_messages(&pool, user_id).await?,
        _ => Message::get_messages(&pool, user_id).await?,
    };

    if messages.is_empty() {
        return Ok("-2".to_string());
    }

    let offset = page * 10;
    let count = messages.len();
    let end_string = format!("#{count}:{offset}:10");

    let mut response = String::new();

    for message in messages {
        response.push_str(&message.to_gd());
        response.push('|');
    }

    response.pop();
    response.push_str(&end_string);

    Ok(response)
}
