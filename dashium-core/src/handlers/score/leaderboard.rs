use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, GDResponse, models::User, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct getGJScores20 {
    accountID: i32,
    gjp2: String,
    #[serde(rename = "type")]
    leaderboard_type: String,
}

pub async fn getGJScores20(
    State(pool): State<PgPool>,
    Form(form): Form<getGJScores20>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;

    let users: Vec<User> = match form.leaderboard_type.as_str() {
        "top" | "relative" => {
            sqlx::query_as("SELECT * FROM user_view ORDER BY stars DESC")
                .fetch_all(&pool)
                .await?
        }
        "creators" => {
            sqlx::query_as("SELECT * FROM user_view ORDER BY creator_points DESC, stars DESC")
                .fetch_all(&pool)
                .await?
        }
        "friends" => {
            if !verify_gjp2(&pool, user_id, gjp2).await? {
                return Ok("-1".to_string());
            }

            sqlx::query_as(
                r#"
                SELECT user_view.*
                FROM friendships f
                JOIN user_view u ON 
                    (f.user1 = $1 AND u.id = f.user2) OR
                    (f.user2 = $1 AND u.id = f.user1)
                UNION
                SELECT * FROM user_view WHERE id = $1
                "#,
            )
            .bind(user_id)
            .fetch_all(&pool)
            .await?
        }
        _ => return Ok("".to_string()),
    };

    let mut response = String::new();

    for user in users {
        response.push_str(&user.to_gd());
        response.push('|');
    }

    response.pop();

    Ok(response)
}
