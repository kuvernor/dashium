use crate::AppError;
use crate::util::verify_gjp2;
use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct updateGJAccSettings20 {
    accountID: i32,
    gjp2: String,
    mS: i16,
    frS: i16,
    cS: i16,
    yt: String,
    twitter: String,
    twitch: String,
    secret: String,
}

pub async fn updateGJAccSettings20(
    State(pool): State<PgPool>,
    Form(form): Form<updateGJAccSettings20>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let message_setting = form.mS;
    let friend_setting = form.frS;
    let comment_setting = form.cS;
    let youtube = &form.yt;
    let twitter = &form.twitter;
    let twitch = &form.twitch;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    sqlx::query!(
        r#"
        UPDATE users SET 
        message_setting = $1,
        friend_setting = $2,
        comment_setting = $3,
        youtube = $4,
        twitter = $5,
        twitch = $6
        WHERE id = $7
        "#,
        message_setting,
        friend_setting,
        comment_setting,
        youtube,
        twitter,
        twitch,
        user_id
    )
    .execute(&pool)
    .await?;

    Ok("1".to_string())
}
