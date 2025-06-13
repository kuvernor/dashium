use axum::Form;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::models::User;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterForm {
    #[serde(rename = "userName")]
    username: String,
    password: String,
    email: String,
    secret: Option<String>,
}

pub async fn register(
    State(pool): State<PgPool>,
    Form(form): Form<RegisterForm>,
) -> Result<String, AppError> {
    let username = &form.username;
    let password = &form.password;
    let email = &form.email;
    let gjp2 = hash_gjp2(password);

    if username.len() > 20 {
        return Ok(String::from("-4"));
    }

    if !is_ascii_alphanumeric(username) {
        return Ok(String::from("-4"));
    }

    if username.len() < 3 {
        return Ok(String::from("-9"));
    }

    if password.len() < 8 {
        return Ok(String::from("-8"));
    }

    if !is_ascii_alphanumeric(password) {
        return Ok(String::from("-5"));
    }

    if User::is_username_taken(&pool, username).await? {
        return Ok(String::from("-2"));
    }

    if User::is_email_taken(&pool, email).await? {
        return Ok(String::from("-3"));
    }

    match User::register(&pool, username, &gjp2, email).await {
        Ok(_) => return Ok(String::from("1")),
        _ => return Ok(String::from("-1")),
    };
}
