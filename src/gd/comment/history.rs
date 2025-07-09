use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::Comment};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetForm {
    userID: i32,
    gjp2: String,
    page: i32,
    mode: u8,
    total: i32,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
    udid: String,
    uuid: String,
}

pub async fn get_history(
    State(pool): State<PgPool>,
    Form(form): Form<GetForm>,
) -> Result<String, AppError> {
    let user_id = form.userID;
    let page = form.page;
    let mode = form.mode;

    let comments: Vec<Comment> = Comment::get_from_user(&pool, user_id, mode).await?;

    if comments.is_empty() {
        return Ok("-2".to_string());
    }

    let offset = page * 10;
    let count = comments.len();
    let end_string = format!("#{count}:{offset}:10");

    let mut response = String::new();

    for comment in &comments {
        let temp = Comment::to_gd(&pool, comment, true).await?;
        response.push_str(&temp);
        response.push('|');
    }

    response.pop();

    response.push_str(&end_string);

    Ok(response)
}
