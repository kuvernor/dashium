use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::models::Post;
use crate::util::{base64_decode, verify_gjp2};

#[derive(Serialize, Deserialize, Debug)]
pub struct uploadGJAccComment20 {
    accountID: i32,
    comment: String,
    gjp2: String,
    secret: String,
    userName: String,
}

pub async fn uploadGJAccComment20(
    State(pool): State<PgPool>,
    Form(form): Form<uploadGJAccComment20>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let body = &String::from_utf8(base64_decode(&form.comment)?)?;
    let username = &form.userName;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let response = Post::upload(&pool, body, user_id, username).await?;

    Ok(response.to_string())
}
