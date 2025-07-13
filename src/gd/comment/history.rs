use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    AppError, GDResponse,
    models::{Comment, Friendship},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetForm {
    accountID: i32,
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
    let user_id = form.accountID;
    let target_id = form.userID;
    let page = form.page;
    let mode = form.mode;

    let comment_setting =
        sqlx::query_scalar!("SELECT comment_setting FROM users WHERE id = $1", target_id)
            .fetch_one(&pool)
            .await?;

    match comment_setting {
        1 => {
            if target_id != user_id && !Friendship::exists(&pool, user_id, target_id).await? {
                return Ok("-1".to_string());
            }
        }
        2 => {
            if target_id != user_id {
                return Ok("-1".to_string());
            }
        }
        _ => (),
    }

    let comments: Vec<Comment> = Comment::get_from_user(&pool, user_id, mode).await?;

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
