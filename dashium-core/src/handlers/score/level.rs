use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, GDResponse, models::User, util::verify_gjp2};

#[derive(Serialize, Deserialize)]
pub struct getGJLevelScores211 {
    accountID: i32,
    gjp2: String,
    levelID: i32,
    percent: i16,
    s1: i32,
    s2: i32,
    s3: i32,
    s6: String,
    s9: i16,
    #[serde(rename = "type")]
    leaderboard_type: u8,
}

pub async fn getGJLevelScores211(
    State(pool): State<PgPool>,
    Form(form): Form<getGJLevelScores211>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let level_id = form.levelID;
    let attempts = form.s1 - 8354;
    let clicks = form.s2 - 3991;
    let time = form.s3 - 4085;
    let progresses = &form.s6;
    let coins = form.s9 - 5819;
    let percent = form.percent;
    let leaderboard_type = form.leaderboard_type;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let all = sqlx::query!(
        "SELECT * FROM level_scores WHERE user_id = $1 AND level_id = $2",
        user_id,
        level_id
    )
    .fetch_all(&pool)
    .await?;

    if all.is_empty() {
        sqlx::query!(
            r#"
            INSERT INTO level_scores (
                user_id,
                level_id,
                attempts,
                clicks,
                coins,
                progresses,
                time,
                percent
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8
            )
            "#,
            user_id,
            level_id,
            attempts,
            clicks,
            coins,
            progresses,
            time,
            percent
        )
        .execute(&pool)
        .await?;
    } else {
        sqlx::query!(
            r#"
            UPDATE level_scores SET
                attempts = $1,
                clicks = $2,
                coins = $3,
                progresses = $4,
                time = $5,
                percent = $6
            "#,
            attempts,
            clicks,
            coins,
            progresses,
            time,
            percent
        )
        .execute(&pool)
        .await?;
    }

    let users: Vec<User> = match leaderboard_type {
        1 => {
            sqlx::query_as(
                r#"
                SELECT user_view.*
                FROM user_view u
                JOIN level_scores l ON u.id = l.user_id
                WHERE l.level_id = $1
                ORDER BY l.percent DESC
                "#,
            )
            .bind(level_id)
            .fetch_all(&pool)
            .await?
        }

        2 => {
            sqlx::query_as(
                r#"
                SELECT user_view.*
                FROM user_view u
                JOIN level_scores l ON u.id = l.user_id
                WHERE l.level_id = $1
                AND l.created_at >= NOW() - INTERVAL '14 days'
                ORDER BY l.percent DESC
                "#,
            )
            .bind(level_id)
            .fetch_all(&pool)
            .await?
        }
        _ => {
            let friend_ids = sqlx::query_scalar!(
                "SELECT id FROM friendships WHERE user1 = $1 OR user2 = $1 ORDER BY created_at DESC",
                user_id
            )
            .fetch_all(&pool)
            .await?;

            sqlx::query_as(
                r#"
                SELECT user_view.*
                FROM user_view u
                JOIN level_scores l ON u.id = l.user_id
                WHERE l.level_id = $1
                AND l.user_id = ANY($2)
                ORDER BY l.percent DESC
                "#,
            )
            .bind(level_id)
            .bind(friend_ids)
            .fetch_all(&pool)
            .await?
        }
    };

    if users.is_empty() {
        return Ok("".to_string());
    }

    let mut response = String::new();

    for user in users {
        response.push_str(&user.to_gd());
        response.push('|');
    }

    response.pop();

    Ok(response)
}
