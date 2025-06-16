use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, info};

use crate::AppError;
use crate::models::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginForm {
    #[serde(rename = "userName")]
    username: String,
    gjp2: String,
    #[serde(rename = "sID")]
    steam_id: Option<String>,
    secret: Option<String>,
    udid: Option<String>,
}

pub async fn login(
    State(pool): State<PgPool>,
    Form(form): Form<LoginForm>,
) -> Result<String, AppError> {
    let username = &form.username;
    let gjp2 = &form.gjp2;
    let user_id = User::get_user_id(&pool, username).await?;
    let verified = User::verify_password(&pool, user_id, gjp2).await?;

    if !verified {
        debug!("{username} failed to login: incorrect username or password");
        return Ok(String::from("-11"));
    }

    info!("{username} logged in succesfully!");
    return Ok(format!("{user_id},{user_id}"));
}
