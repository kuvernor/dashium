use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use sqlx::{PgPool, prelude::FromRow};

use crate::{GDResponse, util::base64_encode};

#[derive(Debug, FromRow)]
#[allow(unused)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub body: String,
    pub likes: i32,
    pub is_spam: i16,
    pub created_at: DateTime<Utc>,
}

impl GDResponse for Post {
    fn to_gd(&self) -> String {
        let response: Vec<String> = vec![
            format!("2~{}", base64_encode(&self.body)),
            format!("3~{}", self.user_id),
            format!("4~{}", self.likes),
            format!("6~{}", self.id),
            format!("7~{}", self.is_spam),
            format!("8~{}", self.user_id),
            format!("9~{}", HumanTime::from(self.created_at)).replace(" ago", ""),
        ];
        response.join("~")
    }
}

impl Post {
    pub async fn get_all(pool: &PgPool, user_id: i32, username: &str) -> Result<Vec<Self>> {
        let posts = sqlx::query_as!(
            Self,
            "SELECT * from posts WHERE user_id = $1 AND username = $2 ORDER BY created_at DESC",
            user_id,
            username
        )
        .fetch_all(pool)
        .await?;

        Ok(posts)
    }

    pub async fn upload(pool: &PgPool, body: &str, user_id: i32, username: &str) -> Result<i32> {
        let post_id = sqlx::query_scalar!(
            "INSERT INTO posts (body, user_id, username) VALUES ($1, $2, $3) RETURNING id",
            body,
            user_id,
            username
        )
        .fetch_one(pool)
        .await?;

        Ok(post_id)
    }

    pub async fn delete(pool: &PgPool, post_id: i32, user_id: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM posts WHERE id = $1 and user_id = $2",
            post_id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
