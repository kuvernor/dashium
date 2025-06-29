use crate::AppError;
use crate::models::User;
use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchForm {
    gjp2: String,
    str: String,
    page: i32,
    total: i32,

    #[serde(rename = "accountID")]
    user_id: i32,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    udid: String,
    uuid: String,
    secret: String,
}

pub async fn search(
    State(pool): State<PgPool>,
    Form(form): Form<SearchForm>,
) -> Result<String, AppError> {
    let search_term = &form.str;
    let page = form.page;

    let users = User::get_users(&pool, search_term).await?;

    let offset = page * 10;
    let count = users.len();
    let end_string = format!("#{}:{}", count, offset);

    let mut response = String::new();

    for user in users {
        let temp = User::to_gd(user);
        response.push_str(&temp);

        response.push('|');
    }

    if response.is_empty() {
        return Ok("".to_string());
    }

    response.pop();

    response.push_str(&end_string);
    Ok(response)
}
