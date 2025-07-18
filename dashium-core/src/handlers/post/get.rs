use axum::extract::{RawForm, State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::{Post, User};
use crate::{AppError, GDResponse};

#[derive(Serialize, Deserialize, Debug)]
pub struct getGJAccountComments20 {
    accountID: Vec<i32>,
    gjp2: String,
    page: i32,
    secret: String,
}

pub async fn getGJAccountComments20(
    State(pool): State<PgPool>,
    RawForm(form): RawForm,
) -> Result<String, AppError> {
    let form: getGJAccountComments20 = serde_html_form::from_bytes(&form)?;

    let user_id = form.accountID;
    let page = form.page;

    let username = &User::username_from_id(&pool, user_id[0]).await?;

    let posts: Vec<Post> = Post::get_all(&pool, user_id[0], username).await?;

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
