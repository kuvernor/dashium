use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, GDResponse, models::Comment};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetForm {
    accountID: i32,
    gjp2: String,
    gameVersion: i16,
    binaryVersion: i16,
    #[serde(default)]
    count: i32,
    mode: u8,
    page: i32,
    total: i32,
    secret: String,
    udid: String,
    uuid: String,
    levelID: i32,
}
pub async fn get_comments(
    State(pool): State<PgPool>,
    Form(form): Form<GetForm>,
) -> Result<String, AppError> {
    let level_id = form.levelID;
    let page = form.page;
    let mode = form.mode;

    let comments: Vec<Comment> = Comment::get_all(&pool, level_id, mode).await?;

    if comments.is_empty() {
        return Ok("-2".to_string());
    }

    let offset = page * 10;
    let count = comments.len();
    let end_string = format!("#{count}:{offset}:10");

    let mut response = String::new();

    for comment in &comments {
        response.push_str(&comment.to_gd());
        response.push('|');
    }

    response.pop();

    response.push_str(&end_string);

    Ok(response)
}
