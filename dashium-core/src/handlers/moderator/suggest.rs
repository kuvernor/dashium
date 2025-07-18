use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct suggestGJStars20 {
    accountID: i32,
    gjp2: String,
    feature: i16,
    levelID: i32,
    stars: i16,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
    udid: String,
    uuid: String,
}

pub async fn suggestGJStars20(
    State(pool): State<PgPool>,
    Form(form): Form<suggestGJStars20>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let level_id = form.levelID;
    let feature = form.feature;
    let stars = form.stars;

    let feature = match feature {
        0 => "Star",
        1 => "Feature",
        2 => "Epic",
        3 => "Legendary",
        4 => "Mythic",
        _ => return Ok("-1".to_string()),
    };

    let mod_level = sqlx::query_scalar!("SELECT mod_level FROM users WHERE id = $1", user_id)
        .fetch_one(&pool)
        .await?;

    if mod_level != 1 && mod_level != 2 {
        return Ok("-1".to_string());
    }

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    sqlx::query!(
        "INSERT INTO suggestions (user_id, level_id, feature, stars) VALUES ($1, $2, $3, $4)",
        user_id,
        level_id,
        feature,
        stars
    )
    .execute(&pool)
    .await?;

    Ok("1".to_string())
}
