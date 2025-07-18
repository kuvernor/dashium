use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::User, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct syncGJAccountNew {
    accountID: i32,
    gjp2: String,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
    udid: String,
    uuid: String,
}

pub async fn syncGJAccountNew(
    State(pool): State<PgPool>,
    Form(form): Form<syncGJAccountNew>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let save_data = User::load_data(&pool, user_id).await?;
    Ok(format!("{save_data};21;30;a;a"))
}
