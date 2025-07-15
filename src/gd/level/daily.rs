use axum::{Form, extract::State};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    AppError,
    util::{
        base64_decode, base64_encode, cyclic_xor, salt_and_sha1, time_until_midnight,
        time_until_monday,
    },
};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetForm {
    #[serde(rename = "type")]
    daily_type: u8,
    secret: String,
    chk: String,
}

pub async fn get_daily(
    State(pool): State<PgPool>,
    Form(form): Form<GetForm>,
) -> Result<String, AppError> {
    let daily_type = form.daily_type;
    let chk = &cyclic_xor(&base64_decode(&form.chk[5..])?, b"59182")?;
    let now = Utc::now();
    let time_left;

    let daily_id = match daily_type {
        0 => {
            time_left = time_until_monday();
            sqlx::query_scalar!(
                r#"
                SELECT id
                FROM daily_levels
                WHERE created_at < $1
                ORDER BY created_at DESC
                LIMIT 1
                "#,
                now
            )
            .fetch_one(&pool)
            .await?
        }
        1 => {
            time_left = time_until_midnight();
            sqlx::query_scalar!(
                r#"
                SELECT id + 100000
                FROM weekly_demons
                WHERE created_at < $1
                ORDER BY created_at DESC
                LIMIT 1
                "#,
                now
            )
            .fetch_one(&pool)
            .await?
            .unwrap_or(0)
        }
        2 => {
            time_left = 10.to_string();
            sqlx::query_scalar!(
                r#"
                SELECT id + 200000
                FROM event_levels
                WHERE created_at < $1
                ORDER BY created_at DESC
                LIMIT 1
                "#,
                now
            )
            .fetch_one(&pool)
            .await?
            .unwrap_or(0)
        }
        _ => return Ok("-1".to_string()),
    };

    let response = if daily_type == 2 {
        let number = daily_id + 19;
        let rewards = "7,500,1001,379";

        let list = format!("EVENT:{chk}:{number}:3:{rewards}");
        let string = base64_encode(&cyclic_xor(list.as_bytes(), b"59182")?);
        let hash = salt_and_sha1(&string, "pC26fpYaQCtg");

        format!("{daily_id}|{time_left}|EVENT{string}|{hash}")
    } else {
        format!("{daily_id}|{time_left}")
    };

    Ok(response)
}
