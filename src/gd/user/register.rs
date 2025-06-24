use axum::Form;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, info};

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

    if username.len() > 20 {
        debug!("{username} failed to register: username too long");
        return Ok(String::from("-4"));
    }

    if !is_ascii_alphanumeric(username) {
        debug!("{username} failed to register: invalid username");
        return Ok(String::from("-4"));
    }

    if username.len() < 3 {
        debug!("{username} failed to register: username too short");
        return Ok(String::from("-9"));
    }

    if password.len() < 8 {
        debug!("{username} failed to register: password too short");
        return Ok(String::from("-8"));
    }

    if !is_ascii_alphanumeric(password) {
        debug!("{username} failed to register: password has invalid characters");
        return Ok(String::from("-5"));
    }

    if User::is_username_taken(&pool, username).await? {
        debug!("{username} failed to register: username is taken");
        return Ok(String::from("-2"));
    }

    if User::is_email_taken(&pool, email).await? {
        debug!("{username} failed to register: email is taken");
        return Ok(String::from("-3"));
    }

    match User::register(&pool, username, password, email).await {
        Ok(_) => {
            info!("{username} registered succesfully!");
            return Ok("1".to_string());
        }
        Err(_) => {
            debug!("{username} failed to register: server error");
            return Ok("-1".to_string());
        }
    };
}
