use crate::models::User;
use crate::{AppError, GDResponse};
use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct getGJUsers20 {
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

pub async fn getGJUsers20(
    State(pool): State<PgPool>,
    Form(form): Form<getGJUsers20>,
) -> Result<String, AppError> {
    let search_term = &form.str;
    let page = form.page;

    let users: Vec<User> = User::get_by_name(&pool, search_term).await?;

    if users.is_empty() {
        return Ok("-2".to_string());
    }

    let offset = page * 10;
    let count = users.len();
    let end_string = format!("#{count}:{offset}:10");

    let mut response = String::new();

    for user in users {
        response.push_str(&user.to_gd());

        response.push('|');
    }

    response.pop();
    response.push_str(&end_string);

    Ok(response)
}
