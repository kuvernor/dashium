use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use sqlx::{PgPool, prelude::FromRow};

use crate::models::User;

#[derive(Debug, FromRow)]
pub struct Comment {
    comment_id: i32,
    level_id: i32,
    user_id: i32,
    username: String,
    comment: String,
    likes: i32,
    is_spam: i16,
    created_at: DateTime<Utc>,
    percent: i16,
    chat_color: String,
}

impl Comment {
    pub async fn to_gd(pool: &PgPool, comment: &Self, include_level_id: bool) -> Result<String> {
        let user = User::get_user(pool, comment.user_id).await?;
        let mut comment_string = vec![
            format!("2~{}", comment.comment),
            format!("3~{}", comment.user_id),
            format!("4~{}", comment.likes),
            format!("6~{}", comment.comment_id),
            format!("7~{}", comment.is_spam),
            format!("8~{}", comment.user_id),
            format!("9~{}", HumanTime::from(comment.created_at)).replace(" ago", ""),
            format!("10~{}", comment.percent),
            format!("11~{}", user.mod_level),
            format!("12~{}", comment.chat_color),
        ];

        if include_level_id {
            comment_string.insert(0, format!("1~{}", comment.level_id));
        }

        let comment_string = comment_string.join("~");

        let user_string = [
            format!("1~{}", comment.username),
            format!("9~{}", user.icon),
            format!("10~{}", user.color1),
            format!("11~{}", user.color2),
            format!("14~{}", user.icon_type),
            format!("15~{}", user.glow),
            format!("16~{}", comment.user_id),
        ];

        let user_string = user_string.join("~");
        let response = format!("{comment_string}:{user_string}");

        Ok(response)
    }

    pub async fn get_all(pool: &PgPool, level_id: i32, mode: u8) -> Result<Vec<Self>> {
        let comments = match mode {
            1 => {
                sqlx::query_as!(
                    Self,
                    "SELECT * FROM comments WHERE level_id = $1 ORDER BY likes DESC",
                    level_id
                )
                .fetch_all(pool)
                .await?
            }
            _ => {
                sqlx::query_as!(
                    Self,
                    "SELECT * FROM comments WHERE level_id = $1 ORDER BY created_at DESC",
                    level_id
                )
                .fetch_all(pool)
                .await?
            }
        };

        Ok(comments)
    }

    pub async fn get_from_user(pool: &PgPool, user_id: i32, mode: u8) -> Result<Vec<Self>> {
        let comments = match mode {
            1 => {
                sqlx::query_as!(
                    Self,
                    "SELECT * FROM comments WHERE user_id = $1 ORDER BY likes DESC",
                    user_id
                )
                .fetch_all(pool)
                .await?
            }
            _ => {
                sqlx::query_as!(
                    Self,
                    "SELECT * FROM comments WHERE user_id = $1 ORDER BY created_at DESC",
                    user_id
                )
                .fetch_all(pool)
                .await?
            }
        };

        Ok(comments)
    }

    pub async fn upload(
        pool: &PgPool,
        user_id: i32,
        username: &str,
        level_id: i32,
        comment: &str,
    ) -> Result<i32> {
        let response = sqlx::query_scalar!(
            "INSERT INTO comments (user_id, username, level_id, comment) VALUES ($1, $2, $3, $4) RETURNING comment_id",
            user_id,
            username,
            level_id,
            comment
        )
        .fetch_one(pool)
        .await?;

        Ok(response)
    }

    pub async fn delete(pool: &PgPool, user_id: i32, level_id: i32, comment_id: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM comments WHERE user_id = $1 AND level_id = $2 AND comment_id = $3",
            user_id,
            level_id,
            comment_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
