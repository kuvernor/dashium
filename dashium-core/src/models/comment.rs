use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use serde::Serialize;
use sqlx::{PgPool, Postgres, QueryBuilder, prelude::FromRow};

use crate::{GDResponse, util::base64_encode};

#[derive(Debug, FromRow, Serialize)]
pub struct Comment {
    pub id: i32,
    pub level_id: i32,
    pub user_id: i32,
    pub username: String,
    pub comment: String,
    pub likes: i32,
    pub spam: bool,
    pub percent: i16,
    pub chat_color: String,
    pub mod_level: i16,
    pub color1: i16,
    pub color2: i16,
    pub glow: i16,
    pub icon_type: i16,
    pub display_icon: i16,
    pub created_at: DateTime<Utc>,
}

impl GDResponse for Comment {
    fn to_gd(&self) -> String {
        let spam = if self.spam { 1 } else { 0 };

        let comment_string = vec![
            format!("1~{}", self.level_id),
            format!("2~{}", base64_encode(&self.comment)),
            format!("3~{}", self.user_id),
            format!("4~{}", self.likes),
            format!("6~{}", self.id),
            format!("7~{spam}"),
            format!("8~{}", self.user_id),
            format!("9~{}", HumanTime::from(self.created_at)).replace(" ago", ""),
            format!("10~{}", self.percent),
            format!("11~{}", self.mod_level),
            format!("12~{}", self.chat_color),
        ];

        let comment_string = comment_string.join("~");

        let user_string = [
            format!("1~{}", self.username),
            format!("9~{}", self.display_icon),
            format!("10~{}", self.color1),
            format!("11~{}", self.color2),
            format!("14~{}", self.icon_type),
            format!("15~{}", self.glow),
            format!("16~{}", self.user_id),
        ];

        let user_string = user_string.join("~");

        format!("{comment_string}:{user_string}")
    }
}

impl Comment {
    pub async fn get_all(pool: &PgPool, level_id: i32, mode: u8) -> Result<Vec<Self>> {
        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT
                c.id,
                c.level_id,
                c.user_id,
                c.username,
                c.comment,
                c.likes,
                c.spam,
                c.created_at,
                c.percent,
                c.chat_color,
                u.mod_level,
                u.display_icon,
                u.color1,
                u.color2,
                u.icon_type,
                u.glow
            FROM comments c
            JOIN users u ON u.id = c.user_id
            WHERE c.level_id = 
            "#,
        );
        query.push_bind(level_id);

        match mode {
            1 => query.push(" ORDER BY c.likes DESC"),
            _ => query.push(" ORDER BY c.created_at DESC"),
        };

        let comments = query.build_query_as().fetch_all(pool).await?;

        Ok(comments)
    }

    pub async fn get_from_user(pool: &PgPool, user_id: i32, mode: u8) -> Result<Vec<Self>> {
        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT
                c.id,
                c.level_id,
                c.user_id,
                c.username,
                c.comment,
                c.likes,
                c.spam,
                c.created_at,
                c.percent,
                c.chat_color,
                u.mod_level,
                u.display_icon,
                u.color1,
                u.color2,
                u.icon_type,
                u.glow
            FROM comments c
            JOIN users u ON u.id = c.user_id
            WHERE c.user_id = 
            "#,
        );
        query.push_bind(user_id);

        match mode {
            1 => query.push(" ORDER BY c.likes DESC"),
            _ => query.push(" ORDER BY c.created_at DESC"),
        };

        let comments = query.build_query_as().fetch_all(pool).await?;

        Ok(comments)
    }

    pub async fn upload(
        pool: &PgPool,
        user_id: i32,
        username: &str,
        level_id: i32,
        comment: &str,
        percent: i16,
    ) -> Result<i32> {
        let comment_id = sqlx::query_scalar!(
            "INSERT INTO comments (user_id, username, level_id, comment, percent) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            user_id,
            username,
            level_id,
            comment,
            percent
        )
        .fetch_one(pool)
        .await?;

        Ok(comment_id)
    }

    pub async fn delete(pool: &PgPool, user_id: i32, level_id: i32, comment_id: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM comments WHERE user_id = $1 AND level_id = $2 AND id = $3",
            user_id,
            level_id,
            comment_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
