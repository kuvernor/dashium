use axum::{Form, extract::State};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, util::time_until_midnight};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetForm {
    #[serde(rename = "type")]
    daily_type: u8,
    secret: String,
}

pub async fn get_daily(
    State(pool): State<PgPool>,
    Form(_form): Form<GetForm>,
) -> Result<String, AppError> {
    let now = Utc::now();

    let daily_id = sqlx::query_scalar!(
        "SELECT id FROM daily_levels WHERE created_at < $1 ORDER BY created_at DESC LIMIT 1",
        now
    )
    .fetch_one(&pool)
    .await?;

    let time_left = time_until_midnight();

    let response = format!("{daily_id}|{time_left}");
    Ok(response)
}
