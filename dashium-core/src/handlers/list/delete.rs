use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::List, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct deleteGJLevelList {
    accountID: i32,
    listID: i32,
    gjp2: String,
    secret: String,
}

pub async fn deleteGJLevelList(
    State(pool): State<PgPool>,
    Form(form): Form<deleteGJLevelList>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let list_id = form.listID;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    List::delete(&pool, user_id, list_id).await?;
    Ok("1".to_string())
}
