use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::User, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct uploadGJLevelList {
    accountID: i32,
    difficulty: i16,
    listDesc: String,
    listID: i32,
    listLevels: String,
    listName: String,
    listVersion: String,
    original: i32,
    unlisted: i16,
    gjp2: String,
    secret: String,
}

pub async fn uploadGJLevelList(
    State(pool): State<PgPool>,
    Form(form): Form<uploadGJLevelList>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let list_name = &form.listName;
    let description = &form.listDesc;
    let original = form.original;
    let unlisted = form.unlisted;
    let difficulty = form.difficulty;
    let levels = &form.listLevels;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let username = &User::username_from_id(&pool, user_id).await?;

    let list_id = sqlx::query_scalar!(
        r#"
        INSERT INTO lists (
            user_id,
            username,
            list_name,
            description,
            unlisted,
            original,
            difficulty,
            levels
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
        user_id,
        username,
        list_name,
        description,
        unlisted,
        original,
        difficulty,
        levels
    )
    .fetch_one(&pool)
    .await?;

    Ok(list_id.to_string())
}
