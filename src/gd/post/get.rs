use axum::{Form, extract::State};
use serde::Serialize;
use serde_deserialize_duplicates::DeserializeLastDuplicate;
use sqlx::PgPool;

use crate::models::{Post, User};
use crate::{AppError, GDResponse};

#[derive(Serialize, DeserializeLastDuplicate, Debug)]
pub struct GetForm {
    accountID: i32,
    gjp2: String,
    page: i32,
    total: i32,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
    uuid: i32,
    udid: String,
}

pub async fn get_posts(
    State(pool): State<PgPool>,
    Form(form): Form<GetForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let page = form.page;

    let username = &User::username_from_id(&pool, user_id).await?;

    let posts: Vec<Post> = Post::get_all(&pool, user_id, username).await?;

    if posts.is_empty() {
        return Ok("#0:0:10".to_string());
    }

    let offset = page * 10;
    let count = posts.len();
    let end_string = format!("#{count}:{offset}:10");

    let mut response = String::new();

    for post in posts {
        response.push_str(&post.to_gd());
        response.push('|');
    }

    response.pop();
    response.push_str(&end_string);

    Ok(response)
}
