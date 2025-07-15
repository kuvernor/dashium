use axum::Form;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::models::User;
use crate::util::is_ascii_alphanumeric;

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterForm {
    userName: String,
    password: String,
    email: String,
    secret: String,
}

pub async fn register(
    State(pool): State<PgPool>,
    Form(form): Form<RegisterForm>,
) -> Result<String, AppError> {
    let username = &form.userName.to_lowercase();
    let password = &form.password;
    let email = &form.email;

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

    User::create(&pool, username, password, email).await?;
    Ok("1".to_string())
}
