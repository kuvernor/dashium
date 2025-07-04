use axum::{Form, extract::State};
use serde::Serialize;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;
use sqlx::PgPool;

use crate::AppError;
use crate::models::Post;

#[derive(Serialize, DeserializeFirstDuplicate, Debug)]
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

    secret: String,
    uuid: String,
    udid: String,
}

pub async fn get_posts(
    State(pool): State<PgPool>,
    Form(form): Form<GetForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let page = form.page;

    let posts: Vec<Post> = Post::get_posts(&pool, user_id).await?;

    if posts.is_empty() {
        return Ok("#0:0:10".to_string());
    }

    let offset = page * 10;
    let count = posts.len();
    let end_string = format!("#{}:{}:10", count, offset);

    let mut response = String::new();

    for post in posts {
        let temp = Post::to_gd(post);
        response.push_str(&temp);
        response.push('|');
    }

    response.pop();
    response.push_str(&end_string);

    Ok(response)
}
