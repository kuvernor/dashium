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
    accountID: i32,
    gameVersion: i16,
    binaryVersion: i16,
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

    let users: Vec<User> = User::get_users(&pool, search_term).await?;

    if users.is_empty() {
        return Ok("-2".to_string());
    }

    let offset = page * 10;
    let count = users.len();
    let end_string = format!("#{count}:{offset}:10");

    let mut response = String::new();

    for user in users {
        let temp = User::to_gd(user);
        response.push_str(&temp);

        response.push('|');
    }

    response.pop();
    response.push_str(&end_string);

    Ok(response)
}
