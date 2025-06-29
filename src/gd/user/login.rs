use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, info};

use crate::AppError;
use crate::models::User;
use crate::util::verify_gjp2;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginForm {
    #[serde(rename = "userName")]
    username: String,
    gjp2: String,

    #[serde(default)]
    #[serde(rename = "sID")]
    steam_id: String,

    secret: String,
    udid: String,
}

pub async fn login(
    State(pool): State<PgPool>,
    Form(form): Form<LoginForm>,
) -> Result<String, AppError> {
    let username = &form.username;
    let gjp2 = &form.gjp2;

    let user_id = User::get_user_id(&pool, username).await?;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        debug!("{username} failed to login: incorrect username or password");
        return Ok(String::from("-11"));
    }

    info!("{username} logged in succesfully!");
    return Ok(format!("{user_id},{user_id}"));
}
