use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::models::User;
use crate::util::verify_gjp2;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginForm {
    userName: String,
    gjp2: String,

    #[serde(default)]
    sID: String,

    secret: String,
    udid: String,
}

pub async fn login(
    State(pool): State<PgPool>,
    Form(form): Form<LoginForm>,
) -> Result<String, AppError> {
    let username = &form.userName.to_lowercase();
    let gjp2 = &form.gjp2;

    let user_id = User::id_from_username(&pool, username).await?;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok(String::from("-11"));
    }

    Ok(format!("{user_id},{user_id}"))
}
