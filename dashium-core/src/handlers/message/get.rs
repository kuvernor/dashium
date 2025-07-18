use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::Message;
use crate::util::verify_gjp2;
use crate::{AppError, GDResponse};

#[derive(Serialize, Deserialize, Debug)]
pub struct getGJMessages20 {
    accountID: i32,
    gjp2: String,
    page: i32,
    secret: String,

    #[serde(default)]
    getSent: i16,
}

pub async fn getGJMessages20(
    State(pool): State<PgPool>,
    Form(form): Form<getGJMessages20>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let page = form.page;
    let get_sent = form.getSent;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let messages = match get_sent {
        1 => Message::get_messages(&pool, user_id, true).await?,
        0 => Message::get_messages(&pool, user_id, false).await?,
        _ => return Ok("-1".to_string()),
    };

    if messages.is_empty() {
        return Ok("-2".to_string());
    }

    let offset = page * 10;
    let count = messages.len();
    let end_string = format!("#{count}:{offset}:10");

    let mut response = String::new();
    let get_sent = format!("9:{get_sent}:");

    for message in messages {
        let mut temp = message.to_gd();
        temp.insert_str(0, &get_sent);

        response.push_str(&temp);
        response.push('|');
    }

    response.pop();
    response.push_str(&end_string);

    Ok(response)
}
